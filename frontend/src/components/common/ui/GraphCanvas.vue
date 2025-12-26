<template>
  <div 
    class="graph-canvas-container" 
    ref="containerRef"
    @mousedown="handleMouseDown"
    @touchstart="handleTouchStart"
    @touchmove="handleTouchMove"
    @touchend="handleTouchEnd"
    @wheel.prevent="handleWheel"
    @dragover.prevent="onDragOver"
    @drop.stop.prevent="onDrop"
    @contextmenu.prevent="handleCanvasContextMenu"
  >
    <svg 
      class="graph-svg" 
      width="100%" 
      height="100%"
    >
      <defs>
        <!-- Grid Pattern -->
        <pattern id="grid" width="40" height="40" patternUnits="userSpaceOnUse">
          <path d="M 40 0 L 0 0 0 40" fill="none" stroke="var(--border-primary)" stroke-width="0.5" opacity="0.3"/>
        </pattern>
        <pattern id="grid-fine" width="10" height="10" patternUnits="userSpaceOnUse">
          <path d="M 10 0 L 0 0 0 10" fill="none" stroke="var(--border-primary)" stroke-width="0.2" opacity="0.2"/>
        </pattern>

        <!-- Arrow markers -->
        <marker id="arrow-end" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto" markerUnits="strokeWidth">
          <path d="M0,0 L0,6 L9,3 z" fill="var(--text-muted)" />
        </marker>
        <marker id="arrow-end-selected" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto" markerUnits="strokeWidth">
          <path d="M0,0 L0,6 L9,3 z" fill="var(--accent)" />
        </marker>
        <marker id="arrow-start" markerWidth="10" markerHeight="10" refX="0" refY="3" orient="auto" markerUnits="strokeWidth">
          <path d="M9,0 L9,6 L0,3 z" fill="var(--text-muted)" />
        </marker>
        <marker id="arrow-start-selected" markerWidth="10" markerHeight="10" refX="0" refY="3" orient="auto" markerUnits="strokeWidth">
          <path d="M9,0 L9,6 L0,3 z" fill="var(--accent)" />
        </marker>
      </defs>
      
      <!-- Background Grid -->
      <rect width="100%" height="100%" fill="url(#grid-fine)" />
      <rect width="100%" height="100%" fill="url(#grid)" />

      <g :transform="`translate(${transform.x}, ${transform.y}) scale(${transform.k})`">
        <!-- Edges -->
        <g class="edges-layer">
          <path 
            v-for="edge in edges" 
            :key="edge.id"
            :d="getEdgePath(edge)"
            :stroke="selectedEdgeId === edge.id ? 'var(--accent)' : (edge.color || 'var(--text-muted)')"
            :stroke-width="edge.thickness || 2"
            :stroke-dasharray="getEdgeDashArray(edge.style)"
            fill="none"
            :marker-start="getMarkerStart(edge)"
            :marker-end="getMarkerEnd(edge)"
            class="graph-edge"
            :class="{ selected: selectedEdgeId === edge.id }"
            @mousedown.stop
            @click.stop="selectEdge(edge)"
            @contextmenu.stop="handleEdgeContextMenu($event, edge)"
          />
          
          <!-- Temporary Edge (Linking) -->
          <line 
            v-if="isLinking && linkingStartNode"
            :x1="linkingStartNode.x"
            :y1="linkingStartNode.y"
            :x2="linkingEndPos.x"
            :y2="linkingEndPos.y"
            stroke="var(--accent)"
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
            :class="{ selected: selectedNodeIds.has(node.id) }"
            :transform="`translate(${node.x}, ${node.y})`"
            @mousedown.stop="handleNodeMouseDown($event, node)"
            @mouseup="handleNodeMouseUp($event, node)"
            @click.stop="selectNode(node)"
            @dblclick.stop="handleNodeDoubleClick(node)"
            @contextmenu.stop.prevent="handleNodeContextMenu($event, node)"
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
              :fill="node.color || 'var(--bg-secondary)'"
              :stroke="selectedNodeIds.has(node.id) ? 'var(--accent)' : (node.border_color || 'var(--border-primary)')"
              :stroke-width="selectedNodeIds.has(node.id) ? 2 : 1"
            />
            
            <!-- Node Shape: Circle -->
            <circle 
              v-else-if="node.shape === 'circle'"
              :r="node.width / 2"
              :fill="node.color || 'var(--bg-secondary)'"
              :stroke="selectedNodeIds.has(node.id) ? 'var(--accent)' : (node.border_color || 'var(--border-primary)')"
              :stroke-width="selectedNodeIds.has(node.id) ? 2 : 1"
            />

            <!-- Node Shape: Diamond -->
            <path
              v-else-if="node.shape === 'diamond'"
              :d="`M 0 ${-node.height / 2} L ${node.width / 2} 0 L 0 ${node.height / 2} L ${-node.width / 2} 0 Z`"
              :fill="node.color || 'var(--bg-secondary)'"
              :stroke="selectedNodeIds.has(node.id) ? 'var(--accent)' : (node.border_color || 'var(--border-primary)')"
              :stroke-width="selectedNodeIds.has(node.id) ? 2 : 1"
            />

            <!-- Node Shape: Ellipse -->
            <ellipse
              v-else-if="node.shape === 'ellipse'"
              :rx="node.width / 2"
              :ry="node.height / 2"
              :fill="node.color || 'var(--bg-secondary)'"
              :stroke="selectedNodeIds.has(node.id) ? 'var(--accent)' : (node.border_color || 'var(--border-primary)')"
              :stroke-width="selectedNodeIds.has(node.id) ? 2 : 1"
            />

            <!-- Node Shape: Hexagon -->
            <path
              v-else-if="node.shape === 'hexagon'"
              :d="getHexagonPath(node.width, node.height)"
              :fill="node.color || 'var(--bg-secondary)'"
              :stroke="selectedNodeIds.has(node.id) ? 'var(--accent)' : (node.border_color || 'var(--border-primary)')"
              :stroke-width="selectedNodeIds.has(node.id) ? 2 : 1"
            />
            
            <!-- Node Label -->
            <text 
              text-anchor="middle" 
              dominant-baseline="middle"
              :fill="node.text_color || 'var(--text-primary)'"
              :font-size="node.font_size || 14"
              style="pointer-events: none; user-select: none;"
            >
              {{ truncateLabel(node.label, node.width) }}
            </text>
            
            <!-- Type Icon (if linked) -->
            <g v-if="node.node_type === 'note'" transform="translate(-8, -25)">
               <!-- Simple file icon -->
               <path d="M4 0h8l4 4v12h-12z" fill="none" stroke="var(--text-muted)" stroke-width="1" transform="scale(0.8)"/>
            </g>
            <g v-if="node.node_type === 'board'" transform="translate(-8, -25)">
               <!-- Simple board icon -->
               <rect x="2" y="2" width="12" height="12" fill="none" stroke="var(--text-muted)" stroke-width="1" transform="scale(0.8)"/>
            </g>
          </g>
        </g>

        <!-- Selection Box -->
        <rect
          v-if="selectionBox.active"
          :x="Math.min(selectionBox.x, selectionBox.x + selectionBox.width)"
          :y="Math.min(selectionBox.y, selectionBox.y + selectionBox.height)"
          :width="Math.abs(selectionBox.width)"
          :height="Math.abs(selectionBox.height)"
          fill="var(--accent)"
          fill-opacity="0.1"
          stroke="var(--accent)"
          stroke-width="1"
          stroke-dasharray="4,2"
          style="pointer-events: none;"
        />
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

    <!-- Context Menu -->
    <div 
      v-if="contextMenu.show" 
      class="context-menu" 
      :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }"
      @click.stop
    >
      <template v-if="contextMenu.type === 'canvas'">
        <div class="menu-section-title">Create</div>
        <div class="menu-item" @click="createNodeAtMenu('bubble')">
          <Icon name="plus" :size="14" />
          <span>Add Bubble</span>
        </div>
        <div class="menu-item" @click="createNodeAtMenu('note')">
          <Icon name="file" :size="14" />
          <span>Add Note</span>
        </div>
        <div class="menu-item" @click="createNodeAtMenu('board')">
          <Icon name="board" :size="14" />
          <span>Add Board</span>
        </div>
      </template>

      <template v-else-if="contextMenu.type === 'node'">
        <div class="menu-item" @click="renameNodeAtMenu">
          <Icon name="edit" :size="14" />
          <span>Rename</span>
        </div>
        
        <div class="menu-divider"></div>
        
        <div class="menu-item has-submenu">
          <span>Shape</span>
          <Icon name="chevron-right" :size="14" />
          <div class="submenu">
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'circle' })">Circle</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'rectangle' })">Rectangle</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'rounded_rect' })">Rounded Rect</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'diamond' })">Diamond</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'ellipse' })">Ellipse</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'hexagon' })">Hexagon</div>
          </div>
        </div>

        <div class="menu-item has-submenu">
          <span>Color</span>
          <Icon name="chevron-right" :size="14" />
          <div class="submenu">
            <div class="menu-item" @click="updateNodeAtMenu({ color: null })">Default</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#f87171' })"><div class="color-dot" style="background: #f87171"></div> Red</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#4ade80' })"><div class="color-dot" style="background: #4ade80"></div> Green</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#60a5fa' })"><div class="color-dot" style="background: #60a5fa"></div> Blue</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#fbbf24' })"><div class="color-dot" style="background: #fbbf24"></div> Yellow</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#a78bfa' })"><div class="color-dot" style="background: #a78bfa"></div> Purple</div>
          </div>
        </div>

        <div class="menu-divider"></div>
        
        <div class="menu-item danger" @click="deleteSelectedNode">
          <Icon name="trash" :size="14" />
          <span>Delete Node</span>
        </div>
      </template>

      <template v-else-if="contextMenu.type === 'edge'">
        <div class="menu-item has-submenu">
          <span>Direction</span>
          <Icon name="chevron-right" :size="14" />
          <div class="submenu">
            <div class="menu-item" @click="updateEdgeAtMenu({ edge_type: 'line' })">None</div>
            <div class="menu-item" @click="updateEdgeAtMenu({ edge_type: 'arrow' })">Forward</div>
            <div class="menu-item" @click="updateEdgeAtMenu({ edge_type: 'reverse_arrow' })">Backward</div>
            <div class="menu-item" @click="updateEdgeAtMenu({ edge_type: 'bidirectional' })">Both</div>
          </div>
        </div>

        <div class="menu-item has-submenu">
          <span>Style</span>
          <Icon name="chevron-right" :size="14" />
          <div class="submenu">
            <div class="menu-item" @click="updateEdgeAtMenu({ style: 'solid' })">Solid</div>
            <div class="menu-item" @click="updateEdgeAtMenu({ style: 'dashed' })">Dashed</div>
            <div class="menu-item" @click="updateEdgeAtMenu({ style: 'dotted' })">Dotted</div>
          </div>
        </div>

        <div class="menu-item has-submenu">
          <span>Color</span>
          <Icon name="chevron-right" :size="14" />
          <div class="submenu">
            <div class="menu-item" @click="updateEdgeAtMenu({ color: null })">Default</div>
            <div class="menu-item" @click="updateEdgeAtMenu({ color: '#f87171' })"><div class="color-dot" style="background: #f87171"></div> Red</div>
            <div class="menu-item" @click="updateEdgeAtMenu({ color: '#4ade80' })"><div class="color-dot" style="background: #4ade80"></div> Green</div>
            <div class="menu-item" @click="updateEdgeAtMenu({ color: '#60a5fa' })"><div class="color-dot" style="background: #60a5fa"></div> Blue</div>
          </div>
        </div>

        <div class="menu-divider"></div>

        <div class="menu-item danger" @click="deleteSelectedEdge">
          <Icon name="trash" :size="14" />
          <span>Delete Edge</span>
        </div>
      </template>

      <template v-else-if="contextMenu.type === 'selection'">
        <div class="menu-section-title">Selection ({{ selectedNodeIds.size }} nodes)</div>
        
        <div class="menu-item has-submenu">
          <span>Shape</span>
          <Icon name="chevron-right" :size="14" />
          <div class="submenu">
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'circle' })">Circle</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'rectangle' })">Rectangle</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'rounded_rect' })">Rounded Rect</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'diamond' })">Diamond</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'ellipse' })">Ellipse</div>
            <div class="menu-item" @click="updateNodeAtMenu({ shape: 'hexagon' })">Hexagon</div>
          </div>
        </div>

        <div class="menu-item has-submenu">
          <span>Color</span>
          <Icon name="chevron-right" :size="14" />
          <div class="submenu">
            <div class="menu-item" @click="updateNodeAtMenu({ color: null })">Default</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#f87171' })"><div class="color-dot" style="background: #f87171"></div> Red</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#4ade80' })"><div class="color-dot" style="background: #4ade80"></div> Green</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#60a5fa' })"><div class="color-dot" style="background: #60a5fa"></div> Blue</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#fbbf24' })"><div class="color-dot" style="background: #fbbf24"></div> Yellow</div>
            <div class="menu-item" @click="updateNodeAtMenu({ color: '#a78bfa' })"><div class="color-dot" style="background: #a78bfa"></div> Purple</div>
          </div>
        </div>

        <div class="menu-divider"></div>
        
        <div class="menu-item danger" @click="deleteSelectedNode">
          <Icon name="trash" :size="14" />
          <span>Delete Selection</span>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import type { GraphNode, GraphEdge, NodeType, EdgeStyle } from '@/types/graph'
