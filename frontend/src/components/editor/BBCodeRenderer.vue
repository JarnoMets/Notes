<template>
  <span class="bbcode-renderer" v-html="renderedHtml"></span>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useNotesStore } from '@/stores/notes'
import { useExplorerStore } from '@/stores/explorer'
import { bbcodeToHtml } from '@/utils/bbcodeFormatter'

interface Props {
  content: string
  paneId?: string
}

const props = defineProps<Props>()

const router = useRouter()
const notesStore = useNotesStore()
const explorerStore = useExplorerStore()

const renderedHtml = computed(() => bbcodeToHtml(props.content, { paneId: props.paneId }))

onMounted(async () => {
  // Ensure data is loaded
  if (explorerStore.notes.length === 0 || explorerStore.boards.length === 0) {
    await explorerStore.fetchAll()
  }

  // Setup global functions for link clicking if not already present
  if (!(window as any).__openNote) {
    ;(window as any).__openNote = (noteId: string) => notesStore.openNote(noteId, props.paneId || 'primary')
  }
  if (!(window as any).__openBoard) {
    ;(window as any).__openBoard = (boardId: string) => {
      localStorage.setItem('selectedBoardId', boardId)
      router.push('/boards')
    }
  }
})
</script>

<style scoped>
.bbcode-renderer {
  display: inline-block;
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: break-word;
  width: 100%;
}

.bbcode-renderer :deep(.bbcode-link) {
  display: inline-block;
  padding: 0.1em 0.4em;
  border-radius: 4px;
  text-decoration: none;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s ease;
  user-select: none;
}

.bbcode-renderer :deep(.bbcode-note-link) {
  background: var(--success-light, rgba(39, 174, 96, 0.15));
  color: var(--success, #27ae60);
  border-left: 2px solid var(--success, #27ae60);
  padding-left: 0.5em;
}

.bbcode-renderer :deep(.bbcode-note-link:hover) {
  background: var(--success-light, rgba(39, 174, 96, 0.3));
  transform: translateX(2px);
}

.bbcode-renderer :deep(.bbcode-board-link) {
  background: var(--accent-light, rgba(25, 118, 210, 0.15));
  color: var(--accent, #1976d2);
  border-left: 2px solid var(--accent, #1976d2);
  padding-left: 0.5em;
}

.bbcode-renderer :deep(.bbcode-board-link:hover) {
  background: var(--accent-light, rgba(25, 118, 210, 0.3));
  transform: translateX(2px);
}

.bbcode-renderer :deep(.bbcode-attachment-link) {
  background: var(--info-light, rgba(13, 110, 253, 0.15));
  color: var(--info, #0d6efd);
  border-left: 2px solid var(--info, #0d6efd);
  padding-left: 0.5em;
}

.bbcode-renderer :deep(.bbcode-attachment-link:hover) {
  background: var(--info-light, rgba(13, 110, 253, 0.3));
  transform: translateX(2px);
}

.bbcode-renderer :deep(.bbcode-list) {
  margin: 0.5rem 0;
  padding-left: 1.5rem;
}

.bbcode-renderer :deep(.bbcode-table) {
  border-collapse: collapse;
  margin: 0.5rem 0;
}

.bbcode-renderer :deep(.bbcode-table td),
.bbcode-renderer :deep(.bbcode-table th) {
  border: 1px solid var(--border-primary);
  padding: 0.4rem;
}
</style>
