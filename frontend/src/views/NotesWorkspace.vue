<template>
  <div class="notes-workspace">
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
        @select="handleSelect"
        @create-note="handleCreateNoteInFolder"
        @create-folder="handleCreateFolderInFolder"
        @create-board="handleCreateBoard"
        @create-board-folder="handleCreateBoardFolder"
        @rename="handleRename"
        @delete="handleDelete"
        @open-note="handleOpenNote"
        @open-board="handleOpenBoard"
        @drop="handleDrop"
      />
    </WorkspaceSidebar>

    <!-- Resize Handle -->
    <ResizeHandle
      direction="vertical"
      @resize-start="startSidebarResize"
    />

    <!-- Editor Area -->
    <div class="editor-area">
      <div 
        class="editor-panes"
        :class="{ 
          'split-horizontal': splitDirection === 'horizontal',
          'split-vertical': splitDirection === 'vertical'
        }"
      >
        <template v-for="(pane, index) in panes" :key="pane.id">
          <div 
            class="editor-pane"
            :class="{ 
              'active-pane': pane.id === activePaneId,
              'drop-target': dropTargetPaneId === pane.id
            }"
            @click="setActivePane(pane.id)"
            @dragenter="onPaneDragEnter($event, pane.id)"
            @dragover="onPaneDragOver($event, pane.id)"
            @dragleave="onPaneDragLeave($event, pane.id)"
            @drop="onPaneDrop($event, pane.id)"
          >
            <!-- Tab Bar -->
            <TabBar
              v-if="pane.tabs.length > 0"
              :tabs="pane.tabs"
              :active-tab-id="pane.activeTabId"
              :has-split="hasSecondPane"
              :drop-target="tabDropTarget && tabDropTarget.paneId === pane.id ? { index: tabDropTarget.index, position: tabDropTarget.position } : null"
              @select-tab="(tabId: string) => setActiveTab(tabId, pane.id)"
              @close-tab="(tabId: string) => closeTab(tabId, pane.id)"
              @split="splitPane"
              @close-split="closeSplit"
              @drag-start="(event: DragEvent, tab: any, _index: number) => onTabDragStart(event, tab, pane.id)"
              @drag-end="onTabDragEnd"
              @drag-over="(index: number, position: 'left' | 'right') => onTabDragOver(null, pane.id, index, position)"
              @drop="(event: DragEvent, tabIndex: number) => onTabDrop(event, pane.id, tabIndex)"
              @tree-drop="(event: DragEvent, insertIndex: number) => onTreeDropToTabs(event, pane.id, insertIndex)"
              @split-drop="(event: DragEvent) => onSplitDrop(event)"
            />

            <!-- Editor Content -->
            <div class="editor-content" v-if="pane.activeTabId">
              <NoteEditor
                :key="getActiveNoteId(pane) || pane.id"
                :note-id="getActiveNoteId(pane)"
                :pane-id="pane.id"
                @dirty="(isDirty: boolean) => setTabDirty(pane.activeTabId!, isDirty)"
              />
            </div>

            <!-- Empty State -->
            <EmptyPane
              v-else
              :is-drop-target="dropTargetPaneId === pane.id"
              @create="createNewNote"
              @dragenter="(e: DragEvent) => onPaneDragEnter(e, pane.id)"
              @dragover="(e: DragEvent) => onPaneDragOver(e, pane.id)"
              @dragleave="(e: DragEvent) => onPaneDragLeave(e, pane.id)"
              @drop="(e: DragEvent) => onPaneDrop(e, pane.id)"
            />
          </div>

          <!-- Split Resize Handle -->
          <ResizeHandle
            v-if="index < panes.length - 1"
            :direction="splitDirection === 'horizontal' ? 'horizontal' : 'vertical'"
            @resize-start="startPaneResize"
          />
        </template>
      </div>
    </div>

    <!-- Context Menu -->
    <div 
      v-if="contextMenu.visible"
      class="context-menu"
      :style="{ left: contextMenu.x + 'px', top: contextMenu.y + 'px' }"
      @click.stop
    >
      <div class="context-menu-item" @click="handleContextAction('open')">Open</div>
      <div class="context-menu-item" @click="handleContextAction('rename')">Rename</div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item" @click="handleContextAction('newNote')">New Note</div>
      <div class="context-menu-item" @click="handleContextAction('newFolder')">New Folder</div>
      <div class="context-menu-divider"></div>
      <div class="context-menu-item danger" @click="handleContextAction('delete')">Delete</div>
    </div>

    <!-- Rename Modal -->
    <div v-if="renameModal.visible" class="modal-overlay" @click.self="renameModal.visible = false">
      <div class="modal rename-modal">
        <div class="modal-header">
          <h3>Rename {{ renameModal.type }}</h3>
          <button class="modal-close" @click="renameModal.visible = false">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <form @submit.prevent="submitRename">
          <input 
            ref="renameInput"
            v-model="renameModal.name"
            type="text"
            class="rename-input"
            @keydown.esc="renameModal.visible = false"
          />
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="renameModal.visible = false">Cancel</button>
            <button type="submit" class="btn btn-primary">Rename</button>
          </div>
        </form>
      </div>
    </div>

    <!-- Prompt Modal for creating notes/folders -->
    <PromptModal
      :visible="promptModal.visible"
      :title="promptModal.title"
      :placeholder="promptModal.placeholder"
      confirm-text="Create"
      @submit="handlePromptSubmit"
      @cancel="promptModal.visible = false"
    />

    <!-- Confirm Modal for deletions -->
    <ConfirmModal
      :visible="confirmModal.visible"
      :title="confirmModal.title"
      :message="confirmModal.message"
      confirm-text="Delete"
      variant="danger"
      @confirm="handleConfirmDelete"
      @cancel="confirmModal.visible = false"
    />

    <!-- Create Board Modal -->
    <div v-if="boardModal.visible" class="modal-overlay" @click.self="boardModal.visible = false">
      <div class="modal">
        <div class="modal-header">
          <h3>New Board</h3>
          <button class="modal-close" @click="boardModal.visible = false">
            <Icon name="x" :size="14" />
          </button>
        </div>
        <form @submit.prevent="saveBoardModal">
          <div class="form-group">
            <label for="boardName">Name</label>
            <input 
              id="boardName" 
              v-model="boardModal.name" 
              type="text" 
              required 
              placeholder="Board name"
            />
          </div>
          <div class="form-group">
            <label for="boardDescription">Description (optional)</label>
            <textarea 
              id="boardDescription" 
              v-model="boardModal.description" 
              rows="3" 
              placeholder="What's this board for?"
            ></textarea>
          </div>
          <div class="form-group">
            <label>Color</label>
            <div class="color-picker">
              <button 
                v-for="color in colorPalette" 
                :key="color"
                type="button"
                class="color-option"
                :class="{ selected: boardModal.color === color }"
                :style="{ backgroundColor: color }"
                @click="boardModal.color = color"
              ></button>
            </div>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="boardModal.visible = false">Cancel</button>
            <button type="submit" class="btn btn-primary">Create</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useNotesStore } from '../stores/notes'
