import { type Ref } from 'vue'
import logger from './logger'
import { useSyncStore } from '@/stores/sync'

export interface TreeItem {
  id: string
  name: string
  parentId: string | null
  position: number
  isExpanded: boolean
  isImportant: boolean
  isUrgent: boolean
  children: TreeItem[]
}

export function useTreeState<T extends { id: string; parent_id?: string | null; folder_id?: string | null; position?: number; is_important?: boolean; is_urgent?: boolean }>(
  itemsRef: Ref<T[]>,
  foldersRef: Ref<any[]>,
  expandedFoldersRef: Ref<Set<string>>,
  api: any,
  folderApi: any,
  options: {
    folderKey: 'parent_id' | 'folder_id'
    nameKey: 'name' | 'title'
  }
) {
  const { folderKey, nameKey } = options
  const syncStore = useSyncStore()

  async function withSync<R>(action: () => Promise<R>): Promise<R> {
    syncStore.startSync()
    try {
      return await action()
    } finally {
      syncStore.endSync()
    }
  }

  async function createFolder(name: string, parentId: string | null = null) {
    return withSync(async () => {
      try {
        const response = await folderApi.create({ name, parent_id: parentId })
        foldersRef.value.push(response.data)
        if (parentId) {
          expandedFoldersRef.value.add(parentId)
        }
        return response.data
      } catch (error) {
        logger.error(`Failed to create folder:`, error)
        throw error
      }
    })
  }

  async function renameFolder(id: string, name: string) {
    const folder = foldersRef.value.find(f => f.id === id)
    if (!folder) return
    const originalName = folder.name

    // Optimistic
    folder.name = name

    return withSync(async () => {
      try {
        const response = await folderApi.update(id, { name })
        const index = foldersRef.value.findIndex(f => f.id === id)
        if (index !== -1) {
          foldersRef.value[index] = response.data
        }
        return response.data
      } catch (error) {
        folder.name = originalName
        logger.error(`Failed to rename folder:`, error)
        throw error
      }
    })
  }

  async function deleteFolder(id: string) {
    const originalFolders = [...foldersRef.value]
    const originalItems = [...itemsRef.value]

    // Optimistic
    foldersRef.value = foldersRef.value.filter(f => f.id !== id)
    itemsRef.value = itemsRef.value.filter(i => (i[folderKey] as any) !== id)

    return withSync(async () => {
      try {
        await folderApi.delete(id)
      } catch (error) {
        foldersRef.value = originalFolders
        itemsRef.value = originalItems
        logger.error(`Failed to delete folder:`, error)
        throw error
      }
    })
  }

  async function moveFolder(id: string, parentId: string | null, position: number) {
    return withSync(async () => {
      try {
        const response = await folderApi.move(id, { parent_id: parentId, position })
        const index = foldersRef.value.findIndex(f => f.id === id)
        if (index !== -1) {
          foldersRef.value[index] = response.data
        }
        return response.data
      } catch (error) {
        logger.error(`Failed to move folder:`, error)
        throw error
      }
    })
  }

  async function createItem(name: string, folderId: string | null = null) {
    return withSync(async () => {
      try {
        const data = { [nameKey]: name, [folderKey]: folderId }
        const response = await api.create(data)
        itemsRef.value.push(response.data)
        if (folderId) {
          expandedFoldersRef.value.add(folderId)
        }
        return response.data
      } catch (error) {
        logger.error(`Failed to create item:`, error)
        throw error
      }
    })
  }

  async function updateItem(id: string, data: Partial<T>) {
    const item = itemsRef.value.find(i => i.id === id)
    if (!item) return
    const originalState = { ...item }

    // Optimistic
    Object.assign(item, data)

    return withSync(async () => {
      try {
        const response = await api.update(id, data)
        const index = itemsRef.value.findIndex(i => i.id === id)
        if (index !== -1) {
          itemsRef.value[index] = { ...itemsRef.value[index], ...response.data }
        }
        return response.data
      } catch (error) {
        Object.assign(item, originalState)
        logger.error(`Failed to update item:`, error)
        throw error
      }
    })
  }

  async function deleteItem(id: string) {
    const originalItems = [...itemsRef.value]

    // Optimistic
    itemsRef.value = itemsRef.value.filter(i => i.id !== id)

    return withSync(async () => {
      try {
        await api.delete(id)
      } catch (error) {
        itemsRef.value = originalItems
        logger.error(`Failed to delete item:`, error)
        throw error
      }
    })
  }

  async function moveItem(id: string, folderId: string | null, position: number) {
    return withSync(async () => {
      try {
        const response = await api.move(id, { [folderKey]: folderId, position })
        const index = itemsRef.value.findIndex(i => i.id === id)
        if (index !== -1) {
          itemsRef.value[index] = response.data
        }
        return response.data
      } catch (error) {
        logger.error(`Failed to move item:`, error)
        throw error
      }
    })
  }

  async function toggleFolderImportance(id: string) {
    const folder = foldersRef.value.find(f => f.id === id)
    if (!folder) return
    const original = folder.is_important
    folder.is_important = !original

    return withSync(async () => {
      try {
        const res = await folderApi.update(id, { is_important: !original })
        const index = foldersRef.value.findIndex(f => f.id === id)
        if (index !== -1) foldersRef.value[index] = res.data
        return res.data
      } catch (error) {
        folder.is_important = original
        throw error
      }
    })
  }

  async function toggleItemImportance(id: string) {
    const item = itemsRef.value.find(i => i.id === id)
    if (!item) return
    const original = item.is_important
    item.is_important = !original

    return withSync(async () => {
      try {
        const res = await api.update(id, { is_important: !original })
        const index = itemsRef.value.findIndex(i => i.id === id)
        if (index !== -1) itemsRef.value[index] = res.data
        return res.data
      } catch (error) {
        item.is_important = original
        throw error
      }
    })
  }

  async function toggleFolderUrgent(id: string) {
    const folder = foldersRef.value.find(f => f.id === id)
    if (!folder) return
    const original = folder.is_urgent
    folder.is_urgent = !original

    return withSync(async () => {
      try {
        const res = await folderApi.update(id, { is_urgent: !original })
        const index = foldersRef.value.findIndex(f => f.id === id)
        if (index !== -1) foldersRef.value[index] = res.data
        return res.data
      } catch (error) {
        folder.is_urgent = original
        throw error
      }
    })
  }

  async function toggleItemUrgent(id: string) {
    const item = itemsRef.value.find(i => i.id === id)
    if (!item) return
    const original = item.is_urgent
    item.is_urgent = !original

    return withSync(async () => {
      try {
        const res = await api.update(id, { is_urgent: !original })
        const index = itemsRef.value.findIndex(i => i.id === id)
        if (index !== -1) itemsRef.value[index] = res.data
        return res.data
      } catch (error) {
        item.is_urgent = original
        throw error
      }
    })
  }

  return {
    createFolder,
    renameFolder,
    deleteFolder,
    moveFolder,
    createItem,
    updateItem,
    deleteItem,
    moveItem,
    toggleFolderImportance,
    toggleItemImportance,
    toggleFolderUrgent,
    toggleItemUrgent
  }
}
