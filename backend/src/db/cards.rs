use super::{Database, DbError, DbResult};
use crate::models::Card;
use chrono::{DateTime, Utc};
use sqlx::Row;

/// Helper function to map a database row to a Card
fn row_to_card(row: sqlx::postgres::PgRow) -> Card {
    Card {
        id: row.get("id"),
        list_id: row.get("list_id"),
        title: row.get("title"),
        description: row.get("description"),
        position: row.get("position"),
        due_date: row.get("due_date"),
        labels: serde_json::from_value(row.get("labels")).unwrap_or_default(),
        archived: row.get("archived"),
        status: row.get::<Option<String>, _>("status").unwrap_or_else(|| "open".to_string()),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

impl Database {
    pub async fn get_cards(&self, list_id: &str) -> DbResult<Vec<Card>> {
        let rows = sqlx::query(
                r#"SELECT id, list_id, title, description, position, due_date, labels, archived, status, created_at, updated_at 
                    FROM cards WHERE list_id = $1 ORDER BY position ASC"#
        )
        .bind(list_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(row_to_card).collect())
    }

    pub async fn get_cards_active(&self, list_id: &str) -> DbResult<Vec<Card>> {
        let rows = sqlx::query(
                r#"SELECT id, list_id, title, description, position, due_date, labels, archived, status, created_at, updated_at 
                    FROM cards WHERE list_id = $1 AND archived = FALSE ORDER BY position ASC"#
        )
        .bind(list_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(row_to_card).collect())
    }

    pub async fn get_archived_cards(&self, board_id: &str) -> DbResult<Vec<Card>> {
        let rows = sqlx::query(
                r#"SELECT c.id, c.list_id, c.title, c.description, c.position, c.due_date, c.labels, c.archived, c.status, c.created_at, c.updated_at 
                    FROM cards c
               JOIN lists l ON c.list_id = l.id
               WHERE l.board_id = $1 AND c.archived = TRUE
               ORDER BY c.updated_at DESC"#
        )
        .bind(board_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(row_to_card).collect())
    }

    pub async fn get_card(&self, id: &str) -> DbResult<Card> {
        let row = sqlx::query(
                r#"SELECT id, list_id, title, description, position, due_date, labels, archived, status, created_at, updated_at 
                    FROM cards WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(row_to_card(row))
    }

    pub async fn create_card(&self, card: &Card) -> DbResult<Card> {
        let labels_json = serde_json::to_value(&card.labels)?;

        sqlx::query(
            "INSERT INTO cards (id, list_id, title, description, position, due_date, labels, archived, status, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
        )
        .bind(&card.id)
        .bind(&card.list_id)
        .bind(&card.title)
        .bind(&card.description)
        .bind(&card.position)
        .bind(&card.due_date)
        .bind(&labels_json)
        .bind(&card.archived)
        .bind(&card.status)
        .bind(&card.created_at)
        .bind(&card.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(card.clone())
    }

    pub async fn get_next_card_position(&self, list_id: &str) -> DbResult<i32> {
        let row: Option<(Option<i32>,)> = sqlx::query_as(
            "SELECT MAX(position) FROM cards WHERE list_id = $1"
        )
        .bind(list_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.and_then(|r| r.0).unwrap_or(-1) + 1)
    }

    pub async fn update_card(
        &self,
        id: &str,
        title: Option<String>,
        description: Option<String>,
        position: Option<i32>,
        due_date: Option<Option<DateTime<Utc>>>,
        labels: Option<Vec<String>>,
        archived: Option<bool>,
        list_id: Option<String>,
        status: Option<String>,
    ) -> DbResult<Card> {
        let existing = self.get_card(id).await?;

        let new_title = title.unwrap_or(existing.title);
        let new_description = description.or(existing.description);
        let new_position = position.unwrap_or(existing.position);
        let new_due_date = due_date.unwrap_or(existing.due_date);
        let new_labels = labels.unwrap_or(existing.labels);
        let new_archived = archived.unwrap_or(existing.archived);
        let new_list_id = list_id.unwrap_or(existing.list_id);
        let new_status = status.unwrap_or(existing.status);
        let labels_json = serde_json::to_value(&new_labels)?;
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE cards SET title = $1, description = $2, position = $3, due_date = $4, labels = $5, archived = $6, list_id = $7, status = $8, updated_at = $9 WHERE id = $10"
        )
        .bind(&new_title)
        .bind(&new_description)
        .bind(&new_position)
        .bind(&new_due_date)
        .bind(&labels_json)
        .bind(&new_archived)
        .bind(&new_list_id)
        .bind(&new_status)
        .bind(&updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_card(id).await
    }

    pub async fn delete_card(&self, id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM cards WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    pub async fn move_card(&self, card_id: &str, target_list_id: &str, position: i32) -> DbResult<Card> {
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE cards SET list_id = $1, position = $2, updated_at = $3 WHERE id = $4"
        )
        .bind(target_list_id)
        .bind(position)
        .bind(&updated_at)
        .bind(card_id)
        .execute(&self.pool)
        .await?;

        self.get_card(card_id).await
    }

    pub async fn reorder_cards(&self, list_id: &str, card_ids: &[String]) -> DbResult<()> {
        for (position, card_id) in card_ids.iter().enumerate() {
            sqlx::query(
                "UPDATE cards SET position = $1, updated_at = $2 WHERE id = $3 AND list_id = $4"
            )
            .bind(position as i32)
            .bind(Utc::now())
            .bind(card_id)
            .bind(list_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn archive_card(&self, id: &str, archived: bool) -> DbResult<Card> {
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE cards SET archived = $1, updated_at = $2 WHERE id = $3"
        )
        .bind(archived)
        .bind(&updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_card(id).await
    }
}