import { useExplorerStore, type ExplorerItem } from '../stores/explorer'
import { storeToRefs } from 'pinia'
import logger from '@/utils/logger'
import NoteEditor from '../components/editor/NoteEditor.vue'
import ExplorerTree from '../components/common/ui/ExplorerTree.vue'
import Icon from '../components/common/ui/Icon.vue'
import ConfirmModal from '../components/common/modals/ConfirmModal.vue'
import PromptModal from '../components/common/modals/PromptModal.vue'
import {
  TabBar,
  WorkspaceSidebar,
  MobileSidebarToggle,
  MobileOverlay,
  ResizeHandle,
  EmptyPane
} from '../components/workspace'

const router = useRouter()
const notesStore = useNotesStore()
const explorerStore = useExplorerStore()
const { panes, activePaneId, splitDirection, hasSecondPane } = storeToRefs(notesStore)

// Sidebar resize
// Use the same default width as BoardsWorkspace for consistency
const sidebarWidth = ref(240)
const isResizingSidebar = ref(false)
const isMobileSidebarOpen = ref(false)

// Pane resize
const paneSize = ref(50) // percentage
const isResizingPane = ref(false)

// Context menu
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  targetId: '',
  targetType: '' as 'note' | 'folder' | ''
})

// Rename modal
const renameModal = ref({
  visible: false,
  id: '',
  type: '' as 'note' | 'folder',
  name: ''
})
const renameInput = ref<HTMLInputElement | null>(null)

