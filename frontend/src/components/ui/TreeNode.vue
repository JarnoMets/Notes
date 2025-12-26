<template>
  <div class="tree-node">
    <div
      class="tree-node-content"
      :class="{ 'is-note': item.type === 'note' }"
      @click="handleClick"
      @dblclick="handleDoubleClick"
      draggable="true"
      @dragstart="handleDragStart"
    >
      <span v-if="item.type === 'folder'" class="tree-icon expand-icon" :class="{ expanded: isExpanded }">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <polyline points="9 18 15 12 9 6"></polyline>
        </svg>
      </span>
      <span v-else class="tree-icon note-icon">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"></path>
          <polyline points="14 2 14 8 20 8"></polyline>
        </svg>
      </span>
      <span class="tree-label">{{ item.name }}</span>
    </div>

    <div v-if="item.type === 'folder' && isExpanded && item.children.length > 0" class="tree-children">
      <tree-node
        v-for="child in item.children"
        :key="child.id"
        :item="child"
        :folders="folders"
        :notes="notes"
        @select-note="(note) => $emit('select-note', note)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { Note, NoteFolder } from '@/types'

interface TreeItem {
  id: string
  name: string
  type: 'folder' | 'note'
  children: TreeItem[]
  noteData?: Note
}

const props = defineProps<{
  item: TreeItem
  folders: NoteFolder[]
  notes: Note[]
}>()

const emit = defineEmits<{
  (e: 'select-note', note: Note): void
}>()

const isExpanded = ref(false)

function handleClick() {
  if (props.item.type === 'folder') {
    isExpanded.value = !isExpanded.value
  }
}

function handleDoubleClick() {
  if (props.item.type === 'note' && props.item.noteData) {
    emit('select-note', props.item.noteData)
  }
}

function handleDragStart(e: DragEvent) {
  if (props.item.type === 'note' && props.item.noteData) {
    e.dataTransfer!.effectAllowed = 'copy'
    e.dataTransfer!.setData('application/json', JSON.stringify({
      type: 'note',
      id: props.item.noteData.id,
      title: props.item.noteData.title
    }))
  }
}
</script>

<style scoped>
.tree-node {
  user-select: none;
}

.tree-node-content {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.15s;
  margin: 0.125rem 0;
}

.tree-node-content:hover {
  background: var(--bg-hover);
}

.tree-node-content.is-note {
  padding-left: 2rem;
}

.tree-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  color: var(--text-secondary);
}

.expand-icon {
  cursor: pointer;
  transition: transform 0.15s;
}

.expand-icon.expanded {
  transform: rotate(90deg);
}

.note-icon {
  color: var(--accent);
}

.tree-label {
  color: var(--text-primary);
  font-size: 0.875rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  flex: 1;
}

.tree-children {
  margin-left: 0.5rem;
  border-left: 1px solid var(--border-primary);
  padding-left: 0;
  margin-top: 0.125rem;
  margin-bottom: 0.125rem;
}
</style>
