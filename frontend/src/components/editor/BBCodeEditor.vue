<template>
  <div class="bbcode-editor">
    <!-- Toolbar -->
    <BBCodeToolbar
      :is-editing="isEditing"
      :can-undo="canUndo"
      :can-redo="canRedo"
      :textarea="textarea"
      @apply-format="applyFormat"
      @apply-color-format="applyColorFormat"
      @apply-highlight-format="applyHighlightFormat"
      @insert-horizontal-rule="insertHorizontalRule"
      @insert-line-break="insertLineBreak"
      @insert-todo="insertTodo"
      @open-link-dialog="openLinkDialog"
      @open-image-dialog="openImageDialog"
      @open-image-upload-dialog="openImageUploadDialog"
      @open-table-dialog="openTableDialog"
      @open-note-link-dialog="openNoteLinkDialog"
      @open-board-link-dialog="openBoardLinkDialog"
      @open-attachment-link-dialog="openAttachmentLinkDialog"
      @undo="undo"
      @redo="redo"
    />

    <!-- Editor or Viewer -->
    <div class="editor-content">
      <textarea
        v-if="isEditing"
        ref="textarea"
        v-model="content"
        class="bbcode-textarea"
        @input="onInput"
        @keydown="onKeyDown"
        @dragover="onEditorDragOver"
        @dragleave="onEditorDragLeave"
        @drop="onEditorDrop"
        :class="{ 'drag-over': isDraggingOverEditor }"
        placeholder="Enter your note content using BBCode formatting..."
      ></textarea>

      <div v-else class="view-mode">
        <div class="view-content" v-html="renderedHtml"></div>
      </div>
    </div>

    <!-- Dialogs -->
    <BBCodeEditorDialogs
      ref="dialogsRef"
      :textarea="textarea"
      :attachments="attachments"
      @insert-tag="handleInsertTag"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import BBCodeToolbar from './BBCodeToolbar.vue'
import BBCodeEditorDialogs from './BBCodeEditorDialogs.vue'
import { useNotesStore } from '@/stores/notes'
import { useExplorerStore } from '@/stores/explorer'
import { bbcodeToHtml, applyBBCodeFormat as applyBBCodeFormatUtil, insertBBCodeTag, toggleTodoById } from '@/utils/bbcodeFormatter'
import type { NoteAttachment } from '@/types'

const props = defineProps<{
  initialContent: string
  isEditing: boolean
  paneId: string
  attachments?: NoteAttachment[]
}>()

const emit = defineEmits<{
  (e: 'update:content', value: string): void
  (e: 'update:isEditing', value: boolean): void
  (e: 'dirty', isDirty: boolean): void
  (e: 'image-files-dropped', files: File[]): void
  (e: 'request-undo'): void
  (e: 'request-redo'): void
  (e: 'request-save'): void
  (e: 'request-done'): void
}>()

const router = useRouter()
const notesStore = useNotesStore()
const explorerStore = useExplorerStore()

const textarea = ref<HTMLTextAreaElement | null>(null)
const dialogsRef = ref<InstanceType<typeof BBCodeEditorDialogs> | null>(null)

const content = ref(props.initialContent)
const isEditing = ref(props.isEditing)
const history = ref<string[]>([props.initialContent])
const historyIndex = ref(0)
const saveTimeout = ref<ReturnType<typeof setTimeout> | null>(null)
const isDraggingOverEditor = ref(false)

const renderedHtml = computed(() => bbcodeToHtml(content.value, { paneId: props.paneId }))

const canUndo = computed(() => historyIndex.value > 0)
const canRedo = computed(() => historyIndex.value < history.value.length - 1)

function onInput() {
  const newContent = content.value
  // Add to history
  if (history.value[historyIndex.value] !== newContent) {
    history.value = history.value.slice(0, historyIndex.value + 1)
    history.value.push(newContent)
    historyIndex.value++
  }
  emit('update:content', newContent)
  emit('dirty', true)

  // Auto-save
  if (saveTimeout.value) clearTimeout(saveTimeout.value)
  saveTimeout.value = setTimeout(() => {
    emit('dirty', false)
  }, 1000)
}