// Prompt modal for creating notes/folders
const promptModal = ref({
  visible: false,
  title: '',
  placeholder: '',
  action: '' as 'note' | 'folder',
  parentId: null as string | null
})

// Confirm modal for deletions
const confirmModal = ref({
  visible: false,
  title: '',
  message: '',
  item: null as ExplorerItem | null
})

// Board creation modal
const boardModal = ref({
  visible: false,
  name: '',
  description: '',
  color: '#3498db'
})

const colorPalette = ['#3498db', '#2ecc71', '#e74c3c', '#f39c12', '#9b59b6', '#1abc9c', '#e67e22', '#34495e']

// Tab drag
const draggedTab = ref<{ tabId: string; paneId: string; tabIndex: number } | null>(null)
const tabDropTarget = ref<{ paneId: string; index: number; position: 'left' | 'right' } | null>(null)
const dropTargetPaneId = ref<string | null>(null)
const paneDragCounters = new Map<string, number>()

// Helper functions
function getActiveNoteId(pane: typeof panes.value[0]): string | null {
  const tab = pane.tabs.find(t => t.id === pane.activeTabId)
  return tab?.noteId || null
}

function setActivePane(paneId: string) {
  notesStore.activePaneId = paneId
}

function setActiveTab(tabId: string, paneId: string) {
  notesStore.setActiveTab(tabId, paneId)
}

function closeTab(tabId: string, paneId: string) {
  notesStore.closeTab(tabId, paneId)
}

function setTabDirty(tabId: string, isDirty: boolean) {
  notesStore.setTabDirty(tabId, isDirty)
}

function splitPane(direction: 'horizontal' | 'vertical') {
  notesStore.splitPane(direction)
}

function closeSplit() {
  notesStore.closeSplit()
}

// Tree handlers
function handleSelect(item: ExplorerItem) {
  explorerStore.selectItem(item.id, item.type as 'note' | 'folder' | 'board' | 'board-folder')
}

function handleOpenNote(noteId: string) {
  notesStore.openNote(noteId)
  // Close mobile sidebar when a note is opened
  isMobileSidebarOpen.value = false
}

function handleOpenBoard(boardId: string) {
  // Navigate to boards page with the board selected
  router.push('/boards')
  // The board selection will be handled by BoardsWorkspace
  localStorage.setItem('selectedBoardId', boardId)
}

async function handleCreateNoteInFolder(folderId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Note',
    placeholder: 'Enter note name',
    action: 'note',
    parentId: folderId
  }
}

async function handleCreateFolderInFolder(parentId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Folder',
    placeholder: 'Enter folder name',
    action: 'folder',
    parentId: parentId
  }
}

async function handleCreateBoard() {
  boardModal.value = {
    visible: true,
    name: '',
    description: '',
    color: '#3498db'
  }
}

async function handleCreateBoardFolder(parentId: string | null) {
  const name = prompt('Enter folder name:')
  if (name) {
    await explorerStore.createBoardFolder(name, parentId)
  }
}

async function handlePromptSubmit(name: string) {
  promptModal.value.visible = false
  if (promptModal.value.action === 'note') {
    const note = await explorerStore.createNote(name, promptModal.value.parentId)
    if (note) {
      notesStore.openNote(note.id)
    }
  } else {
    await explorerStore.createFolder(name, promptModal.value.parentId)
  }
}

