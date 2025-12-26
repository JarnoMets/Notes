import api from './client'
import { createTreeItemApi } from './crud'

export const foldersApi = createTreeItemApi('/folders', 'parent_id')

export const notesApi = {
  ...createTreeItemApi('/notes', 'folder_id'),
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