function onKeyDown(event: KeyboardEvent) {
  if (event.ctrlKey || event.metaKey) {
    switch (event.key.toLowerCase()) {
      case 'b':
        event.preventDefault()
        applyFormat('b')
        break
      case 'i':
        event.preventDefault()
        applyFormat('i')
        break
      case 'u':
        event.preventDefault()
        applyFormat('u')
        break
      case 'z':
        event.preventDefault()
        if (event.shiftKey) redo()
        else undo()
        break
      case 'y':
        event.preventDefault()
        redo()
        break
      case 's':
        // Save (Ctrl/Cmd+S)
        event.preventDefault()
        emit('request-save')
        break
      case 'k':
        // Insert link (Ctrl/Cmd+K)
        event.preventDefault()
        openLinkDialog()
        break
      case 'enter':
        // Finish editing (Ctrl/Cmd+Enter)
        event.preventDefault()
        emit('request-done')
        break
    }
  }
}

function applyFormat(tag: string) {
  if (!textarea.value) return
  applyBBCodeFormatUtil(textarea.value, tag)
  onInput()
  textarea.value.focus()
}

function applyColorFormat(color: string) {
  if (!textarea.value) return
  applyBBCodeFormatUtil(textarea.value, 'color', color)
  onInput()
}

function applyHighlightFormat(color: string) {
  if (!textarea.value) return
  applyBBCodeFormatUtil(textarea.value, 'highlight', color)
  onInput()
}

function insertHorizontalRule() {
  if (!textarea.value) return
  const pos = textarea.value.selectionStart
  content.value = insertBBCodeTag(content.value, pos, 'hr')
  onInput()
}

function insertLineBreak() {
  if (!textarea.value) return
  const pos = textarea.value.selectionStart
  content.value = insertBBCodeTag(content.value, pos, 'br')
  onInput()
}

function insertTodo() {
  if (!textarea.value) return
  const pos = textarea.value.selectionStart
  // Insert an unchecked todo by default. Users can edit the text between tags.
  // Generate a stable id for the todo
  const id = generateTodoId()
  content.value = `${content.value.slice(0, pos)}[todo id="${id}" checked="0"]New task[/todo]${content.value.slice(pos)}`
  onInput()
  // Move cursor into the inserted task text
  nextTick(() => {
    const start = pos + `[todo id="${id}" checked="0"]`.length
    textarea.value!.selectionStart = start
    textarea.value!.selectionEnd = start + 'New task'.length
    textarea.value!.focus()
  })
}

function generateTodoId(): string {
  // Use crypto.randomUUID when available, otherwise fallback to a pseudo-random id
  try {
    // @ts-ignore - browser API
    if (typeof crypto !== 'undefined' && typeof (crypto as any).randomUUID === 'function') {
      return (crypto as any).randomUUID()
    }
  } catch (e) {
    // fallthrough
  }

  // Fallback: timestamp + random
  return `t-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 9)}`
}

function openLinkDialog() {
  dialogsRef.value?.openLinkDialog()
}

function openImageDialog() {
  dialogsRef.value?.openImageDialog()
}

function openImageUploadDialog() {
  dialogsRef.value?.openImageUploadDialog()
}

function openTableDialog() {
  dialogsRef.value?.openTableDialog()
}

function openNoteLinkDialog() {
  if (explorerStore.notes.length === 0) {
    explorerStore.fetchAll()
  }
  dialogsRef.value?.openNoteLinkDialog()
}

function openBoardLinkDialog() {
  if (explorerStore.boards.length === 0) {
    explorerStore.fetchAll()
  }
  dialogsRef.value?.openBoardLinkDialog()
}

function openAttachmentLinkDialog() {
  dialogsRef.value?.openAttachmentLinkDialog()
}

function handleInsertTag(tag: string, contentText: string) {
  if (!textarea.value) return
  const pos = textarea.value.selectionStart
  const tagContent = tag ? `[${tag}]${contentText}[/${tag.split(' ')[0]}]` : contentText
  content.value = `${content.value.slice(0, pos)}${tagContent}${content.value.slice(pos)}`
  onInput()
}

function undo() {
  if (canUndo.value) {
    historyIndex.value--
    content.value = history.value[historyIndex.value]
    emit('update:content', content.value)
  }
}

function redo() {
  if (canRedo.value) {
    historyIndex.value++
    content.value = history.value[historyIndex.value]
    emit('update:content', content.value)
  }
}

watch(() => props.initialContent, (newVal) => {
  content.value = newVal
  history.value = [newVal]
  historyIndex.value = 0
})

watch(() => props.isEditing, (newVal) => {
  isEditing.value = newVal
})

