<template>
  <div class="note-editor" v-if="note">
    <!-- Header Bar -->
    <div class="editor-header">
      <div class="header-left">
        <h2 class="note-title" v-if="!isEditingTitle" @dblclick="startEditingTitle">
          {{ note.title }}
        </h2>
        <input
          v-else
          ref="titleInput"
          v-model="editedTitle"
          class="title-input"
          @blur="saveTitle"
          @keydown.enter="saveTitle"
          @keydown.escape="cancelEditTitle"
        />
      </div>
      <div class="header-actions">
        <button 
          v-if="!isEditing" 
          class="btn-icon" 
          @click="startEditing"
          title="Edit note"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
            <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
          </svg>
        </button>
        <button 
          v-else 
          class="btn-icon btn-done" 
          @click="stopEditing"
          title="Done editing"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <polyline points="20 6 9 17 4 12"/>
          </svg>
        </button>
        <button v-if="note" class="btn-icon" @click="handleUndo" title="Undo">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M9 17L4 12l5-5"/>
            <path d="M20 18a8 8 0 0 0-8-8H4"/>
          </svg>
        </button>
        <button v-if="note" class="btn-icon" @click="handleRedo" title="Redo">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M15 17l5-5-5-5"/>
            <path d="M4 18a8 8 0 0 0 8-8h8"/>
          </svg>
        </button>
        <div class="menu-wrapper" ref="menuWrapper">
          <button class="btn-icon" @click="toggleMenu" title="More options">
            <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
              <circle cx="12" cy="5" r="2"/>
              <circle cx="12" cy="12" r="2"/>
              <circle cx="12" cy="19" r="2"/>
            </svg>
          </button>
          <div v-if="showMenu" class="dropdown-menu">
            <button @click="showAttachments = !showAttachments">
              <Icon name="paperclip" :size="14" /> Attachments ({{ note.attachments?.length || 0 }})
            </button>
            <div class="menu-divider"></div>
            <button @click="duplicateNote">
              <Icon name="copy" :size="14" /> Duplicate
            </button>
            <button @click="exportNote">
              <Icon name="download" :size="14" /> Export
            </button>
            <div class="menu-divider"></div>
            <button class="danger" @click="confirmDeleteNote">
              <Icon name="trash" :size="14" /> Delete
            </button>
          </div>
        </div>
      </div>
    </div>
    <!-- Main Content Area -->
    <div class="editor-main">
      <BBCodeEditor
        v-if="note"
        :initialContent="note.content || ''"
        :isEditing="isEditing"
        :paneId="paneId"
        :attachments="note.attachments || []"
        @update:content="handleContentUpdate"
        @update:isEditing="(val) => (isEditing = val)"
        @dirty="(isDirty) => emit('dirty', isDirty)"
        @image-files-dropped="handleImageFilesDropped"
        @request-undo="handleUndo"
        @request-redo="handleRedo"
      />

      <!-- Attachments Panel -->
      <div 
        v-if="showAttachments && note" 
        class="attachments-panel"
        @dragover.prevent="onAttachmentsDragOver"
        @dragleave="onAttachmentsDragLeave"
        @drop.prevent="onAttachmentsDrop"
        :class="{ 'drag-over': isDraggingOverAttachments }"
      >
        <div class="attachments-header">
          <span>Attachments</span>
          <div class="header-buttons">
            <label class="upload-btn" v-if="isEditing" title="Add image">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>
              <input type="file" accept="image/*" multiple @change="handleImageFileUpload" class="file-input" />
            </label>
            <label class="upload-btn" v-if="isEditing" title="Add attachment">
              <Icon name="paperclip" :size="16" />
              <input type="file" multiple @change="handleFileUpload" class="file-input" />
            </label>
          </div>
        </div>
        
        <div class="attachments-list" v-if="note.attachments && note.attachments.length > 0">
          <div v-for="attachment in note.attachments" :key="attachment.id" class="attachment-item">
            <span class="attachment-icon"><Icon :name="getFileIcon(attachment.mime_type)" :size="16" /></span>
            <div class="attachment-info">
              <span class="attachment-name">{{ attachment.original_filename }}</span>
              <span class="attachment-size">{{ formatFileSize(attachment.size) }}</span>
            </div>
            <div class="attachment-actions">
              <button v-if="isEditing && isImageAttachment(attachment.mime_type)" @click="insertImageFromAttachment(attachment)" title="Insert"><Icon name="insert" :size="12" /></button>
              <button @click="downloadAttachment(attachment)" title="Download"><Icon name="download" :size="12" /></button>
              <button v-if="isEditing" @click="confirmRemoveAttachment(attachment)" title="Delete"><Icon name="trash" :size="12" /></button>
            </div>
          </div>
        </div>
        
        <div v-else class="attachments-empty">
          {{ isEditing ? 'Drop files here or use buttons' : 'No attachments' }}
        </div>
      </div>
    </div>
  </div>

  <div v-else class="loading">
    <div class="loading-spinner"></div>
    <span>Loading...</span>
  </div>

  <!-- Confirm Delete Note Modal -->
  <ConfirmModal
    :visible="deleteNoteModal.visible"
    title="Delete Note"
    :message="deleteNoteModal.message"
    confirm-text="Delete"
    variant="danger"
    @confirm="handleDeleteNote"
    @cancel="deleteNoteModal.visible = false"
  />

  <!-- Confirm Delete Attachment Modal -->
  <ConfirmModal
    :visible="deleteAttachmentModal.visible"
    title="Delete Attachment"
    :message="deleteAttachmentModal.message"
    confirm-text="Delete"
    variant="danger"
    @confirm="handleDeleteAttachment"
    @cancel="deleteAttachmentModal.visible = false"
  />

  <!-- Removed Attachment From BBCode Modal -->
  <div v-if="removedAttachmentModal.visible" class="modal-overlay" @click.self="removedAttachmentModal.visible = false">
    <div class="modal" style="max-width: 400px;">
      <div class="modal-header">
        <h3>Attachment Tag Removed</h3>
        <button class="modal-close" @click="removedAttachmentModal.visible = false">&times;</button>
      </div>
      <div class="modal-body">
        <p>{{ removedAttachmentModal.message }}</p>
      </div>
      <div class="modal-footer">
        <button class="btn btn-danger" @click="handleRemovedAttachmentDecision(true)">
          Delete File
        </button>
        <button class="btn btn-secondary" @click="handleRemovedAttachmentDecision(false)">
          Keep File
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue'
import { useNotesStore } from '../stores/notes'
import { notesApi } from '../api'
import { formatFileSize, isImageMimeType, getFileIcon } from '../composables'
import { extractBBCodeReferences } from '../utils/bbcode'
import type { NoteWithAttachments, NoteAttachment } from '../types'
import Icon from './Icon.vue'
import ConfirmModal from './ConfirmModal.vue'
import BBCodeEditor from './BBCodeEditor.vue'

