<template>
  <div class="graphs-workspace">
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
        default-tab="graphs"
        :show-boards-header="false"
        @select="handleExplorerSelect"
        @create-note="handleCreateNote"
        @create-folder="handleCreateFolder"
        @create-board="handleCreateBoard"
        @create-board-folder="handleCreateBoardFolder"
        @create-graph="handleCreateGraph"
        @create-graph-folder="handleCreateGraphFolder"
        @rename="handleRename"
        @delete="handleExplorerDelete"
        @open-note="handleOpenNote"
        @open-board="handleOpenBoard"
        @open-graph="selectGraph"
        @drop="handleDrop"
      />
    </WorkspaceSidebar>

    <!-- Resize Handle -->
    <ResizeHandle
      direction="vertical"
    />

    <!-- Main Content -->
    <div class="graph-content"
      @dragenter="onGraphContentDragEnter"
      @dragover="onGraphContentDragOver"
      @dragleave="onGraphContentDragLeave"
      @drop="onGraphContentDrop"
      :class="{ 'drag-over': graphContentDragOver }"
    >
      <div v-if="currentGraph" class="graph-container">
        <div class="graph-header">
          <div class="header-left">
            <h2>{{ currentGraph.graph.name }}</h2>
            <span class="graph-description" v-if="currentGraph.graph.description">{{ currentGraph.graph.description }}</span>
          </div>
          <div class="header-actions">
            <button class="btn-icon" title="Graph Settings">
              <Icon name="settings" :size="18" />
            </button>
            <button class="btn-icon danger" title="Delete Graph" @click="confirmDeleteGraph">
              <Icon name="trash" :size="18" />
            </button>
          </div>
        </div>
        
        <!-- Graph Canvas Wrapper -->
        <div class="graph-canvas-wrapper">
          <GraphCanvas
            v-if="currentGraph"
            :nodes="currentGraph.nodes"
            :edges="currentGraph.edges"
            @node-move="handleNodeMove"
            @node-move-finished="handleNodeMoveFinished"
            @create-node="handleCreateNode"
            @create-edge="handleCreateEdge"
            @node-select="handleNodeSelect"
            @edge-select="handleEdgeSelect"
            @open-node="handleOpenNodeFromGraph"
            @delete-node="handleDeleteNode"
            @delete-edge="handleDeleteEdge"
            @rename-node="handleRenameNode"
            @update-node="handleUpdateNode"
            @update-edge="handleUpdateEdge"
            @undo="undo"
            @redo="redo"
          />
        </div>
      </div>

      <!-- Empty State -->
      <div class="graph-empty" v-else>
        <div class="empty-content">
          <Icon name="share-2" :size="48" />
          <h3>No Graph Selected</h3>
          <p>Select a graph from the sidebar or create a new one</p>
        </div>
      </div>
    </div>

    <!-- Prompt Modal for graph folders -->
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
      :visible="deleteGraphModalVisible"
      title="Delete Graph"
      message="Delete this graph? This action cannot be undone."
      confirm-text="Delete"
      variant="danger"
      @confirm="handleDeleteGraph"
      @cancel="deleteGraphModalVisible = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { graphsApi, graphNodesApi, graphEdgesApi } from '../api/graphs'
import { useExplorerStore, type ExplorerItem } from '../stores/explorer'
import { useNotesStore } from '../stores/notes'
import type { GraphWithData } from '../types'
import { WorkspaceSidebar, MobileSidebarToggle, MobileOverlay, ResizeHandle } from '../components/workspace'
import ExplorerTree from '../components/common/ui/ExplorerTree.vue'
import Icon from '../components/common/ui/Icon.vue'
import GraphCanvas from '../components/common/ui/GraphCanvas.vue'
import ConfirmModal from '../components/common/modals/ConfirmModal.vue'
import PromptModal from '../components/common/modals/PromptModal.vue'
import logger from '@/utils/logger'

