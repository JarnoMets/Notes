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
      <button 
        class="explorer-tab"
        :class="{ active: activeTab === 'graphs' }"
        @click="activeTab = 'graphs'"
      >
        <Icon name="share-2" :size="14" />
        <span>Graphs</span>
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

    <!-- Graphs Content -->
    <div v-if="activeTab === 'graphs'" class="explorer-content">
      <div class="content-header">
        <div class="content-actions">
          <button @click="handleCreateGraph" title="New Graph">
            <Icon name="plus" :size="12" />
            <span>Graph</span>
          </button>
          <button @click="handleCreateGraphFolder" title="New Folder">
            <Icon name="folder" :size="12" />
            <span>Folder</span>
          </button>
        </div>
      </div>
      
      <div 
        class="tree-content"
        @dragover.prevent="onGraphsRootDragOver"
        @dragleave="onGraphsRootDragLeave"
        @drop.prevent="onGraphsRootDrop"
        @contextmenu.prevent="openRootContextMenu($event)"
        :class="{ 'root-drop-active': isGraphsRootDropTarget }"
      >
        <!-- Favorites Section (Graphs) -->
        <div v-if="explorerStore.favoritesList.some(i => i.type === 'graph' || i.type === 'graph-folder')" class="favorites-section">
          <div 
            class="favorites-header"
            @click="explorerStore.toggleFavoritesFolder()"
          >
            <span class="expand-toggle">
              <Icon :name="explorerStore.favoritesFolderExpanded ? 'chevron-down' : 'chevron-right'" :size="12" />
            </span>
            <Icon name="star" :size="14" class="favorites-icon" :fill="true" />
            <span class="favorites-title">Favorites</span>
            <span class="favorites-count">{{ explorerStore.favoritesList.filter(i => i.type === 'graph' || i.type === 'graph-folder').length }}</span>
          </div>
          <div v-if="explorerStore.favoritesFolderExpanded" class="favorites-items">
            <ExplorerTreeNode
              v-for="item in explorerStore.favoritesList.filter(i => i.type === 'graph' || i.type === 'graph-folder')"
              :key="'fav-graph-' + item.id"
              :item="item"
              :depth="1"
              :in-favorites="true"
              @select="handleGraphItemSelect"
              @toggle="handleGraphToggle"
              @dblclick="handleGraphItemDoubleClick"
              @create-folder="(parentId: string | null) => $emit('create-graph-folder', parentId)"
              @rename="(item: ExplorerItem) => $emit('rename', item)"
              @delete="(item: ExplorerItem) => $emit('delete', item)"
              @toggle-importance="handleToggleGraphImportance"
              @toggle-urgent="handleToggleGraphUrgent"
              @move-item="handleMoveItem"
            />
          </div>
        </div>

        <div class="graphs-section">
          <template v-if="explorerStore.graphsList.length > 0">
            <ExplorerTreeNode
              v-for="item in explorerStore.graphsList"
              :key="item.id"
              :item="item"
              :depth="0"
              @select="handleGraphItemSelect"
              @toggle="handleGraphToggle"
              @dblclick="handleGraphItemDoubleClick"
              @create-graph="() => {}"
              @create-folder="(parentId: string | null) => $emit('create-graph-folder', parentId)"
              @rename="(item: ExplorerItem) => $emit('rename', item)"
              @delete="(item: ExplorerItem) => $emit('delete', item)"
              @toggle-importance="handleToggleGraphImportance"
              @toggle-urgent="handleToggleGraphUrgent"
              @move-item="handleMoveItem"
            />
            <div v-if="isGraphsRootDropTarget" class="root-drop-indicator">Drop here to move to root</div>
          </template>
          <div v-else class="tree-empty">
            <Icon name="share-2" :size="32" />
            <span>No graphs yet</span>
            <button class="btn btn-primary btn-sm" @click="$emit('create-graph', null)">Create your first graph</button>
          </div>
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
  defaultTab?: 'notes' | 'boards' | 'graphs',
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

const activeTab = ref<'notes' | 'boards' | 'graphs'>(props.defaultTab || 'notes')

// Root drop target states
const isNotesRootDropTarget = ref(false)
const isBoardsRootDropTarget = ref(false)
const isGraphsRootDropTarget = ref(false)
let notesRootDragCounter = 0
let boardsRootDragCounter = 0
let graphsRootDragCounter = 0

// Sync tab with current route - Disabled for Window Manager mode
/*
function syncTabWithRoute() {
  if (route.path === '/boards') {
    activeTab.value = 'boards'
  } else if (route.path === '/notes') {
    activeTab.value = 'notes'
  } else if (route.path === '/graphs') {
    activeTab.value = 'graphs'
  }
}

// Watch for route changes
watch(() => route.path, syncTabWithRoute)
*/

