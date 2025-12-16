use super::{Database, DbError, DbResult};
use crate::models::{Note, NoteAttachment, NoteFolder, NotesTree, NoteWithAttachments, NoteRevision};
use diffy::{apply, Patch};
use sqlx::Row;
use chrono::Utc;

impl Database {
    // ============ Folders ============

    pub async fn get_folders(&self, user_id: &str) -> DbResult<Vec<NoteFolder>> {
        let rows = sqlx::query_as::<_, NoteFolder>(
            r#"SELECT id, user_id, parent_id, name, position, is_important, is_urgent, created_at, updated_at 
               FROM note_folders WHERE user_id = $1 ORDER BY position ASC"#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_folder(&self, id: &str, user_id: &str) -> DbResult<NoteFolder> {
        let folder = sqlx::query_as::<_, NoteFolder>(
            r#"SELECT id, user_id, parent_id, name, position, is_important, is_urgent, created_at, updated_at 
               FROM note_folders WHERE id = $1 AND user_id = $2"#
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(folder)
    }

    pub async fn create_folder(&self, folder: &NoteFolder) -> DbResult<NoteFolder> {
        sqlx::query(
            "INSERT INTO note_folders (id, user_id, parent_id, name, position, is_important, is_urgent, created_at, updated_at) 
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

    pub async fn update_folder(&self, id: &str, user_id: &str, name: Option<String>, parent_id: Option<Option<String>>, position: Option<i32>, is_important: Option<bool>, is_urgent: Option<bool>) -> DbResult<NoteFolder> {
        let existing = self.get_folder(id, user_id).await?;
        
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
                        "UPDATE note_folders SET position = position + 1, updated_at = $1 
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
                        "UPDATE note_folders SET position = position + 1, updated_at = $1 
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
                            "UPDATE note_folders SET position = position - 1, updated_at = $1 
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
                            "UPDATE note_folders SET position = position - 1, updated_at = $1 
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
            "UPDATE note_folders SET name = $1, parent_id = $2, position = $3, is_important = $4, is_urgent = $5, updated_at = $6 WHERE id = $7 AND user_id = $8"
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

        self.get_folder(id, user_id).await
    }

    pub async fn delete_folder(&self, id: &str, user_id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM note_folders WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    pub async fn get_max_folder_position(&self, user_id: &str, parent_id: Option<&str>) -> DbResult<i32> {
        let max: Option<i32> = match parent_id {
            Some(pid) => {
                sqlx::query_scalar("SELECT MAX(position) FROM note_folders WHERE user_id = $1 AND parent_id = $2")
                    .bind(user_id)
                    .bind(pid)
                    .fetch_one(&self.pool)
                    .await?
            }
            None => {
                sqlx::query_scalar("SELECT MAX(position) FROM note_folders WHERE user_id = $1 AND parent_id IS NULL")
                    .bind(user_id)
                    .fetch_one(&self.pool)
                    .await?
            }
        };
        Ok(max.unwrap_or(-1) + 1)
    }

    // ============ Notes ============

    pub async fn get_notes(&self, user_id: &str) -> DbResult<Vec<Note>> {
        let rows = sqlx::query_as::<_, Note>(
            r#"SELECT id, user_id, folder_id, title, description, content, position, is_important, is_urgent, created_at, updated_at 
               FROM notes WHERE user_id = $1 ORDER BY position ASC"#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_notes_tree(&self, user_id: &str) -> DbResult<NotesTree> {
        let folders = self.get_folders(user_id).await?;
        let notes = self.get_notes(user_id).await?;
        Ok(NotesTree { folders, notes })
    }

    pub async fn get_note(&self, id: &str, user_id: &str) -> DbResult<Note> {
        let note = sqlx::query_as::<_, Note>(
            r#"SELECT id, user_id, folder_id, title, description, content, position, is_important, is_urgent, created_at, updated_at 
               FROM notes WHERE id = $1 AND user_id = $2"#
        )
        .bind(id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(note)
    }

    pub async fn get_note_with_attachments(&self, id: &str, user_id: &str) -> DbResult<NoteWithAttachments> {
        let note = self.get_note(id, user_id).await?;
        let attachments = self.get_note_attachments(id).await?;
        
        Ok(NoteWithAttachments { note, attachments })
    }

    pub async fn create_note(&self, note: &Note) -> DbResult<Note> {
        sqlx::query(
            "INSERT INTO notes (id, user_id, folder_id, title, description, content, position, is_important, is_urgent, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
        )
        .bind(&note.id)
        .bind(&note.user_id)
        .bind(&note.folder_id)
        .bind(&note.title)
        .bind(&note.description)
        .bind(&note.content)
        .bind(&note.position)
        .bind(&note.is_important)
        .bind(&note.is_urgent)
        .bind(&note.created_at)
        .bind(&note.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(note.clone())
    }

    pub async fn create_note_revision(&self, revision: &NoteRevision) -> DbResult<NoteRevision> {
        sqlx::query(
            "INSERT INTO note_revisions (id, note_id, user_id, idx, forward_patch, reverse_patch, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&revision.id)
        .bind(&revision.note_id)
        .bind(&revision.user_id)
        .bind(revision.idx)
        .bind(&revision.forward_patch)
        .bind(&revision.reverse_patch)
        .bind(&revision.created_at)
        .execute(&self.pool)
        .await?;

        Ok(revision.clone())
    }

    pub async fn get_note_revisions(&self, note_id: &str, user_id: &str) -> DbResult<Vec<NoteRevision>> {
        let rows = sqlx::query_as::<_, NoteRevision>(
            "SELECT id, note_id, user_id, idx, forward_patch, reverse_patch, created_at FROM note_revisions WHERE note_id = $1 AND user_id = $2 ORDER BY idx ASC"
        )
        .bind(note_id)
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn delete_revisions_after(&self, note_id: &str, idx: i32) -> DbResult<()> {
        sqlx::query("DELETE FROM note_revisions WHERE note_id = $1 AND idx > $2")
            .bind(note_id)
            .bind(idx)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_revision_by_idx(&self, note_id: &str, idx: i32, user_id: &str) -> DbResult<NoteRevision> {
        let rev = sqlx::query_as::<_, NoteRevision>(
            "SELECT id, note_id, user_id, idx, forward_patch, reverse_patch, created_at FROM note_revisions WHERE note_id = $1 AND idx = $2 AND user_id = $3"
        )
        .bind(note_id)
        .bind(idx)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(rev)
    }

    pub async fn get_note_current_revision_index(&self, note_id: &str, user_id: &str) -> DbResult<i32> {
        let idx: Option<i32> = sqlx::query_scalar("SELECT current_revision_index FROM notes WHERE id = $1 AND user_id = $2")
            .bind(note_id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(idx.unwrap_or(-1))
    }

    pub async fn set_note_current_revision_index(&self, note_id: &str, idx: i32, user_id: &str) -> DbResult<()> {
        sqlx::query("UPDATE notes SET current_revision_index = $1, updated_at = $2 WHERE id = $3 AND user_id = $4")
            .bind(idx)
            .bind(Utc::now())
            .bind(note_id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn set_note_current_revision(&self, note_id: &str, revision_id: &str, user_id: &str) -> DbResult<()> {
        // Update notes table to set content/title/description to the revision's values and store current_revision_id if column exists
        // Fetch the target revision
        let rev = sqlx::query_as::<_, NoteRevision>(
            "SELECT id, note_id, user_id, idx, forward_patch, reverse_patch, created_at FROM note_revisions WHERE id = $1 AND user_id = $2"
        )
        .bind(revision_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        // Get current note content and current_revision_index
        let row = sqlx::query("SELECT content, COALESCE(current_revision_index, -1) FROM notes WHERE id = $1 AND user_id = $2")
            .bind(note_id)
            .bind(user_id)
            .fetch_one(&self.pool)
            .await?;

        let current_content: String = row.get(0);
        let current_idx: i32 = row.get(1);

        // Decide whether to apply forward or reverse patch
        // If target idx < current_idx => undo (apply reverse_patch)
        // Else => redo (apply forward_patch)
        let patch_str = if rev.idx < current_idx {
            rev.reverse_patch.clone()
        } else {
            rev.forward_patch.clone()
        };

        // Parse patch and apply using diffy::apply
        let patch = Patch::from_str(&patch_str).map_err(|e| DbError::InvalidData(format!("Patch parse error: {}", e)))?;
        let new_content = apply(&current_content, &patch).map_err(|e| DbError::InvalidData(format!("Patch apply error: {}", e)))?;

        // Update note content and set current_revision_index
        sqlx::query(
            "UPDATE notes SET content = $1, updated_at = $2, current_revision_index = $3 WHERE id = $4 AND user_id = $5"
        )
        .bind(&new_content)
        .bind(Utc::now())
        .bind(rev.idx)
        .bind(note_id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_note(&self, id: &str, user_id: &str, title: Option<String>, folder_id: Option<Option<String>>, description: Option<String>, content: Option<String>, position: Option<i32>, is_important: Option<bool>, is_urgent: Option<bool>) -> DbResult<Note> {
        let existing = self.get_note(id, user_id).await?;
        
        let new_title = title.unwrap_or(existing.title.clone());
        let new_folder_id = folder_id.clone().unwrap_or(existing.folder_id.clone());
        let new_description = description.unwrap_or(existing.description.clone());
        let new_content = content.unwrap_or(existing.content.clone());
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
                        "UPDATE notes SET position = position + 1, updated_at = $1 
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
                        "UPDATE notes SET position = position + 1, updated_at = $1 
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
                            "UPDATE notes SET position = position - 1, updated_at = $1 
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
                            "UPDATE notes SET position = position - 1, updated_at = $1 
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
            "UPDATE notes SET title = $1, folder_id = $2, description = $3, content = $4, position = $5, is_important = $6, is_urgent = $7, updated_at = $8 WHERE id = $9 AND user_id = $10"
        )
        .bind(&new_title)
        .bind(&new_folder_id)
        .bind(&new_description)
        .bind(&new_content)
        .bind(&new_position)
        .bind(&new_is_important)
        .bind(&new_is_urgent)
        .bind(&updated_at)
        .bind(id)
        .bind(user_id)
        .execute(&self.pool)
        .await?;

        self.get_note(id, user_id).await
    }

    pub async fn get_max_note_position(&self, user_id: &str, folder_id: Option<&str>) -> DbResult<i32> {
        let max: Option<i32> = match folder_id {
            Some(fid) => {
                sqlx::query_scalar("SELECT MAX(position) FROM notes WHERE user_id = $1 AND folder_id = $2")
                    .bind(user_id)
                    .bind(fid)
                    .fetch_one(&self.pool)
                    .await?
            }
            None => {
                sqlx::query_scalar("SELECT MAX(position) FROM notes WHERE user_id = $1 AND folder_id IS NULL")
                    .bind(user_id)
                    .fetch_one(&self.pool)
                    .await?
            }
        };
        Ok(max.unwrap_or(-1) + 1)
    }

    pub async fn delete_note(&self, id: &str, user_id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM notes WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    // Note Attachments
    pub async fn get_note_attachments(&self, note_id: &str) -> DbResult<Vec<NoteAttachment>> {
        let rows = sqlx::query_as::<_, NoteAttachment>(
            r#"SELECT id, note_id, filename, original_filename, mime_type, size, created_at 
               FROM note_attachments WHERE note_id = $1 ORDER BY created_at DESC"#
        )
        .bind(note_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_note_attachment(&self, id: &str) -> DbResult<NoteAttachment> {
        let attachment = sqlx::query_as::<_, NoteAttachment>(
            r#"SELECT id, note_id, filename, original_filename, mime_type, size, created_at 
               FROM note_attachments WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(attachment)
    }

    pub async fn create_note_attachment(&self, attachment: &NoteAttachment) -> DbResult<NoteAttachment> {
        sqlx::query(
            "INSERT INTO note_attachments (id, note_id, filename, original_filename, mime_type, size, created_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(&attachment.id)
        .bind(&attachment.note_id)
        .bind(&attachment.filename)
        .bind(&attachment.original_filename)
        .bind(&attachment.mime_type)
        .bind(&attachment.size)
        .bind(&attachment.created_at)
        .execute(&self.pool)
        .await?;

        Ok(attachment.clone())
    }

    pub async fn delete_note_attachment(&self, id: &str) -> DbResult<NoteAttachment> {
        let attachment = self.get_note_attachment(id).await?;
        
        sqlx::query("DELETE FROM note_attachments WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(attachment)
    }

    pub async fn verify_note_ownership(&self, note_id: &str, user_id: &str) -> DbResult<bool> {
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM notes WHERE id = $1 AND user_id = $2)"
        )
        .bind(note_id)
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(exists)
    }

    /// Get total storage used by a user (sum of all attachment sizes)
    pub async fn get_user_storage_used(&self, user_id: &str) -> DbResult<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            r#"SELECT COALESCE(SUM(na.size), 0) 
               FROM note_attachments na 
               JOIN notes n ON na.note_id = n.id 
               WHERE n.user_id = $1"#
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}
