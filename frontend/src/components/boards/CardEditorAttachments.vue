<template>
  <div v-if="showAttachments || (card?.attachments && card.attachments.length > 0)" class="attachments-panel">
    <div class="attachments-header">
      <h4>Attachments</h4>
      <div class="attachments-actions">
        <button
          v-if="!isEditing"
          class="btn btn-sm btn-secondary"
          @click="$emit('toggle-attachments')"
        >
          <Icon name="paperclip" :size="14" />
          {{ showAttachments ? 'Hide' : 'Show' }}
        </button>
        <label v-else class="file-upload-label">
          <Icon name="upload" :size="14" />
          Upload
          <input
            type="file"
            multiple
            @change="handleFileSelect"
            accept="image/*,.pdf,.doc,.docx,.txt,.md"
          />
        </label>
      </div>
    </div>

    <div v-if="showAttachments" class="attachments-content">
      <div v-if="!card?.attachments || card.attachments.length === 0" class="no-attachments">
        <Icon name="paperclip" :size="24" />
        <p>No attachments yet</p>
        <p class="hint">Drag and drop files here or click upload</p>
      </div>

      <div v-else class="attachments-list">
        <div
          v-for="attachment in card.attachments"
          :key="attachment.id"
          class="attachment-item"
          @dragover.prevent
          @drop.prevent="onAttachmentDrop"
        >
          <div class="attachment-icon">
            <Icon :name="getAttachmentIcon(attachment)" :size="20" />
          </div>

          <div class="attachment-info">
            <div class="attachment-name">{{ attachment.original_filename }}</div>
            <div class="attachment-meta">
              {{ formatFileSize(attachment.size) }} • {{ formatDate(attachment.created_at) }}
            </div>
          </div>

          <div class="attachment-actions">
            <button
              class="btn-icon"
              @click="downloadAttachment(attachment)"
              title="Download"
            >
              <Icon name="download" :size="14" />
            </button>

            <button
              v-if="isEditing"
              class="btn-icon"
              @click="confirmRemoveAttachment(attachment)"
              title="Remove"
            >
              <Icon name="trash" :size="14" />
            </button>

            <button
              v-if="isImage(attachment)"
              class="btn-icon"
              @click="insertImage(attachment)"
              title="Insert into card"
            >
              <Icon name="image" :size="14" />
            </button>
          </div>
        </div>
      </div>

      <!-- Drop zone overlay -->
      <div
        v-if="isEditing"
        class="drop-zone"
        :class="{ 'drag-over': isDragOver }"
        @dragover.prevent="isDragOver = true"
        @dragleave="isDragOver = false"
        @drop.prevent="handleDrop"
      >
        <Icon name="upload" :size="24" />
        <p>Drop files here to upload</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Icon from '../ui/Icon.vue'
import type { CardWithAttachments, CardAttachment } from '../../types'

defineProps<{
  card: CardWithAttachments | null
  isEditing: boolean
  showAttachments: boolean
}>()

const emit = defineEmits<{
  'toggle-attachments': []
  'files-uploaded': [files: File[]]
  'image-files-dropped': [files: File[]]
  'download-attachment': [attachment: CardAttachment]
  'confirm-remove-attachment': [attachment: CardAttachment]
  'insert-image': [attachment: CardAttachment]
}>()

const isDragOver = ref(false)

function handleFileSelect(event: Event) {
  const target = event.target as HTMLInputElement
  if (target.files) {
    const files = Array.from(target.files)
    emit('files-uploaded', files)
    target.value = '' // Reset input
  }
}

function handleDrop(event: DragEvent) {
  isDragOver.value = false
  const files = event.dataTransfer?.files
  if (files) {
    const fileArray = Array.from(files)
    const imageFiles = fileArray.filter(file => file.type.startsWith('image/'))
    const otherFiles = fileArray.filter(file => !file.type.startsWith('image/'))

    if (imageFiles.length > 0) {
      emit('image-files-dropped', imageFiles)
    }
    if (otherFiles.length > 0) {
      emit('files-uploaded', otherFiles)
    }
  }
}

function onAttachmentDrop(event: DragEvent) {
  // Prevent drop on individual attachments
  event.preventDefault()
}

function downloadAttachment(attachment: CardAttachment) {
  emit('download-attachment', attachment)
}

function confirmRemoveAttachment(attachment: CardAttachment) {
  emit('confirm-remove-attachment', attachment)
}

function insertImage(attachment: CardAttachment) {
  emit('insert-image', attachment)
}

function getAttachmentIcon(attachment: CardAttachment): string {
  const mimeType = attachment.mime_type
  if (mimeType.startsWith('image/')) return 'image'
  if (mimeType === 'application/pdf') return 'file-text'
  if (mimeType.includes('word') || mimeType.includes('document')) return 'file-text'
  if (mimeType.startsWith('text/')) return 'file-text'
  return 'paperclip'
}

function isImage(attachment: CardAttachment): boolean {
  return attachment.mime_type.startsWith('image/')
}

function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i]
}

function formatDate(dateString: string): string {
  return new Date(dateString).toLocaleDateString()
}
</script>

<style scoped>
.attachments-panel {
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.attachments-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--border-primary);
}

.attachments-header h4 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.attachments-actions {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.file-upload-label {
  display: inline-flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.5rem 0.875rem;
  background: var(--accent);
  color: white;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
}

.file-upload-label:hover {
  background: var(--accent-hover);
}

.file-upload-label input {
  display: none;
}

.attachments-content {
  position: relative;
  padding: 1rem 1.5rem;
  max-height: 300px;
  overflow-y: auto;
}

.no-attachments {
  text-align: center;
  padding: 2rem;
  color: var(--text-muted);
}

.no-attachments .icon {
  margin-bottom: 1rem;
  opacity: 0.5;
}

.no-attachments p {
  margin: 0.5rem 0;
  font-size: 14px;
}

.hint {
  font-size: 12px !important;
  opacity: 0.7;
}

.attachments-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.attachment-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 8px;
  border: 1px solid var(--border-primary);
  transition: all 0.15s;
}

.attachment-item:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
}

.attachment-icon {
  flex-shrink: 0;
  color: var(--accent);
}

.attachment-info {
  flex: 1;
  min-width: 0;
}

.attachment-name {
  font-weight: 500;
  color: var(--text-primary);
  word-break: break-all;
}

.attachment-meta {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 0.25rem;
}

.attachment-actions {
  display: flex;
  gap: 0.25rem;
  flex-shrink: 0;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s;
}

.btn-icon:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.btn-sm {
  padding: 0.375rem 0.75rem;
  font-size: 12px;
}

.drop-zone {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  background: rgba(0, 0, 0, 0.8);
  backdrop-filter: blur(4px);
  border-radius: 8px;
  color: white;
  opacity: 0;
  pointer-events: none;
  transition: opacity 0.2s;
  z-index: 10;
}

.drop-zone.drag-over {
  opacity: 1;
  pointer-events: auto;
}

.drop-zone .icon {
  margin-bottom: 1rem;
}

.drop-zone p {
  margin: 0;
  font-size: 14px;
  font-weight: 500;
}
</style>