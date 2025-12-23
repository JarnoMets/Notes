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
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// File attachment for a card
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct CardAttachment {
    pub id: String,
    pub card_id: String,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub size: i64,
    pub created_at: DateTime<Utc>,
}

impl CardAttachment {
    pub fn new(
        card_id: String,
        filename: String,
        original_filename: String,
        mime_type: String,
        size: i64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            card_id,
            filename,
            original_filename,
            mime_type,
            size,
            created_at: Utc::now(),
        }
    }
}

/// Card with its attachments
#[derive(Debug, Clone, Serialize)]
pub struct CardWithAttachments {
    #[serde(flatten)]
    pub card: Card,
    pub attachments: Vec<CardAttachment>,
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
            status: String::from("open"),
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
    pub status: Option<String>,
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
    pub status: Option<String>,
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
