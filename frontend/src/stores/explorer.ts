import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Note, NoteFolder, Board, BoardFolder } from '../types'
import { notesApi, foldersApi, boardsApi, boardFoldersApi } from '../api'

export interface ExplorerItem {
  id: string
  name: string
  type: 'folder' | 'note' | 'board' | 'board-folder' | 'favorites-folder'
  parentId: string | null
  position: number
  isExpanded: boolean
  isImportant: boolean
  isUrgent: boolean
  hasUrgentDescendant: boolean
  children: ExplorerItem[]
  color?: string // For boards
}

export const useExplorerStore = defineStore('explorer', () => {
  // Data
  const folders = ref<NoteFolder[]>([])
  const notes = ref<Note[]>([])
  const boards = ref<Board[]>([])
  const boardFolders = ref<BoardFolder[]>([])
  
  // UI state
  const expandedFolders = ref<Set<string>>(new Set())
  const expandedBoardFolders = ref<Set<string>>(new Set())
  const expandedSections = ref({
    notes: true,
    boards: true
  })
  const selectedItemId = ref<string | null>(null)
  const selectedItemType = ref<'note' | 'folder' | 'board' | 'board-folder' | null>(null)
  const favoritesFolderExpanded = ref(true)
  
  // Helper to check if a folder has urgent descendants
  function checkUrgentDescendants(folderId: string): boolean {
    // Check notes in this folder
    const folderNotes = notes.value.filter(n => n.folder_id === folderId)
    if (folderNotes.some(n => n.is_urgent)) {
      return true
    }
    // Check subfolders
    const subFolders = folders.value.filter(f => f.parent_id === folderId)
    for (const sub of subFolders) {
      if (sub.is_urgent || checkUrgentDescendants(sub.id)) {
        return true
      }
    }
    return false
  }
  
  // Helper to check if a board folder has urgent descendants
  function checkBoardUrgentDescendants(folderId: string): boolean {
    // Check boards in this folder
    const folderBoards = boards.value.filter(b => b.folder_id === folderId)
    if (folderBoards.some(b => b.is_urgent)) {
      return true
    }
    // Check subfolders
    const subFolders = boardFolders.value.filter(f => f.parent_id === folderId)
    for (const sub of subFolders) {
      if (sub.is_urgent || checkBoardUrgentDescendants(sub.id)) {
        return true
      }
    }
    return false
  }
  
  // Build tree structure for notes section
  const notesTree = computed<ExplorerItem[]>(() => {
    const buildTree = (parentId: string | null): ExplorerItem[] => {
      const items: ExplorerItem[] = []

      // Add folders
      const childFolders = folders.value
        .filter(f => f.parent_id === parentId)
        .sort((a, b) => a.position - b.position)

      for (const folder of childFolders) {
        const children = buildTree(folder.id)
        const isUrgent = folder.is_urgent
        const hasUrgentDescendant = !isUrgent && checkUrgentDescendants(folder.id)
        items.push({
          id: folder.id,
          name: folder.name,
          type: 'folder',
          parentId: folder.parent_id,
          position: folder.position,
          isExpanded: expandedFolders.value.has(folder.id),
          isImportant: folder.is_important,
          isUrgent,
          hasUrgentDescendant,
          children
        })
      }

      // Add notes
      const childNotes = notes.value
        .filter(n => n.folder_id === parentId)
        .sort((a, b) => a.position - b.position)

      for (const note of childNotes) {
        items.push({
          id: note.id,
          name: note.title,
          type: 'note',
          parentId: note.folder_id,
          position: note.position,
          isExpanded: false,
          isImportant: note.is_important,
          isUrgent: note.is_urgent,
          hasUrgentDescendant: false,
          children: []
        })
      }

      return items
    }

    return buildTree(null)
  })
  
  // Favorites virtual folder containing all starred items
  const favoritesList = computed<ExplorerItem[]>(() => {
    const items: ExplorerItem[] = []
    
    // Add starred folders (flatten, don't nest)
    const starredFolders = folders.value.filter(f => f.is_important)
    for (const folder of starredFolders) {
      items.push({
        id: folder.id,
        name: folder.name,
        type: 'folder',
        parentId: '__favorites__',
        position: folder.position,
        isExpanded: false,
        isImportant: true,
        isUrgent: folder.is_urgent,
        hasUrgentDescendant: checkUrgentDescendants(folder.id),
        children: []
      })
    }
    
    // Add starred notes
    const starredNotes = notes.value.filter(n => n.is_important)
    for (const note of starredNotes) {
      items.push({
        id: note.id,
        name: note.title,
        type: 'note',
        parentId: '__favorites__',
        position: note.position,
        isExpanded: false,
        isImportant: true,
        isUrgent: note.is_urgent,
        hasUrgentDescendant: false,
        children: []
      })
    }
    
    // Add starred boards
    const starredBoards = boards.value.filter(b => b.is_important)
    for (const board of starredBoards) {
      items.push({
        id: board.id,
        name: board.name,
        type: 'board',
        parentId: '__favorites__',
        position: board.position || 0,
        isExpanded: false,
        isImportant: true,
        isUrgent: board.is_urgent,
        hasUrgentDescendant: false,
        children: [],
        color: board.color
      })
    }
    
    // Add starred board folders
    const starredBoardFolders = boardFolders.value.filter(f => f.is_important)
    for (const folder of starredBoardFolders) {
      items.push({
        id: folder.id,
        name: folder.name,
        type: 'board-folder',
        parentId: '__favorites__',
        position: folder.position,
        isExpanded: false,
        isImportant: true,
        isUrgent: folder.is_urgent,
        hasUrgentDescendant: checkBoardUrgentDescendants(folder.id),
        children: []
      })
    }
    
    return items
  })
  
  // Build flat list for boards section (will become tree when backend supports folders)
  const boardsList = computed<ExplorerItem[]>(() => {
    const buildBoardTree = (parentId: string | null): ExplorerItem[] => {
      const items: ExplorerItem[] = []

      // Add board folders
      const childFolders = boardFolders.value
        .filter(f => f.parent_id === parentId)
        .sort((a, b) => a.position - b.position)

      for (const folder of childFolders) {
        const children = buildBoardTree(folder.id)
        const isUrgent = folder.is_urgent
        const hasUrgentDescendant = !isUrgent && checkBoardUrgentDescendants(folder.id)
        items.push({
          id: folder.id,
          name: folder.name,
          type: 'board-folder',
          parentId: folder.parent_id,
          position: folder.position,
          isExpanded: expandedBoardFolders.value.has(folder.id),
          isImportant: folder.is_important,
          isUrgent,
          hasUrgentDescendant,
          children
        })
      }

      // Add boards
      const childBoards = boards.value
        .filter(b => (b.folder_id || null) === parentId)
        .sort((a, b) => (a.position || 0) - (b.position || 0))

      for (const board of childBoards) {
        items.push({
          id: board.id,
          name: board.name,
          type: 'board',
          parentId: board.folder_id || null,
          position: board.position || 0,
          isExpanded: false,
          isImportant: board.is_important,
          isUrgent: board.is_urgent,
          hasUrgentDescendant: false,
          children: [],
          color: board.color
        })
      }

      return items
    }

    return buildBoardTree(null)
  })
  
  // Fetch all data
  async function fetchAll() {
    await Promise.all([fetchNotes(), fetchBoards()])
  }
  
  async function fetchNotes() {
    try {
      const response = await notesApi.getTree()
      folders.value = response.data.folders
      notes.value = response.data.notes
    } catch (error) {
      console.error('Failed to fetch notes tree:', error)
    }
  }
  
  async function fetchBoards() {
    try {
      const response = await boardsApi.getTree()
      boardFolders.value = response.data.folders || []
      boards.value = response.data.boards || []
    } catch (error) {
      // Fallback to simple getAll if tree endpoint doesn't exist yet
      try {
        const response = await boardsApi.getAll()
        boards.value = response.data
        boardFolders.value = []
      } catch (e) {
        console.error('Failed to fetch boards:', e)
      }
    }
  }
  
  // Toggle functions
  function toggleFolder(folderId: string) {
    if (expandedFolders.value.has(folderId)) {
      expandedFolders.value.delete(folderId)
    } else {
      expandedFolders.value.add(folderId)
    }
  }
  
  function toggleBoardFolder(folderId: string) {
    if (expandedBoardFolders.value.has(folderId)) {
      expandedBoardFolders.value.delete(folderId)
    } else {
      expandedBoardFolders.value.add(folderId)
    }
  }
  
  function toggleSection(section: 'notes' | 'boards') {
    expandedSections.value[section] = !expandedSections.value[section]
  }
  
  // Selection
  function selectItem(id: string, type: 'note' | 'folder' | 'board' | 'board-folder') {
    selectedItemId.value = id
    selectedItemType.value = type
  }
  
  function clearSelection() {
    selectedItemId.value = null
    selectedItemType.value = null
  }
  
  // Get note by id
  function getNoteById(id: string): Note | undefined {
    return notes.value.find(n => n.id === id)
  }
  
  // Get board by id  
  function getBoardById(id: string): Board | undefined {
    return boards.value.find(b => b.id === id)
  }
  
  // Folder operations
  async function createFolder(name: string, parentId: string | null = null) {
    try {
      const response = await foldersApi.create({ name, parent_id: parentId })
      folders.value.push(response.data)
      if (parentId) {
        expandedFolders.value.add(parentId)
      }
      return response.data
    } catch (error) {
      console.error('Failed to create folder:', error)
      throw error
    }
  }
  
  async function renameFolder(id: string, name: string) {
    try {
      const response = await foldersApi.update(id, { name })
      const index = folders.value.findIndex(f => f.id === id)
      if (index !== -1) {
        folders.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to rename folder:', error)
      throw error
    }
  }
  
  async function deleteFolder(id: string) {
    try {
      await foldersApi.delete(id)
      folders.value = folders.value.filter(f => f.id !== id)
      notes.value = notes.value.filter(n => n.folder_id !== id)
    } catch (error) {
      console.error('Failed to delete folder:', error)
      throw error
    }
  }
  
  async function moveFolder(id: string, parentId: string | null, position: number) {
    try {
      const response = await foldersApi.move(id, { parent_id: parentId, position })
      const index = folders.value.findIndex(f => f.id === id)
      if (index !== -1) {
        folders.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to move folder:', error)
      throw error
    }
  }
  
  // Note operations
  async function createNote(title: string, folderId: string | null = null) {
    try {
      const response = await notesApi.create({ title, folder_id: folderId })
      notes.value.push(response.data)
      if (folderId) {
        expandedFolders.value.add(folderId)
      }
      return response.data
    } catch (error) {
      console.error('Failed to create note:', error)
      throw error
    }
  }
  
  async function updateNote(id: string, data: Partial<Note>) {
    try {
      const response = await notesApi.update(id, data)
      const index = notes.value.findIndex(n => n.id === id)
      if (index !== -1) {
        notes.value[index] = { ...notes.value[index], ...response.data }
      }
      return response.data
    } catch (error) {
      console.error('Failed to update note:', error)
      throw error
    }
  }
  
  async function deleteNote(id: string) {
    try {
      await notesApi.delete(id)
      notes.value = notes.value.filter(n => n.id !== id)
    } catch (error) {
      console.error('Failed to delete note:', error)
      throw error
    }
  }
  
  async function moveNote(id: string, folderId: string | null, position: number) {
    try {
      const response = await notesApi.move(id, { folder_id: folderId, position })
      const index = notes.value.findIndex(n => n.id === id)
      if (index !== -1) {
        notes.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to move note:', error)
      throw error
    }
  }
  
  // Board operations  
  async function createBoard(name: string, description?: string, color?: string, folderId?: string | null) {
    try {
      const response = await boardsApi.create({ name, description, color, folder_id: folderId })
      boards.value.push(response.data)
      if (folderId) {
        expandedBoardFolders.value.add(folderId)
      }
      return response.data
    } catch (error) {
      console.error('Failed to create board:', error)
      throw error
    }
  }
  
  async function updateBoard(id: string, data: Partial<Board>) {
    try {
      const response = await boardsApi.update(id, data)
      const index = boards.value.findIndex(b => b.id === id)
      if (index !== -1) {
        boards.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to update board:', error)
      throw error
    }
  }
  
  async function deleteBoard(id: string) {
    try {
      await boardsApi.delete(id)
      boards.value = boards.value.filter(b => b.id !== id)
    } catch (error) {
      console.error('Failed to delete board:', error)
      throw error
    }
  }

  async function moveBoard(id: string, folderId: string | null, position: number) {
    try {
      const response = await boardsApi.move(id, { folder_id: folderId, position })
      const index = boards.value.findIndex(b => b.id === id)
      if (index !== -1) {
        boards.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to move board:', error)
      throw error
    }
  }

  // Board folder operations
  async function createBoardFolder(name: string, parentId: string | null = null) {
    try {
      const response = await boardFoldersApi.create({ name, parent_id: parentId })
      boardFolders.value.push(response.data)
      if (parentId) {
        expandedBoardFolders.value.add(parentId)
      }
      return response.data
    } catch (error) {
      console.error('Failed to create board folder:', error)
      throw error
    }
  }

  async function renameBoardFolder(id: string, name: string) {
    try {
      const response = await boardFoldersApi.update(id, { name })
      const index = boardFolders.value.findIndex(f => f.id === id)
      if (index !== -1) {
        boardFolders.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to rename board folder:', error)
      throw error
    }
  }

  async function deleteBoardFolder(id: string) {
    try {
      await boardFoldersApi.delete(id)
      boardFolders.value = boardFolders.value.filter(f => f.id !== id)
      // Also remove boards in this folder (they become orphaned)
      boards.value = boards.value.filter(b => b.folder_id !== id)
    } catch (error) {
      console.error('Failed to delete board folder:', error)
      throw error
    }
  }

  async function moveBoardFolder(id: string, parentId: string | null, position: number) {
    try {
      const response = await boardFoldersApi.move(id, { parent_id: parentId, position })
      const index = boardFolders.value.findIndex(f => f.id === id)
      if (index !== -1) {
        boardFolders.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to move board folder:', error)
      throw error
    }
  }

  // Toggle importance for notes/folders
  async function toggleFolderImportance(id: string) {
    const folder = folders.value.find(f => f.id === id)
    if (!folder) return
    try {
      const response = await foldersApi.update(id, { is_important: !folder.is_important })
      const index = folders.value.findIndex(f => f.id === id)
      if (index !== -1) {
        folders.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to toggle folder importance:', error)
      throw error
    }
  }

  async function toggleNoteImportance(id: string) {
    const note = notes.value.find(n => n.id === id)
    if (!note) return
    try {
      const response = await notesApi.update(id, { is_important: !note.is_important })
      const index = notes.value.findIndex(n => n.id === id)
      if (index !== -1) {
        notes.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to toggle note importance:', error)
      throw error
    }
  }
  
  // Toggle urgent status (persisted to database)
  async function toggleNoteUrgent(id: string) {
    const note = notes.value.find(n => n.id === id)
    if (!note) return
    try {
      const response = await notesApi.update(id, { is_urgent: !note.is_urgent })
      const index = notes.value.findIndex(n => n.id === id)
      if (index !== -1) {
        notes.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to toggle note urgent:', error)
      throw error
    }
  }
  
  async function toggleFolderUrgent(id: string) {
    const folder = folders.value.find(f => f.id === id)
    if (!folder) return
    try {
      const response = await foldersApi.update(id, { is_urgent: !folder.is_urgent })
      const index = folders.value.findIndex(f => f.id === id)
      if (index !== -1) {
        folders.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to toggle folder urgent:', error)
      throw error
    }
  }
  
  async function toggleBoardUrgent(id: string) {
    const board = boards.value.find(b => b.id === id)
    if (!board) return
    try {
      const response = await boardsApi.update(id, { is_urgent: !board.is_urgent })
      const index = boards.value.findIndex(b => b.id === id)
      if (index !== -1) {
        boards.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to toggle board urgent:', error)
      throw error
    }
  }
  
  async function toggleBoardFolderUrgent(id: string) {
    const folder = boardFolders.value.find(f => f.id === id)
    if (!folder) return
    try {
      const response = await boardFoldersApi.update(id, { is_urgent: !folder.is_urgent })
      const index = boardFolders.value.findIndex(f => f.id === id)
      if (index !== -1) {
        boardFolders.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to toggle board folder urgent:', error)
      throw error
    }
  }
  
  // Toggle importance for boards/board folders
  async function toggleBoardImportance(id: string) {
    const board = boards.value.find(b => b.id === id)
    if (!board) return
    try {
      const response = await boardsApi.update(id, { is_important: !board.is_important })
      const index = boards.value.findIndex(b => b.id === id)
      if (index !== -1) {
        boards.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to toggle board importance:', error)
      throw error
    }
  }
  
  async function toggleBoardFolderImportance(id: string) {
    const folder = boardFolders.value.find(f => f.id === id)
    if (!folder) return
    try {
      const response = await boardFoldersApi.update(id, { is_important: !folder.is_important })
      const index = boardFolders.value.findIndex(f => f.id === id)
      if (index !== -1) {
        boardFolders.value[index] = response.data
      }
      return response.data
    } catch (error) {
      console.error('Failed to toggle board folder importance:', error)
      throw error
    }
  }
  
  function toggleFavoritesFolder() {
    favoritesFolderExpanded.value = !favoritesFolderExpanded.value
  }
  
  return {
    // State
    folders,
    notes,
    boards,
    boardFolders,
    expandedFolders,
    expandedBoardFolders,
    expandedSections,
    selectedItemId,
    selectedItemType,
    favoritesFolderExpanded,
    
    // Computed
    notesTree,
    boardsList,
    favoritesList,
    
    // Actions
    fetchAll,
    fetchNotes,
    fetchBoards,
    toggleFolder,
    toggleBoardFolder,
    toggleSection,
    selectItem,
    clearSelection,
    getNoteById,
    getBoardById,
    createFolder,
    renameFolder,
    deleteFolder,
    moveFolder,
    createNote,
    updateNote,
    deleteNote,
    moveNote,
    createBoard,
    updateBoard,
    deleteBoard,
    moveBoard,
    createBoardFolder,
    renameBoardFolder,
    deleteBoardFolder,
    moveBoardFolder,
    toggleFolderImportance,
    toggleNoteImportance,
    toggleBoardImportance,
    toggleBoardFolderImportance,
    toggleNoteUrgent,
    toggleFolderUrgent,
    toggleBoardUrgent,
    toggleBoardFolderUrgent,
    toggleFavoritesFolder
  }
})
