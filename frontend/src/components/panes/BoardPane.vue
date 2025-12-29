<template>
  <div class="board-pane">
    <div v-if="loading" class="loading">Loading board...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else-if="currentBoard" class="board-container">
      <!-- Board Header -->
      <div class="board-header">
        <h2 class="board-name">{{ currentBoard.board.name }}</h2>
        <div class="board-actions">
          <button class="btn-icon" @click="openAddListModal" title="Add List">
            <Icon name="plus" :size="18" />
          </button>
        </div>
      </div>

      <KanbanBoard
        v-model:lists="activeLists"
        :labels="currentBoard.labels || []"
        :selected-label-filters="[]"
        :due-date-filter="''"
        :hide-done-cards="false"
        @lists-drag-end="onListsDragEnd"
        @open-card="openCardModal"
        @add-card="openAddCardModal"
        @add-list="openAddListModal"
        @archive-list="archiveList"
        @delete-list="confirmDeleteList"
        @move-card="handleMoveCard"
      />
    </div>

    <!-- Modals -->
    <PromptModal
      :visible="promptModal.visible"
      :title="promptModal.title"
      :placeholder="promptModal.placeholder"
      @submit="handlePromptSubmit"
      @cancel="promptModal.visible = false"
    />

    <ConfirmModal
      :visible="confirmModal.visible"
      :title="confirmModal.title"
      :message="confirmModal.message"
      variant="danger"
      @confirm="handleDeleteList"
      @cancel="confirmModal.visible = false"
    />

    <!-- Card Edit Modal -->
    <CardEditModal
      :visible="showEditCardModal"
      :card="editingCard"
      :labels="currentBoard?.labels || []"
      @close="closeEditCardModal"
      @save="handleCardSave"
      @archive="handleCardArchive"
      @delete="handleCardDelete"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { KanbanBoard } from '../boards'
import CardEditModal from '../boards/CardEditModal.vue'
import PromptModal from '../modals/PromptModal.vue'
import ConfirmModal from '../modals/ConfirmModal.vue'
import Icon from '../ui/Icon.vue'
import { boardsApi, listsApi, cardsApi } from '@/api'
import type { BoardWithLists, Card } from '@/types'
import logger from '@/utils/logger'
import { toApiIso } from '@/utils/dates'
import { useSync } from '@/composables/useSync'

const props = defineProps<{
  boardId?: string
  paneId: string
}>()

const { withSync } = useSync()
const currentBoard = ref<BoardWithLists | null>(null)
const activeLists = ref<any[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

// Modals
const showEditCardModal = ref(false)
const editingCard = ref<Card | null>(null)

const promptModal = ref({
  visible: false,
  title: '',
  placeholder: '',
  action: '',
  listId: ''
})

const confirmModal = ref({
  visible: false,
  title: '',
  message: '',
  listId: ''
})

async function fetchBoard() {
  if (!props.boardId) return
  
  loading.value = true
  error.value = null
  
  return withSync(async () => {
    try {
      const response = await boardsApi.getWithLists(props.boardId!)
      currentBoard.value = response.data
      activeLists.value = response.data.lists.filter((l: any) => !l.list.archived)
    } catch (e) {
      logger.error('Failed to fetch board', e)
      error.value = 'Failed to load board'
    } finally {
      loading.value = false
    }
  })
}

watch(() => props.boardId, fetchBoard)
onMounted(fetchBoard)

// List Operations
function openAddListModal() {
  promptModal.value = {
    visible: true,
    title: 'Add List',
    placeholder: 'List Name',
    action: 'add-list',
    listId: ''
  }
}

async function addList(name: string) {
  if (!currentBoard.value || !name.trim()) return
  return withSync(async () => {
    try {
      await listsApi.create(currentBoard.value!.board.id, { name })
      await fetchBoard()
    } catch (e) {
      logger.error('Failed to add list', e)
    }
  })
}

async function archiveList(listId: string) {
  return withSync(async () => {
    try {
      await listsApi.archive(listId)
      await fetchBoard()
    } catch (e) {
      logger.error('Failed to archive list', e)
    }
  })
}

function confirmDeleteList(listId: string) {
  confirmModal.value = {
    visible: true,
    title: 'Delete List',
    message: 'Are you sure you want to delete this list? All cards in it will be deleted.',
    listId
  }
}

async function handleDeleteList() {
  if (!confirmModal.value.listId) return
  return withSync(async () => {
    try {
      await listsApi.delete(confirmModal.value.listId)
      confirmModal.value.visible = false
      await fetchBoard()
    } catch (e) {
      logger.error('Failed to delete list', e)
    }
  })
}

async function onListsDragEnd() {
  if (!currentBoard.value) return
  const listIds = activeLists.value.map(l => l.list.id)
  return withSync(async () => {
    try {
      await listsApi.reorder({ board_id: currentBoard.value!.board.id, list_ids: listIds })
    } catch (e) {
      logger.error('Failed to reorder lists', e)
      await fetchBoard()
    }
  })
}

// Card Operations
function openAddCardModal(listId: string) {
  promptModal.value = {
    visible: true,
    title: 'Add Card',
    placeholder: 'Card Title',
    action: 'add-card',
    listId
  }
}

async function handlePromptSubmit(value: string) {
  promptModal.value.visible = false
  if (promptModal.value.action === 'add-list') {
    await addList(value)
  } else if (promptModal.value.action === 'add-card') {
    return withSync(async () => {
      try {
        await cardsApi.create(promptModal.value.listId, { title: value })
        await fetchBoard()
      } catch (e) {
        logger.error('Failed to add card', e)
      }
    })
  }
}

function openCardModal(card: Card) {
  editingCard.value = card
  showEditCardModal.value = true
}

function closeEditCardModal() {
  showEditCardModal.value = false
  editingCard.value = null
}

async function handleCardSave(card: Card) {
  if (!card) return
  return withSync(async () => {
    try {
      const payload: any = { ...card }
      if (card.due_date) payload.due_date = toApiIso(card.due_date)
      await cardsApi.update(card.id, payload)
      closeEditCardModal()
      await fetchBoard()
    } catch (e) {
      logger.error('Failed to save card', e)
    }
  })
}

async function handleCardArchive() {
  if (!editingCard.value) return
  return withSync(async () => {
    await cardsApi.archive(editingCard.value!.id)
    closeEditCardModal()
    await fetchBoard()
  })
}

async function handleCardDelete() {
  if (!editingCard.value) return
  return withSync(async () => {
    await cardsApi.delete(editingCard.value!.id)
    closeEditCardModal()
    await fetchBoard()
  })
}

async function handleMoveCard(cardId: string, _from: string, to: string, pos: number) {
  return withSync(async () => {
    await cardsApi.move({ card_id: cardId, target_list_id: to, position: pos })
    await fetchBoard()
  })
}
</script>

<style scoped>
.board-pane {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.board-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.board-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 2rem 2rem 1rem 2rem;
  background: transparent;
}

@media (max-width: 768px) {
  .board-header {
    padding: 1rem 1rem 0.5rem 1rem;
  }
}

.board-name {
  font-size: 2rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0;
  letter-spacing: -0.02em;
}

@media (max-width: 768px) {
  .board-name {
    font-size: 1.25rem;
  }
}

.board-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  opacity: 0;
  transition: opacity 0.2s;
}

.board-header:hover .board-actions {
  opacity: 1;
}

@media (max-width: 768px) {
  .board-actions {
    opacity: 1;
  }
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
  color: var(--text-muted);
  cursor: pointer;
  transition: all 0.2s;
}

.btn-icon:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.loading, .error {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
}
</style>
