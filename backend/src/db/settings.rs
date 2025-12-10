use super::{Database, DbResult};
use crate::models::{IcsCalendar, UserSettings};
use chrono::Utc;

impl Database {
    pub async fn get_user_settings(&self, user_id: &str) -> DbResult<UserSettings> {
        let settings = sqlx::query_as::<_, UserSettings>(
            r#"SELECT user_id, theme, week_starts_on_monday, ics_calendars, created_at, updated_at 
               FROM user_settings WHERE user_id = $1"#
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match settings {
            Some(s) => Ok(s),
            None => {
                // Create default settings for the user
                let default_settings = UserSettings::default_for_user(user_id.to_string());
                self.create_user_settings(&default_settings).await?;
                Ok(default_settings)
            }
        }
    }

    pub async fn create_user_settings(&self, settings: &UserSettings) -> DbResult<UserSettings> {
        sqlx::query(
            "INSERT INTO user_settings (user_id, theme, week_starts_on_monday, ics_calendars, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (user_id) DO NOTHING"
        )
        .bind(&settings.user_id)
        .bind(&settings.theme)
        .bind(&settings.week_starts_on_monday)
        .bind(serde_json::to_value(&settings.ics_calendars)?)
        .bind(&settings.created_at)
        .bind(&settings.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(settings.clone())
    }

    pub async fn update_user_settings(
        &self,
        user_id: &str,
        theme: Option<String>,
        week_starts_on_monday: Option<bool>,
        ics_calendars: Option<Vec<IcsCalendar>>,
    ) -> DbResult<UserSettings> {
        // Ensure settings exist
        let existing = self.get_user_settings(user_id).await?;

        let new_theme = theme.unwrap_or(existing.theme);
        let new_week_starts = week_starts_on_monday.unwrap_or(existing.week_starts_on_monday);
        let new_calendars = ics_calendars.unwrap_or(existing.ics_calendars);
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE user_settings 
             SET theme = $1, week_starts_on_monday = $2, ics_calendars = $3, updated_at = $4 
             WHERE user_id = $5"
        )
        .bind(&new_theme)
        .bind(&new_week_starts)
        .bind(serde_json::to_value(&new_calendars)?)
        .bind(&updated_at)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        self.get_user_settings(user_id).await
    }
}
