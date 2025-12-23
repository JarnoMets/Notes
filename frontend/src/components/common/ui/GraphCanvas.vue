<template>
  <div 
    class="graph-canvas-container" 
    ref="containerRef"
    @mousedown="handleMouseDown"
    @mousemove="handleMouseMove"
    @mouseup="handleMouseUp"
    @mouseleave="handleMouseUp"
    @wheel.prevent="handleWheel"
    @dragover.prevent="onDragOver"
    @drop.prevent="onDrop"
  >
    <svg 
      class="graph-svg" 
      width="100%" 
      height="100%"
    >
      <defs>
        <!-- Arrow markers -->
        <marker id="arrow-end" markerWidth="10" markerHeight="10" refX="18" refY="3" orient="auto" markerUnits="strokeWidth">
          <path d="M0,0 L0,6 L9,3 z" fill="#999" />
        </marker>
        <marker id="arrow-end-selected" markerWidth="10" markerHeight="10" refX="18" refY="3" orient="auto" markerUnits="strokeWidth">
          <path d="M0,0 L0,6 L9,3 z" fill="#007bff" />
        </marker>
      </defs>
      
      <g :transform="`translate(${transform.x}, ${transform.y}) scale(${transform.k})`">
        <!-- Edges -->
        <g class="edges-layer">
          <path 
            v-for="edge in edges" 
            :key="edge.id"
            :d="getEdgePath(edge)"
            :stroke="selectedEdgeId === edge.id ? '#007bff' : (edge.color || '#999')"
            :stroke-width="edge.thickness || 2"
            :stroke-dasharray="getEdgeDashArray(edge.style)"
            fill="none"
            :marker-end="selectedEdgeId === edge.id ? 'url(#arrow-end-selected)' : 'url(#arrow-end)'"
            class="graph-edge"
            :class="{ selected: selectedEdgeId === edge.id }"
            @click.stop="selectEdge(edge)"
          />
          
          <!-- Temporary Edge (Linking) -->
          <line 
            v-if="isLinking && linkingStartNode"
            :x1="linkingStartNode.x"
            :y1="linkingStartNode.y"
            :x2="linkingEndPos.x"
            :y2="linkingEndPos.y"
            stroke="#007bff"
            stroke-width="2"
            stroke-dasharray="5,5"
            marker-end="url(#arrow-end-selected)"
            style="pointer-events: none;"
          />
        </g>
        
        <!-- Nodes -->
        <g class="nodes-layer">
          <g 
            v-for="node in nodes" 
            :key="node.id"
            class="graph-node"
            :class="{ selected: selectedNodeId === node.id }"
            :transform="`translate(${node.x}, ${node.y})`"
            @mousedown.stop="handleNodeMouseDown($event, node)"
            @mouseup.stop="handleNodeMouseUp($event, node)"
            @click.stop="selectNode(node)"
            @dblclick.stop="handleNodeDoubleClick(node)"
          >
            <!-- Node Shape: Rectangle -->
            <rect 
              v-if="node.shape === 'rectangle' || node.shape === 'rounded_rect'"
              :x="-node.width / 2"
              :y="-node.height / 2"
              :width="node.width"
              :height="node.height"
              :rx="node.shape === 'rounded_rect' ? 8 : 0"
              :ry="node.shape === 'rounded_rect' ? 8 : 0"
              :fill="node.color || '#fff'"
              :stroke="selectedNodeId === node.id ? '#007bff' : (node.border_color || '#333')"
              :stroke-width="selectedNodeId === node.id ? 2 : 1"
            />
            
            <!-- Node Shape: Circle -->
            <circle 
              v-else-if="node.shape === 'circle'"
              :r="node.width / 2"
              :fill="node.color || '#fff'"
              :stroke="selectedNodeId === node.id ? '#007bff' : (node.border_color || '#333')"
              :stroke-width="selectedNodeId === node.id ? 2 : 1"
            />
            
            <!-- Node Label -->
            <text 
              text-anchor="middle" 
              dominant-baseline="middle"
              :fill="node.text_color || '#333'"
              :font-size="node.font_size || 14"
              style="pointer-events: none; user-select: none;"
            >
              {{ truncateLabel(node.label, node.width) }}
            </text>
            
            <!-- Type Icon (if linked) -->
            <g v-if="node.node_type === 'note'" transform="translate(-8, -25)">
               <!-- Simple file icon -->
               <path d="M4 0h8l4 4v12h-12z" fill="none" stroke="#666" stroke-width="1" transform="scale(0.8)"/>
            </g>
            <g v-if="node.node_type === 'board'" transform="translate(-8, -25)">
               <!-- Simple board icon -->
               <rect x="2" y="2" width="12" height="12" fill="none" stroke="#666" stroke-width="1" transform="scale(0.8)"/>
            </g>
          </g>
        </g>
      </g>
    </svg>
    
    <!-- Controls -->
    <div class="graph-controls">
      <button class="control-btn" @click="zoomIn" title="Zoom In">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="12" y1="5" x2="12" y2="19"></line><line x1="5" y1="12" x2="19" y2="12"></line></svg>
      </button>
      <button class="control-btn" @click="zoomOut" title="Zoom Out">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><line x1="5" y1="12" x2="19" y2="12"></line></svg>
      </button>
      <button class="control-btn" @click="resetZoom" title="Reset Zoom">
        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import type { GraphNode, GraphEdge, NodeType, NodeShape, EdgeStyle } from '@/types/graph'
