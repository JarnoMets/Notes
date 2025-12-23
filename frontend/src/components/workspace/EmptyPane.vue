<template>
  <div 
    class="empty-pane" 
    :class="{ 'drop-active': isDropTarget }"
    @dragenter="onDragEnter"
    @dragover="onDragOver"
    @dragleave="onDragLeave"
    @drop="onDrop"
  >
    <template v-if="isDropTarget">
      <Icon name="file" :size="32" />
      <p>Drop note or board here to open</p>
    </template>
    <template v-else>
      <p>No file is open</p>
      <p class="empty-hint">Select a note from the explorer or create a new one</p>
      <p class="empty-hint">You can also drag notes or boards here to open them</p>
      <button class="btn btn-primary" @click="$emit('create')">Create New Note</button>
    </template>
  </div>
</template>

<script setup lang="ts">
import Icon from '../common/ui/Icon.vue'

defineProps<{
  isDropTarget?: boolean
}>()

const emit = defineEmits<{
  create: []
  'dragenter': [event: DragEvent]
  'dragover': [event: DragEvent]
  'dragleave': [event: DragEvent]
  'drop': [event: DragEvent]
}>()

function onDragEnter(event: DragEvent) {
  // Check if this is an explorer drag before emitting
  const types = event.dataTransfer?.types || []
  const isExplorer = Array.from(types).includes('application/x-explorer-item')
  if (isExplorer) {
    event.preventDefault()
    emit('dragenter', event)
  }
}

function onDragOver(event: DragEvent) {
  // Ensure we only accept explorer items
  const types = event.dataTransfer?.types || []
  const isExplorer = Array.from(types).includes('application/x-explorer-item')
  if (isExplorer) {
    event.preventDefault()
    event.dataTransfer!.dropEffect = 'copy'
    emit('dragover', event)
  }
}

function onDragLeave(event: DragEvent) {
  event.preventDefault()
  emit('dragleave', event)
}

function onDrop(event: DragEvent) {
  event.preventDefault()
  // Try to get the explorer item data and emit regardless
  const dragData = event.dataTransfer?.getData('application/x-explorer-item')
  if (dragData) {
    emit('drop', event)
  } else {
    // Try fallback for text/plain
    const fallbackData = event.dataTransfer?.getData('text/plain')
    if (fallbackData) {
      emit('drop', event)
    }
  }
}
</script>

<style scoped>
.empty-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  text-align: center;
  color: var(--text-muted);
  padding: 2rem;
  border-radius: 12px;
  transition: all 0.2s;
}

.empty-pane.drop-active {
  background: color-mix(in srgb, var(--accent) 15%, transparent);
  border: 1px dashed var(--accent); /* Subtle dashed border */
  color: var(--accent);
}

.empty-pane.drop-active p {
  color: var(--accent);
  margin-top: 0.5rem;
}

.empty-pane p {
  margin: 0.5rem 0;
}

.empty-hint {
  font-size: 12px;
  color: var(--text-muted);
  opacity: 0.7;
}

.empty-pane .btn {
  margin-top: 1rem;
}

@media (max-width: 768px) {
  .empty-pane {
    padding: 1.5rem;
  }

  .empty-pane p {
    font-size: 13px;
  }

  .empty-hint {
    font-size: 11px;
  }

  .empty-pane .btn {
    width: 100%;
  }
}
</style>
