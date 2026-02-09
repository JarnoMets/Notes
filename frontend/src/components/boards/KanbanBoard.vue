<template>
  <div class="kanban-container">
    <div class="lists-row">
      <Draggable v-model="listsModel" item-key="list.id" class="lists-wrapper" @end="onListsDragEnd" :animation="150" :handle="'.list-drag-handle'">
        <template #item="{ element: listWithCards }">
          <div
            :key="listWithCards.list.id"
            class="kanban-list"
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
                @dragstart="onCardDragStart($event, card, listWithCards.list.id)"
                @dragend="onCardDragEnd"
                @dragover="onCardDragOver($event, card.id)"
                @dragleave="onCardDragLeave"
                @drop="onCardDropOnCard($event, index, listWithCards.list.id)"
              />

              <div
                class="card-drop-zone"
                :class="{ 'active': dropZoneListId === listWithCards.list.id }"
                @dragover="onDropZoneDragOver($event, listWithCards.list.id)"
                @dragleave="onDropZoneDragLeave"
                @drop="onCardDropAtEnd($event, listWithCards.list.id)"
              ></div>
            </div>

            <button class="add-card-btn" @click="$emit('addCard', listWithCards.list.id)">
              <Icon name="plus" :size="14" /> Add Card
            </button>
          </div>
        </template>
      </Draggable>

      <div class="kanban-list add-list-placeholder" @click="$emit('addList')">
        <Icon name="plus" :size="16" />
        <span>Add List</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import Draggable from 'vuedraggable'
import type { Card, BoardLabel, ListWithCards } from '../../types'
import KanbanListHeader from './KanbanListHeader.vue'
import KanbanCard from './KanbanCard.vue'
import Icon from '../ui/Icon.vue'

interface Props {
  lists: ListWithCards[]
  labels: BoardLabel[]
  selectedLabelFilters?: string[]
  dueDateFilter?: string
  hideDoneCards?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  selectedLabelFilters: () => [],
  dueDateFilter: '',
  hideDoneCards: false
})

const emit = defineEmits<{
  'update:lists': [lists: ListWithCards[]]
  listsDragEnd: []
  openCard: [card: Card]
  addCard: [listId: string]
  addList: []
  archiveList: [listId: string]
  deleteList: [listId: string]
  moveCard: [cardId: string, fromListId: string, toListId: string, position: number]
}>()

// Drag and drop state
const draggingCard = ref<Card | null>(null)
const draggingFromListId = ref<string>('')
const dragOverCard = ref<string>('')
const dropZoneListId = ref<string>('')
const noteDropTargetCardId = ref<string>('')

// v-model for lists
const listsModel = computed({
  get: () => props.lists,
  set: (value) => emit('update:lists', value)
})

// Filter cards based on filters
function getFilteredCards(cards: Card[]): Card[] {
  return cards.filter(card => {
    if (card.archived) return false
    if (props.hideDoneCards && card.status === 'done') return false

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

// Event handlers
const onListsDragEnd = () => emit('listsDragEnd')

function onCardDragStart(event: DragEvent, card: Card, listId: string) {
  draggingCard.value = card
  draggingFromListId.value = listId
  event.dataTransfer!.effectAllowed = 'move'
}

function onCardDragEnd() {
  draggingCard.value = null
  draggingFromListId.value = ''
  dragOverCard.value = ''
  dropZoneListId.value = ''
  noteDropTargetCardId.value = ''
}

function onCardDragOver(event: DragEvent, cardId: string) {
  event.preventDefault()
  dragOverCard.value = cardId
}

function onCardDragLeave() {
  dragOverCard.value = ''
}

function onDropZoneDragOver(event: DragEvent, listId: string) {
  event.preventDefault()
  dropZoneListId.value = listId
}

function onDropZoneDragLeave() {
  dropZoneListId.value = ''
}

function onCardDropOnCard(event: DragEvent, index: number, listId: string) {
  event.preventDefault()
  // Handle card drop on another card
  if (draggingCard.value && draggingFromListId.value !== listId) {
    // Move card to new list at specific position
    emit('moveCard', draggingCard.value.id, draggingFromListId.value, listId, index)
  }
  onCardDragEnd()
}

function onCardDropAtEnd(event: DragEvent, listId: string) {
  event.preventDefault()
  dropZoneListId.value = listId
  // Handle card drop at end of list
  if (draggingCard.value && draggingFromListId.value !== listId) {
    // Move card to end of new list
    emit('moveCard', draggingCard.value.id, draggingFromListId.value, listId, -1)
  }
  onCardDragEnd()
}
</script>

<style scoped>
.kanban-container {
  flex: 1;
  overflow-x: auto;
  padding: 0 2rem 2rem 2rem;
}

.lists-row {
  display: flex;
  align-items: flex-start;
  gap: 1.5rem;
}

.lists-wrapper {
  display: flex;
  gap: 1.5rem;
  height: 100%;
  align-items: flex-start;
}

.kanban-list {
  background: var(--bg-secondary);
  border-radius: 8px;
  padding: 1rem;
  min-width: 300px;
  max-width: 300px;
  display: flex;
  flex-direction: column;
  max-height: calc(100vh - 200px);
}

.cards-container {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.card-drop-zone {
  flex: 1;
  min-height: 20px;
  margin: 0.25rem 0;
  border-radius: 2px;
  transition: all 0.2s;
}

.card-drop-zone.active {
  min-height: 40px;
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
  .kanban-container {
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
