<template>
  <div class="board-pane">
    <div v-if="loading" class="loading">Loading board...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else-if="currentBoard" class="board-container">
      <!-- Board Header (Simplified) -->
      <div class="board-header">
        <span class="board-name">{{ currentBoard.board.name }}</span>
        <div class="board-actions">
          <button @click="showAddListModal = true">Add List</button>
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
        @add-list="showAddListModal = true"
        @archive-list="archiveList"
        @delete-list="confirmDeleteList"
        @move-card="handleMoveCard"
      />
    </div>

    <!-- Modals (Simplified) -->
    <!-- Add List Modal -->
    <div v-if="showAddListModal" class="modal-overlay" @click.self="showAddListModal = false">
      <div class="modal">
        <h3>Add List</h3>
        <form @submit.prevent="addList">
          <input v-model="newListName" placeholder="List Name" required />
          <button type="submit">Add</button>
        </form>
      </div>
    </div>

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
import { boardsApi, listsApi, cardsApi } from '@/api'
import type { BoardWithLists, Card } from '@/types'
import logger from '@/utils/logger'
import { toApiIso } from '@/utils/dates'

const props = defineProps<{
  boardId?: string
  paneId: string
}>()

const currentBoard = ref<BoardWithLists | null>(null)
const activeLists = ref<any[]>([])
const loading = ref(false)
const error = ref<string | null>(null)

// Modals
const showAddListModal = ref(false)
const newListName = ref('')
const showEditCardModal = ref(false)
const editingCard = ref<Card | null>(null)

async function fetchBoard() {
  if (!props.boardId) return
  
  loading.value = true
  error.value = null
  
  try {
    const response = await boardsApi.get(props.boardId)
    currentBoard.value = response.data
    activeLists.value = response.data.lists.filter((l: any) => !l.list.archived)
  } catch (e) {
    logger.error('Failed to fetch board', e)
    error.value = 'Failed to load board'
  } finally {
    loading.value = false
  }
}

watch(() => props.boardId, fetchBoard)
onMounted(fetchBoard)

// List Operations
async function addList() {
  if (!currentBoard.value || !newListName.value.trim()) return
  try {
    await listsApi.create(currentBoard.value.board.id, { name: newListName.value })
    showAddListModal.value = false
    newListName.value = ''
    await fetchBoard()
  } catch (e) {
    logger.error('Failed to add list', e)
  }
}

async function archiveList(listId: string) {
  try {
    await listsApi.archive(listId)
    await fetchBoard()
  } catch (e) {
    logger.error('Failed to archive list', e)
  }
}

function confirmDeleteList(listId: string) {
  if (confirm('Delete list?')) {
    listsApi.delete(listId).then(() => fetchBoard())
  }
}

function onListsDragEnd() {
  // TODO: Implement reorder logic similar to BoardsWorkspace
}

// Card Operations
function openAddCardModal(listId: string) {
  const title = prompt('Card Title:')
  if (title) {
    cardsApi.create(listId, { title }).then(() => fetchBoard())
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
  try {
    const payload: any = { ...card }
    if (card.due_date) payload.due_date = toApiIso(card.due_date)
    await cardsApi.update(card.id, payload)
    closeEditCardModal()
    await fetchBoard()
  } catch (e) {
    logger.error('Failed to save card', e)
  }
}

async function handleCardArchive() {
  if (!editingCard.value) return
  await cardsApi.archive(editingCard.value.id)
  closeEditCardModal()
  await fetchBoard()
}

async function handleCardDelete() {
  if (!editingCard.value) return
  await cardsApi.delete(editingCard.value.id)
  closeEditCardModal()
  await fetchBoard()
}

async function handleMoveCard(cardId: string, _from: string, to: string, pos: number) {
  await cardsApi.move({ card_id: cardId, target_list_id: to, position: pos })
  await fetchBoard()
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
  height: 40px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
}

.board-name {
  font-weight: 600;
}

.loading, .error {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
}
</style>
