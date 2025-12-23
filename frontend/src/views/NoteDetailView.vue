<template>
  <div class="note-detail" v-if="note">
    <div class="header">
      <button class="btn btn-secondary" @click="router.push('/notes')">← Back</button>
      <div class="header-actions">
        <span v-if="saving" class="saving-indicator">Saving...</span>
        <span v-else class="saved-indicator">✓ Saved</span>
        <button class="btn btn-danger" @click="deleteNote">Delete</button>
      </div>
    </div>

    <div class="note-content">
      <!-- Title -->
      <input 
        v-model="note.title" 
        class="note-title" 
        @blur="saveNote"
        @keydown.enter.prevent="focusEditor"
        placeholder="Note title"
      />

      <!-- Description -->
      <textarea 
        v-model="note.description" 
        class="note-description"
        @blur="saveNote"
        placeholder="Brief description..."
        rows="2"
      ></textarea>

      <!-- Rich Text Toolbar -->
      <div class="editor-toolbar" v-if="editor">
        <!-- Text Style Group -->
        <div class="toolbar-group">
          <button 
            @click="editor.chain().focus().toggleBold().run()"
            :class="{ 'is-active': editor.isActive('bold') }"
            title="Bold (Ctrl+B)"
          >
            <strong>B</strong>
          </button>
          <button 
            @click="editor.chain().focus().toggleItalic().run()"
            :class="{ 'is-active': editor.isActive('italic') }"
            title="Italic (Ctrl+I)"
          >
            <em>I</em>
          </button>
          <button 
            @click="editor.chain().focus().toggleUnderline().run()"
            :class="{ 'is-active': editor.isActive('underline') }"
            title="Underline (Ctrl+U)"
          >
            <u>U</u>
          </button>
          <button 
            @click="editor.chain().focus().toggleStrike().run()"
            :class="{ 'is-active': editor.isActive('strike') }"
            title="Strikethrough"
          >
            <s>S</s>
          </button>
        </div>

        <!-- Headings Group -->
        <div class="toolbar-group">
          <button 
            @click="editor.chain().focus().toggleHeading({ level: 1 }).run()"
            :class="{ 'is-active': editor.isActive('heading', { level: 1 }) }"
            title="Heading 1"
          >
            H1
          </button>
          <button 
            @click="editor.chain().focus().toggleHeading({ level: 2 }).run()"
            :class="{ 'is-active': editor.isActive('heading', { level: 2 }) }"
            title="Heading 2"
          >
            H2
          </button>
          <button 
            @click="editor.chain().focus().toggleHeading({ level: 3 }).run()"
            :class="{ 'is-active': editor.isActive('heading', { level: 3 }) }"
            title="Heading 3"
          >
            H3
          </button>
        </div>

        <!-- Text Color Group -->
        <div class="toolbar-group">
          <div class="color-picker-wrapper">
            <button title="Text Color" class="color-btn">
              <span class="color-icon">A</span>
              <input 
                type="color" 
                @input="setColor($event)"
                class="color-input"
              />
            </button>
          </div>
          <div class="color-picker-wrapper">
            <button title="Highlight" class="color-btn highlight-btn">
              <span class="highlight-icon">H</span>
              <input 
                type="color" 
                @input="setHighlight($event)"
                class="color-input"
                value="#ffff00"
              />
            </button>
          </div>
          <button 
            @click="editor.chain().focus().unsetColor().unsetHighlight().run()"
            title="Remove Color"
          >
            ✕
          </button>
        </div>

        <!-- Alignment Group -->
        <div class="toolbar-group">
          <button 
            @click="editor.chain().focus().setTextAlign('left').run()"
            :class="{ 'is-active': editor.isActive({ textAlign: 'left' }) }"
            title="Align Left"
          >
            ⫷
          </button>
          <button 
            @click="editor.chain().focus().setTextAlign('center').run()"
            :class="{ 'is-active': editor.isActive({ textAlign: 'center' }) }"
            title="Align Center"
          >
            ≡
          </button>
          <button 
            @click="editor.chain().focus().setTextAlign('right').run()"
            :class="{ 'is-active': editor.isActive({ textAlign: 'right' }) }"
            title="Align Right"
          >
            ⫸
          </button>
        </div>

        <!-- Lists Group -->
        <div class="toolbar-group">
          <button 
            @click="editor.chain().focus().toggleBulletList().run()"
            :class="{ 'is-active': editor.isActive('bulletList') }"
            title="Bullet List"
          >
            •
          </button>
          <button 
            @click="editor.chain().focus().toggleOrderedList().run()"
            :class="{ 'is-active': editor.isActive('orderedList') }"
            title="Numbered List"
          >
            1.
          </button>
        </div>

        <!-- Block Elements Group -->
        <div class="toolbar-group">
          <button 
            @click="editor.chain().focus().toggleBlockquote().run()"
            :class="{ 'is-active': editor.isActive('blockquote') }"
            title="Quote"
          >
            "
          </button>
          <button 
            @click="editor.chain().focus().toggleCodeBlock().run()"
            :class="{ 'is-active': editor.isActive('codeBlock') }"
            title="Code Block"
          >
            &lt;/&gt;
          </button>
          <button 
            @click="editor.chain().focus().setHorizontalRule().run()"
            title="Horizontal Rule"
          >
            ─
          </button>
        </div>

        <!-- Insert Elements Group -->
        <div class="toolbar-group">
          <button 
            @click="addLink"
            :class="{ 'is-active': editor.isActive('link') }"
            title="Add Link"
          >
            🔗
          </button>
          <button 
            @click="addImage"
            title="Add Image"
          >
            🖼️
          </button>
          <button 
            @click="toggleSpoiler"
            :class="{ 'is-active': isSpoilerActive }"
            title="Spoiler"
          >
            👁
          </button>
        </div>

        <!-- Table Group -->
        <div class="toolbar-group">
          <button 
            @click="insertTable"
            title="Insert Table"
          >
            📊
          </button>
          <template v-if="editor.isActive('table')">
            <button @click="editor.chain().focus().addColumnAfter().run()" title="Add Column">+|</button>
            <button @click="editor.chain().focus().addRowAfter().run()" title="Add Row">+─</button>
            <button @click="editor.chain().focus().deleteColumn().run()" title="Delete Column">-|</button>
            <button @click="editor.chain().focus().deleteRow().run()" title="Delete Row">-─</button>
            <button @click="editor.chain().focus().deleteTable().run()" title="Delete Table">🗑</button>
          </template>
        </div>

        <!-- Undo/Redo -->
        <div class="toolbar-group">
          <button 
            @click="editor.chain().focus().undo().run()"
            :disabled="!editor.can().undo()"
            title="Undo (Ctrl+Z)"
          >
            ↩
          </button>
          <button 
            @click="editor.chain().focus().redo().run()"
            :disabled="!editor.can().redo()"
            title="Redo (Ctrl+Y)"
          >
            ↪
          </button>
        </div>
      </div>

      <!-- Rich Text Editor -->
      <editor-content :editor="editor" class="editor-content" />

      <!-- Attachments Section -->
      <div class="attachments-section">
        <div class="attachments-header">
          <h3>Attachments</h3>
          <label class="btn btn-secondary btn-sm upload-btn">
            + Add Files
            <input 
              type="file" 
              multiple 
              @change="handleFileUpload"
              class="file-input"
            />
          </label>
        </div>

        <div class="attachments-list" v-if="note.attachments && note.attachments.length > 0">
          <div 
            v-for="attachment in note.attachments" 
            :key="attachment.id"
            class="attachment-item"
          >
            <div class="attachment-icon">
              {{ getFileIcon(attachment.mime_type) }}
            </div>
            <div class="attachment-info">
              <span class="attachment-name">{{ attachment.original_filename }}</span>
              <span class="attachment-size">{{ formatFileSize(attachment.size) }}</span>
            </div>
            <div class="attachment-actions">
              <button 
                v-if="isImageAttachment(attachment.mime_type)"
                @click="insertAttachmentImage(attachment)"
                class="btn btn-icon"
                title="Insert in note"
              >
                📝
              </button>
              <button 
                @click="downloadAttachment(attachment)"
                class="btn btn-icon"
                title="Download"
              >
                ⬇️
              </button>
              <button 
                @click="deleteAttachment(attachment)"
                class="btn btn-icon btn-danger-icon"
                title="Delete"
              >
                🗑️
              </button>
            </div>
          </div>
        </div>

        <div v-else class="no-attachments">
          <p>No attachments yet. Drag & drop files or click "Add Files".</p>
        </div>
      </div>
    </div>

    <div class="note-footer">
      <span>Last updated: {{ formatDate(note.updated_at) }}</span>
    </div>
  </div>

  <div v-else class="loading">
    Loading...
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import { useExplorerStore } from '../stores/explorer'
import StarterKit from '@tiptap/starter-kit'
import Underline from '@tiptap/extension-underline'
import { TextStyle } from '@tiptap/extension-text-style'
import { Color } from '@tiptap/extension-color'
import Highlight from '@tiptap/extension-highlight'
import Link from '@tiptap/extension-link'
import Image from '@tiptap/extension-image'
import { Table } from '@tiptap/extension-table'
import TableRow from '@tiptap/extension-table-row'
import TableCell from '@tiptap/extension-table-cell'
import TableHeader from '@tiptap/extension-table-header'
import Placeholder from '@tiptap/extension-placeholder'
import TextAlign from '@tiptap/extension-text-align'
import { Extension } from '@tiptap/vue-3'
import { notesApi } from '../api'
import type { NoteWithAttachments, NoteAttachment } from '../types'
import logger from '@/utils/logger'

