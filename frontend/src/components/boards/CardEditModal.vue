<template>
  <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal modal-large modal-xl card-modal">
      <div class="modal-header">
            <h3>{{ isEditing ? 'Edit Card' : 'Card' }}</h3>
            <div class="modal-header-actions">
              <button class="modal-close" @click="$emit('close')">
                <Icon name="x" :size="14" />
              </button>
            </div>
          </div>
          <form @submit.prevent="handleSubmit" class="card-modal-form">
            <div class="card-modal-body">
        <div class="form-group">
          <label for="editCardTitle">Title</label>
          <div v-if="!isEditing" class="view-title">{{ localCard.title }}</div>
          <input v-else id="editCardTitle" v-model="localCard.title" type="text" required />
        </div>
        <div class="form-group description-group">
          <label for="editCardDescription">Description</label>
          <div v-if="!isEditing" class="view-description">
            <BBCodeRenderer 
              v-if="localCard.description" 
              :content="localCard.description" 
              :paneId="'card-modal'" 
            />
            <div v-else class="no-description">
              <em>No description</em>
            </div>
          </div>
          <div v-else class="bbcode-editor-container">
            <BBCodeEditor
              :initialContent="localCard.description || ''"
              :isEditing="true"
              :paneId="'card-modal'"
              :attachments="[]"
              @update:content="localCard.description = $event"
              @print-note="printCard"
            />
          </div>
        </div>
        <div class="form-group">
          <label for="editCardDueDate">Due Date</label>
          <div v-if="!isEditing" class="view-due-date">
            <Icon v-if="localCard.due_date" name="calendar" :size="14" />
            <span>{{ formatDisplayDate(localCard.due_date) }}</span>
          </div>
          <input v-else id="editCardDueDate" v-model="localCard.due_date" type="datetime-local" @focus="onDueDateFocus" @change="onDueDateChange" />
        </div>
        <!-- Labels and Status side-by-side on wide screens, stacked on mobile -->
        <div class="form-row">
          <div class="form-row">
            <div class="form-group">
              <label>Labels</label>
              <div v-if="!isEditing" class="label-view">
                <span v-if="localCard.labels.length === 0" class="no-labels">No labels</span>
                <span v-for="label in labels" :key="label.id" class="view-label-chip" :style="{ backgroundColor: localCard.labels.includes(label.id) ? label.color : 'transparent', color: localCard.labels.includes(label.id) ? '#fff' : 'var(--text-muted)', border: '1px solid ' + label.color }">{{ label.name }}</span>
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
                <div v-if="!isEditing" class="view-status">
                  <span class="status-indicator" :class="{ 'status-done': localCard.status === 'done' }"></span>
                  <span>{{ localCard.status === 'done' ? 'Done' : 'Open' }}</span>
                </div>
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
      </div>
        
        <div class="modal-actions">
          <button type="button" class="btn btn-warning" @click="openArchiveConfirm">Archive</button>
          <button type="button" class="btn btn-danger" @click="openDeleteConfirm">Delete</button>
          <button type="button" class="btn btn-secondary" @click="$emit('close')">Cancel</button>
          <button v-if="!isEditing" type="button" class="btn btn-primary" @click="isEditing = true">Edit</button>
          <button v-else type="submit" class="btn btn-primary">Save</button>
        </div>
      </form>

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
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, computed } from 'vue'
import type { Card, BoardLabel } from '../../types'
import Icon from '../ui/Icon.vue'
import ConfirmModal from '../modals/ConfirmModal.vue'
import BBCodeEditor from '../editor/BBCodeEditor.vue'
import BBCodeRenderer from '../editor/BBCodeRenderer.vue'
import { toDatetimeLocal, normalizeForInput } from '../../utils/dates'
import { bbcodeToHtml } from '../../utils/bbcodeFormatter'

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

