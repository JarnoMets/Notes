<template>
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
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { NoteWithAttachments, NoteAttachment } from '@/types'
import { formatFileSize, isImageMimeType, getFileIcon } from '@/composables'
import Icon from '../ui/Icon.vue'

const props = defineProps<{
  note: NoteWithAttachments | null
  isEditing: boolean
  showAttachments: boolean
}>()

const emit = defineEmits<{
  'image-files-dropped': [files: File[]]
  'files-uploaded': [files: File[]]
  'insert-image': [attachment: NoteAttachment]
  'download-attachment': [attachment: NoteAttachment]
  'confirm-remove-attachment': [attachment: NoteAttachment]
}>()

const isDraggingOverAttachments = ref(false)

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

function onAttachmentsDrop(event: DragEvent) {
  event.preventDefault()
  isDraggingOverAttachments.value = false

  if (!props.isEditing || !props.note) return

  const files = event.dataTransfer?.files
  if (!files || files.length === 0) return

  emit('image-files-dropped', Array.from(files))
}

function handleFileUpload(event: Event) {
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files) return

  emit('files-uploaded', Array.from(files))
  target.value = ''
}

function handleImageFileUpload(event: Event) {
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files) return

  emit('files-uploaded', Array.from(files))
  target.value = ''
}

function insertImageFromAttachment(attachment: NoteAttachment) {
  emit('insert-image', attachment)
}

function downloadAttachment(attachment: NoteAttachment) {
  emit('download-attachment', attachment)
}

function confirmRemoveAttachment(attachment: NoteAttachment) {
  emit('confirm-remove-attachment', attachment)
}

function isImageAttachment(mimeType: string): boolean {
  return isImageMimeType(mimeType)
}
</script>

<style scoped>
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

/* ========================================
   RESPONSIVE DESIGN - TABLET (max-width: 1024px)
   ======================================== */
@media (max-width: 1024px) {
  .attachments-panel {
    width: 220px;
  }
}

/* ========================================
   RESPONSIVE DESIGN - SMARTPHONE (max-width: 768px)
   ======================================== */
@media (max-width: 768px) {
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
}
</style>