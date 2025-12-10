//! List models (columns in a kanban board)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Card;

/// List (column) in a kanban board
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct List {
    pub id: String,
    pub board_id: String,
    pub name: String,
    pub position: i32,
    #[serde(default)]
    #[sqlx(default)]
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl List {
    pub fn new(board_id: String, name: String, position: i32) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            board_id,
            name,
            position,
            archived: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating a list
#[derive(Debug, Deserialize)]
pub struct CreateListRequest {
    pub name: String,
    pub position: Option<i32>,
}

/// Request body for updating a list
#[derive(Debug, Deserialize)]
pub struct UpdateListRequest {
    pub name: Option<String>,
    pub position: Option<i32>,
    pub archived: Option<bool>,
}

/// Request body for reordering lists
#[derive(Debug, Deserialize)]
pub struct ReorderListsRequest {
    pub board_id: String,
    pub list_ids: Vec<String>,
}

/// List with its cards
#[derive(Debug, Clone, Serialize)]
pub struct ListWithCards {
    pub list: List,
    pub cards: Vec<Card>,
}
