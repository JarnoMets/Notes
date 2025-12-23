<template>
  <div class="main-layout">
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
        @create-note="handleCreateNote"
        @create-folder="handleCreateFolder"
        @create-board="handleCreateBoard"
        @create-board-folder="handleCreateBoardFolder"
        @create-graph="handleCreateGraph"
        @create-graph-folder="handleCreateGraphFolder"
        @rename="handleRename"
        @delete="handleDelete"
        @open-note="openNote"
        @open-board="openBoard"
        @open-graph="openGraph"
        @drop="handleDrop"
      />
    </WorkspaceSidebar>

    <!-- Resize Handle -->
    <ResizeHandle
      direction="vertical"
      @resize-start="startSidebarResize"
    />

    <!-- Window Manager -->
    <div class="content-area">
      <WindowManager />
    </div>

    <!-- Modals (Global) -->
    <PromptModal
      :visible="promptModal.visible"
      :title="promptModal.title"
      :placeholder="promptModal.placeholder"
      confirm-text="Create"
      @submit="handlePromptSubmit"
      @cancel="promptModal.visible = false"
    />
    
    <!-- Other modals can be added here or managed by stores -->
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useLayoutStore } from '@/stores/layout'
import { useExplorerStore, type ExplorerItem } from '@/stores/explorer'
import { WorkspaceSidebar, MobileSidebarToggle, MobileOverlay, ResizeHandle } from '@/components/workspace'
import ExplorerTree from '@/components/common/ui/ExplorerTree.vue'
import WindowManager from '@/components/window-manager/WindowManager.vue'
import PromptModal from '@/components/common/modals/PromptModal.vue'
import logger from '@/utils/logger'

const layoutStore = useLayoutStore()
const explorerStore = useExplorerStore()

// Sidebar state
const sidebarWidth = ref(240)
const isMobileSidebarOpen = ref(false)
const isResizingSidebar = ref(false)

// Modals
const promptModal = ref({
  visible: false,
  title: '',
  placeholder: '',
  action: '',
  parentId: null as string | null
})

// Actions
function openNote(noteId: string) {
  const note = explorerStore.notes.find(n => n.id === noteId)
  if (layoutStore.activeLeafId) {
    layoutStore.addTab(layoutStore.activeLeafId, {
      type: 'note',
      entityId: noteId,
      title: note?.title || 'Untitled Note'
    })
  }
  isMobileSidebarOpen.value = false
}

function openBoard(boardId: string) {
  const board = explorerStore.boards.find(b => b.id === boardId)
  if (layoutStore.activeLeafId) {
    layoutStore.addTab(layoutStore.activeLeafId, {
      type: 'board',
      entityId: boardId,
      title: board?.name || 'Untitled Board'
    })
  }
  isMobileSidebarOpen.value = false
}

function openGraph(graphId: string) {
  const graph = explorerStore.graphs.find(g => g.id === graphId)
  if (layoutStore.activeLeafId) {
    layoutStore.addTab(layoutStore.activeLeafId, {
      type: 'graph',
      entityId: graphId,
      title: graph?.name || 'Untitled Graph'
    })
  }
  isMobileSidebarOpen.value = false
}

// Explorer Handlers
function handleSelect(item: ExplorerItem) {
  explorerStore.selectItem(item.id, item.type as any)
}

function refreshTree() {
  explorerStore.fetchAll()
}

function toggleMobileSidebar() {
  isMobileSidebarOpen.value = !isMobileSidebarOpen.value
}

// Sidebar Resize
function startSidebarResize() {
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

// CRUD Handlers (Simplified for now)
function handleCreateNote(folderId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Note',
    placeholder: 'Note Name',
    action: 'note',
    parentId: folderId
  }
}

function handleCreateFolder(parentId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Folder',
    placeholder: 'Folder Name',
    action: 'folder',
    parentId: parentId
  }
}

function handleCreateBoard(folderId: string | null) {
  // TODO: Board creation modal
  const name = prompt('Board Name')
  if (name) explorerStore.createBoard(name, undefined, '#3498db', folderId)
}

function handleCreateBoardFolder(parentId: string | null) {
  const name = prompt('Folder Name')
  if (name) explorerStore.createBoardFolder(name, parentId)
}

function handleCreateGraph(folderId: string | null) {
  const name = prompt('Graph Name')
  if (name) explorerStore.createGraph(name, folderId)
}

function handleCreateGraphFolder(parentId: string | null) {
  const name = prompt('Folder Name')
  if (name) explorerStore.createGraphFolder(name, parentId)
}

function handleRename(item: ExplorerItem) {
  const newName = prompt('Rename', item.name)
  if (newName && newName !== item.name) {
    // Dispatch based on type
    if (item.type === 'note') {
      explorerStore.updateNote(item.id, { title: newName })
      layoutStore.updateTabsByEntityId(item.id, { title: newName })
    }
    else if (item.type === 'folder') explorerStore.renameFolder(item.id, newName)
    else if (item.type === 'board') {
      explorerStore.updateBoard(item.id, { name: newName })
      layoutStore.updateTabsByEntityId(item.id, { title: newName })
    }
    else if (item.type === 'board-folder') explorerStore.renameBoardFolder(item.id, newName)
    else if (item.type === 'graph') {
      explorerStore.updateGraph(item.id, { name: newName })
      layoutStore.updateTabsByEntityId(item.id, { title: newName })
    }
    else if (item.type === 'graph-folder') explorerStore.renameGraphFolder(item.id, newName)
  }
}

function handleDelete(item: ExplorerItem) {
  if (confirm(`Delete ${item.name}?`)) {
    if (item.type === 'note') explorerStore.deleteNote(item.id)
    // ... others
  }
}

function handleDrop(data: any) {
  // TODO: Handle move
}

async function handlePromptSubmit(name: string) {
  promptModal.value.visible = false
  if (promptModal.value.action === 'note') {
    const note = await explorerStore.createNote(name, promptModal.value.parentId)
    if (note) openNote(note.id)
  } else if (promptModal.value.action === 'folder') {
    await explorerStore.createFolder(name, promptModal.value.parentId)
  }
}

onMounted(() => {
  explorerStore.fetchAll()
  
  // Global handlers for BBCode links (override App.vue ones)
  ;(window as any).__openNote = openNote
  ;(window as any).__openBoard = openBoard
  ;(window as any).__openGraph = openGraph
})
</script>

<style scoped>
.main-layout {
  display: flex;
  height: calc(100vh - 56px);
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}
</style>
