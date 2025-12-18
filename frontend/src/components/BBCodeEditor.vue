<template>
  <div class="bbcode-editor">
    <!-- Toolbar -->
    <div class="editor-toolbar" v-if="isEditing">
      <!-- Text Style Group -->
      <div class="toolbar-group">
        <button
          @click="applyFormat('b')"
          :class="{ 'is-active': isTagActive('b') }"
          title="Bold (Ctrl+B)"
          class="toolbar-btn"
        >
          <strong>B</strong>
        </button>
        <button
          @click="applyFormat('i')"
          :class="{ 'is-active': isTagActive('i') }"
          title="Italic (Ctrl+I)"
          class="toolbar-btn"
        >
          <em>I</em>
        </button>
        <button
          @click="applyFormat('u')"
          :class="{ 'is-active': isTagActive('u') }"
          title="Underline (Ctrl+U)"
          class="toolbar-btn"
        >
          <u>U</u>
        </button>
        <button
          @click="applyFormat('s')"
          :class="{ 'is-active': isTagActive('s') }"
          title="Strikethrough"
          class="toolbar-btn"
        >
          <s>S</s>
        </button>
      </div>

      <!-- Headings Group -->
      <div class="toolbar-group">
        <button @click="applyFormat('h1')" title="Heading 1" class="toolbar-btn">H1</button>
        <button @click="applyFormat('h2')" title="Heading 2" class="toolbar-btn">H2</button>
        <button @click="applyFormat('h3')" title="Heading 3" class="toolbar-btn">H3</button>
      </div>

      <!-- Color Group -->
      <div class="toolbar-group">
        <div class="color-picker-wrapper">
          <button title="Text Color" class="color-btn toolbar-btn">
            <span class="color-icon">A</span>
            <input type="color" @change="applyColorFormat" class="color-input" ref="colorInput" />
          </button>
        </div>
        <div class="color-picker-wrapper">
          <button title="Highlight" class="color-btn highlight-btn toolbar-btn">
            <span class="highlight-icon">H</span>
            <input type="color" @change="applyHighlightFormat" class="color-input" ref="highlightInput" value="#ffff00" />
          </button>
        </div>
      </div>

      <!-- Alignment Group -->
      <div class="toolbar-group">
        <button @click="applyFormat('left')" title="Align Left" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="6" x2="21" y2="6"/><line x1="3" y1="12" x2="15" y2="12"/><line x1="3" y1="18" x2="21" y2="18"/></svg>
        </button>
        <button @click="applyFormat('center')" title="Align Center" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="6" x2="21" y2="6"/><line x1="6" y1="12" x2="18" y2="12"/><line x1="3" y1="18" x2="21" y2="18"/></svg>
        </button>
        <button @click="applyFormat('right')" title="Align Right" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="6" x2="21" y2="6"/><line x1="9" y1="12" x2="21" y2="12"/><line x1="3" y1="18" x2="21" y2="18"/></svg>
        </button>
      </div>

      <!-- Lists Group -->
      <div class="toolbar-group">
        <button @click="applyFormat('ul')" title="Bullet List" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="9" y1="6" x2="20" y2="6"/><line x1="9" y1="12" x2="20" y2="12"/><line x1="9" y1="18" x2="20" y2="18"/><line x1="5" y1="6" x2="5" y2="6.01"/><line x1="5" y1="12" x2="5" y2="12.01"/><line x1="5" y1="18" x2="5" y2="18.01"/></svg>
        </button>
        <button @click="applyFormat('ol')" title="Numbered List" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="10" y1="6" x2="21" y2="6"/><line x1="10" y1="12" x2="21" y2="12"/><line x1="10" y1="18" x2="21" y2="18"/><path d="M4 6h1v4"/><path d="M4 10h2"/><path d="M6 18H4c0-1 2-2 2-4s-1-2-2-2"/></svg>
        </button>
        <button @click="insertTodo" title="Insert Todo" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="20 6 9 17 4 12"/></svg>
        </button>
      </div>

      <!-- Block Elements -->
      <div class="toolbar-group">
        <button @click="applyFormat('quote')" title="Quote" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 21c3 0 7-1 7-8V5c0-1.25-4-4-7-4"/><path d="M15 21c3 0 7-1 7-8V5c0-1.25-4-4-7-4"/></svg>
        </button>
        <button @click="applyFormat('code')" title="Code" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="16 18 22 12 16 6"/><polyline points="8 6 2 12 8 18"/></svg>
        </button>
        <button @click="applyFormat('pre')" title="Code Block" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2" ry="2"/><line x1="2" y1="17" x2="22" y2="17"/></svg>
        </button>
      </div>

      <!-- Insert Elements -->
      <div class="toolbar-group">
        <button @click="openLinkDialog" title="Add URL Link" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/></svg>
        </button>
        <button @click="openImageDialog" title="Add Image" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>
        </button>
        <button v-if="isEditing" @click="openImageUploadDialog" title="Upload Image" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
        </button>
        <button @click="openTableDialog" title="Insert Table" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3h18v18H3z"/><path d="M3 9h18M3 15h18M9 3v18M15 3v18"/></svg>
        </button>
      </div>

      <!-- Note/Board Links -->
      <div class="toolbar-group">
        <button @click="openNoteLinkDialog" title="Link to Note" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="12" y1="11" x2="12" y2="17"/><line x1="9" y1="14" x2="15" y2="14"/></svg>
        </button>
        <button @click="openBoardLinkDialog" title="Link to Board" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M2 17h20M6 6h.01M10 6h.01M14 6h.01M18 6h.01"/></svg>
        </button>
        <button v-if="isEditing" @click="openAttachmentLinkDialog" title="Link to Attachment" class="toolbar-btn">
          <Icon name="paperclip" :size="16" />
        </button>
      </div>

      <!-- Other -->
      <div class="toolbar-group">
        <button @click="insertHorizontalRule" title="Horizontal Rule" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="5" y1="12" x2="19" y2="12"/></svg>
        </button>
        <button @click="insertLineBreak" title="Line Break" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="19 14 12 7 5 14"/><polyline points="19 14 12 21 5 14"/></svg>
        </button>
      </div>

      <!-- Undo/Redo -->
      <div class="toolbar-group">
        <button @click="undo" :disabled="!canUndo" title="Undo" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 7v6h6"/><path d="M21 17a9 9 0 00-9-9 9 9 0 00-6 2.3L3 13"/></svg>
        </button>
        <button @click="redo" :disabled="!canRedo" title="Redo" class="toolbar-btn">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 7v6h-6"/><path d="M3 17a9 9 0 019-9 9 9 0 016 2.3l3 2.7"/></svg>
        </button>
      </div>
    </div>

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

    <!-- Link Dialog -->
    <div v-if="dialogs.link.visible" class="modal-overlay" @click.self="dialogs.link.visible = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Add Link</h3>
          <button class="modal-close" @click="dialogs.link.visible = false">&times;</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Display Text</label>
            <input v-model="dialogs.link.text" type="text" placeholder="Link text" />
          </div>
          <div class="form-group">
            <label>URL</label>
            <input v-model="dialogs.link.url" type="text" placeholder="https://example.com" />
          </div>
          <button @click="insertLink" class="btn btn-primary">Insert</button>
        </div>
      </div>
    </div>

    <!-- Image Dialog -->
    <div v-if="dialogs.image.visible" class="modal-overlay" @click.self="dialogs.image.visible = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Add Image</h3>
          <button class="modal-close" @click="dialogs.image.visible = false">&times;</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Image URL</label>
            <input v-model="dialogs.image.url" type="text" placeholder="https://example.com/image.jpg" />
          </div>
          <div class="form-group">
            <label>Alt Text</label>
            <input v-model="dialogs.image.alt" type="text" placeholder="Image description" />
          </div>
          <button @click="insertImage" class="btn btn-primary">Insert</button>
        </div>
      </div>
    </div>

    <!-- Image Upload Dialog -->
    <div v-if="dialogs.imageUpload.visible" class="modal-overlay" @click.self="dialogs.imageUpload.visible = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Upload Image</h3>
          <button class="modal-close" @click="dialogs.imageUpload.visible = false">&times;</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Select Image</label>
            <input 
              ref="imageUploadInput"
              type="file" 
              accept="image/*" 
              @change="handleImageUpload"
              class="file-input"
            />
          </div>
          <div v-if="dialogs.imageUpload.uploading" class="upload-status">
            Uploading...
          </div>
          <button v-else @click="() => imageUploadInput?.click()" class="btn btn-primary">Choose Image</button>
        </div>
      </div>
    </div>

    <!-- Table Dialog -->
    <div v-if="dialogs.table.visible" class="modal-overlay" @click.self="dialogs.table.visible = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Insert Table</h3>
          <button class="modal-close" @click="dialogs.table.visible = false">&times;</button>
        </div>
        <div class="modal-body">
          <div class="form-group">
            <label>Rows</label>
            <input v-model.number="dialogs.table.rows" type="number" min="1" value="3" />
          </div>
          <div class="form-group">
            <label>Columns</label>
            <input v-model.number="dialogs.table.cols" type="number" min="1" value="3" />
          </div>
          <button @click="insertTable" class="btn btn-primary">Insert</button>
        </div>
      </div>
    </div>

    <!-- Note Link Dialog -->
    <div v-if="dialogs.noteLink.visible" class="modal-overlay" @click.self="dialogs.noteLink.visible = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Link to Note</h3>
          <button class="modal-close" @click="dialogs.noteLink.visible = false">&times;</button>
        </div>
        <div class="modal-body">
          <note-tree-view
            :folders="explorerStore.folders"
            :notes="explorerStore.notes"
            @select-note="(note) => { insertNoteLink(note); dialogs.noteLink.visible = false; }"
          />
        </div>
      </div>
    </div>

    <!-- Board Link Dialog -->
    <div v-if="dialogs.boardLink.visible" class="modal-overlay" @click.self="dialogs.boardLink.visible = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Link to Board</h3>
          <button class="modal-close" @click="dialogs.boardLink.visible = false">&times;</button>
        </div>
        <div class="modal-body">
          <div class="link-results" style="max-height: 400px; overflow-y: auto;">
            <div
              v-for="board in explorerStore.boards"
              :key="board.id"
              class="link-result-item"
              @click="insertBoardLink(board)"
            >
              <span class="link-icon">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M2 17h20M6 6h.01M10 6h.01M14 6h.01M18 6h.01"/></svg>
              </span>
              <span class="link-name">{{ board.name }}</span>
            </div>
            <div v-if="explorerStore.boards.length === 0" class="link-empty">No boards found</div>
          </div>
        </div>
      </div>
    </div>

    <!-- Attachment Link Dialog -->
    <div v-if="dialogs.attachmentLink.visible" class="modal-overlay" @click.self="dialogs.attachmentLink.visible = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Link to Attachment</h3>
          <button class="modal-close" @click="dialogs.attachmentLink.visible = false">&times;</button>
        </div>
        <div class="modal-body">
          <div class="link-results" style="max-height: 400px; overflow-y: auto;">
            <div
              v-for="attachment in currentNoteAttachments"
              :key="attachment.id"
              class="link-result-item"
              @click="insertAttachmentLink(attachment)"
            >
              <span class="link-icon">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21.5 2v6h-6M2.5 22v-6h6M2 11.5a10 10 0 0 0 19.8-4.3M22 4.5a10 10 0 0 0-19.8 4.2"/></svg>
              </span>
              <span class="link-name">{{ attachment.original_filename }}</span>
            </div>
            <div v-if="currentNoteAttachments.length === 0" class="link-empty">No attachments in this note</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import Icon from './Icon.vue'
