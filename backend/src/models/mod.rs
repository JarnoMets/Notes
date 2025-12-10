//! Domain models module
//! 
//! This module contains all the data structures used throughout the application.

mod user;
mod note;
mod board;
mod list;
mod card;
mod label;
mod automation;
mod settings;
mod reminder;

// Re-export all models for convenient access
pub use user::*;
pub use note::*;
pub use board::*;
pub use list::*;
pub use card::*;
pub use label::*;
pub use automation::*;
pub use settings::*;
pub use reminder::*;

use crate::db::Database;
use std::sync::Arc;

/// Application state shared across all handlers
pub struct AppState {
    pub db: Arc<Database>,
    pub jwt_secret: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub app_url: String,
}