import Icon from '@/components/common/ui/Icon.vue'
import logger from '@/utils/logger'

const props = defineProps<{
  nodes: GraphNode[]
  edges: GraphEdge[]
  readOnly?: boolean
}>()

const emit = defineEmits<{
  'node-move': [nodeId: string, x: number, y: number]
  'node-move-finished': [nodes: { id: string, x: number, y: number }[]]
  'node-select': [nodeId: string | null]
  'selection-change': [nodeIds: string[]]
  'edge-select': [edgeId: string | null]
  'create-node': [x: number, y: number, type: NodeType, referenceId?: string]
  'create-edge': [sourceId: string, targetId: string]
  'open-node': [node: GraphNode]
  'delete-node': [nodeId: string]
  'delete-edge': [edgeId: string]
  'rename-node': [nodeId: string]
  'update-node': [nodeId: string, updates: any]
  'update-edge': [edgeId: string, updates: any]
  'undo': []
  'redo': []
}>()

// State
const containerRef = ref<HTMLElement | null>(null)
const transform = ref({ x: 0, y: 0, k: 1 })
const isDraggingCanvas = ref(false)
const isDraggingNode = ref(false)
const isLinking = ref(false)
const isSelecting = ref(false)
const selectionBox = ref({ x: 0, y: 0, width: 0, height: 0, active: false })
const linkingStartNode = ref<GraphNode | null>(null)
const linkingEndPos = ref({ x: 0, y: 0 })
const lastMousePos = { x: 0, y: 0 }
const selectedNodeIds = ref<Set<string>>(new Set())
const selectedEdgeId = ref<string | null>(null)
const draggedNodeId = ref<string | null>(null)
const initialNodePositions = new Map<string, { x: number, y: number }>()

