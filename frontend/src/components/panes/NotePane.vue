<template>
  <div class="note-pane">
    <NoteEditor
      v-if="noteId"
      :note-id="noteId"
      :pane-id="paneId"
      @dirty="onDirty"
      @update:title="onTitleUpdate"
    />
    <div v-else class="empty-state">
      <p>No note selected</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import NoteEditor from '../editor/NoteEditor.vue'

defineProps<{
  noteId?: string
  paneId: string
}>()

const emit = defineEmits<{
  'dirty': [isDirty: boolean]
  'update-title': [title: string]
}>()

function onDirty(isDirty: boolean) {
  emit('dirty', isDirty)
}

function onTitleUpdate(title: string) {
  emit('update-title', title)
}
</script>

<style scoped>
.note-pane {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.empty-state {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
}
</style>