const props = defineProps<{
  noteId: string | null
  paneId: string
}>()

const emit = defineEmits<{
  dirty: [isDirty: boolean]
}>()

const notesStore = useNotesStore()
const note = ref<NoteWithAttachments | null>(null)
const isEditing = ref(false)
const isEditingTitle = ref(false)
const editedTitle = ref('')
const showAttachments = ref(false)
const showMenu = ref(false)
const menuWrapper = ref<HTMLElement | null>(null)
const titleInput = ref<HTMLInputElement | null>(null)
const isDirty = ref(false)
const previousContent = ref<string>('')
const isDraggingOverAttachments = ref(false)

// Delete note confirmation
const deleteNoteModal = ref({
  visible: false,
  message: ''
})

// Delete attachment confirmation
const deleteAttachmentModal = ref({
  visible: false,
  message: '',
  attachment: null as NoteAttachment | null
})

// Removed attachment from BBCode dialog
const removedAttachmentModal = ref({
  visible: false,
  message: '',
  attachmentId: '',
  attachment: null as NoteAttachment | null
})

async function loadNote() {
  if (!props.noteId) {
    note.value = null
    return
  }

  const fetchedNote = await notesStore.fetchNote(props.noteId)
  if (fetchedNote) {
    note.value = fetchedNote
    previousContent.value = fetchedNote.content || ''
    isDirty.value = false
    emit('dirty', false)
    isEditing.value = false
  }
}