// Touch state
const lastTouchDistance = ref(0)
const isPinching = ref(false)

const contextMenu = ref({
  show: false,
  x: 0,
  y: 0,
  type: 'canvas' as 'canvas' | 'node' | 'edge' | 'selection',
  data: null as any
})

// Keyboard handlers
function handleKeyDown(event: KeyboardEvent) {
  if (props.readOnly) return
  
  // Only handle if not in an input
  if (event.target instanceof HTMLInputElement || event.target instanceof HTMLTextAreaElement) {
    return
  }

  if (event.key === 'Delete' || event.key === 'Backspace') {
    if (selectedNodeIds.value.size > 0) {
      selectedNodeIds.value.forEach(id => emit('delete-node', id))
      selectedNodeIds.value.clear()
      emit('selection-change', [])
    } else if (selectedEdgeId.value) {
      emit('delete-edge', selectedEdgeId.value)
      selectedEdgeId.value = null
    }
  }

  // Undo/Redo
  if ((event.ctrlKey || event.metaKey) && event.key === 'z') {
    if (event.shiftKey) {
      emit('redo')
    } else {
      emit('undo')
    }
    event.preventDefault()
  }
  if ((event.ctrlKey || event.metaKey) && event.key === 'y') {
    emit('redo')
    event.preventDefault()
  }
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown)
})