const router = useRouter()
const route = useRoute()
const explorerStore = useExplorerStore()
const notesStore = useNotesStore()

// State
const sidebarWidth = ref(240)
const isMobileSidebarOpen = ref(false)

// Graph state
const currentGraph = ref<GraphWithData | null>(null)
const selectedGraphId = ref<string | null>(null)

// Undo/Redo state
const undoStack = ref<string[]>([])
const redoStack = ref<string[]>([])

function saveState() {
  if (!currentGraph.value) return
  const state = JSON.stringify({
    nodes: currentGraph.value.nodes,
    edges: currentGraph.value.edges
  })
  undoStack.value.push(state)
  if (undoStack.value.length > 50) undoStack.value.shift()
  redoStack.value = []
}

function undo() {
  if (undoStack.value.length === 0 || !currentGraph.value) return
  const currentState = JSON.stringify({
    nodes: currentGraph.value.nodes,
    edges: currentGraph.value.edges
  })
  redoStack.value.push(currentState)
  
  const prevState = JSON.parse(undoStack.value.pop()!)
  currentGraph.value.nodes = prevState.nodes
  currentGraph.value.edges = prevState.edges
  
  // Note: In a real app, we'd sync the whole graph or individual changes to the server here
}

function redo() {
  if (redoStack.value.length === 0 || !currentGraph.value) return
  const currentState = JSON.stringify({
    nodes: currentGraph.value.nodes,
    edges: currentGraph.value.edges
  })
  undoStack.value.push(currentState)
  
  const nextState = JSON.parse(redoStack.value.pop()!)
  currentGraph.value.nodes = nextState.nodes
  currentGraph.value.edges = nextState.edges
}

// Persistence
const GRAPHS_SELECTED_KEY = 'graphs.selectedGraphId'

function loadSelectedGraph() {
  try {
    const id = localStorage.getItem(GRAPHS_SELECTED_KEY)
    if (id && id !== 'null') {
      selectedGraphId.value = id
      fetchGraph(id)
    }
  } catch (e) {
    // ignore
  }
}

function saveSelectedGraph(id: string | null) {
  try {
    localStorage.setItem(GRAPHS_SELECTED_KEY, id || '')
  } catch (e) {
    // ignore
  }
}

// Drag and drop
const graphContentDragOver = ref(false)

// Delete modals
const deleteGraphModalVisible = ref(false)

// Prompt modal for graph folders
const promptModal = ref({
  visible: false,
  title: 'New Graph Folder',
  placeholder: 'Enter folder name',
  action: 'graph-folder',
  parentId: null as string | null
})

// Refresh tree
async function refreshTree() {
  await explorerStore.fetchAll()
}

// Mobile sidebar toggle
function toggleMobileSidebar() {
  isMobileSidebarOpen.value = !isMobileSidebarOpen.value
}

// Fetch single graph
async function fetchGraph(graphId: string) {
  try {
    const response = await graphsApi.get(graphId)
    currentGraph.value = response.data
  } catch (error) {
    logger.error('Failed to fetch graph:', error)
    currentGraph.value = null
  }
}

// Select a graph
async function selectGraph(graphId: string) {
  selectedGraphId.value = graphId
  explorerStore.selectItem(graphId, 'graph')
  await fetchGraph(graphId)
  saveSelectedGraph(graphId)
  // Close mobile sidebar when a graph is selected
  isMobileSidebarOpen.value = false
}

onMounted(async () => {
  try {
    const q = route.query
    const graphId = typeof q.graph === 'string' ? q.graph : undefined
    if (graphId) {
      await selectGraph(graphId)
    } else {
      // No query params, load last selected graph
      loadSelectedGraph()
    }
  } catch (e) {
    logger.error('Failed to open graph from query params', e)
  }
})

// Explorer tree handlers
function handleExplorerSelect(item: ExplorerItem) {
  explorerStore.selectItem(item.id, item.type as 'note' | 'folder' | 'board' | 'board-folder' | 'graph' | 'graph-folder')
  if (item.type === 'graph') {
    selectGraph(item.id)
  }
}