import { useNotesStore } from '../stores/notes'
import { useExplorerStore } from '../stores/explorer'
import { bbcodeToHtml, applyBBCodeFormat as applyBBCodeFormatUtil, isTagActive as isTagActiveUtil, insertBBCodeTag, toggleTodoById } from '../utils/bbcodeFormatter'
import NoteTreeView from './NoteTreeView.vue'
import type { Note, Board, NoteAttachment } from '../types'

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
}>()

const router = useRouter()
const notesStore = useNotesStore()
const explorerStore = useExplorerStore()

const textarea = ref<HTMLTextAreaElement | null>(null)
const colorInput = ref<HTMLInputElement | null>(null)
const highlightInput = ref<HTMLInputElement | null>(null)
const imageUploadInput = ref<HTMLInputElement | null>(null)

const content = ref(props.initialContent)
const isEditing = ref(props.isEditing)
const history = ref<string[]>([props.initialContent])
const historyIndex = ref(0)
const saveTimeout = ref<ReturnType<typeof setTimeout> | null>(null)
const isDraggingOverEditor = ref(false)

const dialogs = ref({
  link: { visible: false, text: '', url: '' },
  image: { visible: false, url: '', alt: '' },
  imageUpload: { visible: false, uploading: false },
  table: { visible: false, rows: 3, cols: 3 },
  noteLink: { visible: false, search: '' },
  boardLink: { visible: false },
  attachmentLink: { visible: false },
})

