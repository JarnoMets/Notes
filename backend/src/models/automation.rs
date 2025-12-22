//! Automation rule models for boards

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Automation rule for a board
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct AutomationRule {
    pub id: String,
    pub board_id: String,
    pub name: String,
    pub enabled: bool,
    pub trigger_type: String,
    pub trigger_config: serde_json::Value,
    pub action_type: String,
    pub action_config: serde_json::Value,
    pub last_run_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AutomationRule {
    pub fn new(
        board_id: String,
        name: String,
        trigger_type: String,
        trigger_config: serde_json::Value,
        action_type: String,
        action_config: serde_json::Value,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            board_id,
            name,
            enabled: true,
            trigger_type,
            trigger_config,
            action_type,
            action_config,
            last_run_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Request body for creating an automation
#[derive(Debug, Deserialize)]
pub struct CreateAutomationRequest {
    pub name: String,
    pub trigger_type: String,
    pub trigger_config: serde_json::Value,
    pub action_type: String,
    pub action_config: serde_json::Value,
}

/// Request body for updating an automation
#[derive(Debug, Deserialize)]
pub struct UpdateAutomationRequest {
    pub name: Option<String>,
    pub enabled: Option<bool>,
    pub trigger_type: Option<String>,
    pub trigger_config: Option<serde_json::Value>,
    pub action_type: Option<String>,
    pub action_config: Option<serde_json::Value>,
}
