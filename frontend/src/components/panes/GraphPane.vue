<template>
  <div class="graph-pane">
    <div v-if="loading" class="loading">Loading graph...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else-if="graphData" class="graph-container">
      <div class="graph-status" :class="{ syncing: pendingSyncs > 0 }">
        <Icon :name="pendingSyncs > 0 ? 'refresh' : 'check'" :size="14" />
        <span>{{ pendingSyncs > 0 ? 'Saving...' : 'Saved' }}</span>
      </div>
      <GraphCanvas
        :nodes="graphData.nodes"
        :edges="graphData.edges"
        @node-move="handleNodeMove"
        @create-node="handleCreateNode"
        @create-edge="handleCreateEdge"
        @node-select="handleNodeSelect"
        @edge-select="handleEdgeSelect"
        @open-node="handleOpenNode"
        @delete-node="handleDeleteNode"
        @delete-edge="handleDeleteEdge"
        @rename-node="handleRenameNode"
        @update-node="handleUpdateNode"
        @update-edge="handleUpdateEdge"
      />
    </div>

    <!-- Local Prompt Modal for Bubbles -->
    <PromptModal
      :visible="bubbleModal.visible"
      title="New Bubble"
      placeholder="Enter bubble label..."
      confirm-text="Create"
      @submit="handleBubbleSubmit"
      @cancel="bubbleModal.visible = false"
    />

    <PromptModal
      :visible="renameModal.visible"
      title="Rename Node"
      placeholder="Enter new label..."
      :initial-value="renameModal.initialValue"
      confirm-text="Rename"
      @submit="handleRenameSubmit"
      @cancel="renameModal.visible = false"
    />

    <ResourceSelectModal
      :visible="resourceModal.visible"
      :type="resourceModal.type"
      :title="resourceModal.type === 'note' ? 'Select Note' : 'Select Board'"
      @select="handleResourceSelect"
      @cancel="resourceModal.visible = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import GraphCanvas from '../graphs/GraphCanvas.vue'
import PromptModal from '../modals/PromptModal.vue'
import ResourceSelectModal from '../modals/ResourceSelectModal.vue'
import Icon from '../ui/Icon.vue'
import { graphsApi, graphNodesApi, graphEdgesApi } from '@/api/graphs'
import type { GraphWithData, GraphNode } from '@/types'
import { useExplorerStore } from '@/stores/explorer'
import logger from '@/utils/logger'

const props = defineProps<{
  graphId?: string
  paneId: string
}>()

const explorerStore = useExplorerStore()
const graphData = ref<GraphWithData | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const pendingSyncs = ref(0)

async function withSync(fn: () => Promise<any>) {
  pendingSyncs.value++
  try {
    await fn()
  } finally {
    setTimeout(() => {
      pendingSyncs.value--
    }, 500)
  }
}

const bubbleModal = ref({
  visible: false,
  x: 0,
  y: 0
})

const renameModal = ref({
  visible: false,
  nodeId: '',
  initialValue: ''
})

const resourceModal = ref({
  visible: false,
  type: 'note' as 'note' | 'board',
  x: 0,
  y: 0
})

async function fetchGraph() {
  if (!props.graphId) return
  
  loading.value = true
  error.value = null
  
  try {
    const response = await graphsApi.getWithData(props.graphId)
    graphData.value = response.data
  } catch (e) {
    logger.error('Failed to fetch graph', e)
    error.value = 'Failed to load graph'
  } finally {
    loading.value = false
  }
}

watch(() => props.graphId, fetchGraph)
onMounted(fetchGraph)

// Handlers
async function handleNodeMove(nodeId: string, x: number, y: number) {
  if (!graphData.value) return
  
  const node = graphData.value.nodes.find(n => n.id === nodeId)
  if (node) {
    node.x = x
    node.y = y
  }
  
  withSync(async () => {
    try {
      await graphNodesApi.update(graphData.value!.graph.id, nodeId, { x, y })
    } catch (e) {
      logger.error('Failed to update node position', e)
    }
  })
}

async function handleCreateNode(x: number, y: number, type: any, referenceId?: string) {
  if (!graphData.value) return
  
  try {
    let label = 'New Node'
    
    if (referenceId) {
      if (type === 'note') {
        const note = explorerStore.getNoteById(referenceId)
        if (note) label = note.title
      } else if (type === 'board') {
        const board = explorerStore.getBoardById(referenceId)
        if (board) label = board.name
      }
    } else if (type === 'bubble') {
      bubbleModal.value = { visible: true, x, y }
      return // Wait for modal submit
    } else if (type === 'note' || type === 'board') {
      resourceModal.value = { visible: true, type, x, y }
      return // Wait for modal submit
    }
    
    await createNode(x, y, type, label, referenceId)
  } catch (e) {
    logger.error('Failed to create node', e)
  }
}

async function handleBubbleSubmit(label: string) {
  bubbleModal.value.visible = false
  await createNode(bubbleModal.value.x, bubbleModal.value.y, 'bubble', label)
}

async function handleResourceSelect(itemId: string) {
  const { type, x, y } = resourceModal.value
  resourceModal.value.visible = false
  
  let label = 'New Node'
  if (type === 'note') {
    const note = explorerStore.getNoteById(itemId)
    if (note) label = note.title
  } else if (type === 'board') {
    const board = explorerStore.getBoardById(itemId)
    if (board) label = board.name
  }
  
  await createNode(x, y, type, label, itemId)
}

async function createNode(x: number, y: number, type: any, label: string, referenceId?: string) {
  if (!graphData.value) return

  withSync(async () => {
    const response = await graphNodesApi.create(graphData.value!.graph.id, {
      node_type: type,
      shape: type === 'note' || type === 'board' ? 'rectangle' : 'circle',
      label: label,
      reference_id: referenceId,
      x,
      y,
      width: type === 'note' || type === 'board' ? 140 : 80,
      height: type === 'note' || type === 'board' ? 60 : 80
    })
    
    graphData.value!.nodes.push(response.data)
  })
}

async function handleCreateEdge(sourceId: string, targetId: string) {
  if (!graphData.value) return
  
  withSync(async () => {
    try {
      const response = await graphEdgesApi.create(graphData.value!.graph.id, {
        source_node_id: sourceId,
        target_node_id: targetId,
        edge_type: 'arrow',
        style: 'solid'
      })
      
      graphData.value!.edges.push(response.data)
    } catch (e) {
      logger.error('Failed to create edge', e)
    }
  })
}

function handleNodeSelect(_nodeId: string | null) {
  // TODO
}

function handleEdgeSelect(_edgeId: string | null) {
  // TODO
}

function handleOpenNode(node: GraphNode) {
  if (!node.reference_id) {
    // If it's a bubble or has no reference, maybe rename it?
    handleRenameNode(node.id)
    return
  }

  if (node.node_type === 'note') {
    (window as any).__openNote?.(node.reference_id)
  } else if (node.node_type === 'board') {
    (window as any).__openBoard?.(node.reference_id)
  } else if (node.node_type === 'graph') {
    (window as any).__openGraph?.(node.reference_id)
  }
}

async function handleDeleteNode(nodeId: string) {
  if (!graphData.value) return
  
  const originalNodes = [...graphData.value.nodes]
  const originalEdges = [...graphData.value.edges]

  // Optimistic delete
  graphData.value.nodes = graphData.value.nodes.filter(n => n.id !== nodeId)
  graphData.value.edges = graphData.value.edges.filter(e => 
    e.source_node_id !== nodeId && e.target_node_id !== nodeId
  )

  withSync(async () => {
    try {
      await graphNodesApi.delete(graphData.value!.graph.id, nodeId)
    } catch (e) {
      logger.error('Failed to delete node', e)
      graphData.value!.nodes = originalNodes
      graphData.value!.edges = originalEdges
    }
  })
}

async function handleDeleteEdge(edgeId: string) {
  if (!graphData.value) return
  
  const originalEdges = [...graphData.value.edges]
  graphData.value.edges = graphData.value.edges.filter(e => e.id !== edgeId)

  withSync(async () => {
    try {
      await graphEdgesApi.delete(graphData.value!.graph.id, edgeId)
    } catch (e) {
      logger.error('Failed to delete edge', e)
      graphData.value!.edges = originalEdges
    }
  })
}

async function handleRenameNode(nodeId: string) {
  if (!graphData.value) return
  
  const node = graphData.value.nodes.find(n => n.id === nodeId)
  if (!node) return
  
  renameModal.value = {
    visible: true,
    nodeId,
    initialValue: node.label
  }
}

async function handleRenameSubmit(newLabel: string) {
  if (!graphData.value) return
  
  const nodeId = renameModal.value.nodeId
  const node = graphData.value.nodes.find(n => n.id === nodeId)
  if (!node) return
  
  renameModal.value.visible = false
  if (newLabel === node.label) return
  
  const originalLabel = node.label
  node.label = newLabel

  withSync(async () => {
    try {
      const response = await graphNodesApi.update(graphData.value!.graph.id, nodeId, { label: newLabel })
      node.label = response.data.label
    } catch (e) {
      logger.error('Failed to rename node', e)
      node.label = originalLabel
    }
  })
}

async function handleUpdateNode(nodeId: string, updates: any) {
  if (!graphData.value) return
  
  const node = graphData.value.nodes.find(n => n.id === nodeId)
  if (!node) return
  
  const originalState = { ...node }
  Object.assign(node, updates)

  withSync(async () => {
    try {
      const response = await graphNodesApi.update(graphData.value!.graph.id, nodeId, updates)
      Object.assign(node, response.data)
    } catch (e) {
      logger.error('Failed to update node', e)
      Object.assign(node, originalState)
    }
  })
}

async function handleUpdateEdge(edgeId: string, updates: any) {
  if (!graphData.value) return
  
  const edge = graphData.value.edges.find(e => e.id === edgeId)
  if (!edge) return
  
  const originalState = { ...edge }
  Object.assign(edge, updates)

  withSync(async () => {
    try {
      const response = await graphEdgesApi.update(graphData.value!.graph.id, edgeId, updates)
      Object.assign(edge, response.data)
    } catch (e) {
      logger.error('Failed to update edge', e)
      Object.assign(edge, originalState)
    }
  })
}
</script>

<style scoped>
.graph-pane {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  position: relative;
}

.graph-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  position: relative;
}

.graph-status {
  position: absolute;
  top: 10px;
  right: 10px;
  z-index: 10;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  padding: 4px 8px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 12px;
  color: var(--text-muted);
  pointer-events: none;
  transition: all 0.3s ease;
  opacity: 0.8;
}

.graph-status.syncing {
  color: var(--accent);
  opacity: 1;
}

.graph-status .icon {
  animation: none;
}

.graph-status.syncing .icon {
  animation: spin 2s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.loading, .error {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
}

.error {
  color: var(--danger);
}
</style>
