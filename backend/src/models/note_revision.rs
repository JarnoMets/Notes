use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Diff-based revision of a note. Stores forward and reverse patches and an index.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NoteRevision {
    pub id: String,
    pub note_id: String,
    pub user_id: String,
    pub idx: i32,
    pub forward_patch: String,
    pub reverse_patch: String,
    pub created_at: DateTime<Utc>,
}

impl NoteRevision {
    pub fn new(note_id: String, user_id: String, idx: i32, forward_patch: String, reverse_patch: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            note_id,
            user_id,
            idx,
            forward_patch,
            reverse_patch,
            created_at: Utc::now(),
        }
    }
}
