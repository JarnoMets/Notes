<template>
  <div class="editor-toolbar">
    <!-- Text Style Group -->
    <div class="toolbar-group">
      <button 
        @click="$emit('command', 'bold')"
        :class="{ 'is-active': isActive.bold }"
        title="Bold (Ctrl+B)"
      >
        <strong>B</strong>
      </button>
      <button 
        @click="$emit('command', 'italic')"
        :class="{ 'is-active': isActive.italic }"
        title="Italic (Ctrl+I)"
      >
        <em>I</em>
      </button>
      <button 
        @click="$emit('command', 'underline')"
        :class="{ 'is-active': isActive.underline }"
        title="Underline (Ctrl+U)"
      >
        <u>U</u>
      </button>
      <button 
        @click="$emit('command', 'strike')"
        :class="{ 'is-active': isActive.strike }"
        title="Strikethrough"
      >
        <s>S</s>
      </button>
    </div>

    <!-- Headings Group -->
    <div class="toolbar-group">
      <button 
        @click="$emit('command', 'heading', { level: 1 })"
        :class="{ 'is-active': isActive.h1 }"
        title="Heading 1"
      >H1</button>
      <button 
        @click="$emit('command', 'heading', { level: 2 })"
        :class="{ 'is-active': isActive.h2 }"
        title="Heading 2"
      >H2</button>
      <button 
        @click="$emit('command', 'heading', { level: 3 })"
        :class="{ 'is-active': isActive.h3 }"
        title="Heading 3"
      >H3</button>
    </div>

    <!-- Color Group -->
    <div class="toolbar-group">
      <div class="color-picker-wrapper">
        <button title="Text Color" class="color-btn">
          <span class="color-icon">A</span>
          <input type="color" @input="$emit('setColor', $event)" class="color-input" />
        </button>
      </div>
      <div class="color-picker-wrapper">
        <button title="Highlight" class="color-btn highlight-btn">
          <span class="highlight-icon">H</span>
          <input type="color" @input="$emit('setHighlight', $event)" class="color-input" value="#ffff00" />
        </button>
      </div>
    </div>

    <!-- Alignment Group -->
    <div class="toolbar-group">
      <button 
        @click="$emit('command', 'alignLeft')"
        :class="{ 'is-active': isActive.alignLeft }"
        title="Align Left"
      >⫷</button>
      <button 
        @click="$emit('command', 'alignCenter')"
        :class="{ 'is-active': isActive.alignCenter }"
        title="Align Center"
      >≡</button>
      <button 
        @click="$emit('command', 'alignRight')"
        :class="{ 'is-active': isActive.alignRight }"
        title="Align Right"
      >⫸</button>
    </div>

    <!-- Lists Group -->
    <div class="toolbar-group">
      <button 
        @click="$emit('command', 'bulletList')"
        :class="{ 'is-active': isActive.bulletList }"
        title="Bullet List"
      >•</button>
      <button 
        @click="$emit('command', 'orderedList')"
        :class="{ 'is-active': isActive.orderedList }"
        title="Numbered List"
      >1.</button>
      <button 
        @click="$emit('command', 'taskList')"
        :class="{ 'is-active': isActive.taskList }"
        title="Checklist"
      >☑</button>
    </div>

    <!-- Block Elements -->
    <div class="toolbar-group">
      <button 
        @click="$emit('command', 'blockquote')"
        :class="{ 'is-active': isActive.blockquote }"
        title="Quote"
      >"</button>
      <button 
        @click="$emit('command', 'codeBlock')"
        :class="{ 'is-active': isActive.codeBlock }"
        title="Code Block"
      >&lt;/&gt;</button>
      <button 
        @click="$emit('command', 'horizontalRule')"
        title="Horizontal Rule"
      >─</button>
    </div>

    <!-- Insert Elements -->
    <div class="toolbar-group">
      <button @click="$emit('openUrlPrompt', 'link')" :class="{ 'is-active': isActive.link }" title="Add Link">
        <Icon name="link" :size="14" />
      </button>
      <button @click="$emit('openUrlPrompt', 'image')" title="Add Image">
        <Icon name="image" :size="14" />
      </button>
      <button @click="$emit('command', 'insertTable')" title="Insert Table">
        <Icon name="table" :size="14" />
      </button>
    </div>

    <!-- Table Controls (when in table) -->
    <div class="toolbar-group" v-if="isActive.table">
      <button @click="$emit('command', 'addColumnAfter')" title="Add Column">+|</button>
      <button @click="$emit('command', 'addRowAfter')" title="Add Row">+─</button>
      <button @click="$emit('command', 'deleteColumn')" title="Delete Column">-|</button>
      <button @click="$emit('command', 'deleteRow')" title="Delete Row">-─</button>
      <button @click="$emit('command', 'deleteTable')" title="Delete Table">
        <Icon name="trash" :size="12" />
      </button>
    </div>

    <!-- Undo/Redo -->
    <div class="toolbar-group">
      <button @click="$emit('command', 'undo')" :disabled="!canUndo" title="Undo">↩</button>
      <button @click="$emit('command', 'redo')" :disabled="!canRedo" title="Redo">↪</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import Icon from '../Icon.vue'

export interface ActiveState {
  bold: boolean
  italic: boolean
  underline: boolean
  strike: boolean
  h1: boolean
  h2: boolean
  h3: boolean
  alignLeft: boolean
  alignCenter: boolean
  alignRight: boolean
  bulletList: boolean
  orderedList: boolean
  taskList: boolean
  blockquote: boolean
  codeBlock: boolean
  link: boolean
  table: boolean
}

defineProps<{
  isActive: ActiveState
  canUndo: boolean
  canRedo: boolean
}>()

defineEmits<{
  command: [command: string, options?: Record<string, unknown>]
  setColor: [event: Event]
  setHighlight: [event: Event]
  openUrlPrompt: [type: 'link' | 'image']
}>()
</script>

<style scoped>
.editor-toolbar {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  padding: 0.5rem;
  background: var(--toolbar-bg);
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

.editor-toolbar button {
  width: 28px;
  height: 28px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 3px;
  cursor: pointer;
  font-size: 12px;
  color: var(--text-secondary);
  transition: all 0.15s;
}

.editor-toolbar button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.editor-toolbar button.is-active {
  background: var(--accent-light);
  color: var(--accent);
}

.editor-toolbar button:disabled {
  opacity: 0.3;
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
  font-size: 10px;
}

/* Responsive */
@media (max-width: 768px) {
  .editor-toolbar {
    padding: 0.375rem;
    gap: 0.375rem;
    overflow-x: auto;
    flex-wrap: nowrap;
    -webkit-overflow-scrolling: touch;
  }

  .toolbar-group {
    flex-shrink: 0;
    padding-right: 0.375rem;
  }

  .editor-toolbar button {
    width: 32px;
    height: 32px;
  }
}

@media (hover: none) and (pointer: coarse) {
  .editor-toolbar button {
    min-width: 40px;
    min-height: 40px;
  }
}
</style>
