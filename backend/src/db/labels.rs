use super::{Database, DbError, DbResult};
use crate::models::BoardLabel;

impl Database {
    pub async fn get_labels(&self, board_id: &str) -> DbResult<Vec<BoardLabel>> {
        let rows = sqlx::query_as::<_, BoardLabel>(
            r#"SELECT id, board_id, name, color, created_at 
               FROM board_labels WHERE board_id = $1 ORDER BY created_at ASC"#
        )
        .bind(board_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_label(&self, id: &str) -> DbResult<BoardLabel> {
        let label = sqlx::query_as::<_, BoardLabel>(
            r#"SELECT id, board_id, name, color, created_at 
               FROM board_labels WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(label)
    }

    pub async fn create_label(&self, label: &BoardLabel) -> DbResult<BoardLabel> {
        sqlx::query(
            "INSERT INTO board_labels (id, board_id, name, color, created_at) 
             VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&label.id)
        .bind(&label.board_id)
        .bind(&label.name)
        .bind(&label.color)
        .bind(&label.created_at)
        .execute(&self.pool)
        .await?;

        Ok(label.clone())
    }

    pub async fn update_label(&self, id: &str, name: Option<String>, color: Option<String>) -> DbResult<BoardLabel> {
        let existing = self.get_label(id).await?;
        
        let new_name = name.unwrap_or(existing.name);
        let new_color = color.unwrap_or(existing.color);

        sqlx::query(
            "UPDATE board_labels SET name = $1, color = $2 WHERE id = $3"
        )
        .bind(&new_name)
        .bind(&new_color)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_label(id).await
    }

    pub async fn delete_label(&self, id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM board_labels WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }
}
