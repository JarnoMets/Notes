<template>
  <div class="note-editor" v-if="note">
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
        @request-save="saveNote"
        @request-done="() => { saveNote(); isEditing = false }"
        @toggle-attachments="showAttachments = !showAttachments"
        @duplicate-note="duplicateNote"
        @export-note="exportNote"
        @print-note="printNote"
        @confirm-delete-note="confirmDeleteNote"
      />

      <!-- Attachments Panel -->
      <NoteEditorAttachments
        :note="note"
        :is-editing="isEditing"
        :show-attachments="showAttachments"
        @image-files-dropped="handleImageFilesDropped"
        @files-uploaded="handleFileUpload"
        @insert-image="insertImageFromAttachment"
        @download-attachment="downloadAttachment"
        @confirm-remove-attachment="confirmRemoveAttachment"
      />
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
    <div class="modal modal-small">
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
import { ref, watch, onMounted } from 'vue'
import { useNotesStore } from '@/stores/notes'
import { notesApi } from '@/api'
import { extractBBCodeReferences } from '@/utils/bbcode'
import { bbcodeToHtml } from '@/utils/bbcodeFormatter'
import type { NoteWithAttachments, NoteAttachment } from '@/types'
import ConfirmModal from '../modals/ConfirmModal.vue'
import BBCodeEditor from './BBCodeEditor.vue'
import NoteEditorAttachments from './NoteEditorAttachments.vue'
import { useSync } from '@/composables/useSync'

const props = defineProps<{
  noteId: string | null
  paneId: string
}>()

const emit = defineEmits<{
  dirty: [isDirty: boolean]
}>()

const notesStore = useNotesStore()
const { withSync } = useSync()
const note = ref<NoteWithAttachments | null>(null)
const isEditing = ref(false)
const showAttachments = ref(false)
const isDirty = ref(false)
const previousContent = ref<string>('')
const saveTimeout = ref<NodeJS.Timeout | null>(null)

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
  isDirty.value = true
  emit('dirty', true)
  
  // Debounce save
  if (saveTimeout.value) clearTimeout(saveTimeout.value)
  saveTimeout.value = setTimeout(() => {
    saveNote()
  }, 1000)
}

async function saveNote() {
  if (!note.value) return
  try {
    await notesStore.updateNote(note.value.id, { 
      content: note.value.content,
      title: note.value.title 
    })
    isDirty.value = false
    emit('dirty', false)
  } catch (error) {
    console.error('Failed to save note:', error)
  }
}

async function duplicateNote() {
  if (!note.value) return
  
  return withSync(async () => {
    const newNote = await notesStore.createNote(
      `${note.value.title} (copy)`,
      note.value.folder_id
    )
    
    if (newNote) {
      await notesStore.updateNote(newNote.id, { content: note.value.content })
    }
  })
}

function exportNote() {
  if (!note.value) return
  
  const content = note.value.content
  const blob = new Blob([content], { type: 'text/plain' })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = `${note.value.title}.txt`
  a.click()
  URL.revokeObjectURL(url)
}

function printNote() {
  if (!note.value) return
  
  // Create a hidden iframe
  const iframe = document.createElement('iframe')
  iframe.style.position = 'fixed'
  iframe.style.right = '0'
  iframe.style.bottom = '0'
  iframe.style.width = '0'
  iframe.style.height = '0'
  iframe.style.border = '0'
  document.body.appendChild(iframe)
  
  const doc = iframe.contentWindow?.document
  if (!doc) return
  
  // Get the rendered HTML
  const renderedHtml = bbcodeToHtml(note.value.content || '', { paneId: props.paneId })
  
  // Add some basic styles for printing
  const styles = `
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      line-height: 1.6;
      color: #000;
      padding: 40px;
      max-width: 100%;
      margin: 0;
      background: white;
    }
    h1 { font-size: 2.2em; margin-bottom: 0.5em; border-bottom: 2px solid #eee; padding-bottom: 0.3em; }
    h2 { font-size: 1.8em; margin-top: 1.5em; margin-bottom: 0.5em; }
    h3 { font-size: 1.4em; margin-top: 1.2em; margin-bottom: 0.4em; }
    p { margin-bottom: 1em; }
    blockquote {
      border-left: 4px solid #ddd;
      padding: 0.5em 1em;
      margin: 1em 0;
      color: #444;
      font-style: italic;
      background: #f9f9f9;
    }
    pre {
      background: #f4f4f4;
      padding: 1em;
      border-radius: 4px;
      overflow-x: auto;
      white-space: pre-wrap;
      word-wrap: break-word;
      font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
      font-size: 0.9em;
      border: 1px solid #ddd;
    }
    code {
      background: #f4f4f4;
      padding: 0.2em 0.4em;
      border-radius: 3px;
      font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
      font-size: 0.9em;
      border: 1px solid #ddd;
    }
    table {
      border-collapse: collapse;
      width: 100%;
      margin: 1.5em 0;
    }
    th, td {
      border: 1px solid #ddd;
      padding: 10px;
      text-align: left;
    }
    th { background-color: #f5f5f5; font-weight: 600; }
    img { max-width: 100%; height: auto; border-radius: 4px; margin: 1em 0; }
    hr { border: none; border-top: 1px solid #eee; margin: 2em 0; }
    .bbcode-note-link, .bbcode-board-link {
      color: #27ae60;
      text-decoration: none;
      font-weight: 500;
    }
    .bbcode-list { padding-left: 2em; margin: 1em 0; }
    .bbcode-list li { margin-bottom: 0.5em; }
    
    @media print {
      body { padding: 0; }
      @page { margin: 2cm; }
    }
  `
  
  doc.open()
  doc.write(`
    <!DOCTYPE html>
    <html>
    <head>
      <title>${note.value.title}</title>
      <style>${styles}</style>
    </head>
    <body>
      <h1>${note.value.title}</h1>
      <div class="content">${renderedHtml}</div>
      <script>
        window.onload = function() {
          setTimeout(function() {
            window.print();
            setTimeout(function() {
              window.frameElement.remove();
            }, 100);
          }, 500);
        };
      </' + 'script>
    </body>
    </html>
  `)
  doc.close()
}

