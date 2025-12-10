<template>
  <div class="attachments-panel">
    <div class="attachments-header">
      <span>Attachments</span>
      <label class="upload-btn" v-if="isEditing">
        + Add
        <input type="file" multiple @change="$emit('upload', $event)" class="file-input" />
      </label>
    </div>
    
    <div class="attachments-list" v-if="attachments && attachments.length > 0">
      <div v-for="attachment in attachments" :key="attachment.id" class="attachment-item">
        <span class="attachment-icon">
          <Icon :name="getFileIcon(attachment.mime_type)" :size="16" />
        </span>
        <div class="attachment-info">
          <span class="attachment-name">{{ attachment.original_filename }}</span>
          <span class="attachment-size">{{ formatFileSize(attachment.size) }}</span>
        </div>
        <div class="attachment-actions">
          <button 
            v-if="isEditing && isImage(attachment.mime_type)" 
            @click="$emit('insert', attachment)" 
            title="Insert"
          >
            <Icon name="insert" :size="12" />
          </button>
          <button @click="$emit('download', attachment)" title="Download">
            <Icon name="download" :size="12" />
          </button>
          <button v-if="isEditing" @click="$emit('delete', attachment)" title="Delete">
            <Icon name="trash" :size="12" />
          </button>
        </div>
      </div>
    </div>
    
    <div v-else class="attachments-empty">
      {{ isEditing ? 'Drop files here or click Add' : 'No attachments' }}
    </div>
  </div>
</template>

<script setup lang="ts">
import type { NoteAttachment } from '../../types'
import { formatFileSize, isImageMimeType, getFileIcon } from '../../composables'
import Icon from '../Icon.vue'

defineProps<{
  attachments: NoteAttachment[]
  isEditing: boolean
}>()

defineEmits<{
  upload: [event: Event]
  insert: [attachment: NoteAttachment]
  download: [attachment: NoteAttachment]
  delete: [attachment: NoteAttachment]
}>()

function isImage(mimeType: string): boolean {
  return isImageMimeType(mimeType)
}
</script>

<style scoped>
.attachments-panel {
  width: 260px;
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
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
  color: var(--text-secondary);
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

/* Responsive */
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

@media (hover: none) and (pointer: coarse) {
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
}
</style>
