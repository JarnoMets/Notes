import api from './client'

export const cardsApi = {
  getAll: (listId: string) => api.get(`/lists/${listId}/cards`),
  getArchived: (boardId: string) => api.get(`/boards/${boardId}/cards/archived`),
  get: (id: string) => api.get(`/cards/${id}`),
  create: (listId: string, data: { title: string; description?: string; position?: number; due_date?: string; labels?: string[]; status?: string }) => 
    api.post(`/lists/${listId}/cards`, data),
  update: (id: string, data: { title?: string; description?: string; position?: number; due_date?: string; labels?: string[]; archived?: boolean; list_id?: string; status?: string }) => 
    api.put(`/cards/${id}`, data),
  delete: (id: string) => api.delete(`/cards/${id}`),
  move: (data: { card_id: string; target_list_id: string; position: number }) => api.post('/cards/move', data),
  reorder: (data: { list_id: string; card_ids: string[] }) => api.post('/cards/reorder', data),
  archive: (id: string) => api.post(`/cards/${id}/archive`),
  restore: (id: string) => api.post(`/cards/${id}/restore`),
  // Attachments
  uploadAttachment: (cardId: string, file: File) => {
    const formData = new FormData()
    formData.append('file', file)
    return api.post(`/cards/${cardId}/attachments`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    })
  },
  uploadAttachments: (cardId: string, files: File[]) => {
    const formData = new FormData()
    files.forEach(file => formData.append('file', file))
    return api.post(`/cards/${cardId}/attachments`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    })
  },
  getAttachmentUrl: (attachmentId: string) => {
    const token = localStorage.getItem('notes_auth_token')
    return `${api.defaults.baseURL}/card-attachments/${attachmentId}${token ? `?token=${token}` : ''}`
  },
  downloadAttachment: (attachmentId: string) => api.get(`/card-attachments/${attachmentId}`, { responseType: 'blob' }),
  deleteAttachment: (attachmentId: string) => api.delete(`/card-attachments/${attachmentId}`)
}