function confirmDeleteNote() {
  if (!note.value) return
  
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
  
  return withSync(async () => {
    // Delete all attachments associated with this note
    if (note.value!.attachments && note.value!.attachments.length > 0) {
      try {
        await Promise.all(
          note.value!.attachments.map(attachment =>
            notesApi.deleteAttachment(attachment.id)
          )
        )
      } catch (error) {
        console.error('Failed to delete some attachments:', error)
      }
    }
    
    await notesStore.deleteNote(note.value!.id)
  })
}

async function handleFileUpload(files: File[]) {
  if (!note.value) return

  return withSync(async () => {
    try {
      const response = await notesApi.uploadAttachments(note.value!.id, files)
      if (note.value!.attachments) {
        note.value!.attachments.push(...response.data)
      } else {
        note.value!.attachments = response.data
      }
      notesStore.invalidateNoteCache(note.value!.id)
    } catch (error) {
      console.error('Failed to upload files:', error)
    }
  })
}

async function handleImageFilesDropped(files: File[]) {
  if (!note.value) return
  
  return withSync(async () => {
    try {
      const response = await notesApi.uploadAttachments(note.value!.id, files)
      if (note.value!.attachments) {
        note.value!.attachments.push(...response.data)
      } else {
        note.value!.attachments = response.data
      }
      notesStore.invalidateNoteCache(note.value!.id)
    } catch (error) {
      console.error('Failed to upload images:', error)
    }
  })
}

function insertImageFromAttachment(attachment: NoteAttachment) {
  // This would need to be passed to BBCodeEditor to insert image
  // For now, copy URL to clipboard
  const url = notesApi.getAttachmentUrl(attachment.id)
  navigator.clipboard.writeText(`[img=${url}]${attachment.original_filename}[/img]`)
}

async function downloadAttachment(attachment: NoteAttachment) {
  return withSync(async () => {
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
  })
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
  
  return withSync(async () => {
    try {
      await notesApi.deleteAttachment(attachment.id)
      if (note.value?.attachments) {
        note.value.attachments = note.value.attachments.filter(a => a.id !== attachment.id)
      }
      notesStore.invalidateNoteCache(note.value!.id)
    } catch (error) {
      console.error('Failed to delete attachment:', error)
    }
  })
}

function handleRemovedAttachmentDecision(deleteFile: boolean) {
  removedAttachmentModal.value.visible = false
  
  if (deleteFile) {
    const attachment = removedAttachmentModal.value.attachment
    if (attachment) {
      withSync(async () => {
        try {
          await notesApi.deleteAttachment(attachment.id)
          if (note.value?.attachments) {
            note.value.attachments = note.value.attachments.filter(a => a.id !== attachment.id)
          }
          notesStore.invalidateNoteCache(note.value!.id)
        } catch (error) {
          console.error('Failed to delete attachment:', error)
        }
      })
    }
  }
}

async function handleUndo() {
  if (!note.value) return
  return withSync(async () => {
    try {
      await notesStore.undoNote(note.value.id)
      // reload note
      const fresh = await notesStore.fetchNote(note.value.id)
      if (fresh) note.value = fresh
    } catch (error) {
      console.error('Undo failed', error)
    }
  })
}

async function handleRedo() {
  if (!note.value) return
  return withSync(async () => {
    try {
      await notesStore.redoNote(note.value.id)
      const fresh = await notesStore.fetchNote(note.value.id)
      if (fresh) note.value = fresh
    } catch (error) {
      console.error('Redo failed', error)
    }
  })
}

watch(() => props.noteId, loadNote, { immediate: true })

onMounted(() => {
  // No menu handling needed anymore since it's in the header component
})
</script>

<style scoped>
.note-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  width: 100%;
  overflow: hidden;
}

.editor-main {
  flex: 1;
  display: flex;
  flex-direction: row;
  overflow: hidden;
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

.modal-small {
  max-width: 400px;
}
</style>