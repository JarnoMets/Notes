import api from './client'

export const labelsApi = {
  getAll: (boardId: string) => api.get(`/boards/${boardId}/labels`),
  create: (boardId: string, data: { name: string; color: string }) => api.post(`/boards/${boardId}/labels`, data),
  update: (id: string, data: { name?: string; color?: string }) => api.put(`/labels/${id}`, data),
  delete: (id: string) => api.delete(`/labels/${id}`)
}
