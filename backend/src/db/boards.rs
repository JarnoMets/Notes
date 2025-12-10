use super::{Database, DbError, DbResult};
use crate::models::{Board, BoardFolder, BoardsTree, BoardWithLists, ListWithCards};
use chrono::Utc;

impl Database {
    // ============ Board Folders ============

    pub async fn get_board_folders(&self, user_id: &str) -> DbResult<Vec<BoardFolder>> {
        let rows = sqlx::query_as::<_, BoardFolder>(
            r#"SELECT id, user_id, parent_id, name, position, is_important, is_urgent, created_at, updated_at 
               FROM board_folders WHERE user_id = $1 ORDER BY position ASC"#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_board_folder(&self, id: &str, user_id: &str) -> DbResult<BoardFolder> {
        let folder = sqlx::query_as::<_, BoardFolder>(
            r#"SELECT id, user_id, parent_id, name, position, is_important, is_urgent, created_at, updated_at 
               FROM board_folders WHERE id = $1 AND user_id = $2"#
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(folder)
    }

    pub async fn create_board_folder(&self, folder: &BoardFolder) -> DbResult<BoardFolder> {
        sqlx::query(
            "INSERT INTO board_folders (id, user_id, parent_id, name, position, is_important, is_urgent, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(&folder.id)
        .bind(&folder.user_id)
        .bind(&folder.parent_id)
        .bind(&folder.name)
        .bind(&folder.position)
        .bind(&folder.is_important)
        .bind(&folder.is_urgent)
        .bind(&folder.created_at)
        .bind(&folder.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(folder.clone())
    }

    pub async fn update_board_folder(&self, id: &str, user_id: &str, name: Option<String>, parent_id: Option<Option<String>>, position: Option<i32>, is_important: Option<bool>, is_urgent: Option<bool>) -> DbResult<BoardFolder> {
        let existing = self.get_board_folder(id, user_id).await?;
        
        let new_name = name.unwrap_or(existing.name.clone());
        let new_parent_id = parent_id.clone().unwrap_or(existing.parent_id.clone());
        let new_position = position.unwrap_or(existing.position);
        let new_is_important = is_important.unwrap_or(existing.is_important);
        let new_is_urgent = is_urgent.unwrap_or(existing.is_urgent);
        let updated_at = Utc::now();

        // If position or parent is changing, we need to reorder
        if position.is_some() || parent_id.is_some() {
            let old_parent = existing.parent_id.clone();
            let target_parent = new_parent_id.clone();
            
            // First, shift items in the target parent to make room at new_position
            match target_parent.as_ref() {
                Some(pid) => {
                    sqlx::query(
                        "UPDATE board_folders SET position = position + 1, updated_at = $1 
                         WHERE user_id = $2 AND parent_id = $3 AND position >= $4 AND id != $5"
                    )
                    .bind(&updated_at)
                    .bind(user_id)
                    .bind(pid)
                    .bind(new_position)
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
                }
                None => {
                    sqlx::query(
                        "UPDATE board_folders SET position = position + 1, updated_at = $1 
                         WHERE user_id = $2 AND parent_id IS NULL AND position >= $3 AND id != $4"
                    )
                    .bind(&updated_at)
                    .bind(user_id)
                    .bind(new_position)
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
                }
            }
            
            // If moving from a different parent, compact the old parent
            if old_parent != target_parent {
                match old_parent.as_ref() {
                    Some(pid) => {
                        sqlx::query(
                            "UPDATE board_folders SET position = position - 1, updated_at = $1 
                             WHERE user_id = $2 AND parent_id = $3 AND position > $4"
                        )
                        .bind(&updated_at)
                        .bind(user_id)
                        .bind(pid)
                        .bind(existing.position)
                        .execute(&self.pool)
                        .await?;
                    }
                    None => {
                        sqlx::query(
                            "UPDATE board_folders SET position = position - 1, updated_at = $1 
                             WHERE user_id = $2 AND parent_id IS NULL AND position > $3"
                        )
                        .bind(&updated_at)
                        .bind(user_id)
                        .bind(existing.position)
                        .execute(&self.pool)
                        .await?;
                    }
                }
            }
        }

        sqlx::query(
            "UPDATE board_folders SET name = $1, parent_id = $2, position = $3, is_important = $4, is_urgent = $5, updated_at = $6 WHERE id = $7 AND user_id = $8"
        )
        .bind(&new_name)
        .bind(&new_parent_id)
        .bind(&new_position)
        .bind(&new_is_important)
        .bind(&new_is_urgent)
        .bind(&updated_at)
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        self.get_board_folder(id, user_id).await
    }

    pub async fn delete_board_folder(&self, id: &str, user_id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM board_folders WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    pub async fn get_max_board_folder_position(&self, user_id: &str, parent_id: Option<&str>) -> DbResult<i32> {
        let max: Option<i32> = match parent_id {
            Some(pid) => {
                sqlx::query_scalar("SELECT MAX(position) FROM board_folders WHERE user_id = $1 AND parent_id = $2")
                    .bind(user_id)
                    .bind(pid)
                    .fetch_one(&self.pool)
                    .await?
            }
            None => {
                sqlx::query_scalar("SELECT MAX(position) FROM board_folders WHERE user_id = $1 AND parent_id IS NULL")
                    .bind(user_id)
                    .fetch_one(&self.pool)
                    .await?
            }
        };
        Ok(max.unwrap_or(-1) + 1)
    }

    // ============ Boards ============

    pub async fn get_boards(&self, user_id: &str) -> DbResult<Vec<Board>> {
        let rows = sqlx::query_as::<_, Board>(
            r#"SELECT id, user_id, folder_id, name, description, color, position, is_important, is_urgent, created_at, updated_at 
               FROM boards WHERE user_id = $1 ORDER BY position ASC"#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_boards_tree(&self, user_id: &str) -> DbResult<BoardsTree> {
        let folders = self.get_board_folders(user_id).await?;
        let boards = self.get_boards(user_id).await?;
        Ok(BoardsTree { folders, boards })
    }

    pub async fn get_board(&self, id: &str, user_id: &str) -> DbResult<Board> {
        let board = sqlx::query_as::<_, Board>(
            r#"SELECT id, user_id, folder_id, name, description, color, position, is_important, is_urgent, created_at, updated_at 
               FROM boards WHERE id = $1 AND user_id = $2"#
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(board)
    }

    pub async fn get_board_with_lists(&self, id: &str, user_id: &str) -> DbResult<BoardWithLists> {
        let board = self.get_board(id, user_id).await?;
        let lists = self.get_lists_active(&board.id).await?;
        let labels = self.get_labels(&board.id).await?;
        let automations = self.get_automations(&board.id).await?;
        
        let mut lists_with_cards = Vec::new();
        for list in lists {
            let cards = self.get_cards_active(&list.id).await?;
            lists_with_cards.push(ListWithCards { list, cards });
        }

        Ok(BoardWithLists {
            board,
            lists: lists_with_cards,
            labels,
            automations,
        })
    }

    #[allow(dead_code)]
    pub async fn get_board_with_all(&self, id: &str, user_id: &str) -> DbResult<BoardWithLists> {
        // Includes archived items too
        let board = self.get_board(id, user_id).await?;
        let lists = self.get_lists(&board.id).await?;
        let labels = self.get_labels(&board.id).await?;
        let automations = self.get_automations(&board.id).await?;
        
        let mut lists_with_cards = Vec::new();
        for list in lists {
            let cards = self.get_cards(&list.id).await?;
            lists_with_cards.push(ListWithCards { list, cards });
        }

        Ok(BoardWithLists {
            board,
            lists: lists_with_cards,
            labels,
            automations,
        })
    }

    pub async fn create_board(&self, board: &Board) -> DbResult<Board> {
        sqlx::query(
            "INSERT INTO boards (id, user_id, folder_id, name, description, color, position, is_important, is_urgent, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
        )
        .bind(&board.id)
        .bind(&board.user_id)
        .bind(&board.folder_id)
        .bind(&board.name)
        .bind(&board.description)
        .bind(&board.color)
        .bind(&board.position)
        .bind(&board.is_important)
        .bind(&board.is_urgent)
        .bind(&board.created_at)
        .bind(&board.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(board.clone())
    }

    pub async fn update_board(
        &self, 
        id: &str,
        user_id: &str,
        name: Option<String>, 
        description: Option<String>,
        color: Option<String>,
        folder_id: Option<Option<String>>,
        position: Option<i32>,
        is_important: Option<bool>,
        is_urgent: Option<bool>,
    ) -> DbResult<Board> {
        let existing = self.get_board(id, user_id).await?;
        
        let new_name = name.unwrap_or(existing.name.clone());
        let new_description = description.or(existing.description.clone());
        let new_color = color.or(existing.color.clone());
        let new_folder_id = folder_id.clone().unwrap_or(existing.folder_id.clone());
        let new_position = position.unwrap_or(existing.position);
        let new_is_important = is_important.unwrap_or(existing.is_important);
        let new_is_urgent = is_urgent.unwrap_or(existing.is_urgent);
        let updated_at = Utc::now();

        // If position or folder is changing, we need to reorder
        if position.is_some() || folder_id.is_some() {
            let old_folder = existing.folder_id.clone();
            let target_folder = new_folder_id.clone();
            
            // First, shift items in the target folder to make room at new_position
            match target_folder.as_ref() {
                Some(fid) => {
                    sqlx::query(
                        "UPDATE boards SET position = position + 1, updated_at = $1 
                         WHERE user_id = $2 AND folder_id = $3 AND position >= $4 AND id != $5"
                    )
                    .bind(&updated_at)
                    .bind(user_id)
                    .bind(fid)
                    .bind(new_position)
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
                }
                None => {
                    sqlx::query(
                        "UPDATE boards SET position = position + 1, updated_at = $1 
                         WHERE user_id = $2 AND folder_id IS NULL AND position >= $3 AND id != $4"
                    )
                    .bind(&updated_at)
                    .bind(user_id)
                    .bind(new_position)
                    .bind(id)
                    .execute(&self.pool)
                    .await?;
                }
            }
            
            // If moving from a different folder, compact the old folder
            if old_folder != target_folder {
                match old_folder.as_ref() {
                    Some(fid) => {
                        sqlx::query(
                            "UPDATE boards SET position = position - 1, updated_at = $1 
                             WHERE user_id = $2 AND folder_id = $3 AND position > $4"
                        )
                        .bind(&updated_at)
                        .bind(user_id)
                        .bind(fid)
                        .bind(existing.position)
                        .execute(&self.pool)
                        .await?;
                    }
                    None => {
                        sqlx::query(
                            "UPDATE boards SET position = position - 1, updated_at = $1 
                             WHERE user_id = $2 AND folder_id IS NULL AND position > $3"
                        )
                        .bind(&updated_at)
                        .bind(user_id)
                        .bind(existing.position)
                        .execute(&self.pool)
                        .await?;
                    }
                }
            }
        }

        sqlx::query(
            "UPDATE boards SET name = $1, description = $2, color = $3, folder_id = $4, position = $5, is_important = $6, is_urgent = $7, updated_at = $8 WHERE id = $9 AND user_id = $10"
        )
        .bind(&new_name)
        .bind(&new_description)
        .bind(&new_color)
        .bind(&new_folder_id)
        .bind(&new_position)
        .bind(&new_is_important)
        .bind(&new_is_urgent)
        .bind(&updated_at)
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        self.get_board(id, user_id).await
    }

    pub async fn get_max_board_position(&self, user_id: &str, folder_id: Option<&str>) -> DbResult<i32> {
        let max: Option<i32> = match folder_id {
            Some(fid) => {
                sqlx::query_scalar("SELECT MAX(position) FROM boards WHERE user_id = $1 AND folder_id = $2")
                    .bind(user_id)
                    .bind(fid)
                    .fetch_one(&self.pool)
                    .await?
            }
            None => {
                sqlx::query_scalar("SELECT MAX(position) FROM boards WHERE user_id = $1 AND folder_id IS NULL")
                    .bind(user_id)
                    .fetch_one(&self.pool)
                    .await?
            }
        };
        Ok(max.unwrap_or(-1) + 1)
    }

    pub async fn delete_board(&self, id: &str, user_id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM boards WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }
}