import logger from '@/utils/logger'

const props = defineProps<{
  nodes: GraphNode[]
  edges: GraphEdge[]
  readOnly?: boolean
}>()

const emit = defineEmits<{
  'node-move': [nodeId: string, x: number, y: number]
  'node-select': [nodeId: string | null]
  'edge-select': [edgeId: string | null]
  'create-node': [x: number, y: number, type: NodeType, referenceId?: string]
  'create-edge': [sourceId: string, targetId: string]
  'open-node': [node: GraphNode]
}>()

// State
const containerRef = ref<HTMLElement | null>(null)
const transform = ref({ x: 0, y: 0, k: 1 })
const isDraggingCanvas = ref(false)
const isDraggingNode = ref(false)
const isLinking = ref(false)
const linkingStartNode = ref<GraphNode | null>(null)
const linkingEndPos = ref({ x: 0, y: 0 })
const dragStart = { x: 0, y: 0 }
const lastMousePos = { x: 0, y: 0 }
const selectedNodeId = ref<string | null>(null)
const selectedEdgeId = ref<string | null>(null)
const draggedNodeId = ref<string | null>(null)

// Helper to get node by ID
function getNode(id: string): GraphNode | undefined {
  return props.nodes.find(n => n.id === id)
}

// Edge path calculation
function getEdgePath(edge: GraphEdge): string {
  const source = getNode(edge.source_node_id)
  const target = getNode(edge.target_node_id)
  
  if (!source || !target) return ''
  
  // Simple straight line for now
  // Could be improved to start/end at border of shape
  return `M ${source.x},${source.y} L ${target.x},${target.y}`
}

function getEdgeDashArray(style: EdgeStyle): string {
  switch (style) {
    case 'dashed': return '5,5'
    case 'dotted': return '2,2'
    default: return 'none'
  }
}

function truncateLabel(label: string, width: number): string {
  if (!label) return ''
  const charWidth = 8 // Approximate
  const maxChars = Math.floor((width - 10) / charWidth)
  if (label.length > maxChars) {
    return label.substring(0, maxChars - 1) + '...'
  }
  return label
}

// Interaction Handlers
function handleMouseDown(event: MouseEvent) {
  if (props.readOnly) return
  
  // If clicked on background
  if (event.target === containerRef.value || (event.target as Element).classList.contains('graph-svg')) {
    isDraggingCanvas.value = true
    dragStart.x = event.clientX
    dragStart.y = event.clientY
    lastMousePos.x = event.clientX
    lastMousePos.y = event.clientY
    
    // Deselect
    selectedNodeId.value = null
    selectedEdgeId.value = null
    emit('node-select', null)
    emit('edge-select', null)
  }
}

function handleNodeMouseDown(event: MouseEvent, node: GraphNode) {
  if (props.readOnly) return
  
  if (event.shiftKey) {
    // Start linking
    isLinking.value = true
    linkingStartNode.value = node
    linkingEndPos.value = { x: node.x, y: node.y }
    event.stopPropagation()
  } else {
    isDraggingNode.value = true
    draggedNodeId.value = node.id
    dragStart.x = event.clientX
    dragStart.y = event.clientY
    lastMousePos.x = event.clientX
    lastMousePos.y = event.clientY
  }
}

function handleMouseMove(event: MouseEvent) {
  if (props.readOnly) return
  
  if (isDraggingCanvas.value) {
    const dx = event.clientX - lastMousePos.x
    const dy = event.clientY - lastMousePos.y
    
    transform.value.x += dx
    transform.value.y += dy
    
    lastMousePos.x = event.clientX
    lastMousePos.y = event.clientY
  } else if (isDraggingNode.value && draggedNodeId.value) {
    const dx = (event.clientX - lastMousePos.x) / transform.value.k
    const dy = (event.clientY - lastMousePos.y) / transform.value.k
    
    const node = getNode(draggedNodeId.value)
    if (node) {
      // Optimistic update locally (parent should update prop eventually)
      node.x += dx
      node.y += dy
      
      // Emit move event (maybe throttled in real app)
      emit('node-move', node.id, node.x, node.y)
    }
    
    lastMousePos.x = event.clientX
    lastMousePos.y = event.clientY
  } else if (isLinking.value && linkingStartNode.value) {
    // Update linking end pos
    const rect = containerRef.value?.getBoundingClientRect()
    if (rect) {
      const mouseX = event.clientX - rect.left
      const mouseY = event.clientY - rect.top
      
      const worldX = (mouseX - transform.value.x) / transform.value.k
      const worldY = (mouseY - transform.value.y) / transform.value.k
      
      linkingEndPos.value = { x: worldX, y: worldY }
    }
  }
}

