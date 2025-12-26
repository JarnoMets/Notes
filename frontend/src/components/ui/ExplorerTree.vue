<template>
  <div class="explorer-tree">
    <ExplorerSection
      v-if="activeTab === 'notes'"
      title="Notes"
      :items="explorerStore.notesTree"
      :favorites="explorerStore.favoritesList.filter(i => i.type === 'note' || i.type === 'folder')"
      :favorites-expanded="explorerStore.favoritesFolderExpanded"
      :section-expanded="true"
      :show-header="false"
      empty-icon="file"
      empty-text="No notes yet"
      empty-action-text="Create your first note"
      :actions="noteActions"
      :allowed-drag-types="['note', 'folder']"
      @select="handleSelect"
      @toggle="handleToggle"
      @dblclick="handleDoubleClick"
      @create-note="handleCreateNote"
      @create-folder="handleCreateFolder"
      @rename="(item) => $emit('rename', item)"
      @delete="(item) => $emit('delete', item)"
      @toggle-importance="handleToggleImportance"
      @toggle-urgent="handleToggleUrgent"
      @move-item="handleMoveItem"
      @refresh="explorerStore.fetchNotesTree()"
      @toggle-favorites="explorerStore.toggleFavoritesFolder()"
      @contextmenu="openRootContextMenu"
      @empty-action="$emit('create-note', null)"
    />

    <ExplorerSection
      v-if="activeTab === 'boards'"
      title="Boards"
      :items="explorerStore.boardsList"
      :favorites="explorerStore.favoritesList.filter(i => i.type === 'board' || i.type === 'board-folder')"
      :favorites-expanded="explorerStore.favoritesFolderExpanded"
      :section-expanded="boardsSectionExpanded"
      :show-header="showBoardsHeader"
      empty-icon="board"
      empty-text="No boards yet"
      empty-action-text="Create your first board"
      :actions="boardActions"
      :allowed-drag-types="['board', 'board-folder']"
      @select="handleBoardItemSelect"
      @toggle="handleBoardToggle"
      @dblclick="handleBoardItemDoubleClick"
      @create-folder="handleCreateBoardFolder"
      @rename="(item) => $emit('rename', item)"
      @delete="(item) => $emit('delete', item)"
      @toggle-importance="handleToggleBoardImportance"
      @toggle-urgent="handleToggleBoardUrgent"
      @move-item="handleMoveItem"
      @refresh="explorerStore.fetchBoardsTree()"
      @toggle-favorites="explorerStore.toggleFavoritesFolder()"
      @toggle-section="toggleBoardsSection"
      @contextmenu="openRootContextMenu"
      @empty-action="$emit('create-board', null)"
    />

    <ExplorerSection
      v-if="activeTab === 'graphs'"
      title="Graphs"
      :items="explorerStore.graphsList"
      :favorites="explorerStore.favoritesList.filter(i => i.type === 'graph' || i.type === 'graph-folder')"
      :favorites-expanded="explorerStore.favoritesFolderExpanded"
      :section-expanded="graphsSectionExpanded"
      :show-header="true"
      empty-icon="share-2"
      empty-text="No graphs yet"
      empty-action-text="Create your first graph"
      :actions="graphActions"
      :allowed-drag-types="['graph', 'graph-folder']"
      @select="handleGraphItemSelect"
      @toggle="handleGraphToggle"
      @dblclick="handleGraphItemDoubleClick"
      @create-folder="handleCreateGraphFolder"
      @rename="(item) => $emit('rename', item)"
      @delete="(item) => $emit('delete', item)"
      @toggle-importance="handleToggleGraphImportance"
      @toggle-urgent="handleToggleGraphUrgent"
      @move-item="handleMoveItem"
      @refresh="explorerStore.fetchGraphsTree()"
      @toggle-favorites="explorerStore.toggleFavoritesFolder()"
      @toggle-section="toggleGraphsSection"
      @contextmenu="openRootContextMenu"
      @empty-action="$emit('create-graph', null)"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useExplorerStore, type ExplorerItem } from '@/stores/explorer'
