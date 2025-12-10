import api from './client'

export const listsApi = {
  getAll: (boardId: string) => api.get(`/boards/${boardId}/lists`),
  getArchived: (boardId: string) => api.get(`/boards/${boardId}/lists/archived`),
  create: (boardId: string, data: { name: string; position?: number }) => api.post(`/boards/${boardId}/lists`, data),
  update: (id: string, data: { name?: string; position?: number; archived?: boolean }) => api.put(`/lists/${id}`, data),
  delete: (id: string) => api.delete(`/lists/${id}`),
  reorder: (data: { board_id: string; list_ids: string[] }) => api.post('/lists/reorder', data),
  archive: (id: string) => api.post(`/lists/${id}/archive`),
  restore: (id: string) => api.post(`/lists/${id}/restore`)
}
