import api from './client'

export interface ReminderData {
  note_id: string
  title: string
  date: string // YYYY-MM-DD format
  time: string // HH:MM format
  remind_before?: string // How long before (e.g., "0", "15", "30" in minutes)
}

export interface Reminder extends ReminderData {
  id: string
  created_at?: string
  updated_at?: string
}

export const remindersApi = {
  getAll: () => api.get<Reminder[]>('/reminders'),
  get: (id: string) => api.get<Reminder>(`/reminders/${id}`),
  create: (data: ReminderData) => api.post<Reminder>('/reminders', data),
  update: (id: string, data: Partial<ReminderData>) => api.put<Reminder>(`/reminders/${id}`, data),
  delete: (id: string) => api.delete(`/reminders/${id}`),
  deleteByNote: (noteId: string) => api.delete(`/notes/${noteId}/reminders`)
}
