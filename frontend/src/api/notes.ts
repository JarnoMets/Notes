import api from './client'
import { createTreeItemApi } from './crud'
import type { Note, NoteFolder, NotesTree, NoteWithAttachments, NoteAttachment } from '../types'

export const foldersApi = createTreeItemApi<NoteFolder>('/folders', 'parent_id')

export const notesApi = {
  ...createTreeItemApi<Note, any, any, NotesTree>('/notes', 'folder_id'),
  get: (id: string) => api.get<NoteWithAttachments>(`/notes/${id}`),
  // Revisions / undo-redo
  listRevisions: (noteId: string) => api.get(`/notes/${noteId}/revisions`),
  undo: (noteId: string) => api.post<Note>(`/notes/${noteId}/revisions/undo`),
  redo: (noteId: string) => api.post<Note>(`/notes/${noteId}/revisions/redo`),
  // Attachments
  uploadAttachment: (noteId: string, file: File) => {
    const formData = new FormData()
    formData.append('file', file)
    return api.post<NoteAttachment>(`/notes/${noteId}/attachments`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' }
    })
  },
  uploadAttachments: (noteId: string, files: File[]) => {
    const formData = new FormData()
    files.forEach(file => formData.append('file', file))
    return api.post<NoteAttachment[]>(`/notes/${noteId}/attachments`, formData, {
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
