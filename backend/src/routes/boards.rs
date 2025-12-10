//! Board route handlers

use actix_web::{web, HttpRequest, Responder};

use crate::db::DbError;
use crate::models::{
    AppState, Board, BoardFolder, CreateBoardFolderRequest, CreateBoardRequest,
    MoveBoardFolderRequest, MoveBoardRequest, UpdateBoardFolderRequest, UpdateBoardRequest,
};
use crate::require_auth;

use super::response::{created, internal_error_logged, no_content, not_found, ok};

// ============ Board Folders ============

pub async fn get_board_folders(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    match state.db.get_board_folders(&user_id).await {
        Ok(folders) => ok(folders),
        Err(e) => internal_error_logged("Failed to get board folders", e),
    }
}

pub async fn create_board_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateBoardFolderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);

    let position = match state
        .db
        .get_max_board_folder_position(&user_id, body.parent_id.as_deref())
        .await
    {
        Ok(pos) => pos,
        Err(e) => return internal_error_logged("Failed to get max position", e),
    };

    let folder = BoardFolder::new(user_id, body.parent_id.clone(), body.name.clone(), position);

    match state.db.create_board_folder(&folder).await {
        Ok(folder) => created(folder),
        Err(e) => internal_error_logged("Failed to create board folder", e),
    }
}

pub async fn update_board_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateBoardFolderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();
    let parent_id = body.parent_id.clone().map(Some);

    match state
        .db
        .update_board_folder(&id, &user_id, body.name.clone(), parent_id, body.position, body.is_important, body.is_urgent)
        .await
    {
        Ok(folder) => ok(folder),
        Err(DbError::NotFound) => not_found("Board folder"),
        Err(e) => internal_error_logged("Failed to update board folder", e),
    }
}

pub async fn move_board_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<MoveBoardFolderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state
        .db
        .update_board_folder(
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
        Err(DbError::NotFound) => not_found("Board folder"),
        Err(e) => internal_error_logged("Failed to move board folder", e),
    }
}

pub async fn delete_board_folder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.delete_board_folder(&id, &user_id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Board folder"),
        Err(e) => internal_error_logged("Failed to delete board folder", e),
    }
}

// ============ Boards ============

pub async fn get_boards(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    match state.db.get_boards(&user_id).await {
        Ok(boards) => ok(boards),
        Err(e) => internal_error_logged("Failed to get boards", e),
    }
}

pub async fn get_boards_tree(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    match state.db.get_boards_tree(&user_id).await {
        Ok(tree) => ok(tree),
        Err(DbError::NotFound) => not_found("Boards tree"),
        Err(e) => internal_error_logged("Failed to get boards tree", e),
    }
}

pub async fn get_board(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.get_board_with_lists(&id, &user_id).await {
        Ok(board) => ok(board),
        Err(DbError::NotFound) => not_found("Board"),
        Err(e) => internal_error_logged("Failed to get board", e),
    }
}

pub async fn create_board(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateBoardRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);

    let position = match state
        .db
        .get_max_board_position(&user_id, body.folder_id.as_deref())
        .await
    {
        Ok(pos) => pos,
        Err(e) => return internal_error_logged("Failed to get max position", e),
    };

    let board = Board::new(
        user_id,
        body.folder_id.clone(),
        body.name.clone(),
        body.description.clone(),
        body.color.clone(),
        position,
    );

    match state.db.create_board(&board).await {
        Ok(board) => created(board),
        Err(e) => internal_error_logged("Failed to create board", e),
    }
}

pub async fn update_board(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateBoardRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();
    let folder_id = body.folder_id.clone().map(Some);

    match state
        .db
        .update_board(
            &id,
            &user_id,
            body.name.clone(),
            body.description.clone(),
            body.color.clone(),
            folder_id,
            body.position,
            body.is_important,
            body.is_urgent,
        )
        .await
    {
        Ok(board) => ok(board),
        Err(DbError::NotFound) => not_found("Board"),
        Err(e) => internal_error_logged("Failed to update board", e),
    }
}

pub async fn move_board(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<MoveBoardRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state
        .db
        .update_board(
            &id,
            &user_id,
            None,
            None,
            None,
            Some(body.folder_id.clone()),
            Some(body.position),
            None,
            None,
        )
        .await
    {
        Ok(board) => ok(board),
        Err(DbError::NotFound) => not_found("Board"),
        Err(e) => internal_error_logged("Failed to move board", e),
    }
}

pub async fn delete_board(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.delete_board(&id, &user_id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Board"),
        Err(e) => internal_error_logged("Failed to delete board", e),
    }
}
