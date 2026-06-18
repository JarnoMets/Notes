//! List route handlers

use actix_web::{web, HttpRequest, Responder};

use crate::db::DbError;
use crate::models::{AppState, CreateListRequest, List, ReorderListsRequest, UpdateListRequest};
use crate::require_auth;

use super::response::{created, internal_error_logged, no_content, not_found, ok, ok_status};

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

async fn verify_list_ownership(
    state: &web::Data<AppState>,
    list_id: &str,
    user_id: &str,
) -> Result<(), actix_web::HttpResponse> {
    let list = match state.db.get_list(list_id).await {
        Ok(list) => list,
        Err(DbError::NotFound) => return Err(not_found("List")),
        Err(e) => return Err(internal_error_logged("Failed to get list", e)),
    };

    verify_board_ownership(state, &list.board_id, user_id).await
}

pub async fn get_lists(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let board_id = path.into_inner();

    if let Err(resp) = verify_board_ownership(&state, &board_id, &user_id).await {
        return resp;
    }

    match state.db.get_lists(&board_id).await {
        Ok(lists) => ok(lists),
        Err(e) => internal_error_logged("Failed to get lists", e),
    }
}

pub async fn create_list(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<CreateListRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let board_id = path.into_inner();

    if let Err(resp) = verify_board_ownership(&state, &board_id, &user_id).await {
        return resp;
    }

    let position = match body.position {
        Some(p) => p,
        None => match state.db.get_next_list_position(&board_id).await {
            Ok(p) => p,
            Err(e) => return internal_error_logged("Failed to get next position", e),
        },
    };

    let list = List::new(board_id, body.name.clone(), position);

    match state.db.create_list(&list).await {
        Ok(list) => created(list),
        Err(e) => internal_error_logged("Failed to create list", e),
    }
}

pub async fn update_list(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateListRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    if let Err(resp) = verify_list_ownership(&state, &id, &user_id).await {
        return resp;
    }

    match state
        .db
        .update_list(&id, body.name.clone(), body.position, body.archived)
        .await
    {
        Ok(list) => ok(list),
        Err(DbError::NotFound) => not_found("List"),
        Err(e) => internal_error_logged("Failed to update list", e),
    }
}

pub async fn delete_list(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    if let Err(resp) = verify_list_ownership(&state, &id, &user_id).await {
        return resp;
    }

    match state.db.delete_list(&id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("List"),
        Err(e) => internal_error_logged("Failed to delete list", e),
    }
}

pub async fn reorder_lists(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<ReorderListsRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);

    if let Err(resp) = verify_board_ownership(&state, &body.board_id, &user_id).await {
        return resp;
    }

    match state.db.reorder_lists(&body.board_id, &body.list_ids).await {
        Ok(()) => ok_status(),
        Err(e) => internal_error_logged("Failed to reorder lists", e),
    }
}

pub async fn archive_list(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    if let Err(resp) = verify_list_ownership(&state, &id, &user_id).await {
        return resp;
    }

    match state.db.archive_list(&id, true).await {
        Ok(list) => ok(list),
        Err(DbError::NotFound) => not_found("List"),
        Err(e) => internal_error_logged("Failed to archive list", e),
    }
}

pub async fn restore_list(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    if let Err(resp) = verify_list_ownership(&state, &id, &user_id).await {
        return resp;
    }

    match state.db.archive_list(&id, false).await {
        Ok(list) => ok(list),
        Err(DbError::NotFound) => not_found("List"),
        Err(e) => internal_error_logged("Failed to restore list", e),
    }
}

pub async fn get_archived_lists(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let board_id = path.into_inner();

    // Verify board ownership
    match state.db.get_board(&board_id, &user_id).await {
        Ok(_) => {}
        Err(DbError::NotFound) => return not_found("Board"),
        Err(e) => return internal_error_logged("Failed to verify board ownership", e),
    }

    match state.db.get_archived_lists(&board_id).await {
        Ok(lists) => ok(lists),
        Err(e) => internal_error_logged("Failed to get archived lists", e),
    }
}