function handleOpenNote(noteId: string) {
  router.push('/notes')
  notesStore.openNote(noteId)
}

function handleOpenBoard(boardId: string) {
  router.push({ path: '/boards', query: { board: boardId } })
}

function handleCreateNote(folderId: string | null) {
  localStorage.setItem('createNoteInFolder', folderId || '')
  router.push('/notes')
}

function handleCreateFolder(parentId: string | null) {
  localStorage.setItem('createFolderInParent', parentId || '')
  router.push('/notes')
}

function handleCreateBoard(_folderId: string | null) {
  // Navigate to boards and create board there (or handle here if we want)
  // For now, just navigate
  router.push('/boards')
}

function handleCreateBoardFolder(_parentId: string | null) {
  // Navigate to boards
  router.push('/boards')
}

async function handleCreateGraph(folderId: string | null) {
  const name = prompt('Enter graph name:')
  if (name) {
    try {
      const res = await explorerStore.createGraph(name, undefined, folderId)
      if (res) {
        await selectGraph(res.id)
      }
    } catch (e) {
      logger.error('Failed to create graph', e)
    }
  }
}

async function handleCreateGraphFolder(parentId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Graph Folder',
    placeholder: 'Enter folder name',
    action: 'graph-folder',
    parentId: parentId
  }
}

function handleRename(item: ExplorerItem) {
  if (item.type === 'graph-folder') {
    const newName = prompt('Enter new folder name:', item.name)
    if (newName && newName !== item.name) {
      explorerStore.renameGraphFolder(item.id, newName)
    }
  } else if (item.type === 'graph') {
    const newName = prompt('Enter new graph name:', item.name)
    if (newName && newName !== item.name) {
      explorerStore.updateGraph(item.id, { name: newName })
      if (currentGraph.value && currentGraph.value.graph.id === item.id) {
        currentGraph.value.graph.name = newName
      }
    }
  } else {
    // For notes/folders/boards, navigate to respective workspace
    if (item.type === 'note' || item.type === 'folder') router.push('/notes')
    if (item.type === 'board' || item.type === 'board-folder') router.push('/boards')
  }
}

function handleExplorerDelete(item: ExplorerItem) {
  if (item.type === 'graph') {
    selectedGraphId.value = item.id
    deleteGraphModalVisible.value = true
  } else if (item.type === 'graph-folder') {
    if (confirm('Delete this folder and all its contents?')) {
      explorerStore.deleteGraphFolder(item.id)
    }
  } else {
    // For notes/folders/boards, navigate to respective workspace
    if (item.type === 'note' || item.type === 'folder') router.push('/notes')
    if (item.type === 'board' || item.type === 'board-folder') router.push('/boards')
  }
}

async function handlePromptSubmit(name: string) {
  if (name && promptModal.value.parentId !== undefined) {
    await explorerStore.createGraphFolder(name, promptModal.value.parentId)
    promptModal.value.visible = false
  }
}

async function handleDrop(data: { draggedId: string; draggedType: 'note' | 'folder' | 'board' | 'board-folder' | 'graph' | 'graph-folder'; targetId: string | null; targetType: 'folder' | 'note' | 'board' | 'board-folder' | 'graph' | 'graph-folder' | null; position: number; dropPosition: 'before' | 'after' | 'inside' }) {
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
    } else if (data.draggedType === 'graph') {
      await explorerStore.moveGraph(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchGraphsTree()
    } else if (data.draggedType === 'graph-folder') {
      await explorerStore.moveGraphFolder(data.draggedId, data.targetId, data.position)
      await explorerStore.fetchGraphsTree()
    }
  } catch (error) {
    logger.error('Failed to move item:', error)
  }
}

function confirmDeleteGraph() {
  deleteGraphModalVisible.value = true
}