// Helper to get node by ID
function getNode(id: string): GraphNode | undefined {
  return props.nodes.find(n => n.id === id)
}

// Edge path calculation
function getEdgePath(edge: GraphEdge): string {
  const source = getNode(edge.source_node_id)
  const target = getNode(edge.target_node_id)
  
  if (!source || !target) return ''
  
  // Calculate intersection with node boundaries
  const dx = target.x - source.x
  const dy = target.y - source.y
  const angle = Math.atan2(dy, dx)
  
  // Source boundary point
  let sx = source.x
  let sy = source.y
  if (source.shape === 'circle') {
    sx += (source.width / 2) * Math.cos(angle)
    sy += (source.width / 2) * Math.sin(angle)
  } else {
    // Rectangle boundary (simplified)
    const absCos = Math.abs(Math.cos(angle))
    const absSin = Math.abs(Math.sin(angle))
    const scale = Math.min(source.width / 2 / absCos, source.height / 2 / absSin)
    sx += scale * Math.cos(angle)
    sy += scale * Math.sin(angle)
  }
  
  // Target boundary point
  let tx = target.x
  let ty = target.y
  if (target.shape === 'circle') {
    tx -= (target.width / 2) * Math.cos(angle)
    ty -= (target.width / 2) * Math.sin(angle)
  } else {
    // Rectangle boundary (simplified)
    const absCos = Math.abs(Math.cos(angle))
    const absSin = Math.abs(Math.sin(angle))
    const scale = Math.min(target.width / 2 / absCos, target.height / 2 / absSin)
    tx -= scale * Math.cos(angle)
    ty -= scale * Math.sin(angle)
  }
  
  return `M ${sx},${sy} L ${tx},${ty}`
}