// Set initial tab based on route
onMounted(() => {
  // syncTabWithRoute()
  
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
  isGraphsRootDropTarget.value = false
  notesRootDragCounter = 0
  boardsRootDragCounter = 0
  graphsRootDragCounter = 0
}

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

function handleBoardDoubleClick(board: ExplorerItem) {
  emit('open-board', board.id)
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

function handleGraphDoubleClick(graph: ExplorerItem) {
  emit('open-graph', graph.id)
}

function handleGraphItemDoubleClick(item: ExplorerItem) {
  if (item.type === 'graph') {
    handleGraphDoubleClick(item)
  } else if (item.type === 'graph-folder') {
    explorerStore.toggleGraphFolder(item.id)
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
    } else if (item.type === 'graph') {
      await explorerStore.toggleGraphImportance(item.id)
    } else if (item.type === 'graph-folder') {
      await explorerStore.toggleGraphFolderImportance(item.id)
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
    } else if (item.type === 'graph') {
      await explorerStore.toggleGraphUrgent(item.id)
    } else if (item.type === 'graph-folder') {
      await explorerStore.toggleGraphFolderUrgent(item.id)
    }
  } catch (error) {
    logger.error('Failed to toggle urgent:', error)
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

// Notes root drop zone handlers
function onNotesRootDragOver(event: DragEvent) {
  const hasExplorerType = event.dataTransfer?.types?.includes?.('application/x-explorer-item') || false
  if (!hasExplorerType) return
  
  const target = event.currentTarget as HTMLElement
  const rect = target.getBoundingClientRect()
  const y = event.clientY - rect.top
  const scrollHeight = target.scrollHeight
  
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

// Graphs root drop zone handlers
function onGraphsRootDragOver(event: DragEvent) {
  const hasExplorerType = event.dataTransfer?.types?.includes?.('application/x-explorer-item') || false
  if (!hasExplorerType) return
  
  const target = event.currentTarget as HTMLElement
  const rect = target.getBoundingClientRect()
  const y = event.clientY - rect.top
  const scrollHeight = target.scrollHeight
  
  if (y > scrollHeight - 40 || explorerStore.graphsList.length === 0) {
    graphsRootDragCounter++
    isGraphsRootDropTarget.value = true
    event.dataTransfer!.dropEffect = 'move'
  }
}

function onGraphsRootDragLeave(event: DragEvent) {
  // Check if we're leaving to a child element
  const relatedTarget = event.relatedTarget as Node | null
  const currentTarget = event.currentTarget as HTMLElement
  
  if (relatedTarget && currentTarget.contains(relatedTarget)) {
    return // Moving to a child element, don't reset
  }
  
  graphsRootDragCounter = Math.max(0, graphsRootDragCounter - 1)
  if (graphsRootDragCounter === 0) {
    isGraphsRootDropTarget.value = false
  }
}

function onGraphsRootDrop(event: DragEvent) {
  if (!isGraphsRootDropTarget.value) return
  isGraphsRootDropTarget.value = false
  graphsRootDragCounter = 0
  
  const data = event.dataTransfer?.getData('application/x-explorer-item')
  if (!data) return
  
  try {
    const { id, type } = JSON.parse(data)
    
    // Only allow graphs and graph-folders
    if (type !== 'graph' && type !== 'graph-folder') return
    
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

// Move item function to support drag and drop moving of items (notes, boards, graphs) and folders
async function handleMoveItem({ itemId, itemType, targetFolderId, insertBeforeId }: { itemId: string, itemType: string, targetFolderId: string | null, insertBeforeId: string | null }) {
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
}

.explorer-tabs {
  display: flex;
  border-bottom: 1px solid #e0e0e0;
}

.explorer-tab {
  flex: 1;
  padding: 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.explorer-tab.active {
  font-weight: 600;
  color: #333;
}

.explorer-tab .icon {
  margin-right: 8px;
}

.explorer-content {
  flex: 1;
  padding: 16px;
  overflow-y: auto;
}

.content-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.content-actions {
  display: flex;
  gap: 8px;
}

.tree-content {
  border: 1px solid #e0e0e0;
  border-radius: 4px;
  background-color: #fff;
  padding: 8px;
  position: relative;
}

.tree-empty {
  text-align: center;
  color: #999;
  padding: 32px 0;
}

.btn-primary {
  background-color: #007bff;
  color: #fff;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
}

.btn-primary:hover {
  background-color: #0056b3;
}

.favorites-section {
  margin-bottom: 16px;
}

.favorites-header {
  display: flex;
  align-items: center;
  cursor: pointer;
  padding: 8px;
  background-color: #f9f9f9;
  border: 1px solid #e0e0e0;
  border-radius: 4px;
}

.favorites-icon {
  margin-right: 8px;
}

.favorites-title {
  flex: 1;
  font-weight: 500;
}

.favorites-count {
  background-color: #007bff;
  color: #fff;
  border-radius: 12px;
  padding: 2px 8px;
  font-size: 12px;
}

.expand-toggle {
  display: inline-block;
  transition: transform 0.2s;
}

.expand-toggle.rotate {
  transform: rotate(90deg);
}

.root-drop-active {
  border-color: #007bff;
  background-color: #e6f7ff;
}

.root-drop-indicator {
  position: absolute;
  bottom: 8px;
  left: 0;
  right: 0;
  text-align: center;
  color: #007bff;
  font-weight: 500;
  padding: 8px 0;
  border-top: 1px solid #e0e0e0;
  background-color: #f9f9f9;
}
</style>