function handleContentUpdate(newContent: string) {
  if (!note.value) return
  
  // Check for removed attachment tags
  const oldReferences = extractBBCodeReferences(previousContent.value)
  const newReferences = extractBBCodeReferences(newContent)
  
  const oldAttachmentIds = new Set(oldReferences.filter(r => r.type === 'attachment').map(r => r.id))
  const newAttachmentIds = new Set(newReferences.filter(r => r.type === 'attachment').map(r => r.id))
  
  // Find removed attachments
  const removedAttachmentIds = Array.from(oldAttachmentIds).filter(id => !newAttachmentIds.has(id))
  
  if (removedAttachmentIds.length > 0) {
    const removedAttachment = note.value.attachments?.find(a => a.id === removedAttachmentIds[0])
    if (removedAttachment) {
      removedAttachmentModal.value = {
        visible: true,
        message: `The attachment tag for "${removedAttachment.original_filename}" has been removed. Would you like to delete the file as well?`,
        attachmentId: removedAttachmentIds[0],
        attachment: removedAttachment
      }
    }
  }
  
  previousContent.value = newContent
  note.value.content = newContent
  saveNote()
}

function startEditingTitle() {
  if (!isEditing.value) return
  editedTitle.value = note.value?.title || ''
  isEditingTitle.value = true
  nextTick(() => {
    titleInput.value?.focus()
    titleInput.value?.select()
  })
}

async function saveTitle() {
  if (!note.value || !isEditingTitle.value) return
  const newTitle = editedTitle.value.trim()
  if (newTitle && newTitle !== note.value.title) {
    await notesStore.updateNote(note.value.id, { title: newTitle })
    note.value.title = newTitle
  }
  isEditingTitle.value = false
}

function cancelEditTitle() {
  isEditingTitle.value = false
  editedTitle.value = ''
}

function toggleMenu() {
  showMenu.value = !showMenu.value
}

function closeMenuOnClickOutside(event: MouseEvent) {
  if (menuWrapper.value && !menuWrapper.value.contains(event.target as Node)) {
    showMenu.value = false
  }
}

function startEditing() {
  isEditing.value = true
}

function stopEditing() {
  isEditing.value = false
}

async function saveNote() {
  if (!note.value) return
  await notesStore.updateNote(note.value.id, { content: note.value.content })
  isDirty.value = false
  emit('dirty', false)
}

// Note actions
async function duplicateNote() {
  if (!note.value) return
  showMenu.value = false
  
  const newNote = await notesStore.createNote(
    `${note.value.title} (copy)`,
    note.value.folder_id
  )
  
  if (newNote) {
    await notesStore.updateNote(newNote.id, { content: note.value.content })
  }
}

function exportNote() {
  if (!note.value) return
  showMenu.value = false
  
  const content = note.value.content
  const blob = new Blob([content], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `${note.value.title}.txt`
  a.click()
  URL.revokeObjectURL(url)
}

function confirmDeleteNote() {
  if (!note.value) return
  showMenu.value = false
  
  const attachmentCount = note.value.attachments?.length || 0
  let message = `Are you sure you want to delete "${note.value.title}"? This action cannot be undone.`
  
  if (attachmentCount > 0) {
    message += `\n\n⚠️ This note has ${attachmentCount} attachment${attachmentCount !== 1 ? 's' : ''} that will also be deleted.`
  }
  
  deleteNoteModal.value = {
    visible: true,
    message
  }
}

async function handleDeleteNote() {
  deleteNoteModal.value.visible = false
  if (!note.value) return
  
  // Delete all attachments associated with this note
  if (note.value.attachments && note.value.attachments.length > 0) {
    try {
      await Promise.all(
        note.value.attachments.map(attachment =>
          notesApi.deleteAttachment(attachment.id)
        )
      )
    } catch (error) {
      console.error('Failed to delete some attachments:', error)
    }
  }
  
  await notesStore.deleteNote(note.value.id)
}

// Attachment functions
async function handleFileUpload(event: Event) {
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files || !note.value) return

  try {
    const response = await notesApi.uploadAttachments(note.value.id, Array.from(files))
    if (note.value.attachments) {
      note.value.attachments.push(...response.data)
    } else {
      note.value.attachments = response.data
    }
    notesStore.invalidateNoteCache(note.value.id)
  } catch (error) {
    console.error('Failed to upload files:', error)
  }
  
  target.value = ''
}

async function handleImageFileUpload(event: Event) {
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files || !note.value) return

  try {
    const response = await notesApi.uploadAttachments(note.value.id, Array.from(files))
    if (note.value.attachments) {
      note.value.attachments.push(...response.data)
    } else {
      note.value.attachments = response.data
    }
    notesStore.invalidateNoteCache(note.value.id)
  } catch (error) {
    console.error('Failed to upload images:', error)
  }
  
  target.value = ''
}

