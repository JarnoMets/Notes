<template>
  <div class="graph-pane">
    <div v-if="loading" class="loading">Loading graph...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else-if="graphData" class="graph-container">
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
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import GraphCanvas from '../graphs/GraphCanvas.vue'
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
  
  try {
    await graphNodesApi.update(graphData.value.graph.id, nodeId, { x, y })
  } catch (e) {
    logger.error('Failed to update node position', e)
  }
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
      const input = prompt('Enter bubble label:', 'New Bubble')
      if (input === null) return // Cancelled
      label = input || 'New Bubble'
    }
    
    const response = await graphNodesApi.create(graphData.value.graph.id, {
      node_type: type,
      shape: type === 'note' || type === 'board' ? 'rectangle' : 'circle',
      label: label,
      reference_id: referenceId,
      x,
      y,
      width: type === 'note' || type === 'board' ? 140 : 80,
      height: type === 'note' || type === 'board' ? 60 : 80
    })
    
    graphData.value.nodes.push(response.data)
  } catch (e) {
    logger.error('Failed to create node', e)
  }
}

async function handleCreateEdge(sourceId: string, targetId: string) {
  if (!graphData.value) return
  
  try {
    const response = await graphEdgesApi.create(graphData.value.graph.id, {
      source_node_id: sourceId,
      target_node_id: targetId,
      edge_type: 'arrow',
      style: 'solid'
    })
    
    graphData.value.edges.push(response.data)
  } catch (e) {
    logger.error('Failed to create edge', e)
  }
}

function handleNodeSelect(_nodeId: string | null) {
  // TODO
}

function handleEdgeSelect(_edgeId: string | null) {
  // TODO
}

function handleOpenNode(node: GraphNode) {
  if (node.node_type === 'note' && node.reference_id) {
    (window as any).__openNote?.(node.reference_id)
  } else if (node.node_type === 'board' && node.reference_id) {
    (window as any).__openBoard?.(node.reference_id)
  }
}

async function handleDeleteNode(nodeId: string) {
  if (!graphData.value) return
  
  try {
    await graphNodesApi.delete(graphData.value.graph.id, nodeId)
    graphData.value.nodes = graphData.value.nodes.filter(n => n.id !== nodeId)
    // Also remove connected edges
    graphData.value.edges = graphData.value.edges.filter(e => 
      e.source_node_id !== nodeId && e.target_node_id !== nodeId
    )
  } catch (e) {
    logger.error('Failed to delete node', e)
  }
}

async function handleDeleteEdge(edgeId: string) {
  if (!graphData.value) return
  
  try {
    await graphEdgesApi.delete(graphData.value.graph.id, edgeId)
    graphData.value.edges = graphData.value.edges.filter(e => e.id !== edgeId)
  } catch (e) {
    logger.error('Failed to delete edge', e)
  }
}

async function handleRenameNode(nodeId: string) {
  if (!graphData.value) return
  
  const node = graphData.value.nodes.find(n => n.id === nodeId)
  if (!node) return
  
  const newLabel = prompt('Enter new label:', node.label)
  if (newLabel === null || newLabel === node.label) return
  
  try {
    const response = await graphNodesApi.update(graphData.value.graph.id, nodeId, { label: newLabel })
    node.label = response.data.label
  } catch (e) {
    logger.error('Failed to rename node', e)
  }
}

async function handleUpdateNode(nodeId: string, updates: any) {
  if (!graphData.value) return
  
  const node = graphData.value.nodes.find(n => n.id === nodeId)
  if (!node) return
  
  try {
    const response = await graphNodesApi.update(graphData.value.graph.id, nodeId, updates)
    // Update local state
    Object.assign(node, response.data)
  } catch (e) {
    logger.error('Failed to update node', e)
  }
}

async function handleUpdateEdge(edgeId: string, updates: any) {
  if (!graphData.value) return
  
  const edge = graphData.value.edges.find(e => e.id === edgeId)
  if (!edge) return
  
  try {
    const response = await graphEdgesApi.update(graphData.value.graph.id, edgeId, updates)
    // Update local state
    Object.assign(edge, response.data)
  } catch (e) {
    logger.error('Failed to update edge', e)
  }
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
