<template>
  <div 
    class="kanban-card"
    :class="{ 
      'drag-over': isDragOver,
      'note-drop-target': isNoteDropTarget
    }"
    draggable="true"
    @dragstart="$emit('dragstart', $event)"
    @dragend="$emit('dragend')"
    @dragover.prevent="$emit('dragover', $event)"
    @dragleave="$emit('dragleave')"
    @drop.stop="$emit('drop', $event)"
    @click="$emit('click')"
  >
      <!-- Done badge -->
      <span v-if="card.status === 'done'" class="card-done-badge">Done</span>
    <!-- Note link indicator -->
    <div v-if="hasNoteLinks" class="card-links">
      <span class="link-indicator" title="Has linked notes">
        <Icon name="link" :size="12" />
      </span>
    </div>
    
    <div class="card-labels" v-if="card.labels.length > 0">
      <span 
        v-for="labelId in card.labels" 
        :key="labelId" 
        class="label"
        :style="{ backgroundColor: getLabelColor(labelId) }"
      >{{ getLabelName(labelId) }}</span>
    </div>
    
    <h4>{{ card.title }}</h4>
    
    <p v-if="card.description" class="card-description">{{ preview }}</p>
    
    <div class="card-footer" v-if="card.due_date">
      <span class="due-date" :class="{ overdue: isOverdue }">
        <Icon name="calendar" :size="12" /> {{ formattedDate }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Card, BoardLabel } from '../../types'
import Icon from '../ui/Icon.vue'

const props = defineProps<{
  card: Card
  labels: BoardLabel[]
  isDragOver?: boolean
  isNoteDropTarget?: boolean
}>()

defineEmits<{
  click: []
  dragstart: [event: DragEvent]
  dragend: []
  dragover: [event: DragEvent]
  dragleave: []
  drop: [event: DragEvent]
}>()

const hasNoteLinks = computed(() => {
  if (!props.card.description) return false
  return props.card.description.includes('notes://') || props.card.description.includes('boards://')
})

const preview = computed(() => {
  if (!props.card.description) return ''
  // Strip out link markdown for preview
  const cleanText = props.card.description.replace(/📎 \[.*?\]\(.*?\)/g, '').trim()
  return cleanText.length > 80 ? cleanText.substring(0, 80) + '...' : cleanText
})

const formattedDate = computed(() => {
  if (!props.card.due_date) return ''
  const date = new Date(props.card.due_date)
  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
})

const isOverdue = computed(() => {
  if (!props.card.due_date) return false
  // If card status is 'done' treat as not overdue
  if (props.card.status === 'done') return false
  return new Date(props.card.due_date) < new Date()
})

function getLabelColor(labelId: string): string {
  const label = props.labels.find(l => l.id === labelId)
  return label?.color || '#888'
}

function getLabelName(labelId: string): string {
  const label = props.labels.find(l => l.id === labelId)
  return label?.name || ''
}
</script>

<style scoped>
.kanban-card {
  position: relative;
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  padding: 0.75rem;
  cursor: pointer;
  transition: all 0.15s;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
}

.kanban-card:hover {
  border-color: var(--accent);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

/* Small done badge */
.card-done-badge {
  position: absolute;
  top: 0.35rem;
  right: 0.375rem; /* KanbanCard doesn't render edit button by default, keep small offset */
  background: var(--success);
  color: #fff;
  font-size: 11px;
  padding: 0.08rem 0.45rem;
  border-radius: 999px;
  box-shadow: 0 1px 2px rgba(0,0,0,0.12);
  z-index: 8;
  line-height: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.kanban-card h4 {
  margin: 0;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
}

.kanban-card.drag-over {
  border-top: 2px solid var(--accent);
}

.kanban-card.note-drop-target {
  background: color-mix(in srgb, var(--success) 15%, var(--bg-tertiary));
  border-color: var(--success);
  box-shadow: 0 0 0 2px var(--success);
}

.kanban-card[draggable="true"] {
  cursor: grab;
}

.kanban-card[draggable="true"]:active {
  cursor: grabbing;
}

.card-links {
  position: absolute;
  top: 0.375rem;
  right: 0.375rem;
}

.link-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  background: var(--bg-tertiary);
  border-radius: 4px;
  color: var(--accent);
}

.card-labels {
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem;
  margin-bottom: 0.5rem;
}

.label {
  padding: 0.125rem 0.5rem;
  border-radius: 3px;
  font-size: 10px;
  color: white;
  font-weight: 500;
}

.card-description {
  font-size: 12px;
  color: var(--text-muted);
  margin-top: 0.375rem;
  line-height: 1.4;
}

.card-footer {
  margin-top: 0.5rem;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.due-date {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.125rem 0.375rem;
  background: var(--bg-hover);
  border-radius: 3px;
  font-size: 11px;
  color: var(--text-secondary);
}

.due-date.overdue {
  background: rgba(248, 81, 73, 0.15);
  color: var(--danger);
}

@media (max-width: 768px) {
  .kanban-card {
    padding: 0.625rem;
  }

  .kanban-card h4 {
    font-size: 0.8rem;
  }

  .card-description {
    font-size: 11px;
  }

  .card-labels .label {
    font-size: 9px;
    padding: 0.1rem 0.375rem;
  }
}

@media (hover: none) and (pointer: coarse) {
  .kanban-card {
    padding: 0.75rem;
  }

  .kanban-card:hover {
    border-color: var(--border-primary);
    box-shadow: none;
  }

  .kanban-card:active {
    border-color: var(--accent);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }
}
</style>