import ExplorerSection from './ExplorerSection.vue'
import logger from '@/utils/logger'
import { openFloatingMenu } from '@/utils/floatingMenu'

const props = defineProps<{
  defaultTab?: 'notes' | 'boards' | 'graphs',
  showBoardsHeader?: boolean
}>()

// If parent doesn't specify, show the boards header by default
const showBoardsHeader = props.showBoardsHeader === undefined ? true : props.showBoardsHeader

const explorerStore = useExplorerStore()

// Section states
const BOARDS_SECTION_KEY = 'boardsSectionExpanded'
const boardsSectionExpanded = ref<boolean>(true)
const GRAPHS_SECTION_KEY = 'graphsSectionExpanded'
const graphsSectionExpanded = ref<boolean>(true)

function loadSectionStates() {
  try {
    const b = localStorage.getItem(BOARDS_SECTION_KEY)
    if (b !== null) boardsSectionExpanded.value = b === '1' || b === 'true'
    
    const g = localStorage.getItem(GRAPHS_SECTION_KEY)
    if (g !== null) graphsSectionExpanded.value = g === '1' || g === 'true'
  } catch (e) {
    // ignore
  }
}

function toggleBoardsSection() {
  boardsSectionExpanded.value = !boardsSectionExpanded.value
  localStorage.setItem(BOARDS_SECTION_KEY, boardsSectionExpanded.value ? '1' : '0')
}

function toggleGraphsSection() {
  graphsSectionExpanded.value = !graphsSectionExpanded.value
  localStorage.setItem(GRAPHS_SECTION_KEY, graphsSectionExpanded.value ? '1' : '0')
}

loadSectionStates()

const activeTab = computed(() => props.defaultTab || 'notes')

const emit = defineEmits<{
  'select': [item: ExplorerItem]
  'create-note': [folderId: string | null]
  'create-folder': [parentId: string | null]
  'create-board': [folderId: string | null]
  'create-board-folder': [parentId: string | null]
  'create-graph': [folderId: string | null]
  'create-graph-folder': [parentId: string | null]
  'rename': [item: ExplorerItem]
  'delete': [item: ExplorerItem]
  'open-note': [noteId: string]
  'open-board': [boardId: string]
  'open-graph': [graphId: string]
}>()

const noteActions = [
  { title: 'New Note', icon: 'plus', handler: handleCreateNote },
  { title: 'New Folder', icon: 'folder', handler: handleCreateFolder }
]

const boardActions = [
  { title: 'New Board', icon: 'plus', handler: handleCreateBoard },
  { title: 'New Folder', icon: 'folder', handler: handleCreateBoardFolder }
]

const graphActions = [
  { title: 'New Graph', icon: 'plus', handler: handleCreateGraph },
  { title: 'New Folder', icon: 'folder', handler: handleCreateGraphFolder }
]

// Close any floating context menus created by direct DOM manipulation
function openRootContextMenu(event: MouseEvent) {
  try {
    const items: Array<import('@/utils/floatingMenu').FloatingMenuItem> = []
    if (activeTab.value === 'notes') {
      items.push({ label: 'New Note', action: () => emit('create-note', getTargetFolderId()) })
      items.push({ label: 'New Folder', action: () => emit('create-folder', getTargetFolderId()) })
      items.push({ divider: true })
      items.push({ label: 'Refresh', action: () => explorerStore.fetchNotesTree() })
    } else if (activeTab.value === 'boards') {
      items.push({ label: 'New Board', action: () => emit('create-board', getTargetBoardFolderId()) })
      items.push({ label: 'New Folder', action: () => emit('create-board-folder', getTargetBoardFolderId()) })
      items.push({ divider: true })
      items.push({ label: 'Refresh', action: () => explorerStore.fetchBoardsTree() })
    } else {
      items.push({ label: 'New Graph', action: () => emit('create-graph', getTargetGraphFolderId()) })
      items.push({ label: 'New Folder', action: () => emit('create-graph-folder', getTargetGraphFolderId()) })
      items.push({ divider: true })
      items.push({ label: 'Refresh', action: () => explorerStore.fetchGraphsTree() })
    }

    openFloatingMenu(items, event.clientX, event.clientY)
  } catch (e) {
    logger.error('openRootContextMenu failed', e)
  }
}

