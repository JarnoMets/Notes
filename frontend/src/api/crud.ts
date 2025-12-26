import { AxiosResponse } from 'axios'
import api from './client'

export function createCrudApi<T, CreateT = any, UpdateT = any>(baseUrl: string) {
  return {
    getAll: () => api.get<T[]>(baseUrl),
    get: (id: string) => api.get<T>(`${baseUrl}/${id}`),
    create: (data: CreateT) => api.post<T>(baseUrl, data),
    update: (id: string, data: UpdateT) => api.put<T>(`${baseUrl}/${id}`, data),
    delete: (id: string) => api.delete(`${baseUrl}/${id}`)
  }
}

export function createTreeItemApi<T, CreateT = any, UpdateT = any>(baseUrl: string, folderKey: 'folder_id' | 'parent_id' = 'folder_id') {
  const crud = createCrudApi<T, CreateT, UpdateT>(baseUrl)
  return {
    ...crud,
    getTree: () => api.get(`${baseUrl}/tree`),
    move: (id: string, data: { [key: string]: any; position: number }) => 
      api.post<T>(`${baseUrl}/${id}/move`, data)
  }
}