onMounted(async () => {
  // Ensure explorer data is loaded
  if (explorerStore.notes.length === 0 || explorerStore.boards.length === 0) {
    await explorerStore.fetchAll()
  }

  // Setup global functions for link clicking
  ;(window as any).__openNote = (noteId: string) => notesStore.openNote(noteId, props.paneId)
  ;(window as any).__openBoard = (boardId: string) => {
    localStorage.setItem('selectedBoardId', boardId)
    router.push('/boards')
  }
  // Toggle todo callback used by rendered HTML checkboxes
  ;(window as any).__toggleTodo = (paneId: string, todoId: string) => {
    try {
      if (paneId !== props.paneId) return
      // Update the BBCode content by toggling the todo with given id
      content.value = toggleTodoById(content.value, String(todoId))
      // Emit change and mark dirty/save
      onInput()
    } catch (err) {
      console.error('Failed to toggle todo:', err)
    }
  }
})

function onEditorDragOver(e: DragEvent) {
  e.preventDefault()
  isDraggingOverEditor.value = true
  const isExplorer = e.dataTransfer?.types?.includes('application/x-explorer-item')
  const isFiles = e.dataTransfer?.types?.includes('Files')
  if (isExplorer || isFiles) {
    e.dataTransfer!.dropEffect = isFiles ? 'copy' : 'link'
  }
}

function onEditorDragLeave() {
  isDraggingOverEditor.value = false
}

function onEditorDrop(e: DragEvent) {
  e.preventDefault()
  isDraggingOverEditor.value = false
  
  // Try to get explorer item drag data (from tree)
  let dragData = e.dataTransfer?.getData('application/x-explorer-item')
  
  if (dragData) {
    try {
      const data = JSON.parse(dragData)
      
      if (!textarea.value) return
      
      const pos = textarea.value.selectionStart
      let linkContent = ''
      
      if (data.type === 'note') {
        linkContent = `[note id="${data.id}"]${data.name}[/note]`
      } else if (data.type === 'board') {
        linkContent = `[board id="${data.id}"]${data.name}[/board]`
      } else {
        // Skip folders and other types
        return
      }
      
      content.value = `${content.value.slice(0, pos)}${linkContent}${content.value.slice(pos)}`
      onInput()
    } catch (error) {
      console.error('Failed to parse drag data:', error)
    }
  } else {
    // Try to handle file drops
    const files = e.dataTransfer?.files
    if (files && files.length > 0) {
      // Handle image files
      const imageFiles = Array.from(files).filter(f => f.type.startsWith('image/'))
      if (imageFiles.length > 0) {
        handleDroppedImages(imageFiles)
      }
    }
  }
}

async function handleDroppedImages(files: File[]) {
  if (!textarea.value) return
  
  // We need a note ID to upload attachments
  // This should be passed from parent or we need to emit an event
  // For now, we'll emit an event to the parent component
  emit('image-files-dropped', files)
}
</script>

<style scoped>
.bbcode-editor {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: var(--bg-primary);
}

.editor-content {
  flex: 1;
  display: flex;
  overflow: hidden;
  width: 100%;
  height: 100%;
}

.bbcode-textarea {
  flex: 1;
  padding: 1.5rem 2rem;
  background: var(--bg-primary);
  color: var(--text-primary);
  border: none;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.8;
  resize: none;
  outline: none;
  width: 100%;
  height: 100%;
  white-space: pre-wrap;
  word-wrap: break-word;
  tab-size: 2;
  transition: background-color 0.2s;
}

.bbcode-textarea.drag-over {
  background-color: color-mix(in srgb, var(--accent) 5%, var(--bg-primary));
  border: 2px dashed var(--accent);
}

.bbcode-textarea::placeholder {
  color: var(--text-muted);
}

.view-mode {
  flex: 1;
  width: 100%;
  height: 100%;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 1.5rem 2rem;
  cursor: text;
  background: var(--bg-primary);
}

.view-content {
  min-height: 100%;
  color: var(--text-primary);
  line-height: 1.8;
  word-wrap: break-word;
  overflow-wrap: break-word;
  white-space: pre-wrap;
  word-break: break-word;
}

/* BBCode rendered styles */
.view-content :deep(h1) {
  font-size: 2.2em;
  margin: 1.2em 0 0.6em;
  color: var(--text-primary);
  font-weight: 700;
  line-height: 1.2;
  border-bottom: 2px solid var(--border-primary);
  padding-bottom: 0.5em;
}

