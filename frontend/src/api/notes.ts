import api from './client'

export const foldersApi = {
  getAll: () => api.get('/folders'),
  create: (data: { name: string; parent_id?: string | null }) => api.post('/folders', data),
  update: (id: string, data: { name?: string; parent_id?: string | null; position?: number; is_important?: boolean; is_urgent?: boolean }) => api.put(`/folders/${id}`, data),
  delete: (id: string) => api.delete(`/folders/${id}`),
  move: (id: string, data: { parent_id: string | null; position: number }) => api.post(`/folders/${id}/move`, data)
}

export const notesApi = {
  getAll: () => api.get('/notes'),
  getTree: () => api.get('/notes/tree'),
  get: (id: string) => api.get(`/notes/${id}`),
  create: (data: { title: string; folder_id?: string | null; description?: string; content?: string }) => api.post('/notes', data),
  update: (id: string, data: { title?: string; folder_id?: string | null; description?: string; content?: string; position?: number; is_important?: boolean; is_urgent?: boolean }) => api.put(`/notes/${id}`, data),
  delete: (id: string) => api.delete(`/notes/${id}`),
  move: (id: string, data: { folder_id: string | null; position: number }) => api.post(`/notes/${id}/move`, data),
  // Revisions / undo-redo
  listRevisions: (noteId: string) => api.get(`/notes/${noteId}/revisions`),
  undo: (noteId: string) => api.post(`/notes/${noteId}/revisions/undo`),
  redo: (noteId: string) => api.post(`/notes/${noteId}/revisions/redo`),
  // Attachments
  uploadAttachment: (noteId: string, file: File) => {
    const formData = new FormData()
    formData.append('file', file)
    return api.post(`/notes/${noteId}/attachments`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    })
  },
  uploadAttachments: (noteId: string, files: File[]) => {
    const formData = new FormData()
    files.forEach(file => formData.append('file', file))
    return api.post(`/notes/${noteId}/attachments`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    })
  },
  getAttachmentUrl: (attachmentId: string) => {
    const token = localStorage.getItem('notes_auth_token')
    return `${api.defaults.baseURL}/attachments/${attachmentId}${token ? `?token=${token}` : ''}`
  },
  downloadAttachment: (attachmentId: string) => api.get(`/attachments/${attachmentId}`, { responseType: 'blob' }),
  deleteAttachment: (attachmentId: string) => api.delete(`/attachments/${attachmentId}`)
}
