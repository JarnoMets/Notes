<template>
  <div class="kanban-board">
    <div class="lists-wrapper">
      <div 
        v-for="listWithCards in filteredLists" 
        :key="listWithCards.list.id" 
        class="kanban-list"
        @dragover.prevent="$emit('listDragOver', $event, listWithCards.list.id)"
        @drop="$emit('cardDrop', $event, listWithCards.list.id)"
      >
        <KanbanListHeader
          :list="listWithCards.list"
          :card-count="getFilteredCards(listWithCards.cards).length"
          @archive="$emit('archiveList', listWithCards.list.id)"
          @delete="$emit('deleteList', listWithCards.list.id)"
        />
        
        <div class="cards-container">
          <KanbanCard
            v-for="(card, index) in getFilteredCards(listWithCards.cards)"
            :key="card.id"
            :card="card"
            :labels="labels"
            :is-drag-over="dragOverCard === card.id"
            :is-note-drop-target="noteDropTargetCardId === card.id"
            @click="$emit('openCard', card)"
            @dragstart="$emit('cardDragStart', $event, card, listWithCards.list.id)"
            @dragend="$emit('cardDragEnd')"
            @dragover="$emit('cardDragOver', $event, card.id, index, listWithCards.list.id)"
            @dragleave="$emit('cardDragLeave')"
            @drop="$emit('cardDropOnCard', $event, card.id, index, listWithCards.list.id)"
          />
          
          <div 
            class="card-drop-zone"
            :class="{ 'active': dropZoneListId === listWithCards.list.id }"
            @dragover.prevent
            @drop="$emit('cardDropAtEnd', $event, listWithCards.list.id)"
          ></div>
        </div>

        <button class="add-card-btn" @click="$emit('addCard', listWithCards.list.id)">
          <Icon name="plus" :size="14" /> Add Card
        </button>
      </div>

      <div class="kanban-list add-list-placeholder" @click="$emit('addList')">
        <Icon name="plus" :size="16" />
        <span>Add List</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Card, BoardLabel, ListWithCards } from '../../types'
import KanbanListHeader from './KanbanListHeader.vue'
import KanbanCard from './KanbanCard.vue'
import Icon from '../Icon.vue'

interface Props {
  lists: ListWithCards[]
  labels: BoardLabel[]
  dragOverCard?: string
  noteDropTargetCardId?: string
  dropZoneListId?: string
  selectedLabelFilters?: string[]
  dueDateFilter?: string
}

const props = withDefaults(defineProps<Props>(), {
  dragOverCard: '',
  noteDropTargetCardId: '',
  dropZoneListId: '',
  selectedLabelFilters: () => [],
  dueDateFilter: ''
})

defineEmits<{
  listDragOver: [event: DragEvent, listId: string]
  cardDrop: [event: DragEvent, listId: string]
  cardDragStart: [event: DragEvent, card: Card, listId: string]
  cardDragEnd: []
  cardDragOver: [event: DragEvent, cardId: string, index: number, listId: string]
  cardDragLeave: []
  cardDropOnCard: [event: DragEvent, cardId: string, index: number, listId: string]
  cardDropAtEnd: [event: DragEvent, listId: string]
  openCard: [card: Card]
  addCard: [listId: string]
  addList: []
  archiveList: [listId: string]
  deleteList: [listId: string]
}>()

// Filter lists (exclude archived)
const filteredLists = computed(() => {
  return props.lists.filter(lwc => !lwc.list.archived)
})

// Filter cards based on filters
function getFilteredCards(cards: Card[]): Card[] {
  return cards.filter(card => {
    if (card.archived) return false
    
    // Label filter
    if (props.selectedLabelFilters.length > 0) {
      const hasLabel = props.selectedLabelFilters.some(labelId => card.labels.includes(labelId))
      if (!hasLabel) return false
    }
    
    // Due date filter
    if (props.dueDateFilter) {
      const now = new Date()
      const today = new Date(now.getFullYear(), now.getMonth(), now.getDate())
      const weekEnd = new Date(today.getTime() + 7 * 24 * 60 * 60 * 1000)
      
      switch (props.dueDateFilter) {
        case 'overdue':
          if (!card.due_date || new Date(card.due_date) >= now) return false
          break
        case 'today':
          if (!card.due_date) return false
          const dueDate = new Date(card.due_date)
          if (dueDate < today || dueDate >= new Date(today.getTime() + 24 * 60 * 60 * 1000)) return false
          break
        case 'week':
          if (!card.due_date) return false
          const dueDateWeek = new Date(card.due_date)
          if (dueDateWeek < today || dueDateWeek > weekEnd) return false
          break
        case 'none':
          if (card.due_date) return false
          break
      }
    }
    
    return true
  })
}

import { computed } from 'vue'
</script>

<style scoped>
.kanban-board {
  flex: 1;
  overflow-x: auto;
  padding: 1rem;
}

.lists-wrapper {
  display: flex;
  gap: 1rem;
  height: 100%;
  align-items: flex-start;
}

.kanban-list {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 0.75rem;
  min-width: 280px;
  max-width: 280px;
  display: flex;
  flex-direction: column;
  max-height: calc(100vh - 180px);
}

.cards-container {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.card-drop-zone {
  height: 4px;
  margin: 0.25rem 0;
  border-radius: 2px;
  transition: all 0.2s;
}

.card-drop-zone.active {
  height: 40px;
  background: rgba(52, 152, 219, 0.2);
  border: 2px dashed var(--accent);
}

.add-card-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.375rem;
  background: transparent;
  border: 1px dashed var(--border-primary);
  border-radius: 6px;
  padding: 0.625rem;
  margin-top: 0.5rem;
  color: var(--text-muted);
  font-size: 13px;
  cursor: pointer;
  transition: all 0.15s;
}

.add-card-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
  color: var(--text-primary);
}

.add-list-placeholder {
  background: var(--bg-tertiary);
  border: 2px dashed var(--border-primary);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 2rem 1rem;
  cursor: pointer;
  color: var(--text-muted);
  transition: all 0.15s;
}

.add-list-placeholder:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
  color: var(--text-primary);
}

/* Responsive */
@media (max-width: 768px) {
  .kanban-board {
    padding: 0.75rem;
  }

  .lists-wrapper {
    gap: 0.75rem;
    padding-bottom: 0.5rem;
  }

  .kanban-list {
    min-width: 85vw;
    max-width: 85vw;
    max-height: calc(100vh - 220px);
    max-height: calc(100dvh - 220px);
  }
}

@media (max-width: 480px) {
  .kanban-list {
    min-width: 90vw;
    max-width: 90vw;
  }
}
</style>
