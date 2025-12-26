import api from './client'
import { createTreeItemApi } from './crud'
import type { 
  Graph, GraphFolder, GraphsTree, GraphWithData,
  CreateNodeRequest, UpdateNodeRequest, GraphNode,
  CreateEdgeRequest, UpdateEdgeRequest, GraphEdge
} from '../types'

export const graphsApi = {
  ...createTreeItemApi<Graph, any, any, GraphsTree>('/graphs', 'folder_id'),
  
  getWithData: (id: string) => api.get<GraphWithData>(`/graphs/${id}`),

  nodes: {
    create: (graphId: string, data: CreateNodeRequest) => 
      api.post<GraphNode>(`/graphs/${graphId}/nodes`, data),
    update: (graphId: string, nodeId: string, data: UpdateNodeRequest) => 
      api.put<GraphNode>(`/graphs/${graphId}/nodes/${nodeId}`, data),
    delete: (graphId: string, nodeId: string) => 
      api.delete(`/graphs/${graphId}/nodes/${nodeId}`)
  },

  edges: {
    create: (graphId: string, data: CreateEdgeRequest) => 
      api.post<GraphEdge>(`/graphs/${graphId}/edges`, data),
    update: (graphId: string, edgeId: string, data: UpdateEdgeRequest) => 
      api.put<GraphEdge>(`/graphs/${graphId}/edges/${edgeId}`, data),
    delete: (graphId: string, edgeId: string) => 
      api.delete(`/graphs/${graphId}/edges/${edgeId}`)
  }
}

export const graphFoldersApi = createTreeItemApi<GraphFolder>('/graph-folders', 'parent_id')

export const graphNodesApi = graphsApi.nodes
export const graphEdgesApi = graphsApi.edges