async function saveBoardModal() {
  if (!boardModal.value.name.trim()) return
  try {
    await explorerStore.createBoard(
      boardModal.value.name,
      boardModal.value.description || undefined,
      boardModal.value.color
    )
    boardModal.value.visible = false
  } catch (error) {
    logger.error('Failed to create board:', error)
  }
}

async function handleRename(item: ExplorerItem) {
  if (item.type === 'board') {
    // Handle board rename differently - redirect to boards page
    router.push('/boards')
    localStorage.setItem('selectedBoardId', item.id)
    return
  }
  if (item.type === 'board-folder') {
    const newName = prompt('Enter new folder name:', item.name)
    if (newName && newName !== item.name) {
      await explorerStore.renameBoardFolder(item.id, newName)
    }
    return
  }
  renameModal.value = {
    visible: true,
    id: item.id,
    type: item.type as 'note' | 'folder',
    name: item.name
  }
  await nextTick()
  renameInput.value?.focus()
  renameInput.value?.select()
}

async function handleDelete(item: ExplorerItem) {
  if (item.type === 'board') {
    // Handle board deletion - redirect to boards page
    router.push('/boards')
    localStorage.setItem('selectedBoardId', item.id)
    return
  }
  if (item.type === 'board-folder') {
    if (confirm(`Delete folder "${item.name}" and all its contents?`)) {
      await explorerStore.deleteBoardFolder(item.id)
    }
    return
  }
  confirmModal.value = {
    visible: true,
    title: `Delete ${item.type}`,
    message: item.type === 'folder' 
      ? `Delete folder "${item.name}" and all its contents?`
      : `Delete note "${item.name}"?`,
    item: item
  }
}

async function handleConfirmDelete() {
  const item = confirmModal.value.item
  confirmModal.value.visible = false
  if (!item) return
  
  if (item.type === 'folder') {
    await explorerStore.deleteFolder(item.id)
  } else if (item.type === 'note') {
    await explorerStore.deleteNote(item.id)
    // Also close any tabs with this note
    panes.value.forEach(pane => {
      const tab = pane.tabs.find(t => t.noteId === item.id)
      if (tab) {
        notesStore.closeTab(tab.id, pane.id)
      }
    })
  }
}

