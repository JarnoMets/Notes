//! Board-related models (Trello-like kanban boards)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{AutomationRule, BoardLabel, ListWithCards};

/// Board folder for organizing boards hierarchically
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BoardFolder {
    pub id: String,
    pub user_id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub position: i32,
    pub is_important: bool,
    pub is_urgent: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BoardFolder {
    pub fn new(user_id: String, parent_id: Option<String>, name: String, position: i32) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            parent_id,
            name,
            position,
            is_important: false,
            is_urgent: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating a board folder
#[derive(Debug, Deserialize)]
pub struct CreateBoardFolderRequest {
    pub name: String,
    pub parent_id: Option<String>,
}

/// Request body for updating a board folder
#[derive(Debug, Deserialize)]
pub struct UpdateBoardFolderRequest {
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub position: Option<i32>,
    pub is_important: Option<bool>,
    pub is_urgent: Option<bool>,
}

/// Request body for moving a board folder
#[derive(Debug, Deserialize)]
pub struct MoveBoardFolderRequest {
    pub parent_id: Option<String>,
    pub position: i32,
}

/// Kanban board entity
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Board {
    pub id: String,
    pub user_id: String,
    pub folder_id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub position: i32,
    pub is_important: bool,
    pub is_urgent: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Board {
    pub fn new(
        user_id: String,
        folder_id: Option<String>,
        name: String,
        description: Option<String>,
        color: Option<String>,
        position: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            folder_id,
            name,
            description,
            color,
            position,
            is_important: false,
            is_urgent: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating a board
#[derive(Debug, Deserialize)]
pub struct CreateBoardRequest {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub folder_id: Option<String>,
}

/// Request body for updating a board
#[derive(Debug, Deserialize)]
pub struct UpdateBoardRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
    pub folder_id: Option<String>,
    pub position: Option<i32>,
    pub is_important: Option<bool>,
    pub is_urgent: Option<bool>,
}

/// Request body for moving a board
#[derive(Debug, Deserialize)]
pub struct MoveBoardRequest {
    pub folder_id: Option<String>,
    pub position: i32,
}

/// Board tree with folders and boards
#[derive(Debug, Clone, Serialize)]
pub struct BoardsTree {
    pub folders: Vec<BoardFolder>,
    pub boards: Vec<Board>,
}

/// Board with all its related data (lists, cards, labels, automations)
#[derive(Debug, Clone, Serialize)]
pub struct BoardWithLists {
    pub board: Board,
    pub lists: Vec<ListWithCards>,
    pub labels: Vec<BoardLabel>,
    pub automations: Vec<AutomationRule>,
}