const renderedHtml = computed(() => bbcodeToHtml(content.value, { paneId: props.paneId }))

const currentNoteAttachments = computed(() => props.attachments || [])

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
    }
  }
}

function applyFormat(tag: string) {
  if (!textarea.value) return
  applyBBCodeFormatUtil(textarea.value, tag)
  onInput()
  textarea.value.focus()
}

function isTagActive(tag: string): boolean {
  if (!textarea.value) return false
  return isTagActiveUtil(textarea.value, tag)
}

function applyColorFormat() {
  if (!colorInput.value || !textarea.value) return
  applyBBCodeFormatUtil(textarea.value, 'color', colorInput.value.value)
  onInput()
}

function applyHighlightFormat() {
  if (!highlightInput.value || !textarea.value) return
  applyBBCodeFormatUtil(textarea.value, 'highlight', highlightInput.value.value)
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

function openLinkDialog() {
  dialogs.value.link = { visible: true, text: '', url: '' }
}

function insertLink() {
  if (!textarea.value) return
  const { text, url } = dialogs.value.link
  const pos = textarea.value.selectionStart
  // new-style attribute: href
  content.value = `${content.value.slice(0, pos)}[url href="${url}"]${text}[/url]${content.value.slice(pos)}`
  onInput()
  dialogs.value.link.visible = false
}

function openImageDialog() {
  dialogs.value.image = { visible: true, url: '', alt: '' }
}

function openImageUploadDialog() {
  dialogs.value.imageUpload = { visible: true, uploading: false }
}

function insertImage() {
  if (!textarea.value) return
  const { url, alt } = dialogs.value.image
  const pos = textarea.value.selectionStart
  // new-style attribute: src
  content.value = `${content.value.slice(0, pos)}[img src="${url}"]${alt}[/img]${content.value.slice(pos)}`
  onInput()
  dialogs.value.image.visible = false
}

function openTableDialog() {
  dialogs.value.table = { visible: true, rows: 3, cols: 3 }
}

function insertTable() {
  if (!textarea.value) return
  const { rows, cols } = dialogs.value.table
  let table = '[table]\n'
  for (let r = 0; r < rows; r++) {
    table += '[tr]'
    for (let c = 0; c < cols; c++) {
      table += r === 0 ? '[th][/th]' : '[td][/td]'
    }
    table += '[/tr]\n'
  }
  table += '[/table]'

  const pos = textarea.value.selectionStart
  content.value = `${content.value.slice(0, pos)}${table}${content.value.slice(pos)}`
  onInput()
  dialogs.value.table.visible = false
}

function openNoteLinkDialog() {
  if (explorerStore.notes.length === 0) {
    explorerStore.fetchAll()
  }
  dialogs.value.noteLink = { visible: true, search: '' }
}

function insertNoteLink(note: Note) {
  if (!textarea.value) return
  const pos = textarea.value.selectionStart
  content.value = `${content.value.slice(0, pos)}[note id="${note.id}"]${note.title}[/note]${content.value.slice(pos)}`
  onInput()
  dialogs.value.noteLink.visible = false
}

function openBoardLinkDialog() {
  if (explorerStore.boards.length === 0) {
    explorerStore.fetchAll()
  }
  dialogs.value.boardLink = { visible: true }
}

function insertBoardLink(board: Board) {
  if (!textarea.value) return
  const pos = textarea.value.selectionStart
  content.value = `${content.value.slice(0, pos)}[board id="${board.id}"]${board.name}[/board]${content.value.slice(pos)}`
  onInput()
  dialogs.value.boardLink.visible = false
}

function openAttachmentLinkDialog() {
  dialogs.value.attachmentLink = { visible: true }
}

function insertAttachmentLink(attachment: NoteAttachment) {
  if (!textarea.value) return
  const pos = textarea.value.selectionStart
  content.value = `${content.value.slice(0, pos)}[attachment id="${attachment.id}"]${attachment.original_filename}[/attachment]${content.value.slice(pos)}`
  onInput()
  dialogs.value.attachmentLink.visible = false
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

async function handleImageUpload(event: Event) {
  const target = event.target as HTMLInputElement
  const files = target.files
  if (!files || files.length === 0) return
  
  emit('image-files-dropped', Array.from(files))
  dialogs.value.imageUpload.visible = false
  target.value = ''
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

.editor-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  padding: 0.5rem;
  background: var(--bg-secondary);
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

.toolbar-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  color: var(--text-secondary);
  transition: all 0.2s;
}

.toolbar-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.toolbar-btn.is-active {
  background: var(--accent);
  color: white;
  border-color: var(--accent);
}

.toolbar-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

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

/* Modal Styles */
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
  padding: 1.5rem;
  min-width: 300px;
  max-width: 500px;
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
}

.modal-close {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-muted);
  padding: 0;
}

.modal-close:hover {
  color: var(--text-primary);
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-weight: 500;
  color: var(--text-primary);
}

.form-group input {
  padding: 0.5rem 0.75rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  color: var(--text-primary);
  border-radius: 4px;
  font-size: 14px;
}

.form-group input:focus {
  outline: none;
  border-color: var(--accent);
}

.file-input {
  display: none;
}

.upload-status {
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 4px;
  color: var(--text-secondary);
  text-align: center;
}

.btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s;
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover {
  background: var(--accent-hover);
}

.search-input {
  padding: 0.5rem 0.75rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  color: var(--text-primary);
  border-radius: 4px;
  margin-bottom: 0.75rem;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent);
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
  flex-shrink: 0;
}

.link-name {
  color: var(--text-primary);
  font-size: 0.875rem;
  font-weight: 500;
  flex: 1;
}

.link-breadcrumb {
  color: var(--text-muted);
  font-size: 0.75rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.link-empty {
  text-align: center;
  padding: 1rem;
  color: var(--text-muted);
  font-size: 0.875rem;
}
</style>