// Custom Spoiler Extension
const Spoiler = Extension.create({
  name: 'spoiler',

  addGlobalAttributes() {
    return [
      {
        types: ['textStyle'],
        attributes: {
          class: {
            default: null,
            parseHTML: element => element.getAttribute('class'),
            renderHTML: attributes => {
              if (!attributes.class) {
                return {}
              }
              return { class: attributes.class }
            },
          },
        },
      },
    ]
  },
})

const router = useRouter()
const route = useRoute()
const explorerStore = useExplorerStore()
const note = ref<NoteWithAttachments | null>(null)
const saving = ref(false)
const saveTimeout = ref<ReturnType<typeof setTimeout> | null>(null)

const editor = useEditor({
  extensions: [
    StarterKit.configure({
      heading: {
        levels: [1, 2, 3],
      },
    }),
    Underline,
    TextStyle,
    Color,
    Highlight.configure({
      multicolor: true,
    }),
    Link.configure({
      openOnClick: false,
      HTMLAttributes: {
        class: 'editor-link',
      },
    }),
    Image.configure({
      inline: true,
      allowBase64: true,
      HTMLAttributes: {
        class: 'editor-image',
      },
    }),
    Table.configure({
      resizable: true,
      HTMLAttributes: {
        class: 'editor-table',
      },
    }),
    TableRow,
    TableCell,
    TableHeader,
    Placeholder.configure({
      placeholder: 'Start writing your note...',
    }),
    TextAlign.configure({
      types: ['heading', 'paragraph'],
    }),
    Spoiler,
  ],
  content: '',
  onUpdate: () => {
    debouncedSave()
  },
})

