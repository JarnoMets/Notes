<template>
  <div class="explorer-tree">
    <!-- Tab Bar -->
    <div class="explorer-tabs">
      <button 
        class="explorer-tab"
        :class="{ active: activeTab === 'notes' }"
        @click="activeTab = 'notes'"
      >
        <Icon name="file" :size="14" />
        <span>Notes</span>
      </button>
      <button 
        class="explorer-tab"
        :class="{ active: activeTab === 'boards' }"
        @click="activeTab = 'boards'"
      >
        <Icon name="board" :size="14" />
        <span>Boards</span>
      </button>
    </div>

    <!-- Notes Content -->
    <div v-if="activeTab === 'notes'" class="explorer-content">
      <div class="content-header">
        <div class="content-actions">
          <button @click="handleCreateNote" title="New Note">
            <Icon name="plus" :size="12" />
            <span>Note</span>
          </button>
          <button @click="handleCreateFolder" title="New Folder">
            <Icon name="folder" :size="12" />
            <span>Folder</span>
          </button>
        </div>
      </div>
      
      <div 
        class="tree-content"
        @dragover.prevent="onNotesRootDragOver"
        @dragleave="onNotesRootDragLeave"
        @drop.prevent="onNotesRootDrop"
        @contextmenu.prevent="openRootContextMenu($event)"
        :class="{ 'root-drop-active': isNotesRootDropTarget }"
      >
        <!-- Favorites Section -->
        <div v-if="explorerStore.favoritesList.length > 0" class="favorites-section">
          <div 
            class="favorites-header"
            @click="explorerStore.toggleFavoritesFolder()"
          >
            <span class="expand-toggle">
              <Icon :name="explorerStore.favoritesFolderExpanded ? 'chevron-down' : 'chevron-right'" :size="12" />
            </span>
            <Icon name="star" :size="14" class="favorites-icon" :fill="true" />
            <span class="favorites-title">Favorites</span>
            <span class="favorites-count">{{ explorerStore.favoritesList.length }}</span>
          </div>
          <div v-if="explorerStore.favoritesFolderExpanded" class="favorites-items">
            <ExplorerTreeNode
              v-for="item in explorerStore.favoritesList"
              :key="'fav-' + item.id"
              :item="item"
              :depth="1"
              :in-favorites="true"
              @select="handleSelect"
              @toggle="handleToggle"
              @dblclick="handleDoubleClick"
              @create-note="(folderId: string | null) => $emit('create-note', folderId)"
              @create-folder="(parentId: string | null) => $emit('create-folder', parentId)"
              @rename="(item: ExplorerItem) => $emit('rename', item)"
              @delete="(item: ExplorerItem) => $emit('delete', item)"
              @toggle-importance="handleToggleImportance"
              @toggle-urgent="handleToggleUrgent"
              @move-item="handleMoveItem"
            />
          </div>
        </div>
        
        <template v-if="explorerStore.notesTree.length > 0">
          <ExplorerTreeNode
            v-for="item in explorerStore.notesTree"
            :key="item.id"
            :item="item"
            :depth="0"
            @select="handleSelect"
            @toggle="handleToggle"
            @dblclick="handleDoubleClick"
            @create-note="(folderId: string | null) => $emit('create-note', folderId)"
            @create-folder="(parentId: string | null) => $emit('create-folder', parentId)"
            @rename="(item: ExplorerItem) => $emit('rename', item)"
            @delete="(item: ExplorerItem) => $emit('delete', item)"
            @toggle-importance="handleToggleImportance"
            @toggle-urgent="handleToggleUrgent"
            @move-item="handleMoveItem"
          />
          <!-- Root drop zone at the bottom -->
          <div 
            v-if="isNotesRootDropTarget" 
            class="root-drop-indicator"
          >
            Drop here to move to root
          </div>
        </template>
        <div v-else class="tree-empty">
          <Icon name="file" :size="32" />
          <span>No notes yet</span>
          <button class="btn btn-primary btn-sm" @click="$emit('create-note', null)">
            Create your first note
          </button>
        </div>
      </div>
    </div>

    <!-- Boards Content -->
    <div v-if="activeTab === 'boards'" class="explorer-content">
      <div class="content-header">
        <div class="content-actions">
          <button @click="handleCreateBoard" title="New Board">
            <Icon name="plus" :size="12" />
            <span>Board</span>
          </button>
          <button @click="handleCreateBoardFolder" title="New Folder">
            <Icon name="folder" :size="12" />
            <span>Folder</span>
          </button>
        </div>
      </div>
      
      <div 
        class="tree-content"
        @dragover.prevent="onBoardsRootDragOver"
        @dragleave="onBoardsRootDragLeave"
        @drop.prevent="onBoardsRootDrop"
        @contextmenu.prevent="openRootContextMenu($event)"
        :class="{ 'root-drop-active': isBoardsRootDropTarget }"
      >
        <!-- Boards: render favorites and boards list directly (no collapsible parent) -->
        <!-- Favorites Section (Boards) -->
        <div v-if="explorerStore.favoritesList.some(i => i.type === 'board' || i.type === 'board-folder')" class="favorites-section">
          <div 
            class="favorites-header"
            @click="explorerStore.toggleFavoritesFolder()"
          >
            <span class="expand-toggle">
              <Icon :name="explorerStore.favoritesFolderExpanded ? 'chevron-down' : 'chevron-right'" :size="12" />
            </span>
            <Icon name="star" :size="14" class="favorites-icon" :fill="true" />
            <span class="favorites-title">Favorites</span>
            <span class="favorites-count">{{ explorerStore.favoritesList.filter(i => i.type === 'board' || i.type === 'board-folder').length }}</span>
          </div>
          <div v-if="explorerStore.favoritesFolderExpanded" class="favorites-items">
            <ExplorerTreeNode
              v-for="item in explorerStore.favoritesList.filter(i => i.type === 'board' || i.type === 'board-folder')"
              :key="'fav-board-' + item.id"
              :item="item"
              :depth="1"
              :in-favorites="true"
              @select="handleBoardItemSelect"
              @toggle="handleBoardToggle"
              @dblclick="handleBoardItemDoubleClick"
              @create-folder="(parentId: string | null) => $emit('create-board-folder', parentId)"
              @rename="(item: ExplorerItem) => $emit('rename', item)"
              @delete="(item: ExplorerItem) => $emit('delete', item)"
              @toggle-importance="handleToggleBoardImportance"
              @toggle-urgent="handleToggleBoardUrgent"
              @move-item="handleMoveItem"
            />
          </div>
        </div>

            <div class="boards-section">
              <template v-if="showBoardsHeader">
                <div class="boards-header" @click="toggleBoardsSection">
                  <span class="expand-toggle">
                    <Icon :name="boardsSectionExpanded ? 'chevron-down' : 'chevron-right'" :size="12" />
                  </span>
                  <Icon name="board" :size="14" />
                  <span class="boards-title">Boards</span>
                  <span class="boards-count">{{ explorerStore.boardsList.length }}</span>
                </div>

                <div v-if="boardsSectionExpanded">
                  <template v-if="explorerStore.boardsList.length > 0">
                    <ExplorerTreeNode
                      v-for="item in explorerStore.boardsList"
                      :key="item.id"
                      :item="item"
                      :depth="0"
                      @select="handleBoardItemSelect"
                      @toggle="handleBoardToggle"
                      @dblclick="handleBoardItemDoubleClick"
                      @create-note="() => {}"
                      @create-folder="(parentId: string | null) => $emit('create-board-folder', parentId)"
                      @rename="(item: ExplorerItem) => $emit('rename', item)"
                      @delete="(item: ExplorerItem) => $emit('delete', item)"
                      @toggle-importance="handleToggleBoardImportance"
                      @toggle-urgent="handleToggleBoardUrgent"
                      @move-item="handleMoveItem"
                    />
                    <!-- Root drop zone at the bottom -->
                    <div 
                      v-if="isBoardsRootDropTarget" 
                      class="root-drop-indicator"
                    >
                      Drop here to move to root
                    </div>
                  </template>
                  <div v-else class="tree-empty">
                    <Icon name="board" :size="32" />
                    <span>No boards yet</span>
                    <button class="btn btn-primary btn-sm" @click="$emit('create-board', null)">
                      Create your first board
                    </button>
                  </div>
                </div>
              </template>

              <!-- If parent requests no header, render boards list directly -->
              <template v-else>
                <template v-if="explorerStore.boardsList.length > 0">
                  <ExplorerTreeNode
                    v-for="item in explorerStore.boardsList"
                    :key="item.id"
                    :item="item"
                    :depth="0"
                    @select="handleBoardItemSelect"
                    @toggle="handleBoardToggle"
                    @dblclick="handleBoardItemDoubleClick"
                    @create-note="() => {}"
                    @create-folder="(parentId: string | null) => $emit('create-board-folder', parentId)"
                    @rename="(item: ExplorerItem) => $emit('rename', item)"
                    @delete="(item: ExplorerItem) => $emit('delete', item)"
                    @toggle-importance="handleToggleBoardImportance"
                    @toggle-urgent="handleToggleBoardUrgent"
                    @move-item="handleMoveItem"
                  />
                  <div v-if="isBoardsRootDropTarget" class="root-drop-indicator">Drop here to move to root</div>
                </template>
                <div v-else class="tree-empty">
                  <Icon name="board" :size="32" />
                  <span>No boards yet</span>
                  <button class="btn btn-primary btn-sm" @click="$emit('create-board', null)">Create your first board</button>
                </div>
              </template>
            </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import { useExplorerStore, type ExplorerItem } from '@/stores/explorer'