function getEdgeDashArray(style: EdgeStyle): string {
  switch (style) {
    case 'dashed': return '5,5'
    case 'dotted': return '2,2'
    default: return 'none'
  }
}

function getMarkerStart(edge: GraphEdge): string {
  const isSelected = selectedEdgeId.value === edge.id
  if (edge.edge_type === 'reverse_arrow' || edge.edge_type === 'bidirectional') {
    return isSelected ? 'url(#arrow-start-selected)' : 'url(#arrow-start)'
  }
  return 'none'
}

function getMarkerEnd(edge: GraphEdge): string {
  const isSelected = selectedEdgeId.value === edge.id
  if (edge.edge_type === 'arrow' || edge.edge_type === 'bidirectional') {
    return isSelected ? 'url(#arrow-end-selected)' : 'url(#arrow-end)'
  }
  return 'none'
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

function getHexagonPath(width: number, height: number): string {
  const w = width / 2
  const h = height / 2
  const quarterW = w / 2
  return `M ${-w} 0 L ${-quarterW} ${-h} L ${quarterW} ${-h} L ${w} 0 L ${quarterW} ${h} L ${-quarterW} ${h} Z`
}

// Interaction Handlers
function handleCanvasContextMenu(event: MouseEvent) {
  if (props.readOnly) return
  showContextMenu(event, 'canvas')
}

function handleMouseDown(event: MouseEvent) {
  if (props.readOnly) return
  
  // Close context menu on any click
  contextMenu.value.show = false

  if (event.button === 0) { // Left click
    if (event.shiftKey) {
      // Start box selection
      isSelecting.value = true
      const rect = containerRef.value?.getBoundingClientRect()
      if (rect) {
        const x = (event.clientX - rect.left - transform.value.x) / transform.value.k
        const y = (event.clientY - rect.top - transform.value.y) / transform.value.k
        selectionBox.value = { x, y, width: 0, height: 0, active: true }
      }
    } else {
      isDraggingCanvas.value = true
      lastMousePos.x = event.clientX
      lastMousePos.y = event.clientY
      
      // Deselect
      selectedNodeIds.value.clear()
      selectedEdgeId.value = null
      emit('node-select', null)
      emit('edge-select', null)
      emit('selection-change', [])
    }

    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseup', handleMouseUp)
  }
}

function handleTouchStart(event: TouchEvent) {
  if (props.readOnly) return
  contextMenu.value.show = false

  if (event.touches.length === 1) {
    const touch = event.touches[0]
    
    // Check if we touched a node
    const target = event.target as SVGElement
    const nodeElement = target.closest('.graph-node')
    if (nodeElement) {
      // We'll let the node handle its own touch if we want, 
      // but for simplicity let's just pan for now or implement node drag
      return 
    }

    isDraggingCanvas.value = true
    lastMousePos.x = touch.clientX
    lastMousePos.y = touch.clientY
    
    selectedNodeIds.value.clear()
    selectedEdgeId.value = null
  } else if (event.touches.length === 2) {
    isPinching.value = true
    isDraggingCanvas.value = false
    const dx = event.touches[0].clientX - event.touches[1].clientX
    const dy = event.touches[0].clientY - event.touches[1].clientY
    lastTouchDistance.value = Math.sqrt(dx * dx + dy * dy)
  }
}

function handleTouchMove(event: TouchEvent) {
  if (props.readOnly) return

  if (isDraggingCanvas.value && event.touches.length === 1) {
    const touch = event.touches[0]
    const dx = touch.clientX - lastMousePos.x
    const dy = touch.clientY - lastMousePos.y
    
    transform.value.x += dx
    transform.value.y += dy
    
    lastMousePos.x = touch.clientX
    lastMousePos.y = touch.clientY
    event.preventDefault()
  } else if (isPinching.value && event.touches.length === 2) {
    const dx = event.touches[0].clientX - event.touches[1].clientX
    const dy = event.touches[0].clientY - event.touches[1].clientY
    const distance = Math.sqrt(dx * dx + dy * dy)
    
    const factor = distance / lastTouchDistance.value
    lastTouchDistance.value = distance
    
    const centerX = (event.touches[0].clientX + event.touches[1].clientX) / 2
    const centerY = (event.touches[0].clientY + event.touches[1].clientY) / 2
    
    const rect = containerRef.value?.getBoundingClientRect()
    if (rect) {
      zoomTo(transform.value.k * factor, { x: centerX - rect.left, y: centerY - rect.top })
    }
    event.preventDefault()
  }
}

function handleTouchEnd() {
  isDraggingCanvas.value = false
  isPinching.value = false
}

function handleNodeMouseDown(event: MouseEvent, node: GraphNode) {
  if (props.readOnly) return
  
  // Prevent native drag behavior
  event.preventDefault()

  if (event.shiftKey) {
    // Start linking
    isLinking.value = true
    linkingStartNode.value = node
    linkingEndPos.value = { x: node.x, y: node.y }
    event.stopPropagation()
    
    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseup', handleMouseUp)
  } else {
    // If node is not selected, select it (and deselect others unless Ctrl is pressed)
    if (!selectedNodeIds.value.has(node.id)) {
      if (!event.ctrlKey && !event.metaKey) {
        selectedNodeIds.value.clear()
      }
      selectedNodeIds.value.add(node.id)
      selectedEdgeId.value = null
      emit('node-select', node.id)
    }

    isDraggingNode.value = true
    draggedNodeId.value = node.id
    lastMousePos.x = event.clientX
    lastMousePos.y = event.clientY
    
    // Store initial positions for all selected nodes
    initialNodePositions.clear()
    selectedNodeIds.value.forEach(id => {
      const n = getNode(id)
      if (n) {
        initialNodePositions.set(id, { x: n.x, y: n.y })
      }
    })

    event.stopPropagation()

    window.addEventListener('mousemove', handleMouseMove)
    window.addEventListener('mouseup', handleMouseUp)
  }
}

function handleNodeContextMenu(event: MouseEvent, node: GraphNode) {
  if (props.readOnly) return
  
  // If node is not in selection, select only this node
  if (!selectedNodeIds.value.has(node.id)) {
    selectedNodeIds.value.clear()
    selectedNodeIds.value.add(node.id)
    selectedEdgeId.value = null
  }
  
  if (selectedNodeIds.value.size > 1) {
    showContextMenu(event, 'selection', selectedNodeIds.value)
  } else {
    showContextMenu(event, 'node', node)
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
    
    // Move all selected nodes
    selectedNodeIds.value.forEach(id => {
      const node = getNode(id)
      if (node) {
        node.x += dx
        node.y += dy
        emit('node-move', node.id, node.x, node.y)
      }
    })
    
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
  } else if (isSelecting.value) {
    const rect = containerRef.value?.getBoundingClientRect()
    if (rect) {
      const mouseX = (event.clientX - rect.left - transform.value.x) / transform.value.k
      const mouseY = (event.clientY - rect.top - transform.value.y) / transform.value.k
      
      selectionBox.value.width = mouseX - selectionBox.value.x
      selectionBox.value.height = mouseY - selectionBox.value.y

      // Update selection in real-time
      const x1 = Math.min(selectionBox.value.x, selectionBox.value.x + selectionBox.value.width)
      const x2 = Math.max(selectionBox.value.x, selectionBox.value.x + selectionBox.value.width)
      const y1 = Math.min(selectionBox.value.y, selectionBox.value.y + selectionBox.value.height)
      const y2 = Math.max(selectionBox.value.y, selectionBox.value.y + selectionBox.value.height)
      
      selectedNodeIds.value.clear()
      props.nodes.forEach(node => {
        if (node.x >= x1 && node.x <= x2 && node.y >= y1 && node.y <= y2) {
          selectedNodeIds.value.add(node.id)
        }
      })
    }
  }
}

function handleMouseUp() {
  if (isSelecting.value) {
    emit('selection-change', Array.from(selectedNodeIds.value))
    isSelecting.value = false
    selectionBox.value.active = false
  }

  if (isDraggingNode.value) {
    const movedNodes = Array.from(selectedNodeIds.value).map(id => {
      const node = getNode(id)
      return { id, x: node?.x || 0, y: node?.y || 0 }
    })
    emit('node-move-finished', movedNodes)
  }

  isDraggingCanvas.value = false
  isDraggingNode.value = false
  draggedNodeId.value = null
  
  // If we were linking and mouseup happened on background, cancel linking
  if (isLinking.value) {
    isLinking.value = false
    linkingStartNode.value = null
  }

  window.removeEventListener('mousemove', handleMouseMove)
  window.removeEventListener('mouseup', handleMouseUp)
}

function handleNodeMouseUp(_event: MouseEvent, node: GraphNode) {
  if (isLinking.value && linkingStartNode.value) {
    if (linkingStartNode.value.id !== node.id) {
      emit('create-edge', linkingStartNode.value.id, node.id)
    }
    // Stop linking
    isLinking.value = false
    linkingStartNode.value = null
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
  if (!selectedNodeIds.value.has(node.id)) {
    selectedNodeIds.value.clear()
    selectedNodeIds.value.add(node.id)
  }
  selectedEdgeId.value = null
  emit('node-select', node.id)
  emit('selection-change', Array.from(selectedNodeIds.value))
  emit('edge-select', null)
}

function selectEdge(edge: GraphEdge) {
  selectedEdgeId.value = edge.id
  selectedNodeIds.value.clear()
  emit('selection-change', [])
  emit('edge-select', edge.id)
  emit('node-select', null)
}

function handleEdgeContextMenu(event: MouseEvent, edge: GraphEdge) {
  if (props.readOnly) return
  showContextMenu(event, 'edge', edge)
  event.stopPropagation()
  event.preventDefault()
}

function showContextMenu(event: MouseEvent, type: 'canvas' | 'node' | 'edge' | 'selection', data: any = null) {
  event.preventDefault()
  contextMenu.value = {
    show: true,
    x: event.clientX,
    y: event.clientY,
    type,
    data
  }
  
  if (type === 'node') {
    selectedNodeIds.value.clear()
    selectedNodeIds.value.add(data.id)
    selectedEdgeId.value = null
  } else if (type === 'edge') {
    selectedEdgeId.value = data.id
    selectedNodeIds.value.clear()
  }
}

function createNodeAtMenu(type: NodeType) {
  const rect = containerRef.value?.getBoundingClientRect()
  if (!rect) return
  
  const worldX = (contextMenu.value.x - rect.left - transform.value.x) / transform.value.k
  const worldY = (contextMenu.value.y - rect.top - transform.value.y) / transform.value.k
  
  emit('create-node', worldX, worldY, type)
  contextMenu.value.show = false
}

function renameNodeAtMenu() {
  if (selectedNodeIds.value.size === 1) {
    const id = Array.from(selectedNodeIds.value)[0]
    emit('rename-node', id)
  }
  contextMenu.value.show = false
}

function updateNodeAtMenu(updates: any) {
  selectedNodeIds.value.forEach(id => {
    emit('update-node', id, updates)
  })
  contextMenu.value.show = false
}

function updateEdgeAtMenu(updates: any) {
  if (selectedEdgeId.value) {
    emit('update-edge', selectedEdgeId.value, updates)
  }
  contextMenu.value.show = false
}

function deleteSelectedNode() {
  selectedNodeIds.value.forEach(id => {
    emit('delete-node', id)
  })
  selectedNodeIds.value.clear()
  contextMenu.value.show = false
}

function deleteSelectedEdge() {
  if (selectedEdgeId.value) {
    emit('delete-edge', selectedEdgeId.value)
    selectedEdgeId.value = null
  }
  contextMenu.value.show = false
}

function handleNodeDoubleClick(node: GraphNode) {
  emit('open-node', node)
}

// Drag and Drop from Explorer
function onDragOver(event: DragEvent) {
  const hasExplorerType = event.dataTransfer?.types?.includes?.('application/x-explorer-item') || false
  if (hasExplorerType) {
    event.preventDefault()
    event.dataTransfer!.dropEffect = 'copy'
  }
}

function onDrop(event: DragEvent) {
  event.preventDefault()
  event.stopPropagation()
  
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
  background-color: var(--bg-primary);
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
  background: var(--bg-secondary);
  padding: 8px;
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0,0,0,0.2);
  border: 1px solid var(--border-primary);
}

.control-btn {
  width: 32px;
  height: 32px;
  border: 1px solid var(--border-primary);
  background: var(--bg-tertiary);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  color: var(--text-secondary);
}

.control-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.context-menu {
  position: fixed;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  padding: 4px 0;
  min-width: 160px;
  box-shadow: 0 8px 24px rgba(0,0,0,0.3);
  z-index: 1000;
}

.menu-section-title {
  padding: 4px 12px;
  font-size: 11px;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.menu-item {
  padding: 8px 12px;
  cursor: pointer;
  font-size: 13px;
  color: var(--text-primary);
  display: flex;
  align-items: center;
  gap: 10px;
  position: relative;
}

.menu-item:hover {
  background: var(--bg-hover);
}

.menu-item span {
  flex: 1;
}

.menu-divider {
  height: 1px;
  background: var(--border-primary);
  margin: 4px 0;
}

.menu-item.danger {
  color: var(--danger);
}

.menu-item.danger:hover {
  background: var(--danger);
  color: white;
}

.menu-item.has-submenu:hover > .submenu {
  display: block;
}

.submenu {
  display: none;
  position: absolute;
  left: 100%;
  top: -4px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  padding: 4px 0;
  min-width: 140px;
  box-shadow: 0 8px 24px rgba(0,0,0,0.3);
}

.color-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  border: 1px solid rgba(255,255,255,0.1);
}
</style>
