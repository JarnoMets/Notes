<template>
  <div class="editor-header">
    <div class="header-left">
      <h2 class="note-title" v-if="!isEditingTitle" @dblclick="startEditingTitle">
        {{ note.title }}
      </h2>
      <input
        v-else
        ref="titleInput"
        v-model="editedTitle"
        class="title-input"
        @blur="saveTitle"
        @keydown.enter="saveTitle"
        @keydown.escape="cancelEditTitle"
      />
    </div>
    <div class="header-actions">
      <button
        v-if="!isEditing"
        class="btn-icon"
        @click="startEditing"
        title="Edit note"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
          <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
        </svg>
      </button>
      <button
        v-else
        class="btn-icon btn-done"
        @click="stopEditing"
        title="Done editing"
      >
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="20 6 9 17 4 12"/>
        </svg>
      </button>
      <!-- Undo/Redo controlled via editor toolbar only -->
      <div class="menu-wrapper" ref="menuWrapper">
        <button class="btn-icon" @click="toggleMenu" title="More options">
          <svg width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
            <circle cx="12" cy="5" r="2"/>
            <circle cx="12" cy="12" r="2"/>
            <circle cx="12" cy="19" r="2"/>
          </svg>
        </button>
        <div v-if="showMenu" class="dropdown-menu">
          <button @click="showAttachments = !showAttachments">
            <Icon name="paperclip" :size="14" /> Attachments ({{ note.attachments?.length || 0 }})
          </button>
          <div class="menu-divider"></div>
          <button @click="duplicateNote">
            <Icon name="copy" :size="14" /> Duplicate
          </button>
          <button @click="exportNote">
            <Icon name="download" :size="14" /> Export
          </button>
          <div class="menu-divider"></div>
          <button class="danger" @click="confirmDeleteNote">
            <Icon name="trash" :size="14" /> Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick } from 'vue'
import type { NoteWithAttachments } from '@/types'
import Icon from '../common/ui/Icon.vue'

const props = defineProps<{
  note: NoteWithAttachments
  isEditing: boolean
  isEditingTitle: boolean
  showAttachments: boolean
}>()

const emit = defineEmits<{
  'update:isEditing': [value: boolean]
  'update:isEditingTitle': [value: boolean]
  'update:showAttachments': [value: boolean]
  'update:title': [newTitle: string]
  'duplicate-note': []
  'export-note': []
  'confirm-delete-note': []
}>()

const editedTitle = ref('')
const showMenu = ref(false)
const menuWrapper = ref<HTMLElement | null>(null)
const titleInput = ref<HTMLInputElement | null>(null)

function startEditingTitle() {
  if (!props.isEditing) return
  editedTitle.value = props.note.title || ''
  emit('update:isEditingTitle', true)
  nextTick(() => {
    titleInput.value?.focus()
    titleInput.value?.select()
  })
}

function saveTitle() {
  if (!props.isEditingTitle) return
  const newTitle = editedTitle.value.trim()
  if (newTitle && newTitle !== props.note.title) {
    emit('update:title', newTitle)
  }
  emit('update:isEditingTitle', false)
}

function cancelEditTitle() {
  emit('update:isEditingTitle', false)
  editedTitle.value = ''
}

function toggleMenu() {
  showMenu.value = !showMenu.value
}

function closeMenuOnClickOutside(event: MouseEvent) {
  if (menuWrapper.value && !menuWrapper.value.contains(event.target as Node)) {
    showMenu.value = false
  }
}

function startEditing() {
  emit('update:isEditing', true)
}

function stopEditing() {
  emit('update:isEditing', false)
}

function duplicateNote() {
  showMenu.value = false
  emit('duplicate-note')
}

function exportNote() {
  showMenu.value = false
  emit('export-note')
}

function confirmDeleteNote() {
  showMenu.value = false
  emit('confirm-delete-note')
}

// Watch for menu close
document.addEventListener('click', closeMenuOnClickOutside)
</script>

<style scoped>
.editor-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
}

.header-left {
  flex: 1;
  min-width: 0;
}

.note-title {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  cursor: default;
}

.title-input {
  width: 100%;
  font-size: 1.1rem;
  font-weight: 600;
  background: var(--bg-tertiary);
  border: 1px solid var(--accent);
  color: var(--text-primary);
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  outline: none;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.btn-done {
  color: var(--success);
}

.btn-done:hover {
  background: var(--success);
  color: white;
}

/* Menu */
.menu-wrapper {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.25rem;
  min-width: 180px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
  overflow: hidden;
}

.dropdown-menu button {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  padding: 0.625rem 1rem;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 0.875rem;
  cursor: pointer;
  text-align: left;
}

.dropdown-menu button:hover {
  background: var(--bg-hover);
}

.dropdown-menu button.danger {
  color: var(--danger);
}

.dropdown-menu button.danger:hover {
  background: rgba(248, 81, 73, 0.1);
}

.menu-divider {
  height: 1px;
  background: var(--border-primary);
  margin: 0.25rem 0;
}
</style>