function insertImageFromAttachment(attachment: NoteAttachment) {
  // This would need to be passed to BBCodeEditor to insert image
  // For now, copy URL to clipboard
  const url = notesApi.getAttachmentUrl(attachment.id)
  navigator.clipboard.writeText(`[img=${url}]${attachment.original_filename}[/img]`)
}

function onAttachmentsDragOver(event: DragEvent) {
  event.preventDefault()
  isDraggingOverAttachments.value = true
  if (event.dataTransfer) {
    event.dataTransfer.dropEffect = 'copy'
  }
}

function onAttachmentsDragLeave() {
  isDraggingOverAttachments.value = false
}

async function onAttachmentsDrop(event: DragEvent) {
  event.preventDefault()
  isDraggingOverAttachments.value = false
  
  if (!isEditing.value || !note.value) return
  
  const files = event.dataTransfer?.files
  if (!files || files.length === 0) return
  
  try {
    const response = await notesApi.uploadAttachments(note.value.id, Array.from(files))
    if (note.value.attachments) {
      note.value.attachments.push(...response.data)
    } else {
      note.value.attachments = response.data
    }
    notesStore.invalidateNoteCache(note.value.id)
  } catch (error) {
    console.error('Failed to upload files:', error)
  }
}

async function handleImageFilesDropped(files: File[]) {
  if (!note.value) return
  
  try {
    const response = await notesApi.uploadAttachments(note.value.id, Array.from(files))
    if (note.value.attachments) {
      note.value.attachments.push(...response.data)
    } else {
      note.value.attachments = response.data
    }
    notesStore.invalidateNoteCache(note.value.id)
  } catch (error) {
    console.error('Failed to upload images:', error)
  }
}

async function downloadAttachment(attachment: NoteAttachment) {
  try {
    const response = await notesApi.downloadAttachment(attachment.id)
    const blob = new Blob([response.data], { type: attachment.mime_type })
    const url = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = attachment.original_filename
    document.body.appendChild(a)
    a.click()
    window.URL.revokeObjectURL(url)
    document.body.removeChild(a)
  } catch (error) {
    console.error('Failed to download attachment:', error)
  }
}

async function handleUndo() {
  if (!note.value) return
  try {
    await notesStore.undoNote(note.value.id)
    // reload note
    const fresh = await notesStore.fetchNote(note.value.id)
    if (fresh) note.value = fresh
  } catch (error) {
    console.error('Undo failed', error)
  }
}

async function handleRedo() {
  if (!note.value) return
  try {
    await notesStore.redoNote(note.value.id)
    const fresh = await notesStore.fetchNote(note.value.id)
    if (fresh) note.value = fresh
  } catch (error) {
    console.error('Redo failed', error)
  }
}

function confirmRemoveAttachment(attachment: NoteAttachment) {
  deleteAttachmentModal.value = {
    visible: true,
    message: `Delete "${attachment.original_filename}"?`,
    attachment
  }
}

async function handleDeleteAttachment() {
  const attachment = deleteAttachmentModal.value.attachment
  deleteAttachmentModal.value.visible = false
  if (!attachment) return
  
  try {
    // Note: BBCode reference removal would need to be handled through BBCodeEditor
    // For now, we'll just delete the attachment
    await notesApi.deleteAttachment(attachment.id)
    if (note.value?.attachments) {
      note.value.attachments = note.value.attachments.filter(a => a.id !== attachment.id)
    }
    notesStore.invalidateNoteCache(note.value!.id)
  } catch (error) {
    console.error('Failed to delete attachment:', error)
  }
}

function handleRemovedAttachmentDecision(deleteFile: boolean) {
  removedAttachmentModal.value.visible = false
  
  if (deleteFile) {
    const attachment = removedAttachmentModal.value.attachment
    if (attachment) {
      notesApi.deleteAttachment(attachment.id).then(() => {
        if (note.value?.attachments) {
          note.value.attachments = note.value.attachments.filter(a => a.id !== attachment.id)
        }
        notesStore.invalidateNoteCache(note.value!.id)
      }).catch(error => {
        console.error('Failed to delete attachment:', error)
      })
    }
  }
}

