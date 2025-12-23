<template>
  <div class="editor-header">
    <div class="header-left">
      <h2 class="note-title" v-if="!isEditingTitle" @dblclick="$emit('startEditTitle')">
        {{ title }}
      </h2>
      <input
        v-else
        ref="titleInput"
        :value="title"
        class="title-input"
        @blur="$emit('saveTitle', ($event.target as HTMLInputElement).value)"
        @keydown.enter="$emit('saveTitle', ($event.target as HTMLInputElement).value)"
        @keydown.escape="$emit('cancelEditTitle')"
      />
    </div>
    <div class="header-actions">
      <button 
        v-if="!isEditing" 
        class="btn-icon" 
        @click="$emit('startEdit')"
        title="Edit note"
      >
        <Icon name="edit" :size="16" />
      </button>
      <button 
        v-else 
        class="btn-icon btn-done" 
        @click="$emit('stopEdit')"
        title="Done editing"
      >
        <Icon name="check" :size="16" />
      </button>
      <div class="menu-wrapper" ref="menuWrapper">
        <button class="btn-icon" @click="$emit('toggleMenu')" title="More options">
          <Icon name="more-vertical" :size="16" :fill="true" />
        </button>
        <div v-if="showMenu" class="dropdown-menu">
          <button @click="$emit('menuAction', 'toggleAttachments')">
            <Icon name="paperclip" :size="14" /> Attachments ({{ attachmentCount }})
          </button>
          <button @click="$emit('menuAction', 'linkNote')">
            <Icon name="note" :size="14" /> Link to Note
          </button>
          <button @click="$emit('menuAction', 'linkBoard')">
            <Icon name="board" :size="14" /> Link to Board
          </button>
          <div class="menu-divider"></div>
          <button @click="$emit('menuAction', 'duplicate')">
            <Icon name="copy" :size="14" /> Duplicate
          </button>
          <button @click="$emit('menuAction', 'export')">
            <Icon name="download" :size="14" /> Export
          </button>
          <div class="menu-divider"></div>
          <button class="danger" @click="$emit('menuAction', 'delete')">
            <Icon name="trash" :size="14" /> Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Icon from '../common/ui/Icon.vue'

defineProps<{
  title: string
  isEditing: boolean
  isEditingTitle: boolean
  showMenu: boolean
  attachmentCount: number
}>()

defineEmits<{
  startEdit: []
  stopEdit: []
  startEditTitle: []
  saveTitle: [title: string]
  cancelEditTitle: []
  toggleMenu: []
  menuAction: [action: string]
}>()
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

/* Responsive */
@media (max-width: 768px) {
  .editor-header {
    padding: 0.5rem 0.75rem;
  }

  .note-title {
    font-size: 1rem;
  }

  .title-input {
    font-size: 1rem;
  }

  .dropdown-menu {
    position: fixed;
    left: 1rem !important;
    right: 1rem !important;
    top: auto !important;
    bottom: 1rem;
    border-radius: 12px;
  }

  .dropdown-menu button {
    padding: 0.875rem 1rem;
  }
}

@media (hover: none) and (pointer: coarse) {
  .btn-icon {
    min-width: 44px;
    min-height: 44px;
  }
}
</style>