import { useRouter, useRoute } from 'vue-router'
import ExplorerTreeNode from './ExplorerTreeNode.vue'
import Icon from './Icon.vue'
import logger from '@/utils/logger'
import { openFloatingMenu } from '@/utils/floatingMenu'

const props = defineProps<{
  defaultTab?: 'notes' | 'boards',
  showBoardsHeader?: boolean
}>()

// If parent doesn't specify, show the boards header by default
const showBoardsHeader = props.showBoardsHeader === undefined ? true : props.showBoardsHeader

const explorerStore = useExplorerStore()
const router = useRouter()
const route = useRoute()

// Boards section collapsed state (persist in localStorage so user preference survives reload)
const BOARDS_SECTION_KEY = 'boardsSectionExpanded'
const boardsSectionExpanded = ref<boolean>(true)

function loadBoardsSectionState() {
  try {
    const v = localStorage.getItem(BOARDS_SECTION_KEY)
    if (v === null) {
      boardsSectionExpanded.value = true
    } else {
      boardsSectionExpanded.value = v === '1' || v === 'true'
    }
  } catch (e) {
    boardsSectionExpanded.value = true
  }
}

function saveBoardsSectionState() {
  try {
    localStorage.setItem(BOARDS_SECTION_KEY, boardsSectionExpanded.value ? '1' : '0')
  } catch (e) {
    // ignore
  }
}

