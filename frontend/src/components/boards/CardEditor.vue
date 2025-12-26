<template>
  <div class="card-editor">
    <!-- Header Bar -->
    <div class="editor-header">
      <h3>{{ card?.title || 'Card Editor' }}</h3>
      <div class="editor-header-actions">
        <button v-if="!isEditing" class="btn btn-primary" @click="isEditing = true">Edit</button>
        <button v-else class="btn btn-primary" @click="handleSave">Save</button>
        <button class="btn btn-secondary" @click="$emit('close')">Close</button>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="editor-main">
      <BBCodeEditor
        v-if="card"
        :initialContent="card.description || ''"
        :isEditing="isEditing"
        :paneId="'card-editor'"
        @update:content="handleContentUpdate"
        @update:isEditing="(val) => (isEditing = val)"
        @dirty="(isDirty) => emit('dirty', isDirty)"
        @image-files-dropped="handleImageFilesDropped"
        @request-save="handleSave"
        @request-done="() => { handleSave(); isEditing = false }"
      />

      <!-- Attachments Panel -->
      <CardEditorAttachments
        :card="card"
        :is-editing="isEditing"
        :show-attachments="showAttachments"
        @image-files-dropped="handleImageFilesDropped"
        @files-uploaded="handleFileUpload"
        @insert-image="insertImageFromAttachment"
        @download-attachment="downloadAttachment"
        @confirm-remove-attachment="confirmRemoveAttachment"
      />
    </div>

    <ConfirmModal
      :visible="deleteAttachmentModal.visible"
      title="Delete Attachment"
      :message="deleteAttachmentModal.message"
      confirm-text="Delete"
      variant="danger"
      @confirm="handleDeleteAttachment"
      @cancel="deleteAttachmentModal.visible = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import BBCodeEditor from '../editor/BBCodeEditor.vue'
import CardEditorAttachments from './CardEditorAttachments.vue'
import ConfirmModal from '../common/modals/ConfirmModal.vue'
import { cardsApi } from '../../api'
import type { CardWithAttachments } from '../../types'

const props = defineProps<{
  card: CardWithAttachments | null
}>()

const emit = defineEmits<{
  close: []
  save: [card: CardWithAttachments]
  dirty: [isDirty: boolean]
}>()

const isEditing = ref(false)
const showAttachments = ref(false)

const deleteAttachmentModal = ref({
  visible: false,
  attachment: null as any,
  message: ''
})

watch(() => props.card, (newCard) => {
  if (newCard) {
    isEditing.value = false
  }
})

function handleContentUpdate(newContent: string) {
  if (!props.card) return
  props.card.description = newContent
  emit('dirty', true)
}

function handleSave() {
  if (!props.card) return
  emit('save', props.card)
  isEditing.value = false
}

function handleImageFilesDropped(files: File[]) {
  if (!props.card) return
  uploadFiles(files)
}

function handleFileUpload(files: File[]) {
  if (!props.card) return
  uploadFiles(files)
}

async function uploadFiles(files: File[]) {
  if (!props.card) return

  try {
    const response = await cardsApi.uploadAttachments(props.card.id, files)
    if (props.card.attachments) {
      props.card.attachments.push(...response.data)
    } else {
      props.card.attachments = response.data
    }
    emit('save', props.card)
  } catch (error) {
    console.error('Failed to upload files:', error)
  }
}

function insertImageFromAttachment(attachment: any) {
  // This would need to be passed to BBCodeEditor to insert image
  // For now, copy URL to clipboard
  const url = cardsApi.getAttachmentUrl(attachment.id)
  navigator.clipboard.writeText(`[img=${url}]${attachment.original_filename}[/img]`)
}

async function downloadAttachment(attachment: any) {
  try {
    const response = await cardsApi.downloadAttachment(attachment.id)
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

function confirmRemoveAttachment(attachment: any) {
  deleteAttachmentModal.value = {
    visible: true,
    attachment,
    message: `Are you sure you want to delete "${attachment.original_filename}"?`
  }
}

async function handleDeleteAttachment() {
  const attachment = deleteAttachmentModal.value.attachment
  deleteAttachmentModal.value.visible = false
  if (!attachment) return
  
  try {
    await cardsApi.deleteAttachment(attachment.id)
    if (props.card?.attachments) {
      props.card.attachments = props.card.attachments.filter(a => a.id !== attachment.id)
    }
    if (props.card) {
      emit('save', props.card)
    }
  } catch (error) {
    console.error('Failed to delete attachment:', error)
  }
}
</script>

<style scoped>
.card-editor {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: var(--bg-secondary);
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
  z-index: 10;
}

.editor-header h3 {
  margin: 0;
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text-primary);
}

.editor-header-actions {
  display: flex;
  gap: 0.75rem;
}

.editor-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.375rem;
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  text-decoration: none;
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
}

.btn-secondary {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-primary);
}

.btn-secondary:hover:not(:disabled) {
  background: var(--bg-hover);
}
</style>