//! Common HTTP response helpers for consistent error handling
//!
//! This module provides helper functions to reduce boilerplate in route handlers.

use actix_web::HttpResponse;
use serde::Serialize;

/// Standard error response structure
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

/// Standard success response with status
#[derive(Serialize)]
struct StatusResponse {
    status: &'static str,
}

// ============ Error Responses ============

/// Returns a 401 Unauthorized response
pub fn unauthorized(message: &str) -> HttpResponse {
    HttpResponse::Unauthorized().json(ErrorResponse {
        error: message.to_string(),
    })
}

/// Returns a 401 Unauthorized with default message
pub fn not_authenticated() -> HttpResponse {
    unauthorized("Not authenticated")
}

/// Returns a 404 Not Found response
pub fn not_found(resource: &str) -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResponse {
        error: format!("{} not found", resource),
    })
}

/// Returns a 400 Bad Request response
pub fn bad_request(message: &str) -> HttpResponse {
    HttpResponse::BadRequest().json(ErrorResponse {
        error: message.to_string(),
    })
}

/// Returns a 409 Conflict response
pub fn conflict(message: &str) -> HttpResponse {
    HttpResponse::Conflict().json(ErrorResponse {
        error: message.to_string(),
    })
}

/// Returns a 500 Internal Server Error response
#[allow(dead_code)]
pub fn internal_error(message: &str) -> HttpResponse {
    HttpResponse::InternalServerError().json(ErrorResponse {
        error: message.to_string(),
    })
}

/// Logs an error and returns a 500 Internal Server Error response
pub fn internal_error_logged(context: &str, error: impl std::fmt::Display) -> HttpResponse {
    log::error!("{}: {}", context, error);
    HttpResponse::InternalServerError().json(ErrorResponse {
        error: error.to_string(),
    })
}

// ============ Success Responses ============

/// Returns a 200 OK response with JSON body
pub fn ok<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}

/// Returns a 201 Created response with JSON body
pub fn created<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Created().json(data)
}

/// Returns a 204 No Content response
pub fn no_content() -> HttpResponse {
    HttpResponse::NoContent().finish()
}

/// Returns a 200 OK response with status: "ok"
pub fn ok_status() -> HttpResponse {
    HttpResponse::Ok().json(StatusResponse { status: "ok" })
}

// ============ Helper Macros ============

/// Extracts user_id from request or returns Unauthorized response
/// 
/// Usage:
/// ```ignore
/// let user_id = require_auth!(req, state);
/// ```
#[macro_export]
macro_rules! require_auth {
    ($req:expr, $state:expr) => {
        match $crate::routes::auth::get_user_id_from_request(&$req, &$state.jwt_secret) {
            Some(id) => id,
            None => return $crate::routes::response::not_authenticated(),
        }
    };
}

/// Extracts user_id from request (with query param fallback) or returns Unauthorized response
/// 
/// Usage:
/// ```ignore
/// let user_id = require_auth_or_query!(req, state);
/// ```
#[macro_export]
macro_rules! require_auth_or_query {
    ($req:expr, $state:expr) => {
        match $crate::routes::auth::get_user_id_from_request_or_query(&$req, &$state.jwt_secret) {
            Some(id) => id,
            None => return $crate::routes::response::not_authenticated(),
        }
    };
}


