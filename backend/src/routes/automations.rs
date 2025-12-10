//! Automation route handlers

use actix_web::{web, HttpRequest, Responder};

use crate::db::DbError;
use crate::models::{AppState, AutomationRule, CreateAutomationRequest, UpdateAutomationRequest};
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

pub async fn get_automations(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let board_id = path.into_inner();

    if let Err(resp) = verify_board_ownership(&state, &board_id, &user_id).await {
        return resp;
    }

    match state.db.get_automations(&board_id).await {
        Ok(automations) => ok(automations),
        Err(e) => internal_error_logged("Failed to get automations", e),
    }
}

pub async fn create_automation(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<CreateAutomationRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let board_id = path.into_inner();

    if let Err(resp) = verify_board_ownership(&state, &board_id, &user_id).await {
        return resp;
    }

    let automation = AutomationRule::new(
        board_id,
        body.name.clone(),
        body.trigger_type.clone(),
        body.trigger_config.clone(),
        body.action_type.clone(),
        body.action_config.clone(),
    );

    match state.db.create_automation(&automation).await {
        Ok(automation) => created(automation),
        Err(e) => internal_error_logged("Failed to create automation", e),
    }
}

pub async fn update_automation(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateAutomationRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    // Get automation to verify board ownership
    let automation = match state.db.get_automation(&id).await {
        Ok(a) => a,
        Err(DbError::NotFound) => return not_found("Automation"),
        Err(e) => return internal_error_logged("Failed to get automation", e),
    };

    if let Err(resp) = verify_board_ownership(&state, &automation.board_id, &user_id).await {
        return resp;
    }

    match state
        .db
        .update_automation(
            &id,
            body.name.clone(),
            body.enabled,
            body.trigger_type.clone(),
            body.trigger_config.clone(),
            body.action_type.clone(),
            body.action_config.clone(),
        )
        .await
    {
        Ok(automation) => ok(automation),
        Err(DbError::NotFound) => not_found("Automation"),
        Err(e) => internal_error_logged("Failed to update automation", e),
    }
}

pub async fn delete_automation(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    // Get automation to verify board ownership
    let automation = match state.db.get_automation(&id).await {
        Ok(a) => a,
        Err(DbError::NotFound) => return not_found("Automation"),
        Err(e) => return internal_error_logged("Failed to get automation", e),
    };

    if let Err(resp) = verify_board_ownership(&state, &automation.board_id, &user_id).await {
        return resp;
    }

    match state.db.delete_automation(&id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Automation"),
        Err(e) => internal_error_logged("Failed to delete automation", e),
    }
}

pub async fn toggle_automation(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    // Get automation to verify board ownership
    let automation = match state.db.get_automation(&id).await {
        Ok(a) => a,
        Err(DbError::NotFound) => return not_found("Automation"),
        Err(e) => return internal_error_logged("Failed to get automation", e),
    };

    if let Err(resp) = verify_board_ownership(&state, &automation.board_id, &user_id).await {
        return resp;
    }

    match state.db.toggle_automation(&id).await {
        Ok(automation) => ok(automation),
        Err(DbError::NotFound) => not_found("Automation"),
        Err(e) => internal_error_logged("Failed to toggle automation", e),
    }
}
