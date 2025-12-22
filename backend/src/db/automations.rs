use super::{Database, DbError, DbResult};
use crate::models::AutomationRule;
use chrono::Utc;
use serde_json::Value;
use log::error;

impl Database {
    pub async fn get_automations(&self, board_id: &str) -> DbResult<Vec<AutomationRule>> {
        let rows = sqlx::query_as::<_, AutomationRule>(
            r#"SELECT id, board_id, name, enabled, trigger_type, trigger_config, action_type, action_config, last_run_at, created_at, updated_at 
               FROM automation_rules WHERE board_id = $1 ORDER BY created_at ASC"#
        )
        .bind(board_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn get_automation(&self, id: &str) -> DbResult<AutomationRule> {
        let rule = sqlx::query_as::<_, AutomationRule>(
            r#"SELECT id, board_id, name, enabled, trigger_type, trigger_config, action_type, action_config, last_run_at, created_at, updated_at 
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
            "INSERT INTO automation_rules (id, board_id, name, enabled, trigger_type, trigger_config, action_type, action_config, last_run_at, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
        )
        .bind(&rule.id)
        .bind(&rule.board_id)
        .bind(&rule.name)
        .bind(&rule.enabled)
        .bind(&rule.trigger_type)
        .bind(&rule.trigger_config)
        .bind(&rule.action_type)
        .bind(&rule.action_config)
        .bind(&rule.last_run_at)
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
            r#"SELECT id, board_id, name, enabled, trigger_type, trigger_config, action_type, action_config, last_run_at, created_at, updated_at 
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

    /// Set last_run_at timestamp for an automation rule
    pub async fn set_automation_last_run(&self, id: &str, when: chrono::DateTime<Utc>) -> DbResult<()> {
        sqlx::query("UPDATE automation_rules SET last_run_at = $1 WHERE id = $2")
            .bind(when)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Run due interval automations across all boards. This checks each enabled automation with trigger_type='interval'
    /// and runs it if the configured interval has elapsed since last_run_at. trigger_config should include { "interval_seconds": 300 }
    pub async fn run_due_interval_automations(&self) -> DbResult<()> {
        // Fetch all enabled interval automations
        let rows = sqlx::query_as::<_, AutomationRule>(
            r#"SELECT id, board_id, name, enabled, trigger_type, trigger_config, action_type, action_config, last_run_at, created_at, updated_at
               FROM automation_rules WHERE trigger_type = 'interval' AND enabled = TRUE"#
        )
        .fetch_all(&self.pool)
        .await?;

        let now = Utc::now();
        for rule in rows {
            // Expect trigger_config.interval_seconds
            if let Some(interval_val) = rule.trigger_config.get("interval_seconds") {
                if let Some(interval_num) = interval_val.as_i64() {
                    let due = match rule.last_run_at {
                        Some(last) => (now - last).num_seconds() >= interval_num,
                        None => true,
                    };

                    if due {
                        // Execute without a specific context
                        if let Err(e) = self.execute_automation(&rule, &serde_json::json!({})).await {
                            error!("Failed to execute interval automation {}: {}", rule.id, e);
                        } else {
                            // update last_run_at
                            let _ = self.set_automation_last_run(&rule.id, now).await;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Run all enabled automations for a given trigger on a board.
    /// `context` should contain any relevant keys (e.g. { "card_id": "..." }).
    pub async fn run_automations_for_trigger(&self, board_id: &str, trigger_type: &str, context: Value) -> DbResult<()> {
        let rules = self.get_enabled_automations_by_trigger(board_id, trigger_type).await?;

        for rule in rules {
            // execute each rule but don't stop on single failures
            if let Err(e) = self.execute_automation(&rule, &context).await {
                error!("Failed to execute automation {}: {}", rule.id, e);
            }
        }

        Ok(())
    }

    async fn execute_automation(&self, rule: &AutomationRule, context: &Value) -> DbResult<()> {
        match rule.action_type.as_str() {
            "set_status" => {
                // Expect action_config to have { status: "done" }
                if let Some(status_val) = rule.action_config.get("status") {
                    if let Some(status_str) = status_val.as_str() {
                        // Expect context to include card_id
                        if let Some(card_id_val) = context.get("card_id") {
                            if let Some(card_id) = card_id_val.as_str() {
                                // Call update_card to set status. Ignore other fields.
                                let _ = self.update_card(
                                    card_id,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    None,
                                    Some(status_str.to_string()),
                                ).await?;
                                return Ok(());
                            }
                        }
                    }
                }
                // If we reach here, action config or context incomplete
                Err(DbError::InvalidData("Invalid set_status automation config or missing card_id in context".into()))
            }
            "archive_card" => {
                // archive the card
                if let Some(card_id_val) = context.get("card_id") {
                    if let Some(card_id) = card_id_val.as_str() {
                        let _ = self.archive_card(card_id, true).await?;
                        return Ok(());
                    }
                }
                Err(DbError::InvalidData("Missing card_id in context for archive_card".into()))
            }
            "move_to_list" => {
                // move the card to a target list (append to end)
                if let Some(action_cfg) = rule.action_config.get("target_list_id") {
                    if let Some(target_list_id) = action_cfg.as_str() {
                        if let Some(card_id_val) = context.get("card_id") {
                            if let Some(card_id) = card_id_val.as_str() {
                                // determine next position in target list
                                let pos = self.get_next_card_position(target_list_id).await?;
                                let _ = self.move_card(card_id, target_list_id, pos).await?;
                                return Ok(());
                            }
                        }
                    }
                }
                Err(DbError::InvalidData("Missing target_list_id in action_config or card_id in context for move_to_list".into()))
            }
            "add_label" => {
                if let Some(label_val) = rule.action_config.get("label_id") {
                    if let Some(label_id) = label_val.as_str() {
                        if let Some(card_id_val) = context.get("card_id") {
                            if let Some(card_id) = card_id_val.as_str() {
                                let mut card = self.get_card(card_id).await?;
                                if !card.labels.contains(&label_id.to_string()) {
                                    card.labels.push(label_id.to_string());
                                    let _ = self.update_card(
                                        card_id,
                                        None,
                                        None,
                                        None,
                                        None,
                                        Some(card.labels.clone()),
                                        None,
                                        None,
                                        None,
                                    ).await?;
                                }
                                return Ok(());
                            }
                        }
                    }
                }
                Err(DbError::InvalidData("Missing label_id in action_config or card_id in context for add_label".into()))
            }
            "remove_label" => {
                if let Some(label_val) = rule.action_config.get("label_id") {
                    if let Some(label_id) = label_val.as_str() {
                        if let Some(card_id_val) = context.get("card_id") {
                            if let Some(card_id) = card_id_val.as_str() {
                                let mut card = self.get_card(card_id).await?;
                                card.labels.retain(|l| l != label_id);
                                let _ = self.update_card(
                                    card_id,
                                    None,
                                    None,
                                    None,
                                    None,
                                    Some(card.labels.clone()),
                                    None,
                                    None,
                                    None,
                                ).await?;
                                return Ok(());
                            }
                        }
                    }
                }
                Err(DbError::InvalidData("Missing label_id in action_config or card_id in context for remove_label".into()))
            }
            _ => {
                // Unknown/unimplemented action types are ignored for now
                Ok(())
            }
        }
    }
}