function handleMouseUp() {
  isDraggingCanvas.value = false
  isDraggingNode.value = false
  draggedNodeId.value = null
  
  // If we were linking and mouseup happened on background, cancel linking
  if (isLinking.value) {
    isLinking.value = false
    linkingStartNode.value = null
  }
}

function handleNodeMouseUp(event: MouseEvent, node: GraphNode) {
  if (isLinking.value && linkingStartNode.value) {
    if (linkingStartNode.value.id !== node.id) {
      emit('create-edge', linkingStartNode.value.id, node.id)
    }
    // Stop linking
    isLinking.value = false
    linkingStartNode.value = null
    event.stopPropagation()
  }
}

function handleWheel(event: WheelEvent) {
  const zoomIntensity = 0.1
  const direction = event.deltaY > 0 ? -1 : 1
  const factor = 1 + (direction * zoomIntensity)
  
  const newK = Math.max(0.1, Math.min(5, transform.value.k * factor))
  
  // Zoom towards mouse pointer
  // Current mouse pos in SVG coords
  const rect = containerRef.value?.getBoundingClientRect()
  if (!rect) return
  
  const mouseX = event.clientX - rect.left
  const mouseY = event.clientY - rect.top
  
  // Calculate new offset to keep mouse point stable
  // (mouseX - tx) / k = worldX
  // newTx = mouseX - worldX * newK
  
  const worldX = (mouseX - transform.value.x) / transform.value.k
  const worldY = (mouseY - transform.value.y) / transform.value.k
  
  transform.value.x = mouseX - worldX * newK
  transform.value.y = mouseY - worldY * newK
  transform.value.k = newK
}

function zoomIn() {
  const center = getCenter()
  zoomTo(transform.value.k * 1.2, center)
}

function zoomOut() {
  const center = getCenter()
  zoomTo(transform.value.k / 1.2, center)
}

function resetZoom() {
  transform.value = { x: 0, y: 0, k: 1 }
}

function getCenter() {
  if (!containerRef.value) return { x: 0, y: 0 }
  const rect = containerRef.value.getBoundingClientRect()
  return { x: rect.width / 2, y: rect.height / 2 }
}

function zoomTo(k: number, center: { x: number, y: number }) {
  const newK = Math.max(0.1, Math.min(5, k))
  
  // Keep center stable
  const worldX = (center.x - transform.value.x) / transform.value.k
  const worldY = (center.y - transform.value.y) / transform.value.k
  
  transform.value.x = center.x - worldX * newK
  transform.value.y = center.y - worldY * newK
  transform.value.k = newK
}

function selectNode(node: GraphNode) {
  selectedNodeId.value = node.id
  selectedEdgeId.value = null
  emit('node-select', node.id)
  emit('edge-select', null)
}

function selectEdge(edge: GraphEdge) {
  selectedEdgeId.value = edge.id
  selectedNodeId.value = null
  emit('edge-select', edge.id)
  emit('node-select', null)
}

function handleNodeDoubleClick(node: GraphNode) {
  emit('open-node', node)
}

// Drag and Drop from Explorer
function onDragOver(event: DragEvent) {
  const hasExplorerType = event.dataTransfer?.types?.includes?.('application/x-explorer-item') || false
  if (hasExplorerType) {
    event.dataTransfer!.dropEffect = 'copy'
  }
}

function onDrop(event: DragEvent) {
  const data = event.dataTransfer?.getData('application/x-explorer-item')
  if (!data) return
  
  try {
    const item = JSON.parse(data)
    const rect = containerRef.value?.getBoundingClientRect()
    if (!rect) return
    
    // Calculate drop position in graph coordinates
    const mouseX = event.clientX - rect.left
    const mouseY = event.clientY - rect.top
    
    const worldX = (mouseX - transform.value.x) / transform.value.k
    const worldY = (mouseY - transform.value.y) / transform.value.k
    
    // Emit create node event
    if (item.type === 'note') {
      emit('create-node', worldX, worldY, 'note', item.id)
    } else if (item.type === 'board') {
      emit('create-node', worldX, worldY, 'board', item.id)
    }
  } catch (e) {
    logger.error('Failed to parse drop data', e)
  }
}
</script>

<style scoped>
.graph-canvas-container {
  width: 100%;
  height: 100%;
  background-color: #f5f5f5;
  position: relative;
  overflow: hidden;
  user-select: none;
}

.graph-svg {
  display: block;
  cursor: grab;
}

.graph-svg:active {
  cursor: grabbing;
}

.graph-node {
  cursor: pointer;
  transition: opacity 0.2s;
}

.graph-node:hover {
  opacity: 0.9;
}

.graph-edge {
  cursor: pointer;
  transition: stroke-width 0.2s;
}

.graph-edge:hover {
  stroke-width: 3;
}

.graph-controls {
  position: absolute;
  bottom: 20px;
  right: 20px;
  display: flex;
  flex-direction: column;
  gap: 8px;
  background: white;
  padding: 8px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.1);
}

.control-btn {
  width: 32px;
  height: 32px;
  border: 1px solid #e0e0e0;
  background: white;
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: #666;
}

.control-btn:hover {
  background: #f0f0f0;
  color: #333;
}
</style>
