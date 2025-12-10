//! Card models (items in a list)

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Card (task/item) in a list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub list_id: String,
    pub title: String,
    pub description: Option<String>,
    pub position: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub labels: Vec<String>,
    pub archived: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Card {
    pub fn new(
        list_id: String,
        title: String,
        description: Option<String>,
        position: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            list_id,
            title,
            description,
            position,
            due_date: None,
            labels: Vec::new(),
            archived: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating a card
#[derive(Debug, Deserialize)]
pub struct CreateCardRequest {
    pub title: String,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub labels: Option<Vec<String>>,
}

/// Request body for updating a card
#[derive(Debug, Deserialize)]
pub struct UpdateCardRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub position: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub labels: Option<Vec<String>>,
    pub archived: Option<bool>,
    pub list_id: Option<String>,
}

/// Request body for moving a card between lists
#[derive(Debug, Deserialize)]
pub struct MoveCardRequest {
    pub card_id: String,
    pub target_list_id: String,
    pub position: i32,
}

/// Request body for reordering cards within a list
#[derive(Debug, Deserialize)]
pub struct ReorderCardsRequest {
    pub list_id: String,
    pub card_ids: Vec<String>,
}
