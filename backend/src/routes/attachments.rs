//! Note attachments route handlers

use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use futures_util::StreamExt;
use std::io::Write;
use std::path::PathBuf;

use crate::db::DbError;
use crate::models::{AppState, NoteAttachment, CardAttachment};
use crate::{require_auth, require_auth_or_query};

use super::response::*;

const UPLOAD_DIR: &str = "./uploads";
const MAX_FILE_SIZE: usize = 50 * 1024 * 1024; // 50MB

fn get_upload_dir() -> PathBuf {
    let dir = PathBuf::from(UPLOAD_DIR);
    if !dir.exists() {
        std::fs::create_dir_all(&dir).expect("Failed to create upload directory");
    }
    dir
}

pub async fn upload_attachment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    mut payload: Multipart,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let note_id = path.into_inner();

    // Verify note ownership
    match state.db.verify_note_ownership(&note_id, &user_id).await {
        Ok(true) => {}
        Ok(false) => return not_found("Note"),
        Err(e) => return internal_error_logged("Failed to verify note ownership", e),
    }

    let upload_dir = get_upload_dir();
    let mut attachments = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                log::error!("Failed to read multipart field: {}", e);
                continue;
            }
        };

        let content_disposition = field.content_disposition();
        let original_filename = content_disposition
            .get_filename()
            .unwrap_or("unnamed")
            .to_string();

        let content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Generate unique filename
        let file_ext = original_filename
            .rsplit('.')
            .next()
            .map(|s| format!(".{}", s))
            .unwrap_or_default();
        let stored_filename = format!("{}_{}{}", note_id, uuid::Uuid::new_v4(), file_ext);
        let file_path = upload_dir.join(&stored_filename);

        // Read and write file
        let mut file = match std::fs::File::create(&file_path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Failed to create file: {}", e);
                return internal_error_logged("Failed to save file", e);
            }
        };

        let mut total_size = 0usize;
        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(d) => d,
                Err(e) => {
                    log::error!("Failed to read chunk: {}", e);
                    let _ = std::fs::remove_file(&file_path);
                    return internal_error_logged("Failed to read file data", e);
                }
            };

            total_size += data.len();
            if total_size > MAX_FILE_SIZE {
                let _ = std::fs::remove_file(&file_path);
                return bad_request("File too large (max 50MB)");
            }

            if let Err(e) = file.write_all(&data) {
                log::error!("Failed to write chunk: {}", e);
                let _ = std::fs::remove_file(&file_path);
                return internal_error_logged("Failed to save file", e);
            }
        }

        let attachment = NoteAttachment::new(
            note_id.clone(),
            stored_filename,
            original_filename,
            content_type,
            total_size as i64,
        );

        match state.db.create_note_attachment(&attachment).await {
            Ok(att) => attachments.push(att),
            Err(e) => {
                log::error!("Failed to save attachment metadata: {}", e);
                let _ = std::fs::remove_file(&file_path);
                return internal_error_logged("Failed to save attachment", e);
            }
        }
    }

    created(attachments)
}

pub async fn download_attachment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    // Use the extended auth check that also supports query param tokens (for image loading)
    let user_id = require_auth_or_query!(req, state);
    let attachment_id = path.into_inner();

    let attachment = match state.db.get_note_attachment(&attachment_id).await {
        Ok(a) => a,
        Err(DbError::NotFound) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to get attachment", e),
    };

    // Verify note ownership
    match state
        .db
        .verify_note_ownership(&attachment.note_id, &user_id)
        .await
    {
        Ok(true) => {}
        Ok(false) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to verify note ownership", e),
    }

    let file_path = get_upload_dir().join(&attachment.filename);

    match std::fs::read(&file_path) {
        Ok(data) => HttpResponse::Ok()
            .content_type(attachment.mime_type)
            .insert_header((
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", attachment.original_filename),
            ))
            .body(data),
        Err(e) => {
            log::error!("Failed to read file: {}", e);
            not_found("File")
        }
    }
}

pub async fn delete_attachment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let attachment_id = path.into_inner();

    let attachment = match state.db.get_note_attachment(&attachment_id).await {
        Ok(a) => a,
        Err(DbError::NotFound) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to get attachment", e),
    };

    // Verify note ownership
    match state
        .db
        .verify_note_ownership(&attachment.note_id, &user_id)
        .await
    {
        Ok(true) => {}
        Ok(false) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to verify note ownership", e),
    }

    // Delete from database
    match state.db.delete_note_attachment(&attachment_id).await {
        Ok(att) => {
            // Delete file from disk
            let file_path = get_upload_dir().join(&att.filename);
            let _ = std::fs::remove_file(&file_path);
            no_content()
        }
        Err(DbError::NotFound) => not_found("Attachment"),
        Err(e) => internal_error_logged("Failed to delete attachment", e),
    }
}

