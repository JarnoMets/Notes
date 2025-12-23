//! Note folders route handlers

use actix_web::{web, HttpRequest, Responder};

use crate::db::DbError;
use crate::models::{AppState, CreateFolderRequest, MoveFolderRequest, UpdateFolderRequest};
use crate::require_auth;

use super::response::*;

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

    let folder = crate::models::NoteFolder::new(user_id, body.parent_id.clone(), body.name.clone(), position);

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