// Use composable functions
function isImageAttachment(mimeType: string): boolean {
  return isImageMimeType(mimeType)
}

watch(() => props.noteId, loadNote, { immediate: true })

onMounted(() => {
  document.addEventListener('click', closeMenuOnClickOutside)
})

// Watch for note changes
</script>

<style scoped>
.note-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-primary);
}

.loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 100%;
  gap: 1rem;
  color: var(--text-muted);
}

.loading-spinner {
  width: 32px;
  height: 32px;
  border: 3px solid var(--border-primary);
  border-top-color: var(--accent);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

/* Header */
.editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
}

.header-left {
  flex: 1;
  min-width: 0;
}

.note-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: default;
}

.title-input {
  width: 100%;
  font-size: 1.1rem;
  font-weight: 600;
  background: var(--bg-tertiary);
  border: 1px solid var(--accent);
  color: var(--text-primary);
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  outline: none;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.btn-done {
  color: var(--success);
}

.btn-done:hover {
  background: var(--success);
  color: white;
}

/* Menu */
.menu-wrapper {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.25rem;
  min-width: 180px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
  overflow: hidden;
}

.dropdown-menu button {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  padding: 0.625rem 1rem;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 0.875rem;
  cursor: pointer;
  text-align: left;
}

.dropdown-menu button:hover {
  background: var(--bg-hover);
}

.dropdown-menu button.danger {
  color: var(--danger);
}

.dropdown-menu button.danger:hover {
  background: rgba(248, 81, 73, 0.1);
}

.menu-divider {
  height: 1px;
  background: var(--border-primary);
  margin: 0.25rem 0;
}

/* Toolbar */
.editor-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  padding: 0.5rem;
  background: var(--toolbar-bg);
  border-bottom: 1px solid var(--border-primary);
}

.toolbar-group {
  display: flex;
  gap: 2px;
  padding-right: 0.5rem;
  border-right: 1px solid var(--border-primary);
}

.toolbar-group:last-child {
  border-right: none;
}

.editor-toolbar button {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 3px;
  cursor: pointer;
  font-size: 12px;
  color: var(--text-secondary);
  transition: all 0.15s;
}

.editor-toolbar button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.editor-toolbar button.is-active {
  background: var(--accent-light);
  color: var(--accent);
}

.editor-toolbar button:disabled {
  opacity: 0.3;
  cursor: not-allowed;
}

/* Color Picker */
.color-picker-wrapper {
  position: relative;
}

.color-btn {
  position: relative;
  overflow: hidden;
}

.color-input {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  opacity: 0;
  cursor: pointer;
}

.color-icon {
  background: linear-gradient(45deg, #ff0000, #00ff00, #0000ff);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  font-weight: bold;
}

.highlight-icon {
  background: #ffff00;
  padding: 0 4px;
  border-radius: 2px;
  color: #333;
  font-size: 10px;
}

/* Main Content */
.editor-main {
  flex: 1;
  display: flex;
  overflow: hidden;
  width: 100%;
  height: 100%;
}

.editor-wrapper {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem 2rem;
  position: relative;
}

.editor-wrapper.with-attachments {
  border-right: 1px solid var(--border-primary);
}

.editor-wrapper.board-drop-target {
  background: color-mix(in srgb, var(--accent) 10%, var(--bg-primary));
  border: 2px dashed var(--accent);
  border-radius: 8px;
}

/* View Mode Overlay */
.view-mode-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  overflow-y: auto;
  padding: 1.5rem 2rem;
  cursor: text;
  z-index: 1;
}

.view-mode-content {
  min-height: 100%;
  color: var(--editor-text);
  line-height: 1.7;
}

.tiptap-editor {
  min-height: 100%;
  color: var(--editor-text);
  line-height: 1.7;
  position: relative;
  z-index: 2;
}

.tiptap-editor :deep(.tiptap) {
  outline: none;
  min-height: 100%;
}

.tiptap-editor :deep(.tiptap p.is-editor-empty:first-child::before) {
  color: var(--text-muted);
  content: attr(data-placeholder);
  float: left;
  height: 0;
  pointer-events: none;
}

/* Editor Content Styles */
.tiptap-editor :deep(h1) { 
  font-size: 2em; 
  margin: 1em 0 0.5em; 
  color: var(--text-primary);
  font-weight: 700;
  letter-spacing: -0.02em;
}

