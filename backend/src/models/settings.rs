//! User settings model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ICS Calendar configuration stored per user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IcsCalendar {
    pub id: String,
    pub name: String,
    pub url: String,
    pub color: String,
    pub enabled: bool,
}

/// User settings stored in the database
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserSettings {
    pub user_id: String,
    pub theme: String,
    pub week_starts_on_monday: bool,
    #[sqlx(json)]
    pub ics_calendars: Vec<IcsCalendar>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserSettings {
    /// Create default settings for a new user
    pub fn default_for_user(user_id: String) -> Self {
        let now = Utc::now();
        Self {
            user_id,
            theme: "dark".to_string(),
            week_starts_on_monday: true,
            ics_calendars: vec![],
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for updating settings
#[derive(Debug, Deserialize)]
pub struct UpdateSettingsRequest {
    pub theme: Option<String>,
    pub week_starts_on_monday: Option<bool>,
    pub ics_calendars: Option<Vec<IcsCalendar>>,
}

/// Response for settings
#[derive(Debug, Serialize)]
pub struct SettingsResponse {
    pub theme: String,
    pub week_starts_on_monday: bool,
    pub ics_calendars: Vec<IcsCalendar>,
}

impl From<UserSettings> for SettingsResponse {
    fn from(settings: UserSettings) -> Self {
        Self {
            theme: settings.theme,
            week_starts_on_monday: settings.week_starts_on_monday,
            ics_calendars: settings.ics_calendars,
        }
    }
}