function toggleBoardsSection() {
  boardsSectionExpanded.value = !boardsSectionExpanded.value
  saveBoardsSectionState()
}

loadBoardsSectionState()

// Boards are rendered directly (no separate expanded state persisted)

const activeTab = ref<'notes' | 'boards'>(props.defaultTab || 'notes')

// Root drop target states
const isNotesRootDropTarget = ref(false)
const isBoardsRootDropTarget = ref(false)
let notesRootDragCounter = 0
let boardsRootDragCounter = 0

// Sync tab with current route
function syncTabWithRoute() {
  if (route.path === '/boards') {
    activeTab.value = 'boards'
  } else if (route.path === '/notes') {
    activeTab.value = 'notes'
  }
}

// Watch for route changes
watch(() => route.path, syncTabWithRoute)

// Set initial tab based on route
onMounted(() => {
  syncTabWithRoute()
  
  // Global cleanup handler for when drag ends anywhere
  document.addEventListener('dragend', handleGlobalDragEnd)
})

onUnmounted(() => {
  document.removeEventListener('dragend', handleGlobalDragEnd)
})

// Global drag end handler to clean up any stuck indicators
function handleGlobalDragEnd() {
  isNotesRootDropTarget.value = false
  isBoardsRootDropTarget.value = false
  notesRootDragCounter = 0
  boardsRootDragCounter = 0
}

