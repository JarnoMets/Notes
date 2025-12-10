//! Label models for board cards

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Label that can be applied to cards in a board
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BoardLabel {
    pub id: String,
    pub board_id: String,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

impl BoardLabel {
    pub fn new(board_id: String, name: String, color: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            board_id,
            name,
            color,
            created_at: Utc::now(),
        }
    }
}

/// Request body for creating a label
#[derive(Debug, Deserialize)]
pub struct CreateLabelRequest {
    pub name: String,
    pub color: String,
}

/// Request body for updating a label
#[derive(Debug, Deserialize)]
pub struct UpdateLabelRequest {
    pub name: Option<String>,
    pub color: Option<String>,
}
