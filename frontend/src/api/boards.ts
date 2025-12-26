import api from './client'
import { createTreeItemApi } from './crud'
import type { Board, BoardFolder, BoardsTree, BoardWithLists } from '../types'

export const boardsApi = {
  ...createTreeItemApi<Board, any, any, BoardsTree>('/boards', 'folder_id'),
  getWithLists: (id: string) => api.get<BoardWithLists>(`/boards/${id}/lists`)
}

export const boardFoldersApi = createTreeItemApi<BoardFolder>('/board-folders', 'parent_id')
