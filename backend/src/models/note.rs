//! Note-related models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Note folder for organizing notes hierarchically
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NoteFolder {
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

impl NoteFolder {
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

/// Request body for creating a folder
#[derive(Debug, Deserialize)]
pub struct CreateFolderRequest {
    pub name: String,
    pub parent_id: Option<String>,
}

/// Request body for updating a folder
#[derive(Debug, Deserialize)]
pub struct UpdateFolderRequest {
    pub name: Option<String>,
    pub parent_id: Option<String>,
    pub position: Option<i32>,
    pub is_important: Option<bool>,
    pub is_urgent: Option<bool>,
}

/// Request body for moving a folder
#[derive(Debug, Deserialize)]
pub struct MoveFolderRequest {
    pub parent_id: Option<String>,
    pub position: i32,
}

/// Note entity
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Note {
    pub id: String,
    pub user_id: String,
    pub folder_id: Option<String>,
    pub title: String,
    pub description: String,
    pub content: String,
    pub position: i32,
    pub is_important: bool,
    pub is_urgent: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(
        user_id: String,
        folder_id: Option<String>,
        title: String,
        description: String,
        content: String,
        position: i32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            folder_id,
            title,
            description,
            content,
            position,
            is_important: false,
            is_urgent: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// File attachment for a note
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct NoteAttachment {
    pub id: String,
    pub note_id: String,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub size: i64,
    pub created_at: DateTime<Utc>,
}

impl NoteAttachment {
    pub fn new(
        note_id: String,
        filename: String,
        original_filename: String,
        mime_type: String,
        size: i64,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            note_id,
            filename,
            original_filename,
            mime_type,
            size,
            created_at: Utc::now(),
        }
    }
}

/// Note with its attachments
#[derive(Debug, Clone, Serialize)]
pub struct NoteWithAttachments {
    #[serde(flatten)]
    pub note: Note,
    pub attachments: Vec<NoteAttachment>,
}

/// Hierarchical tree of folders and notes
#[derive(Debug, Clone, Serialize)]
pub struct NotesTree {
    pub folders: Vec<NoteFolder>,
    pub notes: Vec<Note>,
}

/// Request body for creating a note
#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub folder_id: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
}

/// Request body for updating a note
#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub title: Option<String>,
    pub folder_id: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub position: Option<i32>,
    pub is_important: Option<bool>,
    pub is_urgent: Option<bool>,
}

/// Request body for moving a note
#[derive(Debug, Deserialize)]
pub struct MoveNoteRequest {
    pub folder_id: Option<String>,
    pub position: i32,
}
