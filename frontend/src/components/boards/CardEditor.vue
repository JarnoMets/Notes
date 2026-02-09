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
        @print-note="printCard"
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
import ConfirmModal from '../modals/ConfirmModal.vue'
import { cardsApi } from '../../api'
import { bbcodeToHtml } from '../../utils/bbcodeFormatter'
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

function printCard() {
  if (!props.card) return
  
  const printWindow = window.open('', '_blank')
  if (!printWindow) {
    alert('Please allow popups to print the card.')
    return
  }
  
  const renderedHtml = bbcodeToHtml(props.card.description || '', { paneId: 'card-editor' })
  
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
      <title>${props.card.title}</title>
      <style>${styles}</style>
    </head>
    <body>
      <h1>${props.card.title}</h1>
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