// Card attachment routes

pub async fn upload_card_attachment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
    mut payload: Multipart,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let card_id = path.into_inner();

    // Verify card ownership
    match state.db.verify_card_ownership(&card_id, &user_id).await {
        Ok(true) => {}
        Ok(false) => return not_found("Card"),
        Err(e) => return internal_error_logged("Failed to verify card ownership", e),
    }

    let upload_dir = get_upload_dir();
    let mut attachments = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => {
                log::error!("Failed to read multipart field: {}", e);
                continue;
            }
        };

        let content_disposition = field.content_disposition();
        let original_filename = content_disposition
            .get_filename()
            .unwrap_or("unnamed")
            .to_string();

        let content_type = field
            .content_type()
            .map(|ct| ct.to_string())
            .unwrap_or_else(|| "application/octet-stream".to_string());

        // Generate unique filename
        let file_ext = original_filename
            .rsplit('.')
            .next()
            .map(|s| format!(".{}", s))
            .unwrap_or_default();
        let stored_filename = format!("card_{}_{}{}", card_id, uuid::Uuid::new_v4(), file_ext);
        let file_path = upload_dir.join(&stored_filename);

        // Read and write file
        let mut file = match std::fs::File::create(&file_path) {
            Ok(f) => f,
            Err(e) => {
                log::error!("Failed to create file: {}", e);
                return internal_error_logged("Failed to save file", e);
            }
        };

        let mut total_size = 0usize;
        while let Some(chunk) = field.next().await {
            let data = match chunk {
                Ok(d) => d,
                Err(e) => {
                    log::error!("Failed to read chunk: {}", e);
                    let _ = std::fs::remove_file(&file_path);
                    return internal_error_logged("Failed to read file data", e);
                }
            };

            total_size += data.len();
            if total_size > MAX_FILE_SIZE {
                let _ = std::fs::remove_file(&file_path);
                return bad_request("File too large (max 50MB)");
            }

            if let Err(e) = file.write_all(&data) {
                log::error!("Failed to write chunk: {}", e);
                let _ = std::fs::remove_file(&file_path);
                return internal_error_logged("Failed to save file", e);
            }
        }

        let attachment = CardAttachment::new(
            card_id.clone(),
            stored_filename,
            original_filename,
            content_type,
            total_size as i64,
        );

        match state.db.create_card_attachment(&attachment).await {
            Ok(att) => attachments.push(att),
            Err(e) => {
                log::error!("Failed to save attachment metadata: {}", e);
                let _ = std::fs::remove_file(&file_path);
                return internal_error_logged("Failed to save attachment", e);
            }
        }
    }

    created(attachments)
}

pub async fn download_card_attachment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    // Use the extended auth check that also supports query param tokens (for image loading)
    let user_id = require_auth_or_query!(req, state);
    let attachment_id = path.into_inner();

    let attachment = match state.db.get_card_attachment(&attachment_id).await {
        Ok(a) => a,
        Err(DbError::NotFound) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to get attachment", e),
    };

    // Verify card ownership
    match state
        .db
        .verify_card_ownership(&attachment.card_id, &user_id)
        .await
    {
        Ok(true) => {}
        Ok(false) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to verify card ownership", e),
    }

    let file_path = get_upload_dir().join(&attachment.filename);

    match std::fs::read(&file_path) {
        Ok(data) => HttpResponse::Ok()
            .content_type(attachment.mime_type)
            .insert_header((
                "Content-Disposition",
                format!("attachment; filename=\"{}\"", attachment.original_filename),
            ))
            .body(data),
        Err(e) => {
            log::error!("Failed to read file: {}", e);
            not_found("File")
        }
    }
}

pub async fn delete_card_attachment(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> impl Responder {
    let user_id = require_auth!(req, state);
    let attachment_id = path.into_inner();

    let attachment = match state.db.get_card_attachment(&attachment_id).await {
        Ok(a) => a,
        Err(DbError::NotFound) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to get attachment", e),
    };

    // Verify card ownership
    match state
        .db
        .verify_card_ownership(&attachment.card_id, &user_id)
        .await
    {
        Ok(true) => {}
        Ok(false) => return not_found("Attachment"),
        Err(e) => return internal_error_logged("Failed to verify card ownership", e),
    }

    // Delete from database
    match state.db.delete_card_attachment(&attachment_id).await {
        Ok(att) => {
            // Delete file from disk
            let file_path = get_upload_dir().join(&att.filename);
            let _ = std::fs::remove_file(&file_path);
            no_content()
        }
        Err(DbError::NotFound) => not_found("Attachment"),
        Err(e) => internal_error_logged("Failed to delete attachment", e),
    }
}