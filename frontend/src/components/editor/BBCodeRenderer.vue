<template>
  <span class="bbcode-renderer">
    <template v-for="(segment, index) in segments" :key="index">
      <!-- Regular text -->
      <span v-if="segment.type === 'text'" class="bbcode-text">
        {{ segment.content }}
      </span>

      <!-- Note reference -->
      <a
        v-else-if="segment.type === 'note-link' && segment.id"
        :href="`javascript:void(0)`"
        class="bbcode-link bbcode-note-link"
        :data-note-id="segment.id"
        :title="`Open note: ${noteMap.get(segment.id) || segment.id}`"
        @click.prevent="openNote(segment.id)"
      >
        [{{ noteMap.get(segment.id) || segment.id }}]
      </a>

      <!-- Board reference -->
      <a
        v-else-if="segment.type === 'board-link' && segment.id"
        :href="`javascript:void(0)`"
        class="bbcode-link bbcode-board-link"
        :data-board-id="segment.id"
        :title="`Open board: ${boardMap.get(segment.id) || segment.id}`"
        @click.prevent="openBoard(segment.id)"
      >
        [{{ boardMap.get(segment.id) || segment.id }}]
      </a>

      <!-- Attachment reference -->
      <a
        v-else-if="segment.type === 'attachment-link' && segment.id"
        :href="`javascript:void(0)`"
        class="bbcode-link bbcode-attachment-link"
        :data-attachment-id="segment.id"
        :title="`Open attachment: ${segment.id}`"
        @click.prevent="openAttachment(segment.id)"
      >
        [{{ segment.id }}]
      </a>
    </template>
  </span>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useNotesStore } from '@/stores/notes'
import { useExplorerStore } from '@/stores/explorer'
import { notesApi } from '@/api'
import { parseBBCode } from '@/utils/bbcode'

interface Props {
  content: string
}

const props = defineProps<Props>()

const router = useRouter()
const notesStore = useNotesStore()
const explorerStore = useExplorerStore()

const segments = computed(() => parseBBCode(props.content))

const noteMap = computed(() => {
  const map = new Map<string, string>()
  explorerStore.notes.forEach(note => {
    map.set(note.id, note.title)
  })
  return map
})

const boardMap = computed(() => {
  const map = new Map<string, string>()
  explorerStore.boards.forEach(board => {
    map.set(board.id, board.name)
  })
  return map
})

onMounted(async () => {
  // Ensure data is loaded
  if (explorerStore.notes.length === 0 || explorerStore.boards.length === 0) {
    await explorerStore.fetchAll()
  }
})

function openNote(noteId: string) {
  notesStore.openNote(noteId, 'primary')
}

function openBoard(boardId: string) {
  localStorage.setItem('selectedBoardId', boardId)
  router.push('/boards')
}

function openAttachment(attachmentId: string) {
  const url = notesApi.getAttachmentUrl(attachmentId)
  window.open(url, '_blank')
}
</script>

<style scoped>
.bbcode-renderer {
  display: contents;
}

.bbcode-text {
  /* Regular text continues naturally */
}

.bbcode-link {
  display: inline-block;
  padding: 0.1em 0.4em;
  border-radius: 4px;
  text-decoration: none;
  cursor: pointer;
  font-weight: 500;
  transition: all 0.2s ease;
  user-select: none;
}

.bbcode-note-link {
  background: var(--success-light, rgba(39, 174, 96, 0.15));
  color: var(--success, #27ae60);
  border-left: 2px solid var(--success, #27ae60);
  padding-left: 0.5em;
}

.bbcode-note-link:hover {
  background: var(--success-light, rgba(39, 174, 96, 0.3));
  transform: translateX(2px);
}

.bbcode-board-link {
  background: var(--accent-light, rgba(25, 118, 210, 0.15));
  color: var(--accent, #1976d2);
  border-left: 2px solid var(--accent, #1976d2);
  padding-left: 0.5em;
}

.bbcode-board-link:hover {
  background: var(--accent-light, rgba(25, 118, 210, 0.3));
  transform: translateX(2px);
}

.bbcode-attachment-link {
  background: var(--info-light, rgba(13, 110, 253, 0.15));
  color: var(--info, #0d6efd);
  border-left: 2px solid var(--info, #0d6efd);
  padding-left: 0.5em;
}

.bbcode-attachment-link:hover {
  background: var(--info-light, rgba(13, 110, 253, 0.3));
  transform: translateX(2px);
}
</style>
