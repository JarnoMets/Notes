import api from './client'

export const boardsApi = {
  getAll: () => api.get('/boards'),
  getTree: () => api.get('/boards/tree'),
  get: (id: string) => api.get(`/boards/${id}`),
  create: (data: { name: string; description?: string; color?: string; folder_id?: string | null }) => api.post('/boards', data),
  update: (id: string, data: { name?: string; description?: string; color?: string; folder_id?: string | null; position?: number; is_important?: boolean; is_urgent?: boolean }) => api.put(`/boards/${id}`, data),
  delete: (id: string) => api.delete(`/boards/${id}`),
  move: (id: string, data: { folder_id: string | null; position: number }) => api.post(`/boards/${id}/move`, data)
}

export const boardFoldersApi = {
  getAll: () => api.get('/board-folders'),
  create: (data: { name: string; parent_id?: string | null }) => api.post('/board-folders', data),
  update: (id: string, data: { name?: string; parent_id?: string | null; position?: number; is_important?: boolean; is_urgent?: boolean }) => api.put(`/board-folders/${id}`, data),
  delete: (id: string) => api.delete(`/board-folders/${id}`),
  move: (id: string, data: { parent_id: string | null; position: number }) => api.post(`/board-folders/${id}/move`, data)
}