function handleSelect(item: ExplorerItem) {
  explorerStore.selectItem(item.id, item.type as 'note' | 'folder' | 'board' | 'board-folder')
  emit('select', item)
}

// Get the target folder for creating new items
function getTargetFolderId(): string | null {
  if (!explorerStore.selectedItemId || !explorerStore.selectedItemType) {
    return null
  }
  
  // If a folder is selected, create inside it
  if (explorerStore.selectedItemType === 'folder') {
    return explorerStore.selectedItemId
  }
  
  // If a note is selected, create in the same folder as the note
  if (explorerStore.selectedItemType === 'note') {
    const note = explorerStore.notes.find(n => n.id === explorerStore.selectedItemId)
    return note?.folder_id || null
  }
  
  return null
}

function handleCreateNote() {
  const folderId = getTargetFolderId()
  emit('create-note', folderId)
}

function handleCreateFolder() {
  const parentId = getTargetFolderId()
  emit('create-folder', parentId)
}

// Get target folder for creating new boards
function getTargetBoardFolderId(): string | null {
  if (!explorerStore.selectedItemId || !explorerStore.selectedItemType) {
    return null
  }
  
  // If a board folder is selected, create inside it
  if (explorerStore.selectedItemType === 'board-folder') {
    return explorerStore.selectedItemId
  }
  
  // If a board is selected, create in the same folder as the board
  if (explorerStore.selectedItemType === 'board') {
    const board = explorerStore.boards.find(b => b.id === explorerStore.selectedItemId)
    return board?.folder_id || null
  }
  
  return null
}

function handleCreateBoard() {
  const folderId = getTargetBoardFolderId()
  emit('create-board', folderId)
}

function handleCreateBoardFolder() {
  const parentId = getTargetBoardFolderId()
  emit('create-board-folder', parentId)
}

// Get target folder for creating new graphs
function getTargetGraphFolderId(): string | null {
  if (!explorerStore.selectedItemId || !explorerStore.selectedItemType) {
    return null
  }
  
  // If a graph folder is selected, create inside it
  if (explorerStore.selectedItemType === 'graph-folder') {
    return explorerStore.selectedItemId
  }
  
  // If a graph is selected, create in the same folder as the graph
  if (explorerStore.selectedItemType === 'graph') {
    const graph = explorerStore.graphs.find(g => g.id === explorerStore.selectedItemId)
    return graph?.folder_id || null
  }
  
  return null
}

function handleCreateGraph() {
  const folderId = getTargetGraphFolderId()
  emit('create-graph', folderId)
}

function handleCreateGraphFolder() {
  const parentId = getTargetGraphFolderId()
  emit('create-graph-folder', parentId)
}

function handleToggle(item: ExplorerItem) {
  if (item.type === 'folder') {
    explorerStore.toggleFolder(item.id)
  }
}

function handleDoubleClick(item: ExplorerItem) {
  if (item.type === 'note') {
    emit('open-note', item.id)
  } else if (item.type === 'folder') {
    explorerStore.toggleFolder(item.id)
  }
}

async function handleToggleImportance(item: ExplorerItem) {
  try {
    if (item.type === 'note') {
      await explorerStore.toggleNoteImportance(item.id)
    } else if (item.type === 'folder') {
      await explorerStore.toggleFolderImportance(item.id)
    }
  } catch (error) {
    logger.error('Failed to toggle importance:', error)
  }
}

async function handleToggleUrgent(item: ExplorerItem) {
  try {
    if (item.type === 'folder') {
      await explorerStore.toggleFolderUrgent(item.id)
    } else if (item.type === 'note') {
      await explorerStore.toggleNoteUrgent(item.id)
    }
  } catch (error) {
    logger.error('Failed to toggle urgent:', error)
  }
}

function handleGraphItemSelect(item: ExplorerItem) {
  explorerStore.selectItem(item.id, item.type as 'graph' | 'graph-folder')
  emit('select', item)
}

