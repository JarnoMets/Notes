//! Note route handlers

use actix_web::{web, HttpRequest, Responder};

use crate::db::DbError;
use crate::models::{
    AppState, CreateNoteRequest, MoveNoteRequest, Note, UpdateNoteRequest, NoteRevision,
};
use diffy::create_patch;
use crate::require_auth;

use super::response::{
    bad_request, created, internal_error_logged, no_content, not_found, ok,
};

// ============ Notes ============

pub async fn get_notes(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    match state.db.get_notes(&user_id).await {
        Ok(notes) => ok(notes),
        Err(e) => internal_error_logged("Failed to get notes", e),
    }
}

pub async fn get_notes_tree(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    // Check cache first
    if let Some(tree) = state.cache.get_notes_tree(&user_id).await {
        return ok(tree);
    }

    match state.db.get_notes_tree(&user_id).await {
        Ok(tree) => {
            // Cache the result
            let _ = state.cache.set_notes_tree(user_id.clone(), tree.clone()).await;
            ok(tree)
        }
        Err(e) => internal_error_logged("Failed to get notes tree", e),
    }
}

pub async fn get_note(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.get_note_with_attachments(&id, &user_id).await {
        Ok(note) => ok(note),
        Err(DbError::NotFound) => not_found("Note"),
        Err(e) => internal_error_logged("Failed to get note", e),
    }
}

pub async fn create_note(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateNoteRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);

    let position = match state
        .db
        .get_max_note_position(&user_id, body.folder_id.as_deref())
        .await
    {
        Ok(pos) => pos,
        Err(e) => return internal_error_logged("Failed to get max position", e),
    };

    let note = Note::new(
        user_id.clone(),
        body.folder_id.clone(),
        body.title.clone(),
        body.description.clone().unwrap_or_default(),
        body.content.clone().unwrap_or_default(),
        position,
    );

    match state.db.create_note(&note).await {
        Ok(note) => {
            // create initial diff-based revision and set as current (idx 0)
            let forward = create_patch("", &note.content).to_string();
            let reverse = create_patch(&note.content, "").to_string();
            let rev = NoteRevision::new(note.id.clone(), user_id.clone(), 0, forward, reverse);
            if let Err(e) = state.db.create_note_revision(&rev).await {
                log::error!("Failed to create initial revision: {}", e);
            } else {
                let _ = state.db.set_note_current_revision_index(&note.id, 0, &user_id).await;
            }
            // Invalidate cache
            let _ = state.cache.invalidate_notes_tree(&user_id).await;
            created(note)
        }
        Err(e) => internal_error_logged("Failed to create note", e),
    }
}

pub async fn update_note(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateNoteRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();
    let folder_id = body.folder_id.clone().map(Some);

    // Fetch existing note to compute diff
    let existing = match state.db.get_note(&id, &user_id).await {
        Ok(n) => n,
        Err(DbError::NotFound) => return not_found("Note"),
        Err(e) => return internal_error_logged("Failed to fetch existing note", e),
    };

    match state
        .db
        .update_note(
            &id,
            &user_id,
            body.title.clone(),
            folder_id,
            body.description.clone(),
            body.content.clone(),
            body.position,
            body.is_important,
            body.is_urgent,
        )
        .await
    {
        Ok(note) => {
            // Compute diff between existing and updated content and store as a new revision
            let forward_patch = create_patch(&existing.content, &note.content).to_string();
            let reverse_patch = create_patch(&note.content, &existing.content).to_string();

            // Determine current index and prune any future revisions (invalidate redo stack)
            let current_idx = match state.db.get_note_current_revision_index(&note.id, &user_id).await {
                Ok(i) => i,
                Err(_) => -1,
            };
            if let Err(e) = state.db.delete_revisions_after(&note.id, current_idx).await {
                log::error!("Failed to prune future revisions: {}", e);
            }

            let next_idx = current_idx + 1;
            let rev = NoteRevision::new(note.id.clone(), user_id.clone(), next_idx, forward_patch, reverse_patch);
            if let Err(e) = state.db.create_note_revision(&rev).await {
                log::error!("Failed to create revision for update: {}", e);
            } else {
                let _ = state.db.set_note_current_revision_index(&note.id, next_idx, &user_id).await;
            }
            // Invalidate cache
            let _ = state.cache.invalidate_notes_tree(&user_id).await;
            ok(note)
        }
        Err(DbError::NotFound) => not_found("Note"),
        Err(e) => internal_error_logged("Failed to update note", e),
    }
}

pub async fn list_revisions(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.get_note_revisions(&id, &user_id).await {
        Ok(revs) => ok(revs),
        Err(DbError::NotFound) => not_found("Revisions"),
        Err(e) => internal_error_logged("Failed to get revisions", e),
    }
}

pub async fn undo_revision(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let note_id = path.into_inner();
    // Get current revision index
    match state.db.get_note_current_revision_index(&note_id, &user_id).await {
        Ok(current_idx) => {
            if current_idx <= 0 {
                return bad_request("No older revision available");
            }
            let target_idx = current_idx - 1;
            match state.db.get_revision_by_idx(&note_id, target_idx, &user_id).await {
                Ok(rev) => match state.db.set_note_current_revision(&note_id, &rev.id, &user_id).await {
                    Ok(()) => ok("ok"),
                    Err(e) => internal_error_logged("Failed to set revision", e),
                },
                Err(_) => bad_request("No older revision found"),
            }
        }
        Err(e) => internal_error_logged("Failed to fetch current revision index", e),
    }
}

pub async fn redo_revision(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
 ) -> impl Responder {
    let user_id = require_auth!(req, state);
    let note_id = path.into_inner();
    // Decide target based on current index and max idx
    match state.db.get_note_current_revision_index(&note_id, &user_id).await {
        Ok(current_idx) => {
            match sqlx::query_scalar::<_, i32>("SELECT COALESCE(MAX(idx), -1) FROM note_revisions WHERE note_id = $1 AND user_id = $2").bind(&note_id).bind(&user_id).fetch_one(&state.db.pool).await {
                Ok(max_idx) => {
                    if current_idx >= max_idx {
                        return bad_request("No newer revision available");
                    }
                    let target_idx = current_idx + 1;
                    match state.db.get_revision_by_idx(&note_id, target_idx, &user_id).await {
                        Ok(rev) => match state.db.set_note_current_revision(&note_id, &rev.id, &user_id).await {
                            Ok(()) => ok("ok"),
                            Err(e) => internal_error_logged("Failed to set revision", e),
                        },
                        Err(_) => bad_request("No newer revision found"),
                    }
                }
                Err(e) => internal_error_logged("Failed to get max revision index", e),
            }
        }
        Err(e) => internal_error_logged("Failed to fetch current revision index", e),
    }
}

pub async fn move_note(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<MoveNoteRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state
        .db
        .update_note(
            &id,
            &user_id,
            None,
            Some(body.folder_id.clone()),
            None,
            None,
            Some(body.position),
            None,
            None,
        )
        .await
    {
        Ok(note) => ok(note),
        Err(DbError::NotFound) => not_found("Note"),
        Err(e) => internal_error_logged("Failed to move note", e),
    }
}

pub async fn delete_note(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.delete_note(&id, &user_id).await {
        Ok(()) => {
            // Invalidate cache
            let _ = state.cache.invalidate_notes_tree(&user_id).await;
            no_content()
        }
        Err(DbError::NotFound) => not_found("Note"),
        Err(e) => internal_error_logged("Failed to delete note", e),
    }
}