async function handleDeleteGraph() {
  deleteGraphModalVisible.value = false
  if (!selectedGraphId.value) return
  
  try {
    await explorerStore.deleteGraph(selectedGraphId.value)
    selectedGraphId.value = null
    currentGraph.value = null
    saveSelectedGraph(null)
  } catch (error) {
    logger.error('Failed to delete graph:', error)
  }
}

// Graph interaction handlers
// Graph interaction handlers
async function handleNodeMove(nodeId: string, x: number, y: number) {
  if (!currentGraph.value) return
  
  // Optimistic update (local only)
  const node = currentGraph.value.nodes.find(n => n.id === nodeId)
  if (node) {
    node.x = x
    node.y = y
  }
}

async function handleNodeMoveFinished(movedNodes: { id: string, x: number, y: number }[]) {
  if (!currentGraph.value || movedNodes.length === 0) return
  
  saveState()
  try {
    // Update all moved nodes on server
    await Promise.all(movedNodes.map(n => 
      graphNodesApi.update(currentGraph.value!.graph.id, n.id, { x: n.x, y: n.y })
    ))
  } catch (e) {
    logger.error('Failed to update node positions', e)
    undo()
  }
}

async function handleCreateNode(x: number, y: number, type: any, referenceId?: string) {
  if (!currentGraph.value) return
  
  saveState()
  try {
    let label = 'New Node'
    
    // If referenceId is provided, fetch the item to get its name
    if (referenceId) {
      if (type === 'note') {
        const note = notesStore.notes.find(n => n.id === referenceId)
        if (note) label = note.title
      } else if (type === 'board') {
        const board = explorerStore.boards.find(b => b.id === referenceId)
        if (board) label = board.name
      }
    }
    
    const response = await graphNodesApi.create(currentGraph.value.graph.id, {
      node_type: type,
      shape: type === 'note' || type === 'board' ? 'rectangle' : 'circle',
      label: label,
      reference_id: referenceId,
      x,
      y,
      width: 120,
      height: 60
    })
    
    currentGraph.value.nodes.push(response.data)
  } catch (e) {
    logger.error('Failed to create node', e)
    undo()
  }
}

async function handleCreateEdge(sourceId: string, targetId: string) {
  if (!currentGraph.value) return
  
  saveState()
  try {
    const response = await graphEdgesApi.create(currentGraph.value.graph.id, {
      source_node_id: sourceId,
      target_node_id: targetId,
      edge_type: 'line',
      style: 'solid'
    })
    
    currentGraph.value.edges.push(response.data)
  } catch (e) {
    logger.error('Failed to create edge', e)
    undo()
  }
}

async function handleDeleteNode(nodeId: string) {
  if (!currentGraph.value) return
  
  saveState()
  try {
    await graphNodesApi.delete(currentGraph.value.graph.id, nodeId)
    currentGraph.value.nodes = currentGraph.value.nodes.filter(n => n.id !== nodeId)
    // Also remove connected edges
    currentGraph.value.edges = currentGraph.value.edges.filter(e => 
      e.source_node_id !== nodeId && e.target_node_id !== nodeId
    )
  } catch (e) {
    logger.error('Failed to delete node', e)
    undo()
  }
}

async function handleDeleteEdge(edgeId: string) {
  if (!currentGraph.value) return
  
  saveState()
  try {
    await graphEdgesApi.delete(currentGraph.value.graph.id, edgeId)
    currentGraph.value.edges = currentGraph.value.edges.filter(e => e.id !== edgeId)
  } catch (e) {
    logger.error('Failed to delete edge', e)
    undo()
  }
}

async function handleRenameNode(nodeId: string) {
  if (!currentGraph.value) return
  
  const node = currentGraph.value.nodes.find(n => n.id === nodeId)
  if (!node) return
  
  const newLabel = prompt('Enter new label:', node.label)
  if (newLabel !== null && newLabel !== node.label) {
    saveState()
    try {
      await graphNodesApi.update(currentGraph.value.graph.id, nodeId, { label: newLabel })
      node.label = newLabel
    } catch (e) {
      logger.error('Failed to rename node', e)
      undo()
    }
  }
}

