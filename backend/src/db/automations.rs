use super::{Database, DbError, DbResult};
use crate::models::AutomationRule;
use chrono::Utc;

impl Database {
    pub async fn get_automations(&self, board_id: &str) -> DbResult<Vec<AutomationRule>> {
        let rows = sqlx::query_as::<_, AutomationRule>(
            r#"SELECT id, board_id, name, enabled, trigger_type, trigger_config, action_type, action_config, created_at, updated_at 
               FROM automation_rules WHERE board_id = $1 ORDER BY created_at ASC"#
        )
        .bind(board_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_automation(&self, id: &str) -> DbResult<AutomationRule> {
        let rule = sqlx::query_as::<_, AutomationRule>(
            r#"SELECT id, board_id, name, enabled, trigger_type, trigger_config, action_type, action_config, created_at, updated_at 
               FROM automation_rules WHERE id = $1"#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DbError::NotFound)?;

        Ok(rule)
    }

    pub async fn create_automation(&self, rule: &AutomationRule) -> DbResult<AutomationRule> {
        sqlx::query(
            "INSERT INTO automation_rules (id, board_id, name, enabled, trigger_type, trigger_config, action_type, action_config, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
        )
        .bind(&rule.id)
        .bind(&rule.board_id)
        .bind(&rule.name)
        .bind(&rule.enabled)
        .bind(&rule.trigger_type)
        .bind(&rule.trigger_config)
        .bind(&rule.action_type)
        .bind(&rule.action_config)
        .bind(&rule.created_at)
        .bind(&rule.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(rule.clone())
    }

    pub async fn update_automation(
        &self,
        id: &str,
        name: Option<String>,
        enabled: Option<bool>,
        trigger_type: Option<String>,
        trigger_config: Option<serde_json::Value>,
        action_type: Option<String>,
        action_config: Option<serde_json::Value>,
    ) -> DbResult<AutomationRule> {
        let existing = self.get_automation(id).await?;
        
        let new_name = name.unwrap_or(existing.name);
        let new_enabled = enabled.unwrap_or(existing.enabled);
        let new_trigger_type = trigger_type.unwrap_or(existing.trigger_type);
        let new_trigger_config = trigger_config.unwrap_or(existing.trigger_config);
        let new_action_type = action_type.unwrap_or(existing.action_type);
        let new_action_config = action_config.unwrap_or(existing.action_config);
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE automation_rules SET name = $1, enabled = $2, trigger_type = $3, trigger_config = $4, action_type = $5, action_config = $6, updated_at = $7 WHERE id = $8"
        )
        .bind(&new_name)
        .bind(&new_enabled)
        .bind(&new_trigger_type)
        .bind(&new_trigger_config)
        .bind(&new_action_type)
        .bind(&new_action_config)
        .bind(&updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_automation(id).await
    }

    pub async fn delete_automation(&self, id: &str) -> DbResult<()> {
        let result = sqlx::query("DELETE FROM automation_rules WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DbError::NotFound);
        }

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_enabled_automations_by_trigger(&self, board_id: &str, trigger_type: &str) -> DbResult<Vec<AutomationRule>> {
        let rows = sqlx::query_as::<_, AutomationRule>(
            r#"SELECT id, board_id, name, enabled, trigger_type, trigger_config, action_type, action_config, created_at, updated_at 
               FROM automation_rules WHERE board_id = $1 AND trigger_type = $2 AND enabled = TRUE"#
        )
        .bind(board_id)
        .bind(trigger_type)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn toggle_automation(&self, id: &str) -> DbResult<AutomationRule> {
        let existing = self.get_automation(id).await?;
        let updated_at = Utc::now();

        sqlx::query(
            "UPDATE automation_rules SET enabled = $1, updated_at = $2 WHERE id = $3"
        )
        .bind(!existing.enabled)
        .bind(&updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.get_automation(id).await
    }
}