function handleGraphToggle(item: ExplorerItem) {
  if (item.type === 'graph-folder') {
    explorerStore.toggleGraphFolder(item.id)
  }
}

function handleGraphItemDoubleClick(item: ExplorerItem) {
  if (item.type === 'graph') {
    emit('open-graph', item.id)
  } else if (item.type === 'graph-folder') {
    explorerStore.toggleGraphFolder(item.id)
  }
}

function handleBoardDoubleClick(board: ExplorerItem) {
  emit('open-board', board.id)
}

function handleBoardItemSelect(item: ExplorerItem) {
  explorerStore.selectItem(item.id, item.type as 'board' | 'board-folder')
  emit('select', item)
}

function handleBoardToggle(item: ExplorerItem) {
  if (item.type === 'board-folder') {
    explorerStore.toggleBoardFolder(item.id)
  }
}

function handleBoardItemDoubleClick(item: ExplorerItem) {
  if (item.type === 'board') {
    handleBoardDoubleClick(item)
  } else if (item.type === 'board-folder') {
    explorerStore.toggleBoardFolder(item.id)
  }
}

async function handleToggleBoardImportance(item: ExplorerItem) {
  try {
    if (item.type === 'board') {
      await explorerStore.toggleBoardImportance(item.id)
    } else if (item.type === 'board-folder') {
      await explorerStore.toggleBoardFolderImportance(item.id)
    }
  } catch (error) {
    logger.error('Failed to toggle board importance:', error)
  }
}

async function handleToggleBoardUrgent(item: ExplorerItem) {
  try {
    if (item.type === 'board-folder') {
      await explorerStore.toggleBoardFolderUrgent(item.id)
    } else if (item.type === 'board') {
      await explorerStore.toggleBoardUrgent(item.id)
    }
  } catch (error) {
    logger.error('Failed to toggle board urgent:', error)
  }
}

async function handleToggleGraphImportance(item: ExplorerItem) {
  try {
    if (item.type === 'graph') {
      await explorerStore.toggleGraphImportance(item.id)
    } else if (item.type === 'graph-folder') {
      await explorerStore.toggleGraphFolderImportance(item.id)
    }
  } catch (error) {
    logger.error('Failed to toggle graph importance:', error)
  }
}

async function handleToggleGraphUrgent(item: ExplorerItem) {
  try {
    if (item.type === 'graph-folder') {
      await explorerStore.toggleGraphFolderUrgent(item.id)
    } else if (item.type === 'graph') {
      await explorerStore.toggleGraphUrgent(item.id)
    }
  } catch (error) {
    logger.error('Failed to toggle graph urgent:', error)
  }
}

// Move item function to support drag and drop moving of items (notes, boards, graphs) and folders
async function handleMoveItem({ itemId, itemType, targetFolderId, insertBeforeId: _insertBeforeId }: { itemId: string, itemType: string, targetFolderId: string | null, insertBeforeId: string | null }) {
  try {
    // Simple move implementation - appending to end (using timestamp as position)
    // Real reordering would require calculating position based on siblings
    const position = Date.now() 
    
    if (itemType === 'note' || itemType === 'folder') {
      if (itemType === 'note') {
        await explorerStore.moveNote(itemId, targetFolderId, position)
      } else {
        await explorerStore.moveFolder(itemId, targetFolderId, position)
      }
    } else if (itemType === 'board' || itemType === 'board-folder') {
      if (itemType === 'board') {
        await explorerStore.moveBoard(itemId, targetFolderId, position)
      } else {
        await explorerStore.moveBoardFolder(itemId, targetFolderId, position)
      }
    } else if (itemType === 'graph' || itemType === 'graph-folder') {
      if (itemType === 'graph') {
        await explorerStore.moveGraph(itemId, targetFolderId, position)
      } else {
        await explorerStore.moveGraphFolder(itemId, targetFolderId, position)
      }
    }
  } catch (error) {
    logger.error('Failed to move item:', error)
  }
}
</script>

<style scoped>
.explorer-tree {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: var(--bg-secondary);
}
</style>