async function handleUpdateNode(nodeId: string, updates: any) {
  if (!currentGraph.value) return
  
  const node = currentGraph.value.nodes.find(n => n.id === nodeId)
  if (!node) return
  
  saveState()
  try {
    await graphNodesApi.update(currentGraph.value.graph.id, nodeId, updates)
    Object.assign(node, updates)
  } catch (e) {
    logger.error('Failed to update node', e)
    undo()
  }
}

async function handleUpdateEdge(edgeId: string, updates: any) {
  if (!currentGraph.value) return
  
  const edge = currentGraph.value.edges.find(e => e.id === edgeId)
  if (!edge) return
  
  saveState()
  try {
    await graphEdgesApi.update(currentGraph.value.graph.id, edgeId, updates)
    Object.assign(edge, updates)
  } catch (e) {
    logger.error('Failed to update edge', e)
    undo()
  }
}

function handleNodeSelect(_nodeId: string | null) {
  // TODO: Show node properties in sidebar
}

function handleEdgeSelect(_edgeId: string | null) {
  // TODO: Show edge properties in sidebar
}

function handleOpenNodeFromGraph(node: any) {
  if (node.node_type === 'note' && node.reference_id) {
    handleOpenNote(node.reference_id)
  } else if (node.node_type === 'board' && node.reference_id) {
    handleOpenBoard(node.reference_id)
  }
}

// Graph content drag handlers
function onGraphContentDragEnter(event: DragEvent) {
  const types = event.dataTransfer?.types || []
  const typesArray = Array.from(types)
  const isExplorer = typesArray.includes('application/x-explorer-item')
  if (!isExplorer) return
  
  event.preventDefault()
  graphContentDragOver.value = true
}

function onGraphContentDragOver(event: DragEvent) {
  const types = event.dataTransfer?.types || []
  const typesArray = Array.from(types)
  const isExplorer = typesArray.includes('application/x-explorer-item')
  if (!isExplorer) return
  
  event.preventDefault()
  if (event.dataTransfer) event.dataTransfer.dropEffect = 'copy'
  graphContentDragOver.value = true
}

function onGraphContentDragLeave(event: DragEvent) {
  if ((event.target as HTMLElement).classList?.contains('graph-content')) {
    graphContentDragOver.value = false
  }
}

function onGraphContentDrop(event: DragEvent) {
  event.preventDefault()
  graphContentDragOver.value = false
  
  const treeData = event.dataTransfer?.getData('application/x-explorer-item')
  
  if (treeData) {
    try {
      let parsed: any
      if (treeData.startsWith('{')) {
        parsed = JSON.parse(treeData)
      } else {
        parsed = { id: treeData, type: 'note' }
      }
      const { id, type } = parsed
      
      if (type === 'note') {
        notesStore.openNote(id)
        router.push('/notes')
      } else if (type === 'board') {
        router.push({ path: '/boards', query: { board: id } })
      } else if (type === 'graph') {
        selectGraph(id)
      }
    } catch (e) {
      logger.error('Failed to parse dropped item:', e)
    }
  }
}
</script>

<style scoped>
.graphs-workspace {
  display: flex;
  height: calc(100vh - 56px);
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

.graph-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.graph-header {
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

.graph-description {
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

.btn-icon.danger:hover {
  background: rgba(248, 81, 73, 0.1);
  color: var(--danger);
}

.graph-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.graph-canvas-wrapper {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  position: relative;
  overflow: hidden;
}

.graph-canvas-placeholder {
  text-align: center;
  color: var(--text-muted);
}

.graph-canvas-placeholder p {
  margin-top: 1rem;
  font-size: 1.125rem;
  font-weight: 500;
}

.graph-canvas-placeholder .sub-text {
  font-size: 0.875rem;
  margin-top: 0.5rem;
  opacity: 0.7;
}

.graph-empty {
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
</style>