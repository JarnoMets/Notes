//! Label route handlers

use actix_web::{web, HttpRequest, Responder};

use crate::db::DbError;
use crate::models::{AppState, BoardLabel, CreateLabelRequest, UpdateLabelRequest};
use crate::require_auth;

use super::response::{created, internal_error_logged, no_content, not_found, ok};

/// Verify that the user owns the board, returning the result
async fn verify_board_ownership(
    state: &web::Data<AppState>,
    board_id: &str,
    user_id: &str,
) -> Result<(), actix_web::HttpResponse> {
    match state.db.get_board(board_id, user_id).await {
        Ok(_) => Ok(()),
        Err(DbError::NotFound) => Err(not_found("Board")),
        Err(e) => Err(internal_error_logged("Failed to verify board ownership", e)),
    }
}

pub async fn get_labels(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let board_id = path.into_inner();

    if let Err(resp) = verify_board_ownership(&state, &board_id, &user_id).await {
        return resp;
    }

    match state.db.get_labels(&board_id).await {
        Ok(labels) => ok(labels),
        Err(e) => internal_error_logged("Failed to get labels", e),
    }
}

pub async fn create_label(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<CreateLabelRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let board_id = path.into_inner();

    if let Err(resp) = verify_board_ownership(&state, &board_id, &user_id).await {
        return resp;
    }

    let label = BoardLabel::new(board_id, body.name.clone(), body.color.clone());

    match state.db.create_label(&label).await {
        Ok(label) => created(label),
        Err(e) => internal_error_logged("Failed to create label", e),
    }
}

pub async fn update_label(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateLabelRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    // Get label to verify board ownership
    let label = match state.db.get_label(&id).await {
        Ok(l) => l,
        Err(DbError::NotFound) => return not_found("Label"),
        Err(e) => return internal_error_logged("Failed to get label", e),
    };

    if let Err(resp) = verify_board_ownership(&state, &label.board_id, &user_id).await {
        return resp;
    }

    match state
        .db
        .update_label(&id, body.name.clone(), body.color.clone())
        .await
    {
        Ok(label) => ok(label),
        Err(DbError::NotFound) => not_found("Label"),
        Err(e) => internal_error_logged("Failed to update label", e),
    }
}

pub async fn delete_label(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    // Get label to verify board ownership
    let label = match state.db.get_label(&id).await {
        Ok(l) => l,
        Err(DbError::NotFound) => return not_found("Label"),
        Err(e) => return internal_error_logged("Failed to get label", e),
    };

    if let Err(resp) = verify_board_ownership(&state, &label.board_id, &user_id).await {
        return resp;
    }

    match state.db.delete_label(&id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Label"),
        Err(e) => internal_error_logged("Failed to delete label", e),
    }
}
