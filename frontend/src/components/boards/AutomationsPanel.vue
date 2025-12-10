<template>
  <div v-if="visible" class="side-panel-overlay" @click.self="$emit('close')">
    <div class="side-panel side-panel-wide">
      <div class="side-panel-header">
        <h3>Automations</h3>
        <button class="modal-close" @click="$emit('close')">
          <Icon name="x" :size="14" />
        </button>
      </div>
      <div class="side-panel-content">
        <div class="automations-list">
          <div 
            v-for="automation in automations" 
            :key="automation.id" 
            class="automation-item"
            :class="{ disabled: !automation.enabled }"
          >
            <div class="automation-info">
              <div class="automation-name">{{ automation.name }}</div>
              <div class="automation-description">
                When <strong>{{ getTriggerLabel(automation.trigger_type) }}</strong> 
                → <strong>{{ getActionLabel(automation.action_type) }}</strong>
              </div>
            </div>
            <div class="automation-actions">
              <button class="btn btn-sm" @click="$emit('toggle', automation.id)">
                {{ automation.enabled ? 'Disable' : 'Enable' }}
              </button>
              <button class="btn btn-sm btn-danger" @click="$emit('delete', automation.id)">
                <Icon name="trash" :size="12" />
              </button>
            </div>
          </div>
        </div>
        
        <div class="create-automation-form">
          <h4>Create Automation</h4>
          <div class="form-group">
            <label>Name</label>
            <input v-model="newAutomation.name" placeholder="e.g., Move to Done when complete" />
          </div>
          <div class="form-group">
            <label>When...</label>
            <select v-model="newAutomation.trigger_type">
              <option value="card_moved">Card is moved to a list</option>
              <option value="due_date_passed">Due date passes</option>
              <option value="label_added">Label is added</option>
              <option value="card_created">Card is created</option>
            </select>
            <div v-if="newAutomation.trigger_type === 'card_moved'" class="trigger-config">
              <select v-model="newAutomation.trigger_config.target_list_id">
                <option value="">Select list...</option>
                <option v-for="lwc in lists" :key="lwc.list.id" :value="lwc.list.id">
                  {{ lwc.list.name }}
                </option>
              </select>
            </div>
            <div v-if="newAutomation.trigger_type === 'label_added'" class="trigger-config">
              <select v-model="newAutomation.trigger_config.label_id">
                <option value="">Select label...</option>
                <option v-for="label in labels" :key="label.id" :value="label.id">
                  {{ label.name }}
                </option>
              </select>
            </div>
          </div>
          <div class="form-group">
            <label>Then...</label>
            <select v-model="newAutomation.action_type">
              <option value="move_to_list">Move card to list</option>
              <option value="add_label">Add label</option>
              <option value="remove_label">Remove label</option>
              <option value="archive_card">Archive card</option>
            </select>
            <div v-if="newAutomation.action_type === 'move_to_list'" class="action-config">
              <select v-model="newAutomation.action_config.target_list_id">
                <option value="">Select list...</option>
                <option v-for="lwc in lists" :key="lwc.list.id" :value="lwc.list.id">
                  {{ lwc.list.name }}
                </option>
              </select>
            </div>
            <div v-if="newAutomation.action_type === 'add_label' || newAutomation.action_type === 'remove_label'" class="action-config">
              <select v-model="newAutomation.action_config.label_id">
                <option value="">Select label...</option>
                <option v-for="label in labels" :key="label.id" :value="label.id">
                  {{ label.name }}
                </option>
              </select>
            </div>
          </div>
          <button 
            class="btn btn-primary" 
            @click="handleCreate" 
            :disabled="!isValid"
          >
            Create Automation
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { AutomationRule, BoardLabel, ListWithCards } from '../../types'

// Alias for component use
type Automation = AutomationRule
import Icon from '../Icon.vue'

defineProps<{
  visible: boolean
  automations: Automation[]
  lists: ListWithCards[]
  labels: BoardLabel[]
}>()

const emit = defineEmits<{
  close: []
  create: [automation: Omit<Automation, 'id' | 'board_id' | 'enabled' | 'created_at' | 'updated_at'>]
  toggle: [automationId: string]
  delete: [automationId: string]
}>()

const newAutomation = ref({
  name: '',
  trigger_type: 'card_moved',
  trigger_config: {} as Record<string, unknown>,
  action_type: 'move_to_list',
  action_config: {} as Record<string, unknown>
})

const isValid = computed(() => {
  if (!newAutomation.value.name.trim()) return false
  if (!newAutomation.value.trigger_type) return false
  if (!newAutomation.value.action_type) return false
  
  // Check trigger config
  if (newAutomation.value.trigger_type === 'card_moved' && !newAutomation.value.trigger_config.target_list_id) return false
  if (newAutomation.value.trigger_type === 'label_added' && !newAutomation.value.trigger_config.label_id) return false
  
  // Check action config
  if (newAutomation.value.action_type === 'move_to_list' && !newAutomation.value.action_config.target_list_id) return false
  if ((newAutomation.value.action_type === 'add_label' || newAutomation.value.action_type === 'remove_label') && !newAutomation.value.action_config.label_id) return false
  
  return true
})

function getTriggerLabel(type: string): string {
  const labels: Record<string, string> = {
    card_moved: 'card is moved to a list',
    due_date_passed: 'due date passes',
    label_added: 'label is added',
    card_created: 'card is created'
  }
  return labels[type] || type
}

function getActionLabel(type: string): string {
  const labels: Record<string, string> = {
    move_to_list: 'move card to list',
    add_label: 'add label',
    remove_label: 'remove label',
    archive_card: 'archive card'
  }
  return labels[type] || type
}

function handleCreate() {
  if (!isValid.value) return
  emit('create', {
    name: newAutomation.value.name,
    trigger_type: newAutomation.value.trigger_type,
    trigger_config: newAutomation.value.trigger_config,
    action_type: newAutomation.value.action_type,
    action_config: newAutomation.value.action_config
  })
  // Reset form
  newAutomation.value = {
    name: '',
    trigger_type: 'card_moved',
    trigger_config: {},
    action_type: 'move_to_list',
    action_config: {}
  }
}
</script>

<style scoped>
.automations-list {
  margin-bottom: 1.5rem;
}

.automation-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 6px;
  margin-bottom: 0.5rem;
}

.automation-item.disabled {
  opacity: 0.6;
}

.automation-info {
  flex: 1;
}

.automation-name {
  font-weight: 600;
  margin-bottom: 0.25rem;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.automation-description {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.automation-description strong {
  color: var(--text-secondary);
}

.automation-actions {
  display: flex;
  gap: 0.25rem;
}

.create-automation-form {
  background: var(--bg-tertiary);
  padding: 1rem;
  border-radius: 8px;
}

.create-automation-form h4 {
  margin: 0 0 1rem;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.create-automation-form .form-group {
  margin-bottom: 1rem;
}

.create-automation-form label {
  display: block;
  margin-bottom: 0.25rem;
  font-weight: 500;
  font-size: 0.85rem;
  color: var(--text-primary);
}

.create-automation-form input,
.create-automation-form select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.trigger-config,
.action-config {
  margin-top: 0.5rem;
}

@media (max-width: 768px) {
  .automation-item {
    flex-direction: column;
    gap: 0.75rem;
  }

  .automation-actions {
    width: 100%;
    justify-content: flex-end;
  }

  .create-automation-form {
    padding: 0.75rem;
  }
}
</style>
