<template>
  <div class="note-tree-view">
    <div v-if="rootItems.length === 0" class="tree-empty">No notes available</div>
    <div v-else class="tree-container">
      <tree-node
        v-for="item in rootItems"
        :key="item.id"
        :item="item"
        :folders="folders"
        :notes="notes"
        @select-note="(note) => $emit('select-note', note)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Note, NoteFolder } from '../types'
import TreeNode from './TreeNode.vue'

const props = defineProps<{
  folders: NoteFolder[]
  notes: Note[]
}>()

const emit = defineEmits<{
  (e: 'select-note', note: Note): void
}>()

interface TreeItem {
  id: string
  name: string
  type: 'folder' | 'note'
  children: TreeItem[]
  noteData?: Note
}

const rootItems = computed<TreeItem[]>(() => {
  const items: TreeItem[] = []

  // Add root folders
  const rootFolders = props.folders.filter(f => !f.parent_id)
  for (const folder of rootFolders) {
    items.push(buildFolderTree(folder))
  }

  // Add root notes
  const rootNotes = props.notes.filter(n => !n.folder_id)
  for (const note of rootNotes) {
    items.push({
      id: note.id,
      name: note.title,
      type: 'note',
      children: [],
      noteData: note
    })
  }

  return items
})

function buildFolderTree(folder: NoteFolder): TreeItem {
  const children: TreeItem[] = []

  // Add subfolders
  const subFolders = props.folders.filter(f => f.parent_id === folder.id)
  for (const subFolder of subFolders) {
    children.push(buildFolderTree(subFolder))
  }

  // Add notes in this folder
  const folderNotes = props.notes.filter(n => n.folder_id === folder.id)
  for (const note of folderNotes) {
    children.push({
      id: note.id,
      name: note.title,
      type: 'note',
      children: [],
      noteData: note
    })
  }

  return {
    id: folder.id,
    name: folder.name,
    type: 'folder',
    children
  }
}
</script>

<style scoped>
.note-tree-view {
  height: 100%;
  overflow-y: auto;
}

.tree-container {
  padding: 0.25rem 0;
}

.tree-empty {
  text-align: center;
  padding: 1rem;
  color: var(--text-muted);
  font-size: 0.875rem;
}
</style>
