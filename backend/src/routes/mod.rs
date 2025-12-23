//! Routes module

pub mod attachments;
pub mod auth;
pub mod automations;
pub mod boards;
pub mod cards;
pub mod cors;
pub mod folders;
pub mod graphs;
pub mod labels;
pub mod lists;
pub mod notes;
pub mod reminders;
pub mod response;
pub mod routes;
pub mod settings;
pub mod sync;

use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
