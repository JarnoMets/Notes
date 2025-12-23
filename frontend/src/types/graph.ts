// Graph types

// Graph folder types
export interface GraphFolder {
  id: string
  user_id: string
  name: string
  parent_id: string | null
  position: number
  is_important: boolean
  is_urgent: boolean
  created_at: string
  updated_at: string
}

// Graph types
export interface Graph {
  id: string
  user_id: string
  name: string
  description?: string
  folder_id?: string | null
  position?: number
  is_important: boolean
  is_urgent: boolean
  created_at: string
  updated_at: string
}

// Node types
export type NodeType = 'bubble' | 'note' | 'board' | 'card' | 'graph'

export type NodeShape = 'circle' | 'rectangle' | 'rounded_rect' | 'diamond' | 'hexagon' | 'ellipse'

export interface GraphNode {
  id: string
  graph_id: string
  node_type: NodeType
  shape: NodeShape
  label: string
  reference_id?: string | null
  x: number
  y: number
  width: number
  height: number
  color?: string | null
  border_color?: string | null
  text_color?: string | null
  font_size?: number | null
  metadata?: string | null
  created_at: string
  updated_at: string
}

// Edge types
export type EdgeType = 'line' | 'arrow' | 'reverse_arrow' | 'bidirectional'

export type EdgeStyle = 'solid' | 'dashed' | 'dotted'

export interface GraphEdge {
  id: string
  graph_id: string
  source_node_id: string
  target_node_id: string
  edge_type: EdgeType
  style: EdgeStyle
  label?: string | null
  color?: string | null
  thickness?: number | null
  metadata?: string | null
  created_at: string
  updated_at: string
}

// Graph with all data
export interface GraphWithData {
  graph: Graph
  nodes: GraphNode[]
  edges: GraphEdge[]
}

// Graphs tree response
export interface GraphsTree {
  folders: GraphFolder[]
  graphs: Graph[]
}

// Create/Update request types
export interface CreateGraphRequest {
  name: string
  description?: string
  folder_id?: string | null
}

export interface UpdateGraphRequest {
  name?: string
  description?: string
  folder_id?: string | null
  position?: number
  is_important?: boolean
  is_urgent?: boolean
}

export interface CreateNodeRequest {
  node_type?: NodeType
  shape?: NodeShape
  label: string
  reference_id?: string | null
  x: number
  y: number
  width?: number
  height?: number
  color?: string | null
  border_color?: string | null
  text_color?: string | null
  font_size?: number | null
  metadata?: string | null
}

export interface UpdateNodeRequest {
  node_type?: NodeType
  shape?: NodeShape
  label?: string
  reference_id?: string | null
  x?: number
  y?: number
  width?: number
  height?: number
  color?: string | null
  border_color?: string | null
  text_color?: string | null
  font_size?: number | null
  metadata?: string | null
}

export interface CreateEdgeRequest {
  source_node_id: string
  target_node_id: string
  edge_type?: EdgeType
  style?: EdgeStyle
  label?: string | null
  color?: string | null
  thickness?: number | null
  metadata?: string | null
}

export interface UpdateEdgeRequest {
  source_node_id?: string
  target_node_id?: string
  edge_type?: EdgeType
  style?: EdgeStyle
  label?: string | null
  color?: string | null
  thickness?: number | null
  metadata?: string | null
}
