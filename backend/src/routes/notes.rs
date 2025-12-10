//! Note route handlers

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use futures_util::StreamExt;
use std::io::Write;
use std::path::PathBuf;

use crate::db::DbError;
use crate::models::{
    AppState, CreateFolderRequest, CreateNoteRequest, MoveFolderRequest, MoveNoteRequest, Note,
    NoteAttachment, NoteFolder, UpdateFolderRequest, UpdateNoteRequest,
};
use crate::{require_auth, require_auth_or_query};

use super::response::{
    bad_request, created, internal_error_logged, no_content, not_found, ok,
};

const UPLOAD_DIR: &str = "./uploads";
const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50MB

fn get_upload_dir() -> PathBuf {
    let dir = PathBuf::from(UPLOAD_DIR);
    if !dir.exists() {
        std::fs::create_dir_all(&dir).expect("Failed to create upload directory");
    }
    dir
}

// ============ Folders ============

pub async fn get_folders(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    match state.db.get_folders(&user_id).await {
        Ok(folders) => ok(folders),
        Err(e) => internal_error_logged("Failed to get folders", e),
    }
}

pub async fn create_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateFolderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);

    let position = match state
        .db
        .get_max_folder_position(&user_id, body.parent_id.as_deref())
        .await
    {
        Ok(pos) => pos,
        Err(e) => return internal_error_logged("Failed to get max position", e),
    };

    let folder = NoteFolder::new(user_id, body.parent_id.clone(), body.name.clone(), position);

    match state.db.create_folder(&folder).await {
        Ok(folder) => created(folder),
        Err(e) => internal_error_logged("Failed to create folder", e),
    }
}

pub async fn update_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateFolderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();
    let parent_id = body.parent_id.clone().map(Some);

    match state
        .db
        .update_folder(&id, &user_id, body.name.clone(), parent_id, body.position, body.is_important, body.is_urgent)
        .await
    {
        Ok(folder) => ok(folder),
        Err(DbError::NotFound) => not_found("Folder"),
        Err(e) => internal_error_logged("Failed to update folder", e),
    }
}

pub async fn move_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<MoveFolderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state
        .db
        .update_folder(
            &id,
            &user_id,
            None,
            Some(body.parent_id.clone()),
            Some(body.position),
            None,
            None,
        )
        .await
    {
        Ok(folder) => ok(folder),
        Err(DbError::NotFound) => not_found("Folder"),
        Err(e) => internal_error_logged("Failed to move folder", e),
    }
}

pub async fn delete_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.delete_folder(&id, &user_id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Folder"),
        Err(e) => internal_error_logged("Failed to delete folder", e),
    }
}

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

    match state.db.get_notes_tree(&user_id).await {
        Ok(tree) => ok(tree),
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
        user_id,
        body.folder_id.clone(),
        body.title.clone(),
        body.description.clone().unwrap_or_default(),
        body.content.clone().unwrap_or_default(),
        position,
    );

    match state.db.create_note(&note).await {
        Ok(note) => created(note),
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
        Ok(note) => ok(note),
        Err(DbError::NotFound) => not_found("Note"),
        Err(e) => internal_error_logged("Failed to update note", e),
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
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Note"),
        Err(e) => internal_error_logged("Failed to delete note", e),
    }
}

// ============ Attachments ============

pub async fn upload_attachment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    mut payload: Multipart,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let note_id = path.into_inner();

    // Verify note ownership
    match state.db.verify_note_ownership(&note_id, &user_id).await {
        Ok(true) => {}
        Ok(false) => return not_found("Note"),
        Err(e) => return internal_error_logged("Failed to verify note ownership", e),
    }

    let upload_dir = get_upload_dir();
    let mut attachments = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                log::error!("Failed to read multipart field: {}", e);
                continue;
            }
        };

        let content_disposition = field.content_disposition();
        let original_filename = content_disposition
            .get_filename()
            .unwrap_or("unnamed")
            .to_string();

        let content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Generate unique filename
        let file_ext = original_filename
            .rsplit('.')
            .next()
            .map(|s| format!(".{}", s))
            .unwrap_or_default();
        let stored_filename = format!("{}_{}{}", note_id, uuid::Uuid::new_v4(), file_ext);
        let file_path = upload_dir.join(&stored_filename);

        // Read and write file
        let mut file = match std::fs::File::create(&file_path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Failed to create file: {}", e);
                return internal_error_logged("Failed to save file", e);
            }
        };

        let mut total_size = 0usize;
        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(d) => d,
                Err(e) => {
                    log::error!("Failed to read chunk: {}", e);
                    let _ = std::fs::remove_file(&file_path);
                    return internal_error_logged("Failed to read file data", e);
                }
            };

            total_size += data.len();
            if total_size > MAX_FILE_SIZE {
                let _ = std::fs::remove_file(&file_path);
                return bad_request("File too large (max 50MB)");
            }

            if let Err(e) = file.write_all(&data) {
                log::error!("Failed to write chunk: {}", e);
                let _ = std::fs::remove_file(&file_path);
                return internal_error_logged("Failed to save file", e);
            }
        }

        let attachment = NoteAttachment::new(
            note_id.clone(),
            stored_filename,
            original_filename,
            content_type,
            total_size as i64,
        );

        match state.db.create_note_attachment(&attachment).await {
            Ok(att) => attachments.push(att),
            Err(e) => {
                log::error!("Failed to save attachment metadata: {}", e);
                let _ = std::fs::remove_file(&file_path);
                return internal_error_logged("Failed to save attachment", e);
            }
        }
    }

    created(attachments)
}

pub async fn download_attachment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    // Use the extended auth check that also supports query param tokens (for image loading)
    let user_id = require_auth_or_query!(req, state);
    let attachment_id = path.into_inner();

    let attachment = match state.db.get_note_attachment(&attachment_id).await {
        Ok(a) => a,
        Err(DbError::NotFound) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to get attachment", e),
    };

    // Verify note ownership
    match state
        .db
        .verify_note_ownership(&attachment.note_id, &user_id)
        .await
    {
        Ok(true) => {}
        Ok(false) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to verify note ownership", e),
    }

    let file_path = get_upload_dir().join(&attachment.filename);

    match std::fs::read(&file_path) {
        Ok(data) => HttpResponse::Ok()
            .content_type(attachment.mime_type)
            .insert_header((
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", attachment.original_filename),
            ))
            .body(data),
        Err(e) => {
            log::error!("Failed to read file: {}", e);
            not_found("File")
        }
    }
}

pub async fn delete_attachment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let attachment_id = path.into_inner();

    let attachment = match state.db.get_note_attachment(&attachment_id).await {
        Ok(a) => a,
        Err(DbError::NotFound) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to get attachment", e),
    };

    // Verify note ownership
    match state
        .db
        .verify_note_ownership(&attachment.note_id, &user_id)
        .await
    {
        Ok(true) => {}
        Ok(false) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to verify note ownership", e),
    }

    // Delete from database
    match state.db.delete_note_attachment(&attachment_id).await {
        Ok(att) => {
            // Delete file from disk
            let file_path = get_upload_dir().join(&att.filename);
            let _ = std::fs::remove_file(&file_path);
            no_content()
        }
        Err(DbError::NotFound) => not_found("Attachment"),
        Err(e) => internal_error_logged("Failed to delete attachment", e),
    }
}