.tiptap-editor :deep(h2) { 
  font-size: 1.5em; 
  margin: 1em 0 0.5em; 
  color: var(--text-primary);
  font-weight: 600;
}

.tiptap-editor :deep(h3) { 
  font-size: 1.25em; 
  margin: 1em 0 0.5em; 
  color: var(--text-primary);
  font-weight: 600;
}

.tiptap-editor :deep(p) { 
  margin: 0.75em 0; 
}

.tiptap-editor :deep(blockquote) {
  border-left: 4px solid var(--accent);
  padding-left: 1rem;
  margin: 1.5rem 0;
  color: var(--text-secondary);
  font-style: italic;
  background: var(--bg-tertiary);
  padding: 1rem 1rem 1rem 1.5rem;
  border-radius: 0 8px 8px 0;
}

.tiptap-editor :deep(pre) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  padding: 1rem;
  border-radius: 8px;
  overflow-x: auto;
  font-family: 'JetBrains Mono', 'Fira Code', 'Monaco', monospace;
  font-size: 0.9em;
  margin: 1rem 0;
}

.tiptap-editor :deep(code) {
  background: var(--bg-tertiary);
  padding: 0.2em 0.4em;
  border-radius: 4px;
  font-family: 'JetBrains Mono', 'Fira Code', 'Monaco', monospace;
  font-size: 0.9em;
  color: var(--accent);
}

.tiptap-editor :deep(pre code) {
  background: none;
  padding: 0;
  color: inherit;
}

.tiptap-editor :deep(.editor-link) {
  color: var(--info);
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: border-color 0.2s;
}

.tiptap-editor :deep(.editor-link:hover) {
  border-bottom-color: var(--info);
}

.tiptap-editor :deep(.internal-link) {
  background: var(--bg-tertiary);
  padding: 0.1em 0.4em;
  border-radius: 4px;
  text-decoration: none;
  color: var(--text-primary);
  transition: background 0.2s;
}

.tiptap-editor :deep(.internal-link:hover) {
  background: var(--bg-hover);
}

.tiptap-editor :deep(.note-link) {
  border-left: 2px solid var(--success);
}

.tiptap-editor :deep(.board-link) {
  border-left: 2px solid var(--accent);
}

/* BBCode Links (in view mode) */
.view-mode-content :deep(.bbcode-link) {
  display: inline-block;
  padding: 0.1em 0.4em;
  border-radius: 4px;
  text-decoration: none;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s ease;
  user-select: none;
}

