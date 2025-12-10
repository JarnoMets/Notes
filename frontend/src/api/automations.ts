import api from './client'

export const automationsApi = {
  getAll: (boardId: string) => api.get(`/boards/${boardId}/automations`),
  create: (boardId: string, data: { 
    name: string; 
    trigger_type: string; 
    trigger_config: Record<string, unknown>; 
    action_type: string; 
    action_config: Record<string, unknown> 
  }) => api.post(`/boards/${boardId}/automations`, data),
  update: (id: string, data: { 
    name?: string; 
    enabled?: boolean;
    trigger_type?: string; 
    trigger_config?: Record<string, unknown>; 
    action_type?: string; 
    action_config?: Record<string, unknown> 
  }) => api.put(`/automations/${id}`, data),
  delete: (id: string) => api.delete(`/automations/${id}`),
  toggle: (id: string) => api.post(`/automations/${id}/toggle`)
}
