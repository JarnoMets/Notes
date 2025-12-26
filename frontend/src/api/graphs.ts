import api from './client'
import { createCrudApi, createTreeItemApi } from './crud'
import type {
  CreateGraphRequest,
  UpdateGraphRequest,
  CreateNodeRequest,
  UpdateNodeRequest,
  CreateEdgeRequest,
  UpdateEdgeRequest,
} from '../types/graph'

export const graphsApi = createTreeItemApi('/graphs', 'folder_id')

export const graphFoldersApi = createTreeItemApi('/graph-folders', 'parent_id')

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
