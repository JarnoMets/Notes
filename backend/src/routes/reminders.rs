//! Reminder route handlers

use actix_web::{web, HttpRequest, Responder};

use crate::db::DbError;
use crate::models::{AppState, CreateReminderRequest, Reminder, UpdateReminderRequest};
use crate::require_auth;

use super::response::{bad_request, created, internal_error_logged, no_content, not_found, ok};

// ============ Reminders ============

pub async fn get_reminders(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = require_auth!(req, state);

    match state.db.get_reminders(&user_id).await {
        Ok(reminders) => ok(reminders),
        Err(e) => internal_error_logged("Failed to get reminders", e),
    }
}

pub async fn get_reminder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.get_reminder(&id, &user_id).await {
        Ok(reminder) => ok(reminder),
        Err(DbError::NotFound) => not_found("Reminder"),
        Err(e) => internal_error_logged("Failed to get reminder", e),
    }
}

pub async fn create_reminder(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateReminderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);

    // Validate date format (YYYY-MM-DD)
    if !body.date.matches('-').count() == 2 || body.date.len() != 10 {
        return bad_request("Invalid date format. Expected YYYY-MM-DD");
    }

    // Validate time format (HH:MM)
    if !body.time.contains(':') || body.time.len() != 5 {
        return bad_request("Invalid time format. Expected HH:MM");
    }

    let reminder = Reminder::new(
        user_id,
        body.note_id.clone(),
        body.title.clone(),
        body.date.clone(),
        body.time.clone(),
        body.remind_before.clone(),
    );

    match state.db.create_reminder(&reminder).await {
        Ok(reminder) => created(reminder),
        Err(e) => internal_error_logged("Failed to create reminder", e),
    }
}

pub async fn update_reminder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    body: web::Json<UpdateReminderRequest>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.update_reminder(&id, &user_id, &body).await {
        Ok(reminder) => ok(reminder),
        Err(DbError::NotFound) => not_found("Reminder"),
        Err(e) => internal_error_logged("Failed to update reminder", e),
    }
}

pub async fn delete_reminder(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let id = path.into_inner();

    match state.db.delete_reminder(&id, &user_id).await {
        Ok(()) => no_content(),
        Err(DbError::NotFound) => not_found("Reminder"),
        Err(e) => internal_error_logged("Failed to delete reminder", e),
    }
}

pub async fn delete_reminders_by_note(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let note_id = path.into_inner();

    match state.db.delete_reminders_by_note(&note_id, &user_id).await {
        Ok(()) => no_content(),
        Err(e) => internal_error_logged("Failed to delete reminders", e),
    }
}