async function handleDrop(data: { draggedId: string; draggedType: 'note' | 'folder' | 'board' | 'board-folder'; targetId: string | null; targetType: 'folder' | 'note' | 'board' | 'board-folder' | null; position: number; dropPosition: 'before' | 'after' | 'inside' }) {
  try {
    if (data.draggedType === 'note') {
      await explorerStore.moveNote(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchNotesTree()
    } else if (data.draggedType === 'folder') {
      await explorerStore.moveFolder(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchNotesTree()
    } else if (data.draggedType === 'board') {
      await explorerStore.moveBoard(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchBoardsTree()
    } else if (data.draggedType === 'board-folder') {
      await explorerStore.moveBoardFolder(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchBoardsTree()
    }
  } catch (error) {
    logger.error('Failed to move item:', error)
  }
}

async function submitRename() {
  if (!renameModal.value.name.trim()) return
  
  if (renameModal.value.type === 'folder') {
    await explorerStore.renameFolder(renameModal.value.id, renameModal.value.name)
  } else {
    await explorerStore.updateNote(renameModal.value.id, { title: renameModal.value.name })
    // Update tab title if open
    panes.value.forEach(pane => {
      const tab = pane.tabs.find(t => t.noteId === renameModal.value.id)
      if (tab) {
        tab.title = renameModal.value.name
      }
    })
  }
  
  renameModal.value.visible = false
}

async function refreshTree() {
  await explorerStore.fetchAll()
}

// Top-level actions
async function createNewNote() {
  promptModal.value = {
    visible: true,
    title: 'New Note',
    placeholder: 'Enter note name',
    action: 'note',
    parentId: null
  }
}

// Persistence for notes workspace (open tabs / panes)
const NOTES_VIEW_KEY = 'viewState.notes'

function loadNotesViewState() {
  try {
    const raw = localStorage.getItem(NOTES_VIEW_KEY)
    if (!raw) return
    const data = JSON.parse(raw)
    if (data.panes && Array.isArray(data.panes)) {
      // Replace store panes
      notesStore.panes = data.panes
    }
    if (data.activePaneId) notesStore.activePaneId = data.activePaneId
    if (data.splitDirection) notesStore.splitDirection = data.splitDirection
    if (typeof data.sidebarWidth === 'number') sidebarWidth.value = data.sidebarWidth
    if (typeof data.isMobileSidebarOpen === 'boolean') isMobileSidebarOpen.value = data.isMobileSidebarOpen
  } catch (e) {
    // ignore parse errors
  }
}

function saveNotesViewState() {
  try {
    const data: any = {
      panes: notesStore.panes,
      activePaneId: notesStore.activePaneId,
      splitDirection: notesStore.splitDirection,
      sidebarWidth: sidebarWidth.value,
      isMobileSidebarOpen: isMobileSidebarOpen.value
    }
    localStorage.setItem(NOTES_VIEW_KEY, JSON.stringify(data))
  } catch (e) {
    // ignore
  }
}

// Context menu
function handleContextAction(_action: string) {
  contextMenu.value.visible = false
  // Handle actions based on context
}

// Tab drag and drop
function onTabDragStart(event: DragEvent, tab: typeof panes.value[0]['tabs'][0], paneId: string) {
  const pane = panes.value.find(p => p.id === paneId)
  const tabIndex = pane?.tabs.findIndex(t => t.id === tab.id) ?? 0
  draggedTab.value = { tabId: tab.id, paneId, tabIndex }
  event.dataTransfer?.setData('application/x-notes-tab', tab.id)
}

function onTabDragEnd() {
  draggedTab.value = null
  tabDropTarget.value = null
}

function onTabDragOver(_event: DragEvent | null, paneId: string, tabIndex: number, position?: 'left' | 'right') {
  if (!draggedTab.value) return
  
  // Use the provided position or default to 'right'
  const dropPosition = position || 'right'
  
  tabDropTarget.value = { paneId, index: tabIndex, position: dropPosition }
}

function onTabDrop(_event: DragEvent, targetPaneId: string, targetIndex?: number) {
  if (!draggedTab.value) return
  
  const { tabId, paneId: sourcePaneId, tabIndex: sourceIndex } = draggedTab.value
  const dropPosition = tabDropTarget.value?.position || 'right'
  
  if (sourcePaneId === targetPaneId && targetIndex !== undefined) {
    // Reordering within the same pane
    const pane = panes.value.find(p => p.id === targetPaneId)
    if (pane) {
      const tabs = [...pane.tabs]
      const [movedTab] = tabs.splice(sourceIndex, 1)
      
      // Calculate new index
      let newIndex = targetIndex
      if (dropPosition === 'right') newIndex++
      if (sourceIndex < targetIndex) newIndex--
      
      tabs.splice(newIndex, 0, movedTab)
      pane.tabs = tabs
    }
  } else if (sourcePaneId !== targetPaneId) {
    // Moving to a different pane
    notesStore.moveTabToPane(tabId, sourcePaneId, targetPaneId)
  }
  
  draggedTab.value = null
  tabDropTarget.value = null
}

// Pane drag and drop (for opening notes from tree)
function onPaneDragEnter(event: DragEvent, paneId: string) {
  // Check if this is a tree item drag (note)
  const types = event.dataTransfer?.types || []
  // Convert DOMStringList to array for includes check
  const typesArray = Array.from(types)
  const isExplorer = typesArray.includes('application/x-explorer-item')
  if (!isExplorer) return
  
  event.preventDefault()
  // Increment counter for nested elements
  const count = paneDragCounters.get(paneId) || 0
  paneDragCounters.set(paneId, count + 1)
  dropTargetPaneId.value = paneId
}

function onPaneDragOver(event: DragEvent, paneId: string) {
  // Check if this is a tree item drag (note)
  const types = event.dataTransfer?.types || []
  // Convert DOMStringList to array for includes check
  const typesArray = Array.from(types)
  const isExplorer = typesArray.includes('application/x-explorer-item')
  if (!isExplorer) return
  
  event.preventDefault()
  if (event.dataTransfer) event.dataTransfer.dropEffect = 'copy'
  dropTargetPaneId.value = paneId
}

function onPaneDragLeave(_event?: DragEvent, paneId?: string) {
  if (!paneId) {
    dropTargetPaneId.value = null
    return
  }
  const count = paneDragCounters.get(paneId) || 0
  const next = Math.max(0, count - 1)
  if (next === 0) {
    paneDragCounters.delete(paneId)
    if (dropTargetPaneId.value === paneId) dropTargetPaneId.value = null
  } else {
    paneDragCounters.set(paneId, next)
  }
}

function onPaneDrop(event: DragEvent, paneId: string) {
  event.preventDefault()
  dropTargetPaneId.value = null
  paneDragCounters.delete(paneId)
  
  // Handle tree item drops (notes/boards)
  let treeData = event.dataTransfer?.getData('application/x-explorer-item')
  
  if (treeData) {
    try {
      let parsed: any
      if (treeData.startsWith('{')) {
        parsed = JSON.parse(treeData)
      } else {
        // fallback when only note id is set
        parsed = { id: treeData, type: 'note' }
      }
      const { id, type } = parsed
      
      if (type === 'note' || type === 'board') {
        if (type === 'note') {
          // Open the note in this pane
          notesStore.openNote(id, paneId)
        } else if (type === 'board') {
          // Navigate to boards with this board selected
          router.push('/boards')
          localStorage.setItem('selectedBoardId', id)
        }
        return
      }
    } catch (e) {
      logger.error('Failed to parse tree drag data:', e)
    }
  }
  
  // Handle tab drops (from other panes)
  if (draggedTab.value && draggedTab.value.paneId !== paneId) {
    notesStore.moveTabToPane(draggedTab.value.tabId, draggedTab.value.paneId, paneId)
    draggedTab.value = null
    tabDropTarget.value = null
  }
}

// Handle drops from tree to tabs container
function onTreeDropToTabs(event: DragEvent, paneId: string, insertIndex: number) {
  let treeData = event.dataTransfer?.getData('application/x-explorer-item')
  if (!treeData) return

  try {
    let parsed: any
    if (treeData.startsWith('{')) {
      parsed = JSON.parse(treeData)
    } else {
      parsed = { id: treeData, type: 'note' }
    }
    
    const { id, type } = parsed
    if (type === 'note') {
      notesStore.openNoteAtIndex(id, paneId, insertIndex)
    } else if (type === 'board') {
      router.push('/boards')
      localStorage.setItem('selectedBoardId', id)
    }
  } catch (e) {
    logger.error('Failed to parse tree drag data for tabs:', e)
  }
}

// Handle drops on the split zone (open in new split)
function onSplitDrop(event: DragEvent) {
  let treeData = event.dataTransfer?.getData('application/x-explorer-item')
  if (!treeData) return

  try {
    let parsed: any
    if (treeData.startsWith('{')) {
      parsed = JSON.parse(treeData)
    } else {
      parsed = { id: treeData, type: 'note' }
    }
    
    const { id, type } = parsed
    if (type === 'note') {
      notesStore.openNoteInNewSplit(id, 'vertical')
    } else if (type === 'board') {
      router.push('/boards')
      localStorage.setItem('selectedBoardId', id)
    }
  } catch (e) {
    logger.error('Failed to parse tree drag data for split:', e)
  }
}

// Resize handlers
function startSidebarResize(_event: MouseEvent) {
  isResizingSidebar.value = true
  document.addEventListener('mousemove', onSidebarResize)
  document.addEventListener('mouseup', stopSidebarResize)
}

function onSidebarResize(event: MouseEvent) {
  if (isResizingSidebar.value) {
    sidebarWidth.value = Math.max(150, Math.min(500, event.clientX))
  }
}

function stopSidebarResize() {
  isResizingSidebar.value = false
  document.removeEventListener('mousemove', onSidebarResize)
  document.removeEventListener('mouseup', stopSidebarResize)
}

// Mobile sidebar toggle
function toggleMobileSidebar() {
  isMobileSidebarOpen.value = !isMobileSidebarOpen.value
}

function startPaneResize(_event: MouseEvent) {
  isResizingPane.value = true
  document.addEventListener('mousemove', onPaneResize)
  document.addEventListener('mouseup', stopPaneResize)
}

function onPaneResize(event: MouseEvent) {
  if (isResizingPane.value) {
    const container = document.querySelector('.editor-panes')
    if (container) {
      const rect = container.getBoundingClientRect()
      if (splitDirection.value === 'vertical') {
        paneSize.value = ((event.clientX - rect.left) / rect.width) * 100
      } else {
        paneSize.value = ((event.clientY - rect.top) / rect.height) * 100
      }
      paneSize.value = Math.max(20, Math.min(80, paneSize.value))
    }
  }
}

function stopPaneResize() {
  isResizingPane.value = false
  document.removeEventListener('mousemove', onPaneResize)
  document.removeEventListener('mouseup', stopPaneResize)
}

// Close context menu on click outside
function onDocumentClick() {
  contextMenu.value.visible = false
}

onMounted(async () => {
  await explorerStore.fetchAll()
  document.addEventListener('click', onDocumentClick)
  
  // Check if a note was requested from another view (e.g., from board card link)
  const requestedNoteId = localStorage.getItem('openNoteId')
  if (requestedNoteId) {
    localStorage.removeItem('openNoteId')
    notesStore.openNote(requestedNoteId)
  }
  // Restore saved UI state if present (open tabs / panes)
  loadNotesViewState()
})

onUnmounted(() => {
  // Persist open tabs / panes so switching back restores context
  saveNotesViewState()
  document.removeEventListener('click', onDocumentClick)
})
</script>

<style scoped>
.notes-workspace {
  display: flex;
  height: calc(100vh - 56px);
  background: var(--bg-primary);
  color: var(--text-secondary);
  overflow: hidden;
}

/* Editor Area */
.editor-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.editor-panes {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.editor-panes.split-horizontal {
  flex-direction: column;
}

.editor-panes.split-vertical {
  flex-direction: row;
}

/* Editor pane style updated below */

/* Editor Content */
.editor-pane {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  background: var(--bg-primary);
  border: none;
  transition: background 0.15s;
}
.editor-pane.active-pane {
  /* No border for active pane */
}
.editor-pane.drop-target {
  background: color-mix(in srgb, var(--accent) 4%, var(--bg-primary));
}

.editor-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.rename-input:focus {
  outline: none;
  border-color: var(--accent);
}

/* ========================================
   RESPONSIVE DESIGN
   ======================================== */
@media (max-width: 1024px) {
  .notes-workspace {
    flex-direction: column;
  }

  .editor-area {
    flex: 1;
    min-height: 0;
  }
}

@media (max-width: 768px) {
  .notes-workspace {
    position: relative;
    height: calc(100vh - 56px);
    height: calc(100dvh - 56px);
  }

  .editor-area {
    width: 100%;
    height: 100%;
  }

  /* Disable split panes on mobile */
  .editor-panes.split-horizontal,
  .editor-panes.split-vertical {
    flex-direction: column;
  }

  .editor-pane:nth-child(n+2) {
    display: none;
  }

  .editor-content {
    min-height: 0;
  }

  .rename-input {
    font-size: 16px; /* Prevents zoom on iOS */
  }
}

@media (max-width: 768px) and (orientation: landscape) {
  .notes-workspace {
    height: calc(100vh - 48px);
    height: calc(100dvh - 48px);
  }
}
</style>
