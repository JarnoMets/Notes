//! Card route handlers

use actix_web::{web, HttpRequest, Responder};
use actix_web::rt::spawn;
use serde_json::json;

use crate::db::DbError;
use crate::models::{
    AppState, Card, CreateCardRequest, MoveCardRequest, ReorderCardsRequest, UpdateCardRequest,
};
use crate::require_auth;

use super::response::{created, internal_error_logged, no_content, not_found, ok, ok_status};

pub async fn get_cards(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let list_id = path.into_inner();

    match state.db.get_cards(&list_id).await {
        Ok(cards) => ok(cards),
        Err(e) => internal_error_logged("Failed to get cards", e),
    }
}

pub async fn get_card(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    match state.db.get_card(&id).await {
        Ok(card) => ok(card),
        Err(DbError::NotFound) => not_found("Card"),
        Err(e) => internal_error_logged("Failed to get card", e),
    }
}

pub async fn create_card(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<CreateCardRequest>,
) -> impl Responder {
    let list_id = path.into_inner();

    let position = match body.position {
        Some(p) => p,
        None => match state.db.get_next_card_position(&list_id).await {
            Ok(p) => p,
            Err(e) => return internal_error_logged("Failed to get next position", e),
        },
    };

    let mut card = Card::new(list_id, body.title.clone(), body.description.clone(), position);
    if let Some(due_date) = body.due_date {
        card.due_date = Some(due_date);
    }
    if let Some(ref labels) = body.labels {
        card.labels = labels.clone();
    }
    if let Some(ref status) = body.status {
        card.status = status.clone();
    }

    match state.db.create_card(&card).await {
        Ok(card) => {
            // Spawn automation runner for card_created trigger (non-blocking)
            let db = state.db.clone();
            let list_id = card.list_id.clone();
            let card_id = card.id.clone();
            spawn(async move {
                if let Ok(list) = db.get_list(&list_id).await {
                    let ctx = json!({ "card_id": card_id });
                    let _ = db.run_automations_for_trigger(&list.board_id, "card_created", ctx).await;
                }
            });

            created(card)
        }
        Err(e) => internal_error_logged("Failed to create card", e),
    }
}

pub async fn update_card(
    state: web::Data<AppState>,
    path: web::Path<String>,
    body: web::Json<UpdateCardRequest>,
) -> impl Responder {
    let id = path.into_inner();

    // Handle due_date: if it's provided in the request, pass Some(value), otherwise None
    let due_date_update = body.due_date.map(Some);

    match state
        .db
        .update_card(
            &id,
            body.title.clone(),
            body.description.clone(),
            body.position,
            due_date_update,
            body.labels.clone(),
            body.archived,
            body.list_id.clone(),
            body.status.clone(),
        )
        .await
    {
        Ok(card) => ok(card),
        Err(DbError::NotFound) => not_found("Card"),
        Err(e) => internal_error_logged("Failed to update card", e),
    }
}

pub async fn delete_card(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    match state.db.delete_card(&id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Card"),
        Err(e) => internal_error_logged("Failed to delete card", e),
    }
}

pub async fn move_card(
    state: web::Data<AppState>,
    body: web::Json<MoveCardRequest>,
) -> impl Responder {
    match state
        .db
        .move_card(&body.card_id, &body.target_list_id, body.position)
        .await
    {
        Ok(card) => {
            // Spawn automation runner for card_moved trigger
            let db = state.db.clone();
            let target_list_id = body.target_list_id.clone();
            let card_id = body.card_id.clone();
            spawn(async move {
                if let Ok(list) = db.get_list(&target_list_id).await {
                    let ctx = json!({ "card_id": card_id, "target_list_id": target_list_id });
                    let _ = db.run_automations_for_trigger(&list.board_id, "card_moved", ctx).await;
                }
            });

            ok(card)
        }
        Err(DbError::NotFound) => not_found("Card"),
        Err(e) => internal_error_logged("Failed to move card", e),
    }
}

pub async fn reorder_cards(
    state: web::Data<AppState>,
    body: web::Json<ReorderCardsRequest>,
) -> impl Responder {
    match state.db.reorder_cards(&body.list_id, &body.card_ids).await {
        Ok(()) => ok_status(),
        Err(e) => internal_error_logged("Failed to reorder cards", e),
    }
}

pub async fn archive_card(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    match state.db.archive_card(&id, true).await {
        Ok(card) => ok(card),
        Err(DbError::NotFound) => not_found("Card"),
        Err(e) => internal_error_logged("Failed to archive card", e),
    }
}

pub async fn restore_card(state: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();

    match state.db.archive_card(&id, false).await {
        Ok(card) => ok(card),
        Err(DbError::NotFound) => not_found("Card"),
        Err(e) => internal_error_logged("Failed to restore card", e),
    }
}

pub async fn get_archived_cards(
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

    match state.db.get_archived_cards(&board_id).await {
        Ok(cards) => ok(cards),
        Err(e) => internal_error_logged("Failed to get archived cards", e),
    }
}
