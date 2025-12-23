<template>
  <div class="graph-pane">
    <div v-if="loading" class="loading">Loading graph...</div>
    <div v-else-if="error" class="error">{{ error }}</div>
    <div v-else-if="graphData" class="graph-container">
      <div class="graph-toolbar">
        <span class="graph-name">{{ graphData.graph.name }}</span>
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
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import GraphCanvas from '../common/ui/GraphCanvas.vue'
import { graphsApi, graphNodesApi, graphEdgesApi } from '@/api/graphs'
import type { GraphWithData, GraphNode } from '@/types'
import logger from '@/utils/logger'

const props = defineProps<{
  graphId?: string
  paneId: string
}>()

const graphData = ref<GraphWithData | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)

async function fetchGraph() {
  if (!props.graphId) return
  
  loading.value = true
  error.value = null
  
  try {
    const response = await graphsApi.get(props.graphId)
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

// Handlers (copied/adapted from GraphsWorkspace)
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
    // Logic to fetch label from referenceId would go here (needs store access)
    
    const response = await graphNodesApi.create(graphData.value.graph.id, {
      node_type: type,
      shape: type === 'note' || type === 'board' ? 'rectangle' : 'circle',
      label: label,
      reference_id: referenceId,
      x,
      y,
      width: 120,
      height: 60
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
      edge_type: 'line',
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
    // We need a way to open a note in a new tab/split from here
    // For now, use the global handler or store
    // Ideally, emit an event to the WindowManager
    (window as any).__openNote?.(node.reference_id)
  } else if (node.node_type === 'board' && node.reference_id) {
    (window as any).__openBoard?.(node.reference_id)
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

.graph-toolbar {
  height: 32px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
  display: flex;
  align-items: center;
  padding: 0 12px;
}

.graph-name {
  font-weight: 600;
  font-size: 13px;
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
