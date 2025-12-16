use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Snapshot / revision of a note
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NoteRevision {
    pub id: String,
    pub note_id: String,
    pub user_id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl NoteRevision {
    pub fn new(note_id: String, user_id: String, title: String, description: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            note_id,
            user_id,
            title,
            description,
            content,
            created_at: Utc::now(),
        }
    }
}