.view-mode-content :deep(.bbcode-note-link) {
  background: var(--success-light, rgba(39, 174, 96, 0.15));
  color: var(--success, #27ae60);
  border-left: 2px solid var(--success, #27ae60);
  padding-left: 0.5em;
}

.view-mode-content :deep(.bbcode-note-link:hover) {
  background: var(--success-light, rgba(39, 174, 96, 0.3));
  transform: translateX(2px);
}

.view-mode-content :deep(.bbcode-board-link) {
  background: var(--accent-light, rgba(25, 118, 210, 0.15));
  color: var(--accent, #1976d2);
  border-left: 2px solid var(--accent, #1976d2);
  padding-left: 0.5em;
}

.view-mode-content :deep(.bbcode-board-link:hover) {
  background: var(--accent-light, rgba(25, 118, 210, 0.3));
  transform: translateX(2px);
}

.view-mode-content :deep(.bbcode-attachment-link) {
  background: var(--info-light, rgba(13, 110, 253, 0.15));
  color: var(--info, #0d6efd);
  border-left: 2px solid var(--info, #0d6efd);
  padding-left: 0.5em;
}

.view-mode-content :deep(.bbcode-attachment-link:hover) {
  background: var(--info-light, rgba(13, 110, 253, 0.3));
  transform: translateX(2px);
}

.tiptap-editor :deep(.editor-image) {
  max-width: 100%;
  height: auto;
  border-radius: 8px;
  margin: 1rem 0;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
}

.tiptap-editor :deep(ul),
.tiptap-editor :deep(ol) {
  padding-left: 1.5rem;
  margin: 0.75rem 0;
}

.tiptap-editor :deep(li) {
  margin: 0.35rem 0;
}

/* Task List */
.tiptap-editor :deep(.task-list) {
  list-style: none;
  padding-left: 0;
}

.tiptap-editor :deep(.task-item) {
  display: flex;
  align-items: flex-start;
  gap: 0.5rem;
  margin: 0.5rem 0;
}

.tiptap-editor :deep(.task-item > label) {
  flex-shrink: 0;
  margin-top: 0.1em;
}

.tiptap-editor :deep(.task-item > label input[type="checkbox"]) {
  width: 18px;
  height: 18px;
  cursor: pointer;
  accent-color: var(--accent);
}

.tiptap-editor :deep(.task-item[data-checked="true"] > div) {
  text-decoration: line-through;
  color: var(--text-muted);
}

.tiptap-editor :deep(.editor-table) {
  border-collapse: collapse;
  width: 100%;
  margin: 1rem 0;
  border-radius: 8px;
  overflow: hidden;
}

.tiptap-editor :deep(.editor-table td),
.tiptap-editor :deep(.editor-table th) {
  border: 1px solid var(--border-primary);
  padding: 0.75rem;
  min-width: 80px;
}

.tiptap-editor :deep(.editor-table th) {
  background: var(--bg-secondary);
  font-weight: 600;
}

.tiptap-editor :deep(.editor-table .selectedCell) {
  background: var(--accent-light);
}

.tiptap-editor :deep(hr) {
  border: none;
  border-top: 2px solid var(--border-primary);
  margin: 2rem 0;
}

/* Attachments Panel */
.attachments-panel {
  width: 260px;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  transition: background-color 0.2s, border-color 0.2s;
}

.attachments-panel.drag-over {
  background: color-mix(in srgb, var(--accent) 5%, var(--bg-secondary));
  border-left: 2px solid var(--accent);
}

.attachments-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border-primary);
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-primary);
}

.header-buttons {
  display: flex;
  gap: 0.5rem;
}

.upload-btn {
  background: var(--accent);
  color: white;
  padding: 0.375rem 0.75rem;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.75rem;
  font-weight: 500;
  transition: background 0.2s;
}

.upload-btn:hover {
  background: var(--accent-hover);
}

.file-input {
  display: none;
}

.attachments-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.5rem;
}

.attachment-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.625rem;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.attachment-item:hover {
  background: var(--bg-hover);
}

.attachment-icon {
  font-size: 18px;
}

.attachment-info {
  flex: 1;
  overflow: hidden;
}

.attachment-name {
  display: block;
  font-size: 0.8125rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--text-primary);
}

.attachment-size {
  font-size: 0.6875rem;
  color: var(--text-muted);
}

.attachment-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.15s;
}

.attachment-item:hover .attachment-actions {
  opacity: 1;
}

.attachment-actions button {
  background: transparent;
  border: none;
  padding: 4px;
  cursor: pointer;
  font-size: 12px;
  border-radius: 4px;
  transition: background 0.15s;
}

.attachment-actions button:hover {
  background: var(--bg-tertiary);
}

.attachments-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  font-size: 0.8125rem;
  padding: 1rem;
  text-align: center;
}

/* Link Modal */
.link-modal {
  width: 400px;
}

.search-input {
  width: 100%;
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 0.875rem;
  margin-bottom: 0.75rem;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent);
}

.search-input::placeholder {
  color: var(--text-muted);
}

.link-results {
  max-height: 300px;
  overflow-y: auto;
}

.link-result-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.625rem 0.75rem;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.link-result-item:hover {
  background: var(--bg-hover);
}

.link-icon {
  font-size: 16px;
}

.link-name {
  color: var(--text-primary);
  font-size: 0.875rem;
}

.link-empty {
  text-align: center;
  padding: 2rem;
  color: var(--text-muted);
  font-size: 0.875rem;
}

/* Modal styles for theme */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  padding: 1.25rem;
  max-height: 80vh;
  overflow-y: auto;
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.modal-header h3 {
  margin: 0;
  color: var(--text-primary);
  font-size: 1.125rem;
}

.modal-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-muted);
  padding: 0;
  line-height: 1;
}

.modal-close:hover {
  color: var(--text-primary);
}

.modal-body {
  margin-bottom: 1rem;
  color: var(--text-primary);
  line-height: 1.5;
}

.modal-body p {
  margin: 0.5rem 0;
}

.modal-footer {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
}

.btn {
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover {
  background: var(--accent-hover);
}

.btn-danger {
  background: var(--danger);
  color: white;
}

.btn-danger:hover {
  background: var(--danger-hover);
}

.btn-secondary {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-primary);
}