const isSpoilerActive = computed(() => {
  if (!editor.value) return false
  const { from, to } = editor.value.state.selection
  let hasSpoiler = false
  editor.value.state.doc.nodesBetween(from, to, node => {
    if (node.marks) {
      node.marks.forEach(mark => {
        if (mark.attrs.class === 'spoiler') {
          hasSpoiler = true
        }
      })
    }
  })
  return hasSpoiler
})

const fetchNote = async () => {
  try {
    const id = route.params.id as string
    const response = await notesApi.get(id)
    note.value = response.data
    if (editor.value && note.value) {
      editor.value.commands.setContent(note.value.content || '')
    }
  } catch (error) {
    logger.error('Failed to fetch note:', error)
    router.push('/notes')
  }
}

const debouncedSave = () => {
  if (saveTimeout.value) {
    clearTimeout(saveTimeout.value)
  }
  saveTimeout.value = setTimeout(() => {
    saveNote()
  }, 1000)
}

const saveNote = async () => {
  if (!note.value || !editor.value) return
  
  saving.value = true
  try {
    const content = editor.value.getHTML()
    await notesApi.update(note.value.id, {
      title: note.value.title,
      description: note.value.description,
      content: content
    })
  } catch (error) {
    logger.error('Failed to save note:', error)
  } finally {
    saving.value = false
  }
}

const deleteNote = async () => {
  if (!note.value) return
  if (!confirm('Are you sure you want to delete this note?')) return

  try {
    await notesApi.delete(note.value.id)
    // Refresh the explorer tree to reflect the deletion
    await explorerStore.fetchNotes()
    router.push('/notes')
  } catch (error) {
    logger.error('Failed to delete note:', error)
  }
}

const focusEditor = () => {
  editor.value?.commands.focus()
}

const setColor = (event: Event) => {
  const target = event.target as HTMLInputElement
  editor.value?.chain().focus().setColor(target.value).run()
}

const setHighlight = (event: Event) => {
  const target = event.target as HTMLInputElement
  editor.value?.chain().focus().toggleHighlight({ color: target.value }).run()
}

const addLink = () => {
  const url = prompt('Enter URL:')
  if (url) {
    editor.value?.chain().focus().setLink({ href: url }).run()
  }
}

const addImage = () => {
  const url = prompt('Enter image URL:')
  if (url) {
    editor.value?.chain().focus().setImage({ src: url }).run()
  }
}

