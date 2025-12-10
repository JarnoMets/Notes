//! Reminder-related models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Reminder entity for calendar reminders and note links
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Reminder {
    pub id: String,
    pub user_id: String,
    pub note_id: String,  // Empty string for standalone reminders
    pub title: String,
    pub date: String,     // YYYY-MM-DD format
    pub time: String,     // HH:MM format
    pub remind_before: String,  // Minutes before reminder (e.g., "0", "15", "30")
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Reminder {
    pub fn new(
        user_id: String,
        note_id: String,
        title: String,
        date: String,
        time: String,
        remind_before: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            user_id,
            note_id,
            title,
            date,
            time,
            remind_before,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating a reminder
#[derive(Debug, Deserialize)]
pub struct CreateReminderRequest {
    pub note_id: String,
    pub title: String,
    pub date: String,
    pub time: String,
    #[serde(default)]
    pub remind_before: String,
}

/// Request body for updating a reminder
#[derive(Debug, Deserialize)]
pub struct UpdateReminderRequest {
    pub note_id: Option<String>,
    pub title: Option<String>,
    pub date: Option<String>,
    pub time: Option<String>,
    pub remind_before: Option<String>,
}