const emit = defineEmits<{
  'select': [item: ExplorerItem]
  'create-note': [folderId: string | null]
  'create-folder': [parentId: string | null]
  'create-board': [folderId: string | null]
  'create-board-folder': [parentId: string | null]
  'rename': [item: ExplorerItem]
  'delete': [item: ExplorerItem]
  'open-note': [noteId: string]
  'open-board': [boardId: string]
}>()

// Close any floating context menus created by direct DOM manipulation
function openRootContextMenu(event: MouseEvent) {
  try {
    const items: Array<import('@/utils/floatingMenu').FloatingMenuItem> = []
    if (activeTab.value === 'notes') {
      items.push({ label: 'New Note', action: () => emit('create-note', getTargetFolderId()) })
      items.push({ label: 'New Folder', action: () => emit('create-folder', getTargetFolderId()) })
      items.push({ divider: true })
      items.push({ label: 'Refresh', action: () => explorerStore.fetchNotesTree() })
    } else {
      items.push({ label: 'New Board', action: () => emit('create-board', getTargetBoardFolderId()) })
      items.push({ label: 'New Folder', action: () => emit('create-board-folder', getTargetBoardFolderId()) })
      items.push({ divider: true })
      items.push({ label: 'Refresh', action: () => explorerStore.fetchBoardsTree() })
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

function handleToggle(item: ExplorerItem) {
  if (item.type === 'folder') {
    explorerStore.toggleFolder(item.id)
  }
}

function handleDoubleClick(item: ExplorerItem) {
  if (item.type === 'note') {
    // If we're on boards page, navigate to notes
    if (router.currentRoute.value.path === '/boards') {
      router.push('/notes')
    }
    emit('open-note', item.id)
  } else if (item.type === 'folder') {
    explorerStore.toggleFolder(item.id)
  }
}

function handleBoardDoubleClick(board: ExplorerItem) {
  // Navigate to boards page if not already there
  if (router.currentRoute.value.path !== '/boards') {
    router.push('/boards')
  }
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

interface MoveItemData {
  itemId: string
  itemType: 'note' | 'folder' | 'board' | 'board-folder'
  targetFolderId: string | null
  insertBeforeId: string | null
}

async function handleMoveItem(data: MoveItemData) {
  try {
    const { itemId, itemType, targetFolderId } = data
    let { insertBeforeId } = data
    
    // Handle "after" marker - find the next sibling
    let insertAfterItemId: string | null = null
    if (insertBeforeId && insertBeforeId.startsWith('__AFTER__')) {
      insertAfterItemId = insertBeforeId.slice(9) // Remove '__AFTER__' prefix
      insertBeforeId = null // Will be calculated below
    }
    
    // Calculate the position based on insertBeforeId
    let position: number
    
    if (itemType === 'note' || itemType === 'folder') {
      // Get all items at the target level (folders first, then notes)
      const foldersAtLevel = explorerStore.folders
        .filter(f => f.parent_id === targetFolderId)
        .sort((a, b) => a.position - b.position)
      const notesAtLevel = explorerStore.notes
        .filter(n => n.folder_id === targetFolderId)
        .sort((a, b) => a.position - b.position)
      
      // Build ordered list of all items at this level
      const allItems = [
        ...foldersAtLevel.map(f => ({ id: f.id, position: f.position, type: 'folder' as const })),
        ...notesAtLevel.map(n => ({ id: n.id, position: n.position, type: 'note' as const }))
      ].sort((a, b) => a.position - b.position)
      
      if (insertAfterItemId) {
        // Find the item we want to insert after
        const afterIndex = allItems.findIndex(item => item.id === insertAfterItemId)
        if (afterIndex !== -1 && afterIndex + 1 < allItems.length) {
          // There's a next item - insert before it
          position = allItems[afterIndex + 1].position
        } else {
          // No next item - append at end
          position = allItems.length > 0 ? allItems[allItems.length - 1].position + 1 : 0
        }
      } else if (insertBeforeId === null) {
        // Append at end - get max position + 1
        const allPositions = allItems.map(item => item.position)
        position = allPositions.length > 0 ? Math.max(...allPositions) + 1 : 0
      } else {
        // Find the item we're inserting before
        const beforeItem = allItems.find(item => item.id === insertBeforeId)
        if (beforeItem) {
          position = beforeItem.position
        } else {
          position = 0
        }
      }
      
      if (itemType === 'note') {
        await explorerStore.moveNote(itemId, targetFolderId, position)
      } else {
        await explorerStore.moveFolder(itemId, targetFolderId, position)
      }
    } else {
      // Board types
      const foldersAtLevel = explorerStore.boardFolders
        .filter(f => f.parent_id === targetFolderId)
        .sort((a, b) => a.position - b.position)
      const boardsAtLevel = explorerStore.boards
        .filter(b => (b.folder_id ?? null) === targetFolderId)
        .sort((a, b) => (a.position ?? 0) - (b.position ?? 0))
      
      // Build ordered list of all items at this level
      const allItems = [
        ...foldersAtLevel.map(f => ({ id: f.id, position: f.position, type: 'board-folder' as const })),
        ...boardsAtLevel.map(b => ({ id: b.id, position: b.position ?? 0, type: 'board' as const }))
      ].sort((a, b) => a.position - b.position)
      
      if (insertAfterItemId) {
        // Find the item we want to insert after
        const afterIndex = allItems.findIndex(item => item.id === insertAfterItemId)
        if (afterIndex !== -1 && afterIndex + 1 < allItems.length) {
          // There's a next item - insert before it
          position = allItems[afterIndex + 1].position
        } else {
          // No next item - append at end
          position = allItems.length > 0 ? allItems[allItems.length - 1].position + 1 : 0
        }
      } else if (insertBeforeId === null) {
        const allPositions = allItems.map(item => item.position)
        position = allPositions.length > 0 ? Math.max(...allPositions) + 1 : 0
      } else {
        const beforeItem = allItems.find(item => item.id === insertBeforeId)
        if (beforeItem) {
          position = beforeItem.position
        } else {
          position = 0
        }
      }
      
      if (itemType === 'board') {
        await explorerStore.moveBoard(itemId, targetFolderId, position)
      } else {
        await explorerStore.moveBoardFolder(itemId, targetFolderId, position)
      }
    }
    
    // Refresh to get updated positions
    if (itemType === 'note' || itemType === 'folder') {
      await explorerStore.fetchNotesTree()
    } else {
      await explorerStore.fetchBoardsTree()
    }
  } catch (error) {
    logger.error('Failed to move item:', error)
  }
}

async function handleToggleImportance(item: ExplorerItem) {
  try {
    if (item.type === 'folder') {
      await explorerStore.toggleFolderImportance(item.id)
    } else if (item.type === 'note') {
      await explorerStore.toggleNoteImportance(item.id)
    } else if (item.type === 'board') {
      await explorerStore.toggleBoardImportance(item.id)
    } else if (item.type === 'board-folder') {
      await explorerStore.toggleBoardFolderImportance(item.id)
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
    } else if (item.type === 'board') {
      await explorerStore.toggleBoardUrgent(item.id)
    } else if (item.type === 'board-folder') {
      await explorerStore.toggleBoardFolderUrgent(item.id)
    }
  } catch (error) {
    logger.error('Failed to toggle urgent:', error)
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

// Notes root drop zone handlers
function onNotesRootDragOver(event: DragEvent) {
  const hasExplorerType = event.dataTransfer?.types?.includes?.('application/x-explorer-item') || false
  if (!hasExplorerType) return
  
  // Check if dragging near the bottom of the tree (root area)
  const target = event.currentTarget as HTMLElement
  const rect = target.getBoundingClientRect()
  const y = event.clientY - rect.top
  const scrollHeight = target.scrollHeight
  
  // Only show root drop zone when dragging near bottom or in empty space
  if (y > scrollHeight - 40 || explorerStore.notesTree.length === 0) {
    notesRootDragCounter++
    isNotesRootDropTarget.value = true
    event.dataTransfer!.dropEffect = 'move'
  }
}

function onNotesRootDragLeave(event: DragEvent) {
  // Check if we're leaving to a child element
  const relatedTarget = event.relatedTarget as Node | null
  const currentTarget = event.currentTarget as HTMLElement
  
  if (relatedTarget && currentTarget.contains(relatedTarget)) {
    return // Moving to a child element, don't reset
  }
  
  notesRootDragCounter = Math.max(0, notesRootDragCounter - 1)
  if (notesRootDragCounter === 0) {
    isNotesRootDropTarget.value = false
  }
}

function onNotesRootDrop(event: DragEvent) {
  if (!isNotesRootDropTarget.value) return
  isNotesRootDropTarget.value = false
  notesRootDragCounter = 0
  
  const data = event.dataTransfer?.getData('application/x-explorer-item')
  if (!data) return
  
  try {
    const { id, type } = JSON.parse(data)
    
    // Only allow notes and folders
    if (type !== 'note' && type !== 'folder') return
    
    // Move to root with insertBeforeId = null (append at end)
    handleMoveItem({
      itemId: id,
      itemType: type,
      targetFolderId: null,
      insertBeforeId: null
    })
  } catch (e) {
    logger.error('Failed to parse drag data:', e)
  }
}

// Boards root drop zone handlers
function onBoardsRootDragOver(event: DragEvent) {
  const hasExplorerType = event.dataTransfer?.types?.includes?.('application/x-explorer-item') || false
  if (!hasExplorerType) return
  
  const target = event.currentTarget as HTMLElement
  const rect = target.getBoundingClientRect()
  const y = event.clientY - rect.top
  const scrollHeight = target.scrollHeight
  
  if (y > scrollHeight - 40 || explorerStore.boardsList.length === 0) {
    boardsRootDragCounter++
    isBoardsRootDropTarget.value = true
    event.dataTransfer!.dropEffect = 'move'
  }
}

function onBoardsRootDragLeave(event: DragEvent) {
  // Check if we're leaving to a child element
  const relatedTarget = event.relatedTarget as Node | null
  const currentTarget = event.currentTarget as HTMLElement
  
  if (relatedTarget && currentTarget.contains(relatedTarget)) {
    return // Moving to a child element, don't reset
  }
  
  boardsRootDragCounter = Math.max(0, boardsRootDragCounter - 1)
  if (boardsRootDragCounter === 0) {
    isBoardsRootDropTarget.value = false
  }
}

function onBoardsRootDrop(event: DragEvent) {
  if (!isBoardsRootDropTarget.value) return
  isBoardsRootDropTarget.value = false
  boardsRootDragCounter = 0
  
  const data = event.dataTransfer?.getData('application/x-explorer-item')
  if (!data) return
  
  try {
    const { id, type } = JSON.parse(data)
    
    // Only allow boards and board-folders
    if (type !== 'board' && type !== 'board-folder') return
    
    // Move to root with insertBeforeId = null (append at end)
    handleMoveItem({
      itemId: id,
      itemType: type,
      targetFolderId: null,
      insertBeforeId: null
    })
  } catch (e) {
    logger.error('Failed to parse drag data:', e)
  }
}
</script>

<style scoped>
.explorer-tree {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

/* Tab Bar */
.explorer-tabs {
  display: flex;
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.explorer-tab {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.375rem;
  padding: 0.625rem 0.75rem;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--text-muted);
  font-size: 0.8125rem;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
}

.explorer-tab:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.explorer-tab.active {
  color: var(--accent);
  border-bottom-color: var(--accent);
  background: var(--bg-tertiary);
}

/* Content Area */
.explorer-content {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
}

.content-header {
  padding: 0.5rem 0.75rem;
  border-bottom: 1px solid var(--border-primary);
}

.content-actions {
  display: flex;
  gap: 0.375rem;
}

.content-actions button {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  color: var(--text-secondary);
  font-size: 0.75rem;
  cursor: pointer;
  transition: all 0.15s;
}

.content-actions button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
  border-color: var(--accent);
}

.tree-content {
  flex: 1;
  overflow-y: auto;
  padding: 0.25rem 0;
  position: relative;
}

.tree-content.root-drop-active {
  background: color-mix(in srgb, var(--accent) 5%, transparent);
}

/* Favorites Section */
.favorites-section {
  border-bottom: 1px solid var(--border-primary);
  margin-bottom: 0.25rem;
  padding-bottom: 0.25rem;
}

.favorites-header {
  display: flex;
  align-items: center;
  height: 28px;
  padding: 0 8px;
  gap: 4px;
  cursor: pointer;
  user-select: none;
  color: var(--text-secondary);
  transition: background 0.1s;
}

.favorites-header:hover {
  background: var(--bg-hover);
}

.favorites-header .expand-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  color: var(--text-muted);
}

.favorites-icon {
  color: var(--warning);
  flex-shrink: 0;
}

.favorites-title {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.favorites-count {
  font-size: 11px;
  color: var(--text-muted);
  background: var(--bg-tertiary);
  padding: 0 6px;
  border-radius: 10px;
}

.favorites-items {
  /* Items are already indented via depth prop */
}

.boards-section {
  border-bottom: 1px solid var(--border-primary);
  margin-bottom: 0.25rem;
  padding-bottom: 0.25rem;
}

.boards-header {
  display: flex;
  align-items: center;
  height: 28px;
  padding: 0 8px;
  gap: 6px;
  cursor: pointer;
  user-select: none;
  color: var(--text-secondary);
}

.boards-header .expand-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  color: var(--text-muted);
}

.boards-title {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  color: var(--text-primary);
}

.boards-count {
  font-size: 11px;
  color: var(--text-muted);
  background: var(--bg-tertiary);
  padding: 0 6px;
  border-radius: 10px;
}

.root-drop-indicator {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 32px;
  margin: 4px 8px;
  border: 2px dashed var(--accent);
  border-radius: 4px;
  background: color-mix(in srgb, var(--accent) 10%, transparent);
  color: var(--accent);
  font-size: 12px;
  font-weight: 500;
  animation: pulse 0.5s ease-in-out infinite alternate;
}

@keyframes pulse {
  from { opacity: 0.7; }
  to { opacity: 1; }
}

.tree-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  padding: 2rem 1rem;
  color: var(--text-muted);
  font-size: 0.8125rem;
  text-align: center;
}

.tree-empty .btn-sm {
  font-size: 0.75rem;
  padding: 0.375rem 0.75rem;
}

/* Board items */
.tree-item {
  display: flex;
  align-items: center;
  height: 28px;
  padding-left: 12px;
  padding-right: 8px;
  gap: 8px;
  cursor: pointer;
  user-select: none;
  transition: background 0.1s;
}

.tree-item:hover {
  background: var(--bg-hover);
}

.tree-item.is-selected {
  background: var(--accent-light);
}

.board-color-dot {
  width: 10px;
  height: 10px;
  border-radius: 3px;
  flex-shrink: 0;
}

.tree-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
  color: var(--text-primary);
}

/* Button styles */
.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.375rem;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
  border: none;
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover {
  background: var(--accent-hover);
}
</style>
