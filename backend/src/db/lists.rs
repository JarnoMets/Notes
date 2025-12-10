use super::{Database, DbError, DbResult};
use crate::models::List;
use chrono::Utc;

impl Database {
    pub async fn get_lists(&self, board_id: &str) -> DbResult<Vec<List>> {
        let rows = sqlx::query_as::<_, List>(
            r#"SELECT id, board_id, name, position, archived, created_at, updated_at 
               FROM lists WHERE board_id = $1 ORDER BY position ASC"#
        )
        .bind(board_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_lists_active(&self, board_id: &str) -> DbResult<Vec<List>> {
        let rows = sqlx::query_as::<_, List>(
            r#"SELECT id, board_id, name, position, archived, created_at, updated_at 
               FROM lists WHERE board_id = $1 AND archived = FALSE ORDER BY position ASC"#
        )
        .bind(board_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_list(&self, id: &str) -> DbResult<List> {
        let list = sqlx::query_as::<_, List>(
            r#"SELECT id, board_id, name, position, archived, created_at, updated_at 
               FROM lists WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(list)
    }

    pub async fn create_list(&self, list: &List) -> DbResult<List> {
        sqlx::query(
            "INSERT INTO lists (id, board_id, name, position, archived, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&list.id)
        .bind(&list.board_id)
        .bind(&list.name)
        .bind(&list.position)
        .bind(&list.archived)
        .bind(&list.created_at)
        .bind(&list.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(list.clone())
    }

    pub async fn get_next_list_position(&self, board_id: &str) -> DbResult<i32> {
        let row: Option<(Option<i32>,)> = sqlx::query_as(
            "SELECT MAX(position) FROM lists WHERE board_id = $1"
        )
        .bind(board_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.and_then(|r| r.0).unwrap_or(-1) + 1)
    }

    pub async fn update_list(&self, id: &str, name: Option<String>, position: Option<i32>, archived: Option<bool>) -> DbResult<List> {
        let existing = self.get_list(id).await?;
        
        let new_name = name.unwrap_or(existing.name);
        let new_position = position.unwrap_or(existing.position);
        let new_archived = archived.unwrap_or(existing.archived);
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE lists SET name = $1, position = $2, archived = $3, updated_at = $4 WHERE id = $5"
        )
        .bind(&new_name)
        .bind(&new_position)
        .bind(&new_archived)
        .bind(&updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_list(id).await
    }

    pub async fn delete_list(&self, id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM lists WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    pub async fn reorder_lists(&self, board_id: &str, list_ids: &[String]) -> DbResult<()> {
        for (position, list_id) in list_ids.iter().enumerate() {
            sqlx::query(
                "UPDATE lists SET position = $1, updated_at = $2 WHERE id = $3 AND board_id = $4"
            )
            .bind(position as i32)
            .bind(Utc::now())
            .bind(list_id)
            .bind(board_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn archive_list(&self, id: &str, archived: bool) -> DbResult<List> {
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE lists SET archived = $1, updated_at = $2 WHERE id = $3"
        )
        .bind(archived)
        .bind(&updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_list(id).await
    }

    pub async fn get_archived_lists(&self, board_id: &str) -> DbResult<Vec<List>> {
        let rows = sqlx::query_as::<_, List>(
            r#"SELECT id, board_id, name, position, archived, created_at, updated_at 
               FROM lists WHERE board_id = $1 AND archived = TRUE ORDER BY position ASC"#
        )
        .bind(board_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}
