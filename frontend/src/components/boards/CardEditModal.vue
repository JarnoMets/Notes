<template>
  <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal modal-large">
      <div class="modal-header">
        <h3>Edit Card</h3>
        <button class="modal-close" @click="$emit('close')">
          <Icon name="x" :size="14" />
        </button>
      </div>
      <form @submit.prevent="handleSubmit">
        <div class="form-group">
          <label for="editCardTitle">Title</label>
          <input 
            id="editCardTitle" 
            v-model="localCard.title" 
            type="text" 
            required 
          />
        </div>
        <div class="form-group">
          <label for="editCardDescription">Description</label>
          <textarea 
            id="editCardDescription" 
            v-model="localCard.description" 
            rows="6" 
            placeholder="Add a more detailed description..."
          ></textarea>
        </div>
        <div class="form-group">
          <label for="editCardDueDate">Due Date</label>
          <input 
            id="editCardDueDate" 
            v-model="localCard.due_date" 
            type="datetime-local" 
            @focus="onDueDateFocus"
            @change="onDueDateChange"
          />
        </div>
        <div class="form-group">
          <label>Labels</label>
          <div class="label-selector">
            <button 
              v-for="label in labels" 
              :key="label.id"
              type="button"
              class="label-option"
              :class="{ selected: localCard.labels.includes(label.id) }"
              :style="{ 
                backgroundColor: localCard.labels.includes(label.id) ? label.color : 'transparent', 
                borderColor: label.color 
              }"
              @click="toggleLabel(label.id)"
            >
              {{ label.name }}
            </button>
          </div>
        </div>
        
        <!-- Linked Items Section -->
        <div v-if="linkedItems.length > 0" class="form-group">
          <label>Linked Items</label>
          <div class="linked-items-list">
            <div 
              v-for="item in linkedItems" 
              :key="item.href"
              class="linked-item"
              @click="$emit('navigateToItem', item)"
            >
              <Icon :name="item.type === 'note' ? 'file' : 'board'" :size="14" />
              <span class="linked-item-name">{{ item.name }}</span>
              <button 
                type="button" 
                class="linked-item-remove" 
                @click.stop="removeLinkedItem(item)"
                title="Remove link"
              >
                <Icon name="x" :size="12" />
              </button>
            </div>
          </div>
        </div>
        
        <div class="modal-actions">
          <button type="button" class="btn btn-warning" @click="$emit('archive')">Archive</button>
          <button type="button" class="btn btn-danger" @click="$emit('delete')">Delete</button>
          <button type="button" class="btn btn-secondary" @click="$emit('close')">Cancel</button>
          <button type="submit" class="btn btn-primary">Save Changes</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import type { Card, BoardLabel } from '../../types'
import Icon from '../Icon.vue'
import { toDatetimeLocal, normalizeForInput } from '../../utils/dates'

export interface LinkedItem {
  type: 'note' | 'board'
  name: string
  id: string
  href: string
  fullMatch: string
}

const props = defineProps<{
  visible: boolean
  card: Card | null
  labels: BoardLabel[]
}>()

const emit = defineEmits<{
  close: []
  save: [card: Card]
  archive: []
  delete: []
  navigateToItem: [item: LinkedItem]
}>()

const localCard = ref<Card>({
  id: '',
  list_id: '',
  title: '',
  description: '',
  due_date: undefined,
  labels: [],
  position: 0,
  archived: false,
  created_at: '',
  updated_at: ''
})

// Reset form when modal opens
watch(() => props.visible, (visible) => {
  if (visible && props.card) {
    // Convert incoming ISO/UTC due_date to a value suitable for datetime-local input
    localCard.value = { ...props.card, due_date: props.card?.due_date ? normalizeForInput(props.card.due_date) : undefined }
  }
})

const linkedItems = computed((): LinkedItem[] => {
  if (!localCard.value.description) return []
  
  const items: LinkedItem[] = []
  const linkPattern = /📎?\s*\[([^\]]+)\]\((notes|boards):\/\/([^)]+)\)/g
  let match
  
  while ((match = linkPattern.exec(localCard.value.description)) !== null) {
    items.push({
      type: match[2] as 'note' | 'board',
      name: match[1],
      id: match[3],
      href: `${match[2]}://${match[3]}`,
      fullMatch: match[0]
    })
  }
  
  return items
})

function toggleLabel(labelId: string) {
  const index = localCard.value.labels.indexOf(labelId)
  if (index === -1) {
    localCard.value.labels.push(labelId)
  } else {
    localCard.value.labels.splice(index, 1)
  }
}

function removeLinkedItem(item: LinkedItem) {
  const newDescription = localCard.value.description?.replace(item.fullMatch, '').trim() || ''
  localCard.value.description = newDescription
}

function handleSubmit() {
  emit('save', localCard.value)
}

function onDueDateFocus() {
  if (!localCard.value.due_date) {
    // default to today at 18:00
    localCard.value.due_date = toDatetimeLocal(new Date(new Date().setHours(18, 0, 0, 0)))
  }
}

function onDueDateChange(e: Event) {
  const val = (e.target as HTMLInputElement).value
  if (!val) return
  // If user selected a date-only value or ended up at midnight, default to 18:00
  const normalized = normalizeForInput(val)
  if (normalized && normalized !== val) {
    localCard.value.due_date = normalized
  }
}
</script>

<style scoped>
.label-selector {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.label-option {
  padding: 0.3rem 0.75rem;
  border-radius: 4px;
  border: 2px solid;
  cursor: pointer;
  font-size: 0.85rem;
  transition: all 0.2s;
  background: transparent;
  color: var(--text-primary);
}

.label-option.selected {
  color: white;
}

.linked-items-list {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.linked-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.linked-item:hover {
  background: var(--bg-hover);
}

.linked-item-name {
  flex: 1;
  font-size: 0.875rem;
  color: var(--text-primary);
}

.linked-item-remove {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
  opacity: 0;
  transition: all 0.15s;
}

.linked-item:hover .linked-item-remove {
  opacity: 1;
}

.linked-item-remove:hover {
  background: color-mix(in srgb, var(--danger) 20%, transparent);
  color: var(--danger);
}

@media (max-width: 768px) {
  .label-selector {
    gap: 0.375rem;
  }

  .label-option {
    padding: 0.375rem 0.625rem;
    font-size: 0.8rem;
  }

  .linked-item {
    padding: 0.625rem;
  }

  .linked-item-remove {
    opacity: 1;
    width: 24px;
    height: 24px;
  }
}
</style>