.btn-secondary:hover {
  background: var(--bg-hover);
}

/* ========================================
   RESPONSIVE DESIGN - TABLET (max-width: 1024px)
   ======================================== */
@media (max-width: 1024px) {
  .editor-wrapper {
    padding: 1rem 1.5rem;
  }

  .attachments-panel {
    width: 220px;
  }
}

/* ========================================
   RESPONSIVE DESIGN - SMARTPHONE (max-width: 768px)
   ======================================== */
@media (max-width: 768px) {
  .editor-header {
    padding: 0.5rem 0.75rem;
  }

  .note-title {
    font-size: 1rem;
  }

  .title-input {
    font-size: 1rem;
  }

  /* Toolbar mobile - scrollable */
  .editor-toolbar {
    padding: 0.375rem;
    gap: 0.375rem;
    overflow-x: auto;
    flex-wrap: nowrap;
    -webkit-overflow-scrolling: touch;
  }

  .toolbar-group {
    flex-shrink: 0;
    padding-right: 0.375rem;
  }

  .editor-toolbar button {
    width: 32px;
    height: 32px;
  }

  /* Editor content mobile */
  .editor-wrapper {
    padding: 0.75rem 1rem;
  }

  .tiptap-editor :deep(h1) {
    font-size: 1.5em;
  }

  .tiptap-editor :deep(h2) {
    font-size: 1.25em;
  }

  .tiptap-editor :deep(h3) {
    font-size: 1.1em;
  }

  /* Attachments panel as bottom sheet on mobile */
  .editor-main {
    flex-direction: column;
  }

  .editor-wrapper.with-attachments {
    border-right: none;
    border-bottom: 1px solid var(--border-primary);
  }

  .attachments-panel {
    width: 100%;
    max-height: 200px;
    border-top: 1px solid var(--border-primary);
  }

  .attachments-list {
    flex-direction: row;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0.5rem;
  }

  .attachment-item {
    flex: 1 1 calc(50% - 0.25rem);
    min-width: 120px;
  }

  .attachment-actions {
    opacity: 1;
  }

  /* Dropdown menu mobile */
  .dropdown-menu {
    position: fixed;
    left: 1rem !important;
    right: 1rem !important;
    top: auto !important;
    bottom: 1rem;
    border-radius: 12px;
  }

  .dropdown-menu button {
    padding: 0.875rem 1rem;
  }

  /* Link modal mobile */
  .link-modal {
    width: 100%;
    max-width: 100%;
    margin: 0;
    border-radius: 16px 16px 0 0;
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    max-height: 70vh;
  }

  .modal-overlay {
    align-items: flex-end;
  }

  .search-input {
    font-size: 16px; /* Prevents zoom on iOS */
  }

  .link-results {
    max-height: 40vh;
  }

  .link-result-item {
    padding: 0.875rem;
  }
}

/* ========================================
   RESPONSIVE DESIGN - SMALL PHONES (max-width: 480px)
   ======================================== */
@media (max-width: 480px) {
  .editor-header {
    padding: 0.375rem 0.5rem;
  }

  .note-title {
    font-size: 0.9rem;
  }

  .btn-icon {
    width: 36px;
    height: 36px;
  }

  .editor-toolbar button {
    width: 36px;
    height: 36px;
    font-size: 13px;
  }

  .editor-wrapper {
    padding: 0.5rem 0.75rem;
  }

  .attachment-item {
    flex: 1 1 100%;
  }
}

/* ========================================
   TOUCH DEVICE OPTIMIZATIONS
   ======================================== */
@media (hover: none) and (pointer: coarse) {
  .btn-icon {
    min-width: 44px;
    min-height: 44px;
  }

  .editor-toolbar button {
    min-width: 40px;
    min-height: 40px;
  }

  .attachment-actions {
    opacity: 1;
  }

  .attachment-actions button {
    min-width: 36px;
    min-height: 36px;
    padding: 8px;
  }

  .upload-btn {
    min-height: 40px;
    display: flex;
    align-items: center;
    padding: 0.5rem 1rem;
  }

  .link-result-item {
    min-height: 48px;
  }

  /* Better tap targets for checkboxes */
  .tiptap-editor :deep(.task-item > label input[type="checkbox"]) {
    width: 24px;
    height: 24px;
  }
}
</style>
