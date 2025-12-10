//! Reminder database operations

use crate::db::DbError;
use crate::models::{Reminder, UpdateReminderRequest};

impl super::Database {
    pub async fn get_reminders(&self, user_id: &str) -> Result<Vec<Reminder>, DbError> {
        let reminders = sqlx::query_as::<_, Reminder>(
            "SELECT id, user_id, note_id, title, date, time, remind_before, created_at, updated_at 
             FROM reminders 
             WHERE user_id = $1 
             ORDER BY date ASC, time ASC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(reminders)
    }

    pub async fn get_reminder(&self, id: &str, user_id: &str) -> Result<Reminder, DbError> {
        let reminder = sqlx::query_as::<_, Reminder>(
            "SELECT id, user_id, note_id, title, date, time, remind_before, created_at, updated_at 
             FROM reminders 
             WHERE id = $1 AND user_id = $2",
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(reminder)
    }

    pub async fn create_reminder(&self, reminder: &Reminder) -> Result<Reminder, DbError> {
        sqlx::query(
            "INSERT INTO reminders (id, user_id, note_id, title, date, time, remind_before, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(&reminder.id)
        .bind(&reminder.user_id)
        .bind(&reminder.note_id)
        .bind(&reminder.title)
        .bind(&reminder.date)
        .bind(&reminder.time)
        .bind(&reminder.remind_before)
        .bind(&reminder.created_at)
        .bind(&reminder.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(reminder.clone())
    }

    pub async fn update_reminder(
        &self,
        id: &str,
        user_id: &str,
        req: &UpdateReminderRequest,
    ) -> Result<Reminder, DbError> {
        let existing = self.get_reminder(id, user_id).await?;

        let updated = Reminder {
            id: existing.id,
            user_id: existing.user_id,
            note_id: req.note_id.clone().unwrap_or(existing.note_id),
            title: req.title.clone().unwrap_or(existing.title),
            date: req.date.clone().unwrap_or(existing.date),
            time: req.time.clone().unwrap_or(existing.time),
            remind_before: req.remind_before.clone().unwrap_or(existing.remind_before),
            created_at: existing.created_at,
            updated_at: chrono::Utc::now(),
        };

        sqlx::query(
            "UPDATE reminders 
             SET note_id = $1, title = $2, date = $3, time = $4, remind_before = $5, updated_at = $6 
             WHERE id = $7 AND user_id = $8",
        )
        .bind(&updated.note_id)
        .bind(&updated.title)
        .bind(&updated.date)
        .bind(&updated.time)
        .bind(&updated.remind_before)
        .bind(&updated.updated_at)
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(updated)
    }

    pub async fn delete_reminder(&self, id: &str, user_id: &str) -> Result<(), DbError> {
        let result = sqlx::query("DELETE FROM reminders WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    pub async fn delete_reminders_by_note(&self, note_id: &str, user_id: &str) -> Result<(), DbError> {
        sqlx::query("DELETE FROM reminders WHERE note_id = $1 AND user_id = $2")
            .bind(note_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
