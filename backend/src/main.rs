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

    // Spawn background task to run interval-based automations
    {
        let db_clone = app_state.db.clone();
        tokio::spawn(async move {
            // Polling loop: every 30 seconds check for due interval automations
            let poll_interval = std::time::Duration::from_secs(30);
            loop {
                match db_clone.run_due_interval_automations().await {
                    Ok(_) => {
                        // ok
                    }
                    Err(e) => {
                        log::error!("Error running interval automations: {}", e);
                    }
                }
                tokio::time::sleep(poll_interval).await;
            }
        });
    }

    log::info!("Starting Notes Server API on http://0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(routes::cors::configure_cors())
            .app_data(app_state.clone())
            .configure(routes::routes::configure_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