function printCard() {
  if (!localCard.value) return
  
  const printWindow = window.open('', '_blank')
  if (!printWindow) {
    alert('Please allow popups to print the card.')
    return
  }
  
  const renderedHtml = bbcodeToHtml(localCard.value.description || '', { paneId: 'card-modal' })
  
  const styles = `
    @page {
      size: auto;
      margin: 0mm;
    }
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      line-height: 1.6;
      color: #000;
      margin: 0;
      padding: 20mm;
      background: white;
    }
    h1 { font-size: 24pt; margin-bottom: 12pt; border-bottom: 1pt solid #ccc; padding-bottom: 6pt; }
    h2 { font-size: 18pt; margin-top: 20pt; margin-bottom: 10pt; }
    h3 { font-size: 14pt; margin-top: 16pt; margin-bottom: 8pt; }
    p { margin-bottom: 10pt; }
    blockquote {
      border-left: 3pt solid #ddd;
      padding: 5pt 15pt;
      margin: 15pt 0;
      color: #444;
      font-style: italic;
      background: #f9f9f9;
    }
    pre {
      background: #f4f4f4;
      padding: 10pt;
      border-radius: 4pt;
      white-space: pre-wrap;
      word-wrap: break-word;
      font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
      font-size: 10pt;
      border: 0.5pt solid #ddd;
      margin: 10pt 0;
    }
    code {
      background: #f4f4f4;
      padding: 2pt 4pt;
      border-radius: 2pt;
      font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
      font-size: 10pt;
      border: 0.5pt solid #ddd;
    }
    table {
      border-collapse: collapse;
      width: 100%;
      margin: 15pt 0;
    }
    th, td {
      border: 0.5pt solid #ddd;
      padding: 8pt;
      text-align: left;
    }
    th { background-color: #f5f5f5; font-weight: 600; }
    img { max-width: 100%; height: auto; border-radius: 4pt; margin: 10pt 0; }
    hr { border: none; border-top: 0.5pt solid #eee; margin: 20pt 0; }
    .bbcode-list { padding-left: 25pt; margin: 10pt 0; }
    .bbcode-list li { margin-bottom: 5pt; }
    
    @media print {
      body { padding: 20mm; }
    }
  `
  
  printWindow.document.open()
  printWindow.document.write(`
    <!DOCTYPE html>
    <html>
    <head>
      <title>${localCard.value.title}</title>
      <style>${styles}</style>
    </head>
    <body>
      <h1>${localCard.value.title}</h1>
      <div class="content">${renderedHtml}</div>
      <script>
        window.onload = function() {
          window.print();
          window.onafterprint = function() {
            window.close();
          };
        };
      </' + 'script>
    </body>
    </html>
  `)
  printWindow.document.close()
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

/* View mode styling */
.view-title {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text-primary);
  line-height: 1.4;
  padding: 0.5rem 0;
  border-bottom: 2px solid var(--accent);
  margin-bottom: 0.5rem;
}

.view-description {
  font-size: 0.95rem;
  line-height: 1.6;
  color: var(--text-primary);
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 8px;
  border: 1px solid var(--border-primary);
  min-height: 3rem;
  white-space: pre-wrap;
}

.no-description {
  color: var(--text-muted);
  font-style: italic;
  padding: 0.75rem;
}

.bbcode-editor-container {
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  overflow: hidden;
  background: var(--bg-tertiary);
}

.view-due-date {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 6px;
  border: 1px solid var(--border-primary);
  font-size: 0.9rem;
  color: var(--text-primary);
}

.view-due-date .icon {
  color: var(--accent);
}

.view-status {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 6px;
  border: 1px solid var(--border-primary);
  font-size: 0.9rem;
  font-weight: 500;
}

.status-indicator {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--text-muted);
  flex-shrink: 0;
}

.status-indicator.status-done {
  background: var(--success);
}

.label-view {
  display: flex;
  flex-wrap: wrap;
  gap: 0.375rem;
  padding: 0.5rem 0;
}

.no-labels {
  color: var(--text-muted);
  font-style: italic;
  font-size: 0.85rem;
}

.view-label-chip {
  display: inline-flex;
  align-items: center;
  padding: 0.25rem 0.625rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 500;
  transition: transform 0.15s;
}

.view-label-chip:hover {
  transform: translateY(-1px);
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

.modal-xl {
  max-width: 90vw;
  max-height: 90vh;
  width: 90vw;
  height: 90vh;
}

.card-modal {
  display: flex;
  flex-direction: column;
  padding: 0 !important;
  overflow: hidden !important;
}

.card-modal-form {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

.card-modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.modal-header {
  padding: 1rem 1.5rem;
  margin-bottom: 0;
  border-bottom: 1px solid var(--border-primary);
}

.modal-actions {
  padding: 1rem 1.5rem;
  margin-top: 0;
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.description-group {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 300px;
}

.description-group .view-description,
.description-group .bbcode-editor-container {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.description-group .bbcode-editor-container :deep(.bbcode-editor) {
  flex: 1;
}
</style>
