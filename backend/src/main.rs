use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

mod db;
mod models;
mod routes;

use db::Database;
use models::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgresuser:postgrespwd@localhost:5432/notesdb".to_string()
    });

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        log::warn!("JWT_SECRET not set, using default (insecure for production!)");
        "your-super-secret-jwt-key-change-in-production".to_string()
    });

    let google_client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default();
    let google_client_secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default();
    let app_url = std::env::var("APP_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    if google_client_id.is_empty() || google_client_secret.is_empty() {
        log::warn!("Google OAuth not configured. Set GOOGLE_CLIENT_ID and GOOGLE_CLIENT_SECRET to enable.");
    }

    let db = match Database::new(&database_url).await {
        Ok(db) => {
            log::info!("Connected to PostgreSQL database");
            db
        }
        Err(e) => {
            log::error!("Failed to connect to database: {}", e);
            panic!("Database connection failed: {}", e);
        }
    };

    let app_state = web::Data::new(AppState {
        db: Arc::new(db),
        jwt_secret,
        google_client_id,
        google_client_secret,
        app_url,
    });

    log::info!("Starting Notes Server API on http://0.0.0.0:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .route("/health", web::get().to(routes::health_check))
                    // Auth routes
                    .route("/auth/register", web::post().to(routes::auth::register))
                    .route("/auth/login", web::post().to(routes::auth::login))
                    .route("/auth/google", web::post().to(routes::auth::google_auth))
                    .route("/auth/google/mobile", web::post().to(routes::auth::google_auth_mobile))
                    .route("/auth/google/url", web::get().to(routes::auth::get_google_auth_url))
                    .route("/auth/me", web::get().to(routes::auth::get_me))
                    .route("/auth/storage", web::get().to(routes::auth::get_storage))
                    // Notes routes
                    .route("/notes", web::get().to(routes::notes::get_notes))
                    .route("/notes", web::post().to(routes::notes::create_note))
                    .route("/notes/tree", web::get().to(routes::notes::get_notes_tree))
                    .route("/notes/{id}", web::get().to(routes::notes::get_note))
                    .route("/notes/{id}", web::put().to(routes::notes::update_note))
                    .route("/notes/{id}", web::delete().to(routes::notes::delete_note))
                    .route("/notes/{id}/move", web::post().to(routes::notes::move_note))
                    // Note folders routes
                    .route("/folders", web::get().to(routes::notes::get_folders))
                    .route("/folders", web::post().to(routes::notes::create_folder))
                    .route("/folders/{id}", web::put().to(routes::notes::update_folder))
                    .route("/folders/{id}", web::delete().to(routes::notes::delete_folder))
                    .route("/folders/{id}/move", web::post().to(routes::notes::move_folder))
                    // Note attachments routes
                    .route("/notes/{id}/attachments", web::post().to(routes::notes::upload_attachment))
                    .route("/attachments/{id}", web::get().to(routes::notes::download_attachment))
                    .route("/attachments/{id}", web::delete().to(routes::notes::delete_attachment))
                    // Boards routes
                    .route("/boards", web::get().to(routes::boards::get_boards))
                    .route("/boards", web::post().to(routes::boards::create_board))
                    .route("/boards/tree", web::get().to(routes::boards::get_boards_tree))
                    .route("/boards/{id}", web::get().to(routes::boards::get_board))
                    .route("/boards/{id}", web::put().to(routes::boards::update_board))
                    .route("/boards/{id}", web::delete().to(routes::boards::delete_board))
                    .route("/boards/{id}/move", web::post().to(routes::boards::move_board))
                    // Board folders routes
                    .route("/board-folders", web::get().to(routes::boards::get_board_folders))
                    .route("/board-folders", web::post().to(routes::boards::create_board_folder))
                    .route("/board-folders/{id}", web::put().to(routes::boards::update_board_folder))
                    .route("/board-folders/{id}", web::delete().to(routes::boards::delete_board_folder))
                    .route("/board-folders/{id}/move", web::post().to(routes::boards::move_board_folder))
                    // Lists routes
                    .route("/boards/{board_id}/lists", web::get().to(routes::lists::get_lists))
                    .route("/boards/{board_id}/lists", web::post().to(routes::lists::create_list))
                    .route("/lists/{id}", web::put().to(routes::lists::update_list))
                    .route("/lists/{id}", web::delete().to(routes::lists::delete_list))
                    .route("/lists/reorder", web::post().to(routes::lists::reorder_lists))
                    // Cards routes
                    .route("/lists/{list_id}/cards", web::get().to(routes::cards::get_cards))
                    .route("/lists/{list_id}/cards", web::post().to(routes::cards::create_card))
                    .route("/cards/{id}", web::get().to(routes::cards::get_card))
                    .route("/cards/{id}", web::put().to(routes::cards::update_card))
                    .route("/cards/{id}", web::delete().to(routes::cards::delete_card))
                    .route("/cards/move", web::post().to(routes::cards::move_card))
                    .route("/cards/reorder", web::post().to(routes::cards::reorder_cards))
                    .route("/cards/{id}/archive", web::post().to(routes::cards::archive_card))
                    .route("/cards/{id}/restore", web::post().to(routes::cards::restore_card))
                    .route("/boards/{board_id}/cards/archived", web::get().to(routes::cards::get_archived_cards))
                    // Labels routes
                    .route("/boards/{board_id}/labels", web::get().to(routes::labels::get_labels))
                    .route("/boards/{board_id}/labels", web::post().to(routes::labels::create_label))
                    .route("/labels/{id}", web::put().to(routes::labels::update_label))
                    .route("/labels/{id}", web::delete().to(routes::labels::delete_label))
                    // Automations routes
                    .route("/boards/{board_id}/automations", web::get().to(routes::automations::get_automations))
                    .route("/boards/{board_id}/automations", web::post().to(routes::automations::create_automation))
                    .route("/automations/{id}", web::put().to(routes::automations::update_automation))
                    .route("/automations/{id}", web::delete().to(routes::automations::delete_automation))
                    .route("/automations/{id}/toggle", web::post().to(routes::automations::toggle_automation))
                    // Lists archive routes
                    .route("/lists/{id}/archive", web::post().to(routes::lists::archive_list))
                    .route("/lists/{id}/restore", web::post().to(routes::lists::restore_list))
                    .route("/boards/{board_id}/lists/archived", web::get().to(routes::lists::get_archived_lists))
                    // Sync routes for mobile app
                    .route("/sync", web::get().to(routes::sync::full_sync))
                    .route("/sync/notes/{id}", web::get().to(routes::sync::get_note_full))
                    .route("/sync/boards/{id}", web::get().to(routes::sync::get_board_full))
                                        // Settings routes
                    .route("/settings", web::get().to(routes::settings::get_settings))
                    .route("/settings", web::put().to(routes::settings::update_settings))
                    // Reminders routes
                    .route("/reminders", web::get().to(routes::reminders::get_reminders))
                    .route("/reminders", web::post().to(routes::reminders::create_reminder))
                    .route("/reminders/{id}", web::get().to(routes::reminders::get_reminder))
                    .route("/reminders/{id}", web::put().to(routes::reminders::update_reminder))
                    .route("/reminders/{id}", web::delete().to(routes::reminders::delete_reminder))
                    .route("/notes/{id}/reminders", web::delete().to(routes::reminders::delete_reminders_by_note)),
            )
        })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
