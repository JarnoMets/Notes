//! Settings route handlers

use actix_web::{web, HttpRequest, Responder};

use crate::models::{AppState, SettingsResponse, UpdateSettingsRequest};

use super::auth::get_user_id_from_request;
use super::response::{internal_error_logged, not_authenticated, ok};

pub async fn get_settings(state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let user_id = match get_user_id_from_request(&req, &state.jwt_secret) {
        Some(id) => id,
        None => return not_authenticated(),
    };

    match state.db.get_user_settings(&user_id).await {
        Ok(settings) => ok(SettingsResponse::from(settings)),
        Err(e) => internal_error_logged("Failed to get settings", e),
    }
}

pub async fn update_settings(
    state: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<UpdateSettingsRequest>,
) -> impl Responder {
    let user_id = match get_user_id_from_request(&req, &state.jwt_secret) {
        Some(id) => id,
        None => return not_authenticated(),
    };

    match state
        .db
        .update_user_settings(
            &user_id,
            body.theme.clone(),
            body.week_starts_on_monday,
            body.ics_calendars.clone(),
        )
        .await
    {
        Ok(settings) => ok(SettingsResponse::from(settings)),
        Err(e) => internal_error_logged("Failed to update settings", e),
    }
}