.view-content :deep(h2) {
  font-size: 1.8em;
  margin: 1em 0 0.5em;
  color: var(--text-primary);
  font-weight: 600;
  line-height: 1.3;
  border-bottom: 1px solid var(--border-primary);
  padding-bottom: 0.3em;
}

.view-content :deep(h3) {
  font-size: 1.4em;
  margin: 0.8em 0 0.4em;
  color: var(--text-primary);
  font-weight: 600;
  line-height: 1.3;
}

.view-content :deep(strong) {
  font-weight: 700;
}

.view-content :deep(em) {
  font-style: italic;
}

.view-content :deep(u) {
  text-decoration: underline;
}

.view-content :deep(s) {
  text-decoration: line-through;
}

.view-content :deep(blockquote) {
  border-left: 4px solid var(--accent);
  padding: 0.75rem 1.25rem;
  margin: 1rem 0;
  color: var(--text-secondary);
  font-style: italic;
  background: var(--bg-tertiary);
  border-radius: 0 8px 8px 0;
}

.view-content :deep(code) {
  background: var(--bg-tertiary);
  padding: 0.3em 0.6em;
  border-radius: 4px;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 0.9em;
  color: var(--accent);
  white-space: pre-wrap;
}

.view-content :deep(pre) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  padding: 1.25rem;
  border-radius: 8px;
  overflow-x: auto;
  font-family: 'Monaco', 'Menlo', 'Courier New', monospace;
  font-size: 0.9em;
  margin: 1rem 0;
  border-left: 4px solid var(--accent);
}

.view-content :deep(pre code) {
  background: none;
  padding: 0;
  color: inherit;
  white-space: pre-wrap;
}

.view-content :deep(a) {
  color: var(--accent);
  text-decoration: none;
  border-bottom: 1px solid transparent;
  transition: all 0.2s ease;
  cursor: pointer;
}

.view-content :deep(a:hover) {
  border-bottom-color: var(--accent);
  opacity: 0.8;
}

.view-content :deep(.bbcode-url-link) {
  color: var(--info, #2196f3);
}

.view-content :deep(.bbcode-link) {
  display: inline-block;
  padding: 0.1em 0.4em;
  border-radius: 4px;
  text-decoration: none;
  font-weight: 500;
  transition: all 0.2s ease;
  user-select: none;
}

.view-content :deep(.bbcode-note-link) {
  background: rgba(39, 174, 96, 0.12);
  color: #27ae60;
  border-left: 3px solid #27ae60;
  padding: 0.25em 0.5em;
  border-radius: 3px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.view-content :deep(.bbcode-note-link:hover) {
  background: rgba(39, 174, 96, 0.25);
  transform: translateX(2px);
  border-left-width: 4px;
}

.view-content :deep(.bbcode-board-link) {
  background: rgba(25, 118, 210, 0.12);
  color: #1976d2;
  border-left: 3px solid #1976d2;
  padding: 0.25em 0.5em;
  border-radius: 3px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.view-content :deep(.bbcode-board-link:hover) {
  background: rgba(25, 118, 210, 0.25);
  transform: translateX(2px);
  border-left-width: 4px;
}

.view-content :deep(.bbcode-list) {
  margin: 0.75rem 0;
  padding-left: 2rem;
}

.view-content :deep(.bbcode-list li) {
  margin: 0.35rem 0;
}

.view-content :deep(.bbcode-table) {
  border-collapse: collapse;
  width: 100%;
  margin: 1rem 0;
  border-radius: 8px;
  overflow: hidden;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.view-content :deep(.bbcode-table td),
.view-content :deep(.bbcode-table th) {
  border: 1px solid var(--border-primary);
  padding: 0.75rem;
  min-width: 80px;
  text-align: left;
}

.view-content :deep(.bbcode-table th) {
  background-color: var(--bg-tertiary);
  font-weight: 600;
  color: var(--text-primary);
}

.view-content :deep(.bbcode-table td) {
  background-color: var(--bg-secondary);
}

.view-content :deep(.bbcode-image) {
  max-width: 100%;
  height: auto;
  border-radius: 8px;
  margin: 1rem 0;
}

.view-content :deep(hr) {
  border: none;
  border-top: 2px solid var(--border-primary);
  margin: 2rem 0;
}

.view-content :deep(br) {
  content: '';
  display: block;
}
</style>
