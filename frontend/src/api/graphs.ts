import api from './client'
import type {
  CreateGraphRequest,
  UpdateGraphRequest,
  CreateNodeRequest,
  UpdateNodeRequest,
  CreateEdgeRequest,
  UpdateEdgeRequest,
} from '../types/graph'

export const graphsApi = {
  // Graphs
  getAll: () => api.get('/graphs'),
  getTree: () => api.get('/graphs/tree'),
  get: (id: string) => api.get(`/graphs/${id}`),
  create: (data: CreateGraphRequest) => api.post('/graphs', data),
  update: (id: string, data: UpdateGraphRequest) => api.put(`/graphs/${id}`, data),
  delete: (id: string) => api.delete(`/graphs/${id}`),
  move: (id: string, data: { folder_id: string | null; position: number }) =>
    api.post(`/graphs/${id}/move`, data),
}

export const graphFoldersApi = {
  getAll: () => api.get('/graph-folders'),
  create: (data: { name: string; parent_id?: string | null }) =>
    api.post('/graph-folders', data),
  update: (
    id: string,
    data: {
      name?: string
      parent_id?: string | null
      position?: number
      is_important?: boolean
      is_urgent?: boolean
    }
  ) => api.put(`/graph-folders/${id}`, data),
  delete: (id: string) => api.delete(`/graph-folders/${id}`),
  move: (id: string, data: { parent_id: string | null; position: number }) =>
    api.post(`/graph-folders/${id}/move`, data),
}

export const graphNodesApi = {
  getAll: (graphId: string) => api.get(`/graphs/${graphId}/nodes`),
  create: (graphId: string, data: CreateNodeRequest) =>
    api.post(`/graphs/${graphId}/nodes`, data),
  update: (graphId: string, nodeId: string, data: UpdateNodeRequest) =>
    api.put(`/graphs/${graphId}/nodes/${nodeId}`, data),
  delete: (graphId: string, nodeId: string) =>
    api.delete(`/graphs/${graphId}/nodes/${nodeId}`),
}

export const graphEdgesApi = {
  getAll: (graphId: string) => api.get(`/graphs/${graphId}/edges`),
  create: (graphId: string, data: CreateEdgeRequest) =>
    api.post(`/graphs/${graphId}/edges`, data),
  update: (graphId: string, edgeId: string, data: UpdateEdgeRequest) =>
    api.put(`/graphs/${graphId}/edges/${edgeId}`, data),
  delete: (graphId: string, edgeId: string) =>
    api.delete(`/graphs/${graphId}/edges/${edgeId}`),
}
