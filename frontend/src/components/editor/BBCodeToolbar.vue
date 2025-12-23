<template>
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
      <button @click="$emit('open-link-dialog')" title="Add URL Link" class="toolbar-btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/></svg>
      </button>
      <button @click="$emit('open-image-dialog')" title="Add Image" class="toolbar-btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2" ry="2"/><circle cx="8.5" cy="8.5" r="1.5"/><polyline points="21 15 16 10 5 21"/></svg>
      </button>
      <button v-if="isEditing" @click="$emit('open-image-upload-dialog')" title="Upload Image" class="toolbar-btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
      </button>
      <button @click="$emit('open-table-dialog')" title="Insert Table" class="toolbar-btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 3h18v18H3z"/><path d="M3 9h18M3 15h18M9 3v18M15 3v18"/></svg>
      </button>
    </div>

    <!-- Note/Board Links -->
    <div class="toolbar-group">
      <button @click="$emit('open-note-link-dialog')" title="Link to Note" class="toolbar-btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="12" y1="11" x2="12" y2="17"/><line x1="9" y1="14" x2="15" y2="14"/></svg>
      </button>
      <button @click="$emit('open-board-link-dialog')" title="Link to Board" class="toolbar-btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="2" y="3" width="20" height="14" rx="2"/><path d="M2 17h20M6 6h.01M10 6h.01M14 6h.01M18 6h.01"/></svg>
      </button>
      <button v-if="isEditing" @click="$emit('open-attachment-link-dialog')" title="Link to Attachment" class="toolbar-btn">
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
      <button @click="$emit('undo')" :disabled="!canUndo" title="Undo" class="toolbar-btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 7v6h6"/><path d="M21 17a9 9 0 00-9-9 9 9 0 00-6 2.3L3 13"/></svg>
      </button>
      <button @click="$emit('redo')" :disabled="!canRedo" title="Redo" class="toolbar-btn">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 7v6h-6"/><path d="M3 17a9 9 0 019-9 9 9 0 016 2.3l3 2.7"/></svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Icon from '../common/ui/Icon.vue'
import { isTagActive as isTagActiveUtil } from '@/utils/bbcodeFormatter'

const props = defineProps<{
  isEditing: boolean
  canUndo: boolean
  canRedo: boolean
  textarea: HTMLTextAreaElement | null
}>()

const emit = defineEmits<{
  'apply-format': [tag: string]
  'apply-color-format': [color: string]
  'apply-highlight-format': [color: string]
  'insert-horizontal-rule': []
  'insert-line-break': []
  'insert-todo': []
  'open-link-dialog': []
  'open-image-dialog': []
  'open-image-upload-dialog': []
  'open-table-dialog': []
  'open-note-link-dialog': []
  'open-board-link-dialog': []
  'open-attachment-link-dialog': []
  'undo': []
  'redo': []
}>()

const colorInput = ref<HTMLInputElement | null>(null)
const highlightInput = ref<HTMLInputElement | null>(null)

function applyFormat(tag: string) {
  emit('apply-format', tag)
}

function isTagActive(tag: string): boolean {
  if (!props.textarea) return false
  return isTagActiveUtil(props.textarea, tag)
}

function applyColorFormat() {
  if (!colorInput.value) return
  emit('apply-color-format', colorInput.value.value)
}

function applyHighlightFormat() {
  if (!highlightInput.value) return
  emit('apply-highlight-format', highlightInput.value.value)
}

function insertHorizontalRule() {
  emit('insert-horizontal-rule')
}

function insertLineBreak() {
  emit('insert-line-break')
}

function insertTodo() {
  emit('insert-todo')
}
</script>

<style scoped>
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
</style>