<template>
  <div class="boards-workspace">
    <!-- Mobile Sidebar Toggle -->
    <MobileSidebarToggle
      :is-open="isMobileSidebarOpen"
      @toggle="toggleMobileSidebar"
    />

    <!-- Mobile Sidebar Overlay -->
    <MobileOverlay
      :is-open="isMobileSidebarOpen"
      @close="isMobileSidebarOpen = false"
    />

    <!-- Sidebar -->
    <WorkspaceSidebar
      :width="sidebarWidth"
      :is-open="isMobileSidebarOpen"
      @refresh="refreshTree"
    >
      <ExplorerTree
        default-tab="boards"
        :show-boards-header="false"
        @select="handleExplorerSelect"
        @create-note="handleCreateNote"
        @create-folder="handleCreateFolder"
        @create-board-folder="handleCreateBoardFolder"
        @rename="handleRename"
        @delete="handleExplorerDelete"
        @open-note="handleOpenNote"
        @open-board="selectBoard"
        @drop="handleDrop"
      />
    </WorkspaceSidebar>

    <!-- Resize Handle -->
    <ResizeHandle
      direction="vertical"
    />

    <!-- Main Content -->
    <div class="board-content"
      @dragenter="onBoardContentDragEnter"
      @dragover="onBoardContentDragOver"
      @dragleave="onBoardContentDragLeave"
      @drop="onBoardContentDrop"
      :class="{ 'drag-over': boardContentDragOver }"
    >
      <!-- Board Header -->
      <BoardHeader
        :board="currentBoard?.board || null"
        :has-active-filters="hasActiveFilters"
        @toggle-filters="showFiltersPanel = !showFiltersPanel"
        @open-labels="showLabelsPanel = true"
        @open-automations="showAutomationsPanel = true"
        @open-archive="openArchivePanel"
        @add-list="openAddListModal"
        @delete-board="confirmDeleteBoard"
      />

      <!-- Filters Bar -->
      <BoardFilters
        :show-filters-panel="showFiltersPanel"
        :current-board="currentBoard"
        :selected-label-filters="selectedLabelFilters"
        :due-date-filter="dueDateFilter"
        :hide-done-cards="hideDoneCards"
        @toggle-label-filter="toggleLabelFilter"
        @update:due-date-filter="dueDateFilter = $event"
        @update:hide-done-cards="hideDoneCards = $event"
        @clear-filters="clearFilters"
      />

      <div v-if="currentBoard">
        <KanbanBoard
          v-model:lists="activeLists"
          :labels="currentBoard?.labels || []"
          :selected-label-filters="selectedLabelFilters"
          :due-date-filter="dueDateFilter"
          :hide-done-cards="hideDoneCards"
          @lists-drag-end="onListsDragEnd"
          @open-card="openCardModal"
          @add-card="openAddCardModal"
          @add-list="openAddListModal"
          @archive-list="archiveList"
          @delete-list="confirmDeleteList"
          @move-card="handleMoveCard"
        />
      </div>

      <!-- Empty State -->
      <div class="board-empty" v-else>
        <div class="empty-content">
          <Icon name="board" :size="48" />
          <h3>No Board Selected</h3>
          <p>Select a board from the sidebar or create a new one</p>
        </div>
      </div>
    </div>





    <!-- Add List Modal -->
    <div v-if="showAddListModal" class="modal-overlay" @click.self="showAddListModal = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Add List</h3>
          <button class="modal-close" @click="showAddListModal = false">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <form @submit.prevent="addList">
          <div class="form-group">
            <label for="listName">List Name</label>
            <input 
              id="listName" 
              v-model="newListName" 
              type="text" 
              required 
              placeholder="e.g., To Do, In Progress, Done"
            />
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="showAddListModal = false">Cancel</button>
            <button type="submit" class="btn btn-primary">Add List</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Add Card Modal -->
    <div v-if="showAddCardModal" class="modal-overlay" @click.self="showAddCardModal = false">
      <div class="modal">
        <div class="modal-header">
          <h3>Add Card</h3>
          <button class="modal-close" @click="showAddCardModal = false">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <form @submit.prevent="addCard">
          <div class="form-group">
            <label for="cardTitle">Title</label>
            <input 
              id="cardTitle" 
              v-model="newCard.title" 
              type="text" 
              required 
              placeholder="Card title"
            />
          </div>
          <div class="form-group">
            <label for="cardDescription">Description (optional)</label>
            <textarea 
              id="cardDescription" 
              v-model="newCard.description" 
              rows="3" 
              placeholder="Add more details..."
            ></textarea>
          </div>
          <div class="form-group">
            <label for="cardDueDate">Due Date (optional)</label>
            <input
              id="cardDueDate"
              v-model="newCard.due_date"
              type="datetime-local"
              @focus="onAddDueDateFocus"
              @change="onAddDueDateChange"
            />
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="showAddCardModal = false">Cancel</button>
            <button type="submit" class="btn btn-primary">Add Card</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Card modal (view-first) -->
    <CardEditModal
      :visible="showEditCardModal"
      :card="editingCard"
      :labels="currentBoard?.labels || []"
      @close="closeEditCardModal"
      @save="handleCardSave"
      @archive="handleCardArchive"
      @delete="handleCardDelete"
      @navigateToItem="navigateToLinkedItem"
    />

    <!-- Labels Management Panel -->
    <LabelsPanel
      :visible="showLabelsPanel"
      :labels="currentBoard?.labels || []"
      @close="showLabelsPanel = false"
      @edit="handleEditLabel"
      @delete="deleteLabelById"
      @create="handleCreateLabel"
    />

    <!-- Archive Panel -->
    <ArchivePanel
      :visible="showArchivePanel"
      :archived-lists="archivedLists"
      :archived-cards="archivedCards"
      @close="showArchivePanel = false"
      @restore-list="restoreList"
      @restore-card="restoreCard"
    />

    <!-- Automations Panel -->
    <AutomationsPanel
      :visible="showAutomationsPanel"
      :automations="currentBoard?.automations || []"
      :lists="currentBoard?.lists || []"
      :labels="currentBoard?.labels || []"
      @close="showAutomationsPanel = false"
      @create="handleAutomationCreate"
      @toggle="handleAutomationToggle"
      @delete="handleAutomationDelete"
    />

    <!-- Confirmation Modals -->
    <ConfirmModal
      :visible="deleteListModalVisible"
      title="Delete List"
      message="Delete this list and all its cards? This action cannot be undone."
      confirm-text="Delete"
      variant="danger"
      @confirm="handleDeleteList"
      @cancel="deleteListModalVisible = false"
    />

    <ConfirmModal
      :visible="deleteCardModalVisible"
      title="Delete Card"
      message="Delete this card? This action cannot be undone."
      confirm-text="Delete"
      variant="danger"
      @confirm="handleDeleteCard"
      @cancel="deleteCardModalVisible = false"
    />

    <ConfirmModal
      :visible="deleteBoardModalVisible"
      title="Delete Board"
      message="Delete this board and all its lists and cards? This action cannot be undone."
      confirm-text="Delete"
      variant="danger"
      @confirm="handleDeleteBoard"
      @cancel="deleteBoardModalVisible = false"
    />

    <ConfirmModal
      :visible="deleteLabelModal.visible"
      title="Delete Label"
      message="Delete this label? It will be removed from all cards."
      confirm-text="Delete"
      variant="danger"
      @confirm="confirmDeleteLabel"
      @cancel="deleteLabelModal.visible = false"
    />

    <!-- Prompt Modal for board folders -->
    <PromptModal
      :visible="promptModal.visible"
      :title="promptModal.title"
      :placeholder="promptModal.placeholder"
      confirm-text="Create"
      @submit="handlePromptSubmit"
      @cancel="promptModal.visible = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { boardsApi, listsApi, cardsApi, labelsApi, automationsApi } from '../api'
import { useExplorerStore, type ExplorerItem } from '../stores/explorer'
import { useNotesStore } from '../stores/notes'
import type { BoardWithLists, Card, BoardLabel } from '../types'
import { WorkspaceSidebar, MobileSidebarToggle, MobileOverlay, ResizeHandle } from '../components/workspace'
import ExplorerTree from '../components/common/ui/ExplorerTree.vue'
import Icon from '../components/common/ui/Icon.vue'
import ConfirmModal from '../components/common/modals/ConfirmModal.vue'
import PromptModal from '../components/common/modals/PromptModal.vue'
import CardEditModal from '../components/boards/CardEditModal.vue'
import { KanbanBoard } from '../components/boards'
import BoardHeader from '../components/boards/BoardHeader.vue'
import BoardFilters from '../components/boards/BoardFilters.vue'
import LabelsPanel from '../components/boards/LabelsPanel.vue'
import ArchivePanel from '../components/boards/ArchivePanel.vue'
import AutomationsPanel from '../components/boards/AutomationsPanel.vue'
import { toApiIso, normalizeForInput } from '../utils/dates'
import logger from '@/utils/logger'
import { toDatetimeLocal } from '../utils/dates'

const router = useRouter()
const route = useRoute()
const explorerStore = useExplorerStore()
const notesStore = useNotesStore()

// State
const sidebarWidth = ref(240)
const isMobileSidebarOpen = ref(false)

// Board state
const currentBoard = ref<BoardWithLists | null>(null)
const selectedBoardId = ref<string | null>(null)

// Archive state
const archivedLists = ref<any[]>([])
const archivedCards = ref<any[]>([])

// List modal
const showAddListModal = ref(false)
const newListName = ref('')

// Card modals
const showAddCardModal = ref(false)
const showEditCardModal = ref(false)
const selectedListId = ref('')
const newCard = ref({ title: '', description: '', due_date: undefined as string | undefined })
const editingCard = ref<Card | null>(null)

// Side panels
const showLabelsPanel = ref(false)
const showArchivePanel = ref(false)
const showAutomationsPanel = ref(false)
const showFiltersPanel = ref(false)

// Filter: hide done cards
const hideDoneCards = ref(false)

// Labels
const newLabelName = ref('')
const newLabelColor = ref('#e74c3c')

// Filters
const selectedLabelFilters = ref<string[]>([])
const dueDateFilter = ref('')

// Automations
// (State moved to AutomationsPanel component)

// Drag and drop
const boardContentDragOver = ref(false)

// Delete modals
const deleteListModalVisible = ref(false)
const deleteListId = ref('')
const deleteCardModalVisible = ref(false)
const deleteBoardModalVisible = ref(false)
const deleteLabelModal = ref({ visible: false, labelId: '' })

// Prompt modal for board folders
const promptModal = ref({
  visible: false,
  title: 'New Board Folder',
  placeholder: 'Enter folder name',
  action: 'board-folder',
  parentId: null as string | null
})

// Colors
// (Board colors handled by WorkspaceSidebar component)

// Computed
const hasActiveFilters = computed(() => {
  return selectedLabelFilters.value.length > 0 || dueDateFilter.value !== ''
})

// NOTE: `activeLists` is used for the visible (non-archived) lists and drag-and-drop.

// Active lists used for drag-and-drop. We keep a local copy so vuedraggable can mutate it,
// and sync back to `currentBoard` when reorder finishes.
const activeLists = ref<Array<any>>([])

watch(currentBoard, (newBoard) => {
  if (!newBoard) {
    activeLists.value = []
    return
  }
  // sync active (non-archived) lists order
  activeLists.value = newBoard.lists.filter((lwc: any) => !lwc.list.archived)
}, { immediate: true })



// Debounced/queued reorder: schedule rather than immediately calling API on every quick reorder.
let _reorderTimer: ReturnType<typeof setTimeout> | null = null
let _lastReorderIds: string[] = []
const REORDER_DEBOUNCE_MS = 450

function scheduleReorder(newActiveIds: string[]) {
  _lastReorderIds = newActiveIds.slice()
  if (_reorderTimer) clearTimeout(_reorderTimer)
  _reorderTimer = setTimeout(async () => {
    _reorderTimer = null
    await performReorder(_lastReorderIds)
  }, REORDER_DEBOUNCE_MS)
}

async function performReorder(listIds: string[]) {
  if (!currentBoard.value) return
  try {
    await listsApi.reorder({ board_id: currentBoard.value.board.id, list_ids: listIds })
    // Refresh board once after successful reorder
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    logger.error('Failed to persist list reorder:', error)
    // Try to refresh to re-sync UI
    if (currentBoard.value) await fetchBoard(currentBoard.value.board.id)
  }
}

// Handle lists reorder (drag end) — update local UI immediately and schedule persistence
function onListsDragEnd() {
  if (!currentBoard.value) return

  // New order of active list ids
  const newActiveIds = activeLists.value.map((l: any) => l.list.id)

  // Rebuild currentBoard.lists: put active lists in new order, then append archived lists
  const allLists = currentBoard.value.lists
  const archived = allLists.filter((l: any) => l.list.archived)
  const byId = new Map(allLists.map((l: any) => [l.list.id, l]))
  const newOrdered: any[] = []
  for (const id of newActiveIds) {
    const item = byId.get(id)
    if (item) newOrdered.push(item)
  }
  // Preserve archived lists after active ones
  for (const a of archived) newOrdered.push(a)

  currentBoard.value.lists = newOrdered

  // Schedule API call (debounced)
  scheduleReorder(newActiveIds)
}

// Refresh tree
async function refreshTree() {
  await explorerStore.fetchAll()
}

// Mobile sidebar toggle
function toggleMobileSidebar() {
  isMobileSidebarOpen.value = !isMobileSidebarOpen.value
}

// Fetch single board with lists
async function fetchBoard(boardId: string) {
  try {
    const response = await boardsApi.get(boardId)
    currentBoard.value = response.data
  } catch (error) {
    logger.error('Failed to fetch board:', error)
    currentBoard.value = null
  }
}

// Select a board
async function selectBoard(boardId: string) {
  selectedBoardId.value = boardId
  explorerStore.selectItem(boardId, 'board')
  await fetchBoard(boardId)
  // Close mobile sidebar when a board is selected
  isMobileSidebarOpen.value = false
}

// If navigated to /boards?board=...&card=..., open the specified board and card
onMounted(async () => {
  try {
    const q = route.query
    const boardId = typeof q.board === 'string' ? q.board : undefined
    const cardId = typeof q.card === 'string' ? q.card : undefined
    if (boardId) {
      await selectBoard(boardId)
      // After selecting board, if a card id was provided, try to find and open it
      if (cardId && currentBoard.value) {
        // Search lists in currentBoard for the card
        for (const lwc of currentBoard.value.lists) {
          const found = lwc.cards.find(c => c.id === cardId)
          if (found) {
            // Normalize due_date for input and open modal
            openCardModal(found)
            break
          }
        }
      }
    }
  } catch (e) {
    logger.error('Failed to open card from query params', e)
  }
})

// Explorer tree handlers
function handleExplorerSelect(item: ExplorerItem) {
  explorerStore.selectItem(item.id, item.type as 'note' | 'folder' | 'board' | 'board-folder')
  if (item.type === 'board') {
    selectBoard(item.id)
  }
}

function handleOpenNote(noteId: string) {
  router.push('/notes')
  notesStore.openNote(noteId)
}

function handleCreateNote(folderId: string | null) {
  // Navigate to notes and create note there
  localStorage.setItem('createNoteInFolder', folderId || '')
  router.push('/notes')
}

function handleCreateFolder(parentId: string | null) {
  // Navigate to notes and create folder there
  localStorage.setItem('createFolderInParent', parentId || '')
  router.push('/notes')
}

function handleRename(item: ExplorerItem) {
  if (item.type === 'board-folder') {
    const newName = prompt('Enter new folder name:', item.name)
    if (newName && newName !== item.name) {
      explorerStore.renameBoardFolder(item.id, newName)
    }
  } else {
    // For notes/folders, navigate to notes
    router.push('/notes')
  }
}

function handleExplorerDelete(item: ExplorerItem) {
  if (item.type === 'board') {
    selectedBoardId.value = item.id
    deleteBoardModalVisible.value = true
  } else if (item.type === 'board-folder') {
    // TODO: Add board folder deletion confirmation
    explorerStore.deleteBoardFolder(item.id)
  } else {
    // For notes/folders, navigate to notes
    router.push('/notes')
  }
}

async function handleCreateBoardFolder(parentId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Board Folder',
    placeholder: 'Enter folder name',
    action: 'board-folder',
    parentId: parentId
  }
}

async function handlePromptSubmit(name: string) {
  if (name && promptModal.value.parentId !== undefined) {
    await explorerStore.createBoardFolder(name, promptModal.value.parentId)
    promptModal.value.visible = false
  }
}

async function handleDrop(data: { draggedId: string; draggedType: 'note' | 'folder' | 'board' | 'board-folder'; targetId: string | null; targetType: 'folder' | 'note' | 'board' | 'board-folder' | null; position: number; dropPosition: 'before' | 'after' | 'inside' }) {
  try {
    if (data.draggedType === 'note') {
      await explorerStore.moveNote(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchNotes()
    } else if (data.draggedType === 'folder') {
      await explorerStore.moveFolder(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchNotes()
    } else if (data.draggedType === 'board') {
      await explorerStore.moveBoard(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchBoards()
    } else if (data.draggedType === 'board-folder') {
      await explorerStore.moveBoardFolder(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchBoards()
    }
  } catch (error) {
    logger.error('Failed to move item:', error)
  }
}

// Board menu
function confirmDeleteBoard() {
  deleteBoardModalVisible.value = true
}

async function handleDeleteBoard() {
  deleteBoardModalVisible.value = false
  if (!selectedBoardId.value) return
  
  try {
    await explorerStore.deleteBoard(selectedBoardId.value)
    selectedBoardId.value = null
    currentBoard.value = null
  } catch (error) {
    logger.error('Failed to delete board:', error)
  }
}

// List CRUD
function openAddListModal() {
  newListName.value = ''
  showAddListModal.value = true
}

// (renameListPrompt removed — not currently used; renaming lists is available via the board menu)

// (renameList removed — list renaming can be added later if desired)

async function addList() {
  if (!currentBoard.value || !newListName.value.trim()) return
  
  try {
    await listsApi.create(currentBoard.value.board.id, { name: newListName.value })
    showAddListModal.value = false
    newListName.value = ''
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    logger.error('Failed to add list:', error)
  }
}

function confirmDeleteList(listId: string) {
  deleteListId.value = listId
  deleteListModalVisible.value = true
}

async function handleDeleteList() {
  deleteListModalVisible.value = false
  try {
    await listsApi.delete(deleteListId.value)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    logger.error('Failed to delete list:', error)
  }
}

async function archiveList(listId: string) {
  try {
    await listsApi.archive(listId)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    logger.error('Failed to archive list:', error)
  }
}

async function restoreList(listId: string) {
  try {
    await listsApi.restore(listId)
    if (currentBoard.value) {
      await Promise.all([fetchBoard(currentBoard.value.board.id), fetchArchivedItems()])
    }
  } catch (error) {
    logger.error('Failed to restore list:', error)
  }
}

// Card CRUD
function openAddCardModal(listId: string) {
  selectedListId.value = listId
  newCard.value = { title: '', description: '', due_date: undefined }
  showAddCardModal.value = true
}

async function addCard() {
  if (!newCard.value.title.trim()) return
  
  try {
    // Ensure due_date is formatted as ISO with timezone for backend
    const payload: any = { ...newCard.value }
    if (payload.due_date) {
      payload.due_date = toApiIso(payload.due_date)
    }
    await cardsApi.create(selectedListId.value, payload)
    showAddCardModal.value = false
    newCard.value = { title: '', description: '', due_date: undefined }
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    logger.error('Failed to add card:', error)
  }
}

function openCardModal(card: Card) {
  // Convert due_date (ISO / with timezone) into a datetime-local friendly value
  editingCard.value = { ...card, due_date: card.due_date ? normalizeForInput(card.due_date) : undefined }
  showEditCardModal.value = true
}

function closeEditCardModal() {
  showEditCardModal.value = false
  editingCard.value = null
}
 
// Handler for CardEditModal 'save' event
async function handleCardSave(card: Card) {
  if (!card) return
  try {
    const payload: any = {
      title: card.title,
      description: card.description,
      labels: card.labels || []
    }
    if (card.due_date) payload.due_date = toApiIso(card.due_date)
    if (card.status) payload.status = card.status

    await cardsApi.update(card.id, payload)
    closeEditCardModal()
    if (currentBoard.value) await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    logger.error('Failed to save card from modal:', error)
  }
}

// Handler for CardEditModal 'archive' event
async function handleCardArchive() {
  if (!editingCard.value) return
  try {
    await cardsApi.archive(editingCard.value.id)
    closeEditCardModal()
    if (currentBoard.value) await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    logger.error('Failed to archive card from modal:', error)
  }
}

// Handler for CardEditModal 'delete' event
async function handleCardDelete() {
  if (!editingCard.value) return
  try {
    await cardsApi.delete(editingCard.value.id)
    closeEditCardModal()
    if (currentBoard.value) await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    logger.error('Failed to delete card from modal:', error)
  }
}

// Due date handlers for the edit card modal
 

// Due date handlers for the add card modal
function onAddDueDateFocus() {
  if (!newCard.value) return
  if (!newCard.value.due_date) {
    // default to today at 18:00 local
    newCard.value.due_date = toDatetimeLocal(new Date(new Date().setHours(18, 0, 0, 0)))
  }
}

function onAddDueDateChange(e: Event) {
  if (!newCard.value) return
  const val = (e.target as HTMLInputElement).value
  if (!val) return
  const normalized = normalizeForInput(val)
  if (normalized && normalized !== val) {
    newCard.value.due_date = normalized
  }
}

 

async function handleDeleteCard() {
  deleteCardModalVisible.value = false
  if (!editingCard.value) return
  
  try {
    await cardsApi.delete(editingCard.value.id)
    closeEditCardModal()
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    logger.error('Failed to delete card:', error)
  }
}

// Note: status toggling is handled inside the Edit Card modal (editingCard.status)

async function restoreCard(cardId: string) {
  try {
    await cardsApi.restore(cardId)
    if (currentBoard.value) {
      await Promise.all([fetchBoard(currentBoard.value.board.id), fetchArchivedItems()])
    }
  } catch (error) {
    logger.error('Failed to restore card:', error)
  }
}

 

// Archive Panel
async function fetchArchivedItems() {
  if (!currentBoard.value) return
  try {
    const [listsRes, cardsRes] = await Promise.all([
      listsApi.getArchived(currentBoard.value.board.id),
      cardsApi.getArchived(currentBoard.value.board.id)
    ])
    archivedLists.value = listsRes.data
    archivedCards.value = cardsRes.data
  } catch (error) {
    logger.error('Failed to fetch archived items:', error)
  }
}

async function openArchivePanel() {
  await fetchArchivedItems()
  showArchivePanel.value = true
}

// Label operations
async function createLabel() {
  if (!currentBoard.value || !newLabelName.value.trim()) return
  try {
    await labelsApi.create(currentBoard.value.board.id, {
      name: newLabelName.value.trim(),
      color: newLabelColor.value
    })
    newLabelName.value = ''
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    logger.error('Failed to create label:', error)
  }
}

function editLabel(label: BoardLabel) {
  const newName = prompt('Edit label name:', label.name)
  if (newName && newName.trim() && currentBoard.value) {
    labelsApi.update(label.id, { name: newName.trim() }).then(() => fetchBoard(currentBoard.value!.board.id))
  }
}

function handleEditLabel(label: BoardLabel) {
  editLabel(label)
}

function handleCreateLabel(data: { name: string; color: string }) {
  if (!currentBoard.value) return
  // Update the reactive variables and call createLabel
  newLabelName.value = data.name
  newLabelColor.value = data.color
  createLabel()
}

function deleteLabelById(labelId: string) {
  deleteLabelModal.value = { visible: true, labelId }
}

async function confirmDeleteLabel() {
  const labelId = deleteLabelModal.value.labelId
  deleteLabelModal.value.visible = false
  try {
    await labelsApi.delete(labelId)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    logger.error('Failed to delete label:', error)
  }
}

// Filter operations
function toggleLabelFilter(labelId: string) {
  const index = selectedLabelFilters.value.indexOf(labelId)
  if (index === -1) {
    selectedLabelFilters.value.push(labelId)
  } else {
    selectedLabelFilters.value.splice(index, 1)
  }
}

function clearFilters() {
  selectedLabelFilters.value = []
  dueDateFilter.value = ''
  hideDoneCards.value = false
}

// Automation operations
async function createAutomation(automation: any) {
  if (!currentBoard.value || !automation) return
  
  try {
    await automationsApi.create(currentBoard.value.board.id, automation)
    await fetchBoard(currentBoard.value.board.id)
  } catch (error) {
    logger.error('Failed to create automation:', error)
  }
}

async function handleMoveCard(cardId: string, _fromListId: string, toListId: string, position: number) {
  try {
    await cardsApi.move({ card_id: cardId, target_list_id: toListId, position })
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    logger.error('Failed to move card:', error)
  }
}

async function toggleAutomation(id: string) {
  try {
    await automationsApi.toggle(id)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    logger.error('Failed to toggle automation:', error)
  }
}

async function deleteAutomation(id: string) {
  try {
    await automationsApi.delete(id)
    if (currentBoard.value) {
      await fetchBoard(currentBoard.value.board.id)
    }
  } catch (error) {
    logger.error('Failed to delete automation:', error)
  }
}

// AutomationsPanel event handlers
async function handleAutomationCreate(automation: any) {
  await createAutomation(automation)
}

async function handleAutomationToggle(automationId: string) {
  await toggleAutomation(automationId)
}

async function handleAutomationDelete(automationId: string) {
  await deleteAutomation(automationId)
}





// Board content drag handlers for opening notes/boards
function onBoardContentDragEnter(event: DragEvent) {
  const types = event.dataTransfer?.types || []
  const typesArray = Array.from(types)
  const isExplorer = typesArray.includes('application/x-explorer-item')
  if (!isExplorer) return
  
  event.preventDefault()
  boardContentDragOver.value = true
}

function onBoardContentDragOver(event: DragEvent) {
  const types = event.dataTransfer?.types || []
  const typesArray = Array.from(types)
  const isExplorer = typesArray.includes('application/x-explorer-item')
  if (!isExplorer) return
  
  event.preventDefault()
  if (event.dataTransfer) event.dataTransfer.dropEffect = 'copy'
  boardContentDragOver.value = true
}

function onBoardContentDragLeave(event: DragEvent) {
  // Only clear if we're leaving the board-content element entirely
  if ((event.target as HTMLElement).classList?.contains('board-content')) {
    boardContentDragOver.value = false
  }
}

function onBoardContentDrop(event: DragEvent) {
  event.preventDefault()
  boardContentDragOver.value = false
  
  // Handle tree item drops (notes/boards from explorer)
  const treeData = event.dataTransfer?.getData('application/x-explorer-item')
  
  if (treeData) {
    try {
      let parsed: any
      if (treeData.startsWith('{')) {
        parsed = JSON.parse(treeData)
      } else {
        // fallback when only id is set
        parsed = { id: treeData, type: 'note' }
      }
      const { id, type } = parsed
      
      if (type === 'note') {
        // Open the note in notes workspace
        notesStore.openNote(id)
      } else if (type === 'board') {
        // Switch to the board
        const boards = explorerStore.boards
        const targetBoard = findBoardById(boards, id)
        if (targetBoard) {
          currentBoard.value = targetBoard
          localStorage.setItem('selectedBoardId', id)
        }
      }
    } catch (e) {
      logger.error('Failed to parse dropped item:', e)
    }
  }
}

// Helper to find board by id recursively
function findBoardById(boards: any[], id: string): any {
  for (const board of boards) {
    if (board.board.id === id) return board
    if (board.children && board.children.length > 0) {
      const found = findBoardById(board.children, id)
      if (found) return found
    }
  }
  return null
}

// Get linked items from card description
interface LinkedItem {
  type: 'note' | 'board'
  name: string
  id: string
  href: string
  fullMatch: string
}

function navigateToLinkedItem(item: LinkedItem) {
  closeEditCardModal()

  if (item.type === 'note') {
    localStorage.setItem('openNoteId', item.id)
    router.push('/notes')
  } else {
    selectBoard(item.id)
  }
}
</script>

<style scoped>
.boards-workspace {
  display: flex;
  height: calc(100vh - 56px);
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

/* Board Content */
.board-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.board-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1.25rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
  min-width: 0;
}

.header-left h2 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  white-space: nowrap;
}

.board-description {
  font-size: 13px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s;
}

.btn-icon:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.menu-wrapper {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.25rem;
  min-width: 160px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
  overflow: hidden;
}

.dropdown-menu button {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  width: 100%;
  padding: 0.5rem 0.875rem;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  text-align: left;
}

.dropdown-menu button:hover {
  background: var(--bg-hover);
}

.dropdown-menu button.danger {
  color: var(--danger);
}

.dropdown-menu button.danger:hover {
  background: rgba(248, 81, 73, 0.1);
}

.menu-divider {
  height: 1px;
  background: var(--border-primary);
  margin: 0.25rem 0;
}

/* Kanban Board */
.kanban-container {
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

.list-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0 0.25rem 0.75rem;
  border-bottom: 1px solid var(--border-primary);
  margin-bottom: 0.75rem;
}

.list-header h3 {
  flex: 1;
  margin: 0;
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-primary);
}

.card-count {
  background: var(--bg-tertiary);
  color: var(--text-muted);
  font-size: 11px;
  padding: 0.125rem 0.5rem;
  border-radius: 10px;
}

.list-menu-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  border-radius: 4px;
  opacity: 0;
  transition: all 0.15s;
}

.kanban-list:hover .list-menu-btn {
  opacity: 1;
}

.list-menu-btn:hover {
  background: var(--bg-hover);
  color: var(--danger);
}

.list-drag-handle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: transparent;
  border: none;
  cursor: grab;
  color: var(--text-muted);
  border-radius: 4px;
}

.list-drag-handle:active {
  cursor: grabbing;
}

.kanban-list:hover .list-drag-handle {
  color: var(--text-primary);
}

.cards-container {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.kanban-card {
  position: relative;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  padding: 0.75rem;
  cursor: pointer;
  transition: all 0.15s;
}

.kanban-card:hover {
  border-color: var(--accent);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

/* edit button removed: clicking card opens edit modal */

/* Done badge: small pill in the top-right corner. Reserve space so it doesn't overlap the edit button */

.card-done-badge {
  position: absolute;
  top: 0.35rem;
  right: 0.375rem;
  background: var(--success);
  color: #fff;
  font-size: 11px;
  padding: 0.08rem 0.45rem;
  border-radius: 999px;
  box-shadow: 0 1px 2px rgba(0,0,0,0.12);
  z-index: 9;
  line-height: 1;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.card-done-badge small {
  opacity: 0.9;
}

.kanban-card h4 {
  margin: 0;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
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

.label.removable {
  cursor: pointer;
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

/* Empty State */
.board-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.empty-content {
  text-align: center;
  color: var(--text-muted);
}

.empty-content h3 {
  margin: 1rem 0 0.5rem;
  color: var(--text-primary);
}

.empty-content p {
  margin-bottom: 1.5rem;
  font-size: 14px;
}

.empty-content .btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
}

/* Context Menu */
.context-menu {
  position: fixed;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  padding: 0.375rem 0;
  min-width: 140px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
  z-index: 1000;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  padding: 0.5rem 0.875rem;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-primary);
  transition: background 0.15s;
}

.context-menu-item:hover {
  background: var(--bg-hover);
}

.context-menu-item.danger {
  color: var(--danger);
}

.context-menu-item.danger:hover {
  background: rgba(241, 76, 76, 0.1);
}

.context-menu-divider {
  height: 1px;
  background: var(--border-primary);
  margin: 0.375rem 0;
}

/* Component-specific styles */

/* Filters Bar */
.filters-bar {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  padding: 0.75rem 1.25rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
  flex-wrap: wrap;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.filter-group label {
  font-weight: 500;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.filter-labels {
  display: flex;
  gap: 0.25rem;
  flex-wrap: wrap;
}

.filter-label {
  padding: 0.2rem 0.5rem;
  border-radius: 3px;
  font-size: 0.75rem;
  cursor: pointer;
  border: 2px solid;
  background: transparent;
  color: var(--text-primary);
  transition: all 0.2s;
}

.filter-label.selected {
  color: white;
}

.filter-group select {
  padding: 0.3rem 0.5rem;
  border-radius: 4px;
  border: 1px solid var(--border-primary);
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-size: 0.85rem;
}

/* Drag and Drop */
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

/* Card links indicator */
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

/* Linked Items in Card Modal */
.linked-items-list {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.linked-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.linked-item:hover {
  background: var(--bg-hover);
}

.linked-item-name {
  flex: 1;
  font-size: 0.875rem;
  color: var(--text-primary);
}

.linked-item-remove {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
  opacity: 0;
  transition: all 0.15s;
}

.linked-item:hover .linked-item-remove {
  opacity: 1;
}

.linked-item-remove:hover {
  background: color-mix(in srgb, var(--danger) 20%, transparent);
  color: var(--danger);
}

/* Labels Management */
.labels-list-manage {
  margin-bottom: 1.5rem;
}

.label-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.label-preview {
  flex: 1;
  padding: 0.5rem;
  border-radius: 4px;
  color: white;
  font-weight: 500;
  font-size: 0.875rem;
}

.label-actions {
  display: flex;
  gap: 0.25rem;
  margin-left: 0.5rem;
}

.create-label-form {
  background: var(--bg-tertiary);
  padding: 1rem;
  border-radius: 8px;
}

.create-label-form h4 {
  margin: 0 0 0.75rem;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.create-label-form input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  margin-bottom: 0.75rem;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

/* Archive Panel */
.archive-section {
  margin-bottom: 1.5rem;
}

.archive-section h4 {
  margin: 0 0 0.75rem;
  color: var(--text-muted);
  font-size: 0.85rem;
}

.archived-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background: var(--bg-tertiary);
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.archived-item span {
  font-size: 0.875rem;
  color: var(--text-primary);
}

.empty-state {
  color: var(--text-muted);
  font-size: 0.85rem;
  padding: 0.5rem;
}

/* Automations Panel */
.automations-list {
  margin-bottom: 1.5rem;
}

.automation-item {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 6px;
  margin-bottom: 0.5rem;
}

.automation-item.disabled {
  opacity: 0.6;
}

.automation-info {
  flex: 1;
}

.automation-name {
  font-weight: 600;
  margin-bottom: 0.25rem;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.automation-description {
  font-size: 0.8rem;
  color: var(--text-muted);
}

.automation-description strong {
  color: var(--text-secondary);
}

.automation-actions {
  display: flex;
  gap: 0.25rem;
}

.create-automation-form {
  background: var(--bg-tertiary);
  padding: 1rem;
  border-radius: 8px;
}

.create-automation-form h4 {
  margin: 0 0 1rem;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.create-automation-form .form-group {
  margin-bottom: 1rem;
}

.create-automation-form label {
  display: block;
  margin-bottom: 0.25rem;
  font-weight: 500;
  font-size: 0.85rem;
  color: var(--text-primary);
}

.create-automation-form input,
.create-automation-form select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.trigger-config,
.action-config {
  margin-top: 0.5rem;
}

/* ========================================
   RESPONSIVE DESIGN - TABLET (max-width: 1024px)
   ======================================== */
@media (max-width: 1024px) {
  .board-content {
    flex: 1;
    min-height: 0;
  }

  .kanban-list {
    min-width: 260px;
    max-width: 260px;
  }

  .board-header {
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0.625rem 1rem;
  }

  .header-left {
    flex: 1 1 100%;
    min-width: 0;
  }

  .header-left h2 {
    font-size: 1rem;
  }

  .board-description {
    display: none;
  }

  .header-actions {
    flex: 1 1 100%;
    justify-content: flex-end;
  }

  .side-panel {
    width: 100%;
    max-width: 400px;
  }

  .side-panel-wide {
    width: 100%;
    max-width: 450px;
  }

  .modal {
    min-width: 90%;
    max-width: 90%;
    margin: 1rem;
  }

  .modal-large {
    max-width: 95%;
  }
}

/* ========================================
   RESPONSIVE DESIGN - SMARTPHONE (max-width: 768px)
   ======================================== */
@media (max-width: 768px) {
  .boards-workspace {
    position: relative;
    height: calc(100vh - 56px);
    height: calc(100dvh - 56px);
  }

  .board-content {
    width: 100%;
    height: 100%;
  }

  .board-header {
    padding: 0.5rem 0.75rem;
    gap: 0.5rem;
  }

  .header-left h2 {
    font-size: 0.9rem;
    white-space: normal;
    line-height: 1.3;
  }

  .header-actions {
    gap: 0.25rem;
  }

  .btn-icon {
    width: 36px;
    height: 36px;
  }

  /* Filters bar mobile */
  .filters-bar {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
  }

  .filter-group {
    width: 100%;
    flex-wrap: wrap;
  }

  .filter-labels {
    width: 100%;
  }

  /* Kanban mobile - horizontal scroll with full-width cards option */
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

  .list-header {
    padding: 0 0.125rem 0.5rem;
  }

  .list-header h3 {
    font-size: 0.8rem;
  }

  .list-menu-btn {
    opacity: 1;
    width: 28px;
    height: 28px;
  }

  .cards-container {
    gap: 0.375rem;
  }

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

  .add-card-btn {
    padding: 0.5rem;
    font-size: 12px;
  }

  .add-list-placeholder {
    min-width: 200px;
    max-width: 200px;
    padding: 1.5rem 1rem;
  }

  /* Side panels full-screen on mobile */
  .side-panel-overlay {
    align-items: flex-end;
  }

  .side-panel,
  .side-panel-wide {
    width: 100%;
    max-width: 100%;
    height: 85vh;
    height: 85dvh;
    border-radius: 16px 16px 0 0;
    animation: slideUpPanel 0.3s ease;
  }

  @keyframes slideUpPanel {
    from {
      transform: translateY(100%);
    }
    to {
      transform: translateY(0);
    }
  }

  .side-panel-header {
    padding: 1rem;
    border-bottom: 1px solid var(--border-primary);
  }

  .side-panel-content {
    padding: 0.75rem;
  }

  /* Labels management mobile */
  .label-item {
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .label-preview {
    flex: 1 1 100%;
  }

  .label-actions {
    margin-left: 0;
    width: 100%;
    justify-content: flex-end;
  }

  /* Archive panel mobile */
  .archived-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .archived-item button {
    align-self: flex-end;
  }

  /* Automations mobile */
  .automation-item {
    flex-direction: column;
    gap: 0.75rem;
  }

  .automation-actions {
    width: 100%;
    justify-content: flex-end;
  }

  .create-automation-form {
    padding: 0.75rem;
  }

  /* Modals mobile */
  .modal-overlay {
    align-items: flex-end;
    padding: 0;
  }

  .modal,
  .modal-large {
    min-width: 100%;
    max-width: 100%;
    max-height: 90vh;
    max-height: 90dvh;
    margin: 0;
    border-radius: 16px 16px 0 0;
    animation: slideUpModal 0.3s ease;
  }

  @keyframes slideUpModal {
    from {
      opacity: 0;
      transform: translateY(100%);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .modal-header {
    position: sticky;
    top: 0;
    background: var(--bg-secondary);
    z-index: 1;
    padding-bottom: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .modal-actions {
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .modal-actions .btn {
    flex: 1 1 auto;
    min-width: 100px;
    text-align: center;
    justify-content: center;
  }

  /* Form groups mobile */
  .form-group input,
  .form-group textarea,
  .form-group select {
    font-size: 16px; /* Prevents zoom on iOS */
  }

  .color-picker {
    gap: 0.625rem;
  }

  .color-option {
    width: 32px;
    height: 32px;
  }

  /* Label selector mobile */
  .label-selector {
    gap: 0.375rem;
  }

  .label-option {
    padding: 0.375rem 0.625rem;
    font-size: 0.8rem;
  }

  /* Linked items mobile */
  .linked-item {
    padding: 0.625rem;
  }

  .linked-item-remove {
    opacity: 1;
    width: 24px;
    height: 24px;
  }

  /* Dropdown menu mobile */
  .dropdown-menu {
    position: fixed;
    left: 1rem !important;
    right: 1rem !important;
    bottom: 1rem;
    top: auto !important;
    margin-top: 0;
    max-width: none;
    border-radius: 12px;
  }

  .dropdown-menu button {
    padding: 0.75rem 1rem;
    font-size: 14px;
  }

  /* Context menu mobile */
  .context-menu {
    position: fixed;
    left: 1rem !important;
    right: 1rem !important;
    bottom: 1rem;
    top: auto !important;
    border-radius: 12px;
  }

  .context-menu-item {
    padding: 0.75rem 1rem;
  }

  /* Empty state mobile */
  .board-empty {
    padding: 1rem;
  }

  .empty-content {
    padding: 2rem 1rem;
  }

  .empty-content h3 {
    font-size: 1.1rem;
  }

  .empty-content p {
    font-size: 13px;
  }
}

/* ========================================
   RESPONSIVE DESIGN - SMALL PHONES (max-width: 480px)
   ======================================== */
@media (max-width: 480px) {
  .kanban-list {
    min-width: 90vw;
    max-width: 90vw;
  }

  .header-actions {
    gap: 0.125rem;
  }

  .btn-icon {
    width: 32px;
    height: 32px;
  }

  .board-header {
    padding: 0.375rem 0.5rem;
  }

  .header-left h2 {
    font-size: 0.85rem;
  }

  .kanban-container {
    padding: 0.5rem;
  }

  .kanban-list {
    padding: 0.5rem;
    max-height: calc(100vh - 200px);
    max-height: calc(100dvh - 200px);
  }

  .kanban-card {
    padding: 0.5rem;
  }

  .kanban-card h4 {
    font-size: 0.75rem;
  }

  .modal {
    padding: 1rem;
  }

  .modal-header h3 {
    font-size: 1rem;
  }

  .side-panel,
  .side-panel-wide {
    height: 90vh;
    height: 90dvh;
  }
}

/* ========================================
   TOUCH DEVICE OPTIMIZATIONS
   ======================================== */
@media (hover: none) and (pointer: coarse) {
  /* Make touch targets larger */
  .btn-icon {
    min-width: 44px;
    min-height: 44px;
  }

  .list-menu-btn {
    min-width: 44px;
    min-height: 44px;
    opacity: 1;
  }

  .kanban-card {
    padding: 0.75rem;
  }

  .tab-close,
  .linked-item-remove {
    opacity: 1;
    min-width: 44px;
    min-height: 44px;
  }

  .color-option {
    min-width: 44px;
    min-height: 44px;
  }

  .label-option,
  .filter-label {
    min-height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  /* Disable hover effects that don't work well on touch */
  .kanban-card:hover {
    border-color: var(--border-primary);
    box-shadow: none;
  }

  .kanban-card:active {
    border-color: var(--accent);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  }

  /* Improve scrolling */
  .cards-container,
  .lists-wrapper,
  .kanban-container {
    -webkit-overflow-scrolling: touch;
  }
}

/* ========================================
   LANDSCAPE PHONE OPTIMIZATIONS
   ======================================== */
@media (max-width: 768px) and (orientation: landscape) {
  .boards-workspace {
    height: calc(100vh - 48px);
    height: calc(100dvh - 48px);
  }

  .kanban-list {
    min-width: 45vw;
    max-width: 45vw;
    max-height: calc(100vh - 140px);
    max-height: calc(100dvh - 140px);
  }

  .modal,
  .modal-large {
    max-height: 85vh;
    max-height: 85dvh;
  }

  .side-panel,
  .side-panel-wide {
    height: 100%;
    width: 60%;
    max-width: 400px;
    border-radius: 0;
    animation: slideInRight 0.3s ease;
  }

  @keyframes slideInRight {
    from {
      transform: translateX(100%);
    }
    to {
      transform: translateX(0);
    }
  }
}
</style>
