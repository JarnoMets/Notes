<template>
  <!-- Link Dialog -->
  <FormModal
    :visible="showLinkDialog"
    title="Add Link"
    :fields="linkFields"
    submit-label="Insert Link"
    @close="showLinkDialog = false"
    @submit="handleLinkConfirm"
  />

  <!-- Image Dialog -->
  <FormModal
    :visible="showImageDialog"
    title="Add Image"
    :fields="imageFields"
    submit-label="Insert Image"
    @close="showImageDialog = false"
    @submit="handleImageConfirm"
  />

  <!-- Image Upload Dialog -->
  <FormModal
    :visible="showImageUploadDialog"
    title="Upload Image"
    :fields="imageUploadFields"
    submit-label="Upload & Insert"
    @close="showImageUploadDialog = false"
    @submit="handleImageUploadConfirm"
  />

  <!-- Table Dialog -->
  <FormModal
    :visible="showTableDialog"
    title="Insert Table"
    :fields="tableFields"
    submit-label="Insert Table"
    @close="showTableDialog = false"
    @submit="handleTableConfirm"
  />

  <!-- Note Link Dialog -->
  <FormModal
    :visible="showNoteLinkDialog"
    title="Link to Note"
    :fields="noteLinkFields"
    submit-label="Insert Link"
    @close="showNoteLinkDialog = false"
    @submit="handleNoteLinkConfirm"
  />

  <!-- Board Link Dialog -->
  <FormModal
    :visible="showBoardLinkDialog"
    title="Link to Board"
    :fields="boardLinkFields"
    submit-label="Insert Link"
    @close="showBoardLinkDialog = false"
    @submit="handleBoardLinkConfirm"
  />

  <!-- Attachment Link Dialog -->
  <FormModal
    :visible="showAttachmentLinkDialog"
    title="Link to Attachment"
    :fields="attachmentLinkFields"
    submit-label="Insert Link"
    @close="showAttachmentLinkDialog = false"
    @submit="handleAttachmentLinkConfirm"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'
import FormModal from '../common/modals/FormModal.vue'
import { useNotesStore } from '@/stores/notes'
import { useExplorerStore } from '@/stores/explorer'
import type { NoteAttachment } from '@/types'

const props = defineProps<{
  textarea: HTMLTextAreaElement | null
  attachments?: NoteAttachment[]
}>()

const emit = defineEmits<{
  'insert-tag': [tag: string, content: string]
}>()

const notesStore = useNotesStore()
const explorerStore = useExplorerStore()

// Dialog visibility states
const showLinkDialog = ref(false)
const showImageDialog = ref(false)
const showImageUploadDialog = ref(false)
const showTableDialog = ref(false)
const showNoteLinkDialog = ref(false)
const showBoardLinkDialog = ref(false)
const showAttachmentLinkDialog = ref(false)

// Field definitions for dialogs
const linkFields = [
  { name: 'url', label: 'URL', type: 'url' as const, placeholder: 'https://example.com', required: true },
  { name: 'text', label: 'Link Text', type: 'text' as const, placeholder: 'Click here', required: true }
]

const imageFields = [
  { name: 'url', label: 'Image URL', type: 'url' as const, placeholder: 'https://example.com/image.jpg', required: true },
  { name: 'alt', label: 'Alt Text', type: 'text' as const, placeholder: 'Image description' }
]

const imageUploadFields = [
  { name: 'url', label: 'Image URL', type: 'url' as const, placeholder: 'https://example.com/image.jpg', required: true },
  { name: 'alt', label: 'Alt Text', type: 'text' as const, placeholder: 'Image description' }
]

const tableFields = [
  { name: 'rows', label: 'Rows', type: 'text' as const, placeholder: '3', required: true },
  { name: 'cols', label: 'Columns', type: 'text' as const, placeholder: '3', required: true }
]

const noteLinkFields = [
  { name: 'note', label: 'Note', type: 'select' as const, required: true, options: notesStore.notes.map((note: any) => ({ value: note.id, label: note.title })) }
]

const boardLinkFields = [
  { name: 'board', label: 'Board', type: 'select' as const, required: true, options: explorerStore.boards.map((board: any) => ({ value: board.id, label: board.name })) }
]

const attachmentLinkFields = [
  { name: 'attachment', label: 'Attachment', type: 'select' as const, required: true, options: props.attachments?.map((att: NoteAttachment) => ({ value: att.id, label: att.filename })) || [] }
]

// Dialog handlers
function handleLinkConfirm(data: Record<string, string>) {
  const tag = `url=${data.url}`
  emit('insert-tag', tag, data.text)
}

function handleImageConfirm(data: Record<string, string>) {
  const tag = `img${data.alt ? ` alt=${data.alt}` : ''}`
  emit('insert-tag', tag, data.url)
}

async function handleImageUploadConfirm(data: Record<string, string>) {
  // For now, treat upload as regular image insert
  const tag = `img${data.alt ? ` alt=${data.alt}` : ''}`
  emit('insert-tag', tag, data.url)
}

function handleTableConfirm(data: Record<string, string>) {
  const rows = parseInt(data.rows) || 3
  const cols = parseInt(data.cols) || 3

  let tableContent = ''

  // Add header row
  tableContent += '|'
  for (let c = 0; c < cols; c++) {
    tableContent += ` Header ${c + 1} |`
  }
  tableContent += '\n|'
  for (let c = 0; c < cols; c++) {
    tableContent += '---|'
  }
  tableContent += '\n'

  // Add data rows
  for (let r = 1; r < rows; r++) {
    tableContent += '|'
    for (let c = 0; c < cols; c++) {
      tableContent += ` Cell ${r + 1}-${c + 1} |`
    }
    tableContent += '\n'
  }

  emit('insert-tag', 'table', tableContent.trim())
}

function handleNoteLinkConfirm(data: Record<string, string>) {
  const note = notesStore.notes.find(n => n.id === data.note)
  if (note) {
    const tag = `note=${note.id}`
    emit('insert-tag', tag, note.title)
  }
}

function handleBoardLinkConfirm(data: Record<string, string>) {
  const board = explorerStore.boards.find(b => b.id === data.board)
  if (board) {
    const tag = `board=${board.id}`
    emit('insert-tag', tag, board.name)
  }
}

function handleAttachmentLinkConfirm(data: Record<string, string>) {
  const attachment = props.attachments?.find((att: NoteAttachment) => att.id === data.attachment)
  if (attachment) {
    const tag = `attachment=${attachment.id}`
    emit('insert-tag', tag, attachment.filename)
  }
}

// Expose methods to parent component
defineExpose({
  openLinkDialog: () => showLinkDialog.value = true,
  openImageDialog: () => showImageDialog.value = true,
  openImageUploadDialog: () => showImageUploadDialog.value = true,
  openTableDialog: () => showTableDialog.value = true,
  openNoteLinkDialog: () => showNoteLinkDialog.value = true,
  openBoardLinkDialog: () => showBoardLinkDialog.value = true,
  openAttachmentLinkDialog: () => showAttachmentLinkDialog.value = true
})
</script>