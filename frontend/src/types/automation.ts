// Automation types
export interface AutomationRule {
  id: string
  board_id: string
  name: string
  enabled: boolean
  trigger_type: string
  trigger_config: Record<string, unknown>
  action_type: string
  action_config: Record<string, unknown>
  created_at: string
  updated_at: string
}
