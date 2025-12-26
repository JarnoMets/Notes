import { createTreeItemApi } from './crud'

export const boardsApi = createTreeItemApi('/boards', 'folder_id')

export const boardFoldersApi = createTreeItemApi('/board-folders', 'parent_id')