const toggleSpoiler = () => {
  if (!editor.value) return
  
  const { from, to } = editor.value.state.selection
  let hasSpoiler = false
  
  editor.value.state.doc.nodesBetween(from, to, node => {
    if (node.marks) {
      node.marks.forEach(mark => {
        if (mark.attrs.class === 'spoiler') {
          hasSpoiler = true
        }
      })
    }
  })

  if (hasSpoiler) {
    editor.value.chain().focus().unsetMark('textStyle').run()
  } else {
    editor.value.chain().focus().setMark('textStyle', { class: 'spoiler' }).run()
  }
}

const insertTable = () => {
  editor.value?.chain().focus().insertTable({ rows: 3, cols: 3, withHeaderRow: true }).run()
}

// Attachment handling
const handleFileUpload = async (event: Event) => {
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
  } catch (error) {
    logger.error('Failed to upload files:', error)
    alert('Failed to upload files')
  }
  
  // Reset input
  target.value = ''
}

const downloadAttachment = async (attachment: NoteAttachment) => {
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
    logger.error('Failed to download attachment:', error)
  }
}

const deleteAttachment = async (attachment: NoteAttachment) => {
  if (!confirm(`Delete "${attachment.original_filename}"?`)) return
  
  try {
    await notesApi.deleteAttachment(attachment.id)
    if (note.value?.attachments) {
      note.value.attachments = note.value.attachments.filter(a => a.id !== attachment.id)
    }
  } catch (error) {
    logger.error('Failed to delete attachment:', error)
  }
}

const insertAttachmentImage = (attachment: NoteAttachment) => {
  const url = notesApi.getAttachmentUrl(attachment.id)
  editor.value?.chain().focus().setImage({ src: url, alt: attachment.original_filename }).run()
}

const isImageAttachment = (mimeType: string): boolean => {
  return mimeType.startsWith('image/')
}

const getFileIcon = (mimeType: string): string => {
  if (mimeType.startsWith('image/')) return '🖼️'
  if (mimeType.startsWith('video/')) return '🎬'
  if (mimeType.startsWith('audio/')) return '🎵'
  if (mimeType.includes('pdf')) return '📄'
  if (mimeType.includes('word') || mimeType.includes('document')) return '📝'
  if (mimeType.includes('sheet') || mimeType.includes('excel')) return '📊'
  if (mimeType.includes('zip') || mimeType.includes('archive')) return '📦'
  return '📎'
}

const formatFileSize = (bytes: number): string => {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
  return (bytes / (1024 * 1024 * 1024)).toFixed(1) + ' GB'
}

const formatDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  return date.toLocaleString('en-US', { 
    month: 'short', 
    day: 'numeric',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

onMounted(fetchNote)

onBeforeUnmount(() => {
  if (saveTimeout.value) {
    clearTimeout(saveTimeout.value)
  }
  editor.value?.destroy()
})
</script>

<style scoped>
.note-detail {
  max-width: 900px;
  margin: 0 auto;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.saving-indicator {
  color: #f39c12;
  font-size: 0.9rem;
}

.saved-indicator {
  color: #27ae60;
  font-size: 0.9rem;
}

.note-content {
  background: white;
  border-radius: 8px;
  padding: 2rem;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.note-title {
  width: 100%;
  border: none;
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 0.5rem;
  padding: 0.5rem 0;
  color: #2c3e50;
}

.note-title:focus {
  outline: none;
  border-bottom: 2px solid #3498db;
}

.note-description {
  width: 100%;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  font-size: 1rem;
  padding: 0.75rem;
  margin-bottom: 1rem;
  resize: vertical;
  color: #666;
}

.note-description:focus {
  outline: none;
  border-color: #3498db;
}

/* Editor Toolbar */
.editor-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  padding: 0.75rem;
  background: #f8f9fa;
  border: 1px solid #e0e0e0;
  border-bottom: none;
  border-radius: 8px 8px 0 0;
}

.toolbar-group {
  display: flex;
  gap: 0.25rem;
  padding-right: 0.5rem;
  border-right: 1px solid #ddd;
}

.toolbar-group:last-child {
  border-right: none;
}

.editor-toolbar button {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: white;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.editor-toolbar button:hover {
  background: #e9ecef;
}

.editor-toolbar button.is-active {
  background: #3498db;
  color: white;
  border-color: #3498db;
}

.editor-toolbar button:disabled {
  opacity: 0.5;
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
}

/* Editor Content */
.editor-content {
  border: 1px solid #e0e0e0;
  border-radius: 0 0 8px 8px;
  min-height: 400px;
  padding: 1rem;
}

.editor-content :deep(.tiptap) {
  outline: none;
  min-height: 380px;
}

.editor-content :deep(.tiptap p.is-editor-empty:first-child::before) {
  color: #adb5bd;
  content: attr(data-placeholder);
  float: left;
  height: 0;
  pointer-events: none;
}

/* Editor Styles */
.editor-content :deep(h1) {
  font-size: 2em;
  margin: 1em 0 0.5em;
}

.editor-content :deep(h2) {
  font-size: 1.5em;
  margin: 1em 0 0.5em;
}

.editor-content :deep(h3) {
  font-size: 1.2em;
  margin: 1em 0 0.5em;
}

.editor-content :deep(blockquote) {
  border-left: 4px solid #3498db;
  padding-left: 1rem;
  margin: 1rem 0;
  color: #666;
  font-style: italic;
}

.editor-content :deep(pre) {
  background: #2d2d2d;
  color: #f8f8f2;
  padding: 1rem;
  border-radius: 4px;
  overflow-x: auto;
  font-family: 'Monaco', 'Menlo', monospace;
}

.editor-content :deep(code) {
  background: #f4f4f4;
  padding: 0.2em 0.4em;
  border-radius: 3px;
  font-family: 'Monaco', 'Menlo', monospace;
  font-size: 0.9em;
}

.editor-content :deep(pre code) {
  background: none;
  padding: 0;
}

.editor-content :deep(.editor-link) {
  color: #3498db;
  text-decoration: underline;
  cursor: pointer;
}

.editor-content :deep(.editor-image) {
  max-width: 100%;
  height: auto;
  border-radius: 4px;
  margin: 1rem 0;
}

.editor-content :deep(.spoiler) {
  background: #333;
  color: #333;
  padding: 0.1em 0.3em;
  border-radius: 3px;
  cursor: pointer;
  transition: all 0.3s;
}

.editor-content :deep(.spoiler:hover) {
  color: #fff;
}

/* Tables */
.editor-content :deep(.editor-table) {
  border-collapse: collapse;
  width: 100%;
  margin: 1rem 0;
}

.editor-content :deep(.editor-table td),
.editor-content :deep(.editor-table th) {
  border: 1px solid #ddd;
  padding: 0.5rem;
  min-width: 100px;
}

.editor-content :deep(.editor-table th) {
  background: #f8f9fa;
  font-weight: bold;
}

.editor-content :deep(.editor-table .selectedCell) {
  background: #e3f2fd;
}

/* Lists */
.editor-content :deep(ul),
.editor-content :deep(ol) {
  padding-left: 1.5rem;
  margin: 0.5rem 0;
}

.editor-content :deep(ul) {
  list-style-type: disc;
}

.editor-content :deep(ol) {
  list-style-type: decimal;
}

.editor-content :deep(li) {
  margin: 0.25rem 0;
}

.editor-content :deep(li p) {
  margin: 0;
}

/* Horizontal Rule */
.editor-content :deep(hr) {
  border: none;
  border-top: 2px solid #e0e0e0;
  margin: 1.5rem 0;
}

/* Attachments Section */
.attachments-section {
  margin-top: 2rem;
  padding-top: 2rem;
  border-top: 1px solid #e0e0e0;
}

.attachments-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.attachments-header h3 {
  margin: 0;
  color: #2c3e50;
}

.upload-btn {
  position: relative;
  cursor: pointer;
}

.file-input {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  opacity: 0;
  cursor: pointer;
}

.attachments-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.attachment-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem;
  background: #f8f9fa;
  border-radius: 6px;
  transition: background 0.2s;
}

.attachment-item:hover {
  background: #e9ecef;
}

.attachment-icon {
  font-size: 1.5rem;
}

.attachment-info {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.attachment-name {
  font-weight: 500;
  color: #2c3e50;
}

.attachment-size {
  font-size: 0.8rem;
  color: #666;
}

.attachment-actions {
  display: flex;
  gap: 0.25rem;
}

.btn-icon {
  width: 32px;
  height: 32px;
  padding: 0;
  background: transparent;
  border: none;
  cursor: pointer;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-icon:hover {
  background: #ddd;
}

.btn-danger-icon:hover {
  background: #fee;
}

.no-attachments {
  text-align: center;
  padding: 2rem;
  color: #666;
  background: #f8f9fa;
  border-radius: 6px;
  border: 2px dashed #ddd;
}

.note-footer {
  display: flex;
  justify-content: flex-end;
  margin-top: 1rem;
  font-size: 0.9rem;
  color: #666;
}

.loading {
  text-align: center;
  padding: 4rem;
  color: #666;
}

/* Button Styles */
.btn-sm {
  padding: 0.4rem 0.8rem;
  font-size: 0.9rem;
}
</style>
