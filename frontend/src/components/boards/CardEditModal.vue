<template>
  <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal modal-large">
      <div class="modal-header">
            <h3>{{ isEditing ? 'Edit Card' : 'Card' }}</h3>
            <div class="modal-header-actions">
              <button v-if="!isEditing" class="btn btn-primary" @click="isEditing = true">Edit</button>
              <button v-else class="btn btn-primary" @click="handleSubmit">Save</button>
              <button class="modal-close" @click="$emit('close')">
                <Icon name="x" :size="14" />
              </button>
            </div>
          </div>
          <form @submit.prevent="handleSubmit">
        <div class="form-group">
          <label for="editCardTitle">Title</label>
          <div v-if="!isEditing" class="view-field">{{ localCard.title }}</div>
          <input v-else id="editCardTitle" v-model="localCard.title" type="text" required />
        </div>
        <div class="form-group">
          <label for="editCardDescription">Description</label>
          <div v-if="!isEditing" class="view-field description-view" v-html="localCard.description || '<em>No description</em>'"></div>
          <textarea v-else id="editCardDescription" v-model="localCard.description" rows="6" placeholder="Add a more detailed description..."></textarea>
        </div>
        <div class="form-group">
          <label for="editCardDueDate">Due Date</label>
          <div v-if="!isEditing" class="view-field">{{ formatDisplayDate(localCard.due_date) }}</div>
          <input v-else id="editCardDueDate" v-model="localCard.due_date" type="datetime-local" @focus="onDueDateFocus" @change="onDueDateChange" />
        </div>
        <!-- Labels and Status side-by-side on wide screens, stacked on mobile -->
        <div class="form-row">
          <div class="form-row">
            <div class="form-group">
              <label>Labels</label>
              <div v-if="!isEditing" class="label-view">
                <span v-for="label in labels" :key="label.id" class="label-chip" :style="{ backgroundColor: localCard.labels.includes(label.id) ? label.color : 'transparent', color: localCard.labels.includes(label.id) ? '#fff' : 'var(--text-muted)', border: '1px solid ' + label.color }">{{ label.name }}</span>
              </div>
              <div v-else class="label-selector">
                <button v-for="label in labels" :key="label.id" type="button" class="label-option" :class="{ selected: localCard.labels.includes(label.id) }" :style="{ backgroundColor: localCard.labels.includes(label.id) ? label.color : 'transparent', borderColor: label.color }" @click="toggleLabel(label.id)">
                  {{ label.name }}
                </button>
              </div>
            </div>

            <div class="form-group form-group--small">
              <label>Status</label>
              <div class="status-field">
                <div v-if="!isEditing" class="view-field">{{ localCard.status === 'done' ? 'Done' : 'Open' }}</div>
                <label v-else class="checkbox-label">
                  <input type="checkbox" v-model="isDone" />
                  <span class="checkbox-text">Done</span>
                </label>
              </div>
            </div>
          </div>

        </div>
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
          <button type="button" class="btn btn-warning" @click="openArchiveConfirm">Archive</button>
          <button type="button" class="btn btn-danger" @click="openDeleteConfirm">Delete</button>
          <button type="button" class="btn btn-secondary" @click="$emit('close')">Cancel</button>
          <button v-if="!isEditing" type="button" class="btn btn-primary" @click="isEditing = true">Edit</button>
          <button v-else type="submit" class="btn btn-primary">Save Changes</button>
        </div>

        <!-- Confirmation modals -->
        <ConfirmModal
          :visible="showArchiveConfirm"
          title="Archive Card"
          message="Are you sure you want to archive this card?"
          confirm-text="Archive"
          variant="primary"
          @confirm="handleArchiveConfirm"
          @cancel="showArchiveConfirm = false"
        />

        <ConfirmModal
          :visible="showDeleteConfirm"
          title="Delete Card"
          message="Delete this card? This action cannot be undone."
          confirm-text="Delete"
          variant="danger"
          @confirm="handleDeleteConfirm"
          @cancel="showDeleteConfirm = false"
        />
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import type { Card, BoardLabel } from '../../types'
import Icon from '../common/ui/Icon.vue'
import ConfirmModal from '../common/modals/ConfirmModal.vue'
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
  status: undefined,
  created_at: '',
  updated_at: ''
})

// Local editing state: start in view mode when modal opens
const isEditing = ref(false)

// Reset form when modal opens
watch(() => props.visible, (visible) => {
  if (visible && props.card) {
    // Convert incoming ISO/UTC due_date to a value suitable for datetime-local input
    localCard.value = { ...props.card, due_date: props.card?.due_date ? normalizeForInput(props.card.due_date) : undefined }
  }
})

// Reset editing mode when modal opens/closes
watch(() => props.visible, (visible) => {
  if (visible) {
    isEditing.value = false
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

const isDone = computed({
  get: () => localCard.value.status === 'done',
  set: (value: boolean) => {
    localCard.value.status = value ? 'done' : 'open'
  }
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
  // After saving, switch back to view mode
  isEditing.value = false
}

// Confirmation modal state
const showArchiveConfirm = ref(false)
const showDeleteConfirm = ref(false)

function openArchiveConfirm() {
  showArchiveConfirm.value = true
}

function openDeleteConfirm() {
  showDeleteConfirm.value = true
}

function handleArchiveConfirm() {
  showArchiveConfirm.value = false
  emit('archive')
}

function handleDeleteConfirm() {
  showDeleteConfirm.value = false
  emit('delete')
}

function formatDisplayDate(val?: string) {
  if (!val) return '—'
  try {
    const d = new Date(val)
    return d.toLocaleString()
  } catch (e) {
    return val
  }
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

/* Layout: labels + status in one row */
.form-row {
  display: flex;
  gap: 1rem;
  align-items: flex-start;
}

.form-group--small {
  flex: 0 0 140px; /* fixed-ish width for status column */
}

.label-option {
  padding: 0.35rem 0.8rem;
  border-radius: 6px;
  border-width: 2px;
  box-shadow: none;
}

.label-option:hover {
  transform: translateY(-1px);
}

.status-field {
  display: flex;
  align-items: center;
  height: 100%;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  font-weight: 500;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.checkbox-text {
  font-size: 0.95rem;
  color: var(--text-primary);
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

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  font-weight: normal;
  margin: 0;
}

.checkbox-label input[type="checkbox"] {
  width: 16px;
  height: 16px;
  cursor: pointer;
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

  /* Stack the labels/status row on small screens */
  .form-row {
    flex-direction: column;
  }
  .form-group--small {
    flex: 1 1 auto;
  }
}
</style>
