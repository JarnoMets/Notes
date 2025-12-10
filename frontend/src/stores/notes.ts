import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Note, NoteFolder, NotesTree, NoteWithAttachments } from '../types'
import { notesApi, foldersApi } from '../api'
import { useExplorerStore } from './explorer'

export interface OpenTab {
  id: string
  noteId: string
  title: string
  isDirty: boolean
}

export interface EditorPane {
  id: string
  tabs: OpenTab[]
  activeTabId: string | null
}

// We need to import the explorer store dynamically to avoid circular deps
// Use the explorer store directly. Importing here is fine since explorer doesn't import notes.

export const useNotesStore = defineStore('notes', () => {
  // File tree state
  const folders = ref<NoteFolder[]>([])
  const notes = ref<Note[]>([])
  const expandedFolders = ref<Set<string>>(new Set())
  const selectedItemId = ref<string | null>(null)
  const selectedItemType = ref<'note' | 'folder' | null>(null)

  // Editor panes state
  const panes = ref<EditorPane[]>([
    { id: 'pane-1', tabs: [], activeTabId: null }
  ])
  const activePaneId = ref<string>('pane-1')
  const splitDirection = ref<'horizontal' | 'vertical' | null>(null)

  // Note content cache
  const noteCache = ref<Map<string, NoteWithAttachments>>(new Map())

  // Computed
  const activePane = computed(() => panes.value.find(p => p.id === activePaneId.value))
  const hasSecondPane = computed(() => panes.value.length > 1)

  // File tree actions
  async function fetchTree() {
    try {
      const response = await notesApi.getTree()
      const tree: NotesTree = response.data
      folders.value = tree.folders
      notes.value = tree.notes
    } catch (error) {
      console.error('Failed to fetch notes tree:', error)
    }
  }

  function toggleFolder(folderId: string) {
    if (expandedFolders.value.has(folderId)) {
      expandedFolders.value.delete(folderId)
    } else {
      expandedFolders.value.add(folderId)
    }
  }

  function selectItem(id: string, type: 'note' | 'folder') {
    selectedItemId.value = id
    selectedItemType.value = type
  }

  // Folder actions
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

  async function deleteFolder(id: string) {
    try {
      await foldersApi.delete(id)
      folders.value = folders.value.filter(f => f.id !== id)
      // Also remove notes in the folder from cache
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

  // Note actions
  async function createNote(title: string, folderId: string | null = null) {
    try {
      const response = await notesApi.create({ title, folder_id: folderId })
      notes.value.push(response.data)
      if (folderId) {
        expandedFolders.value.add(folderId)
      }
      // Open the new note
      openNote(response.data.id)
      return response.data
    } catch (error) {
      console.error('Failed to create note:', error)
      throw error
    }
  }

  async function deleteNote(id: string) {
    try {
      await notesApi.delete(id)
      notes.value = notes.value.filter(n => n.id !== id)
      noteCache.value.delete(id)
      // Close tabs for this note
      panes.value.forEach(pane => {
        pane.tabs = pane.tabs.filter(t => t.noteId !== id)
        if (pane.activeTabId && !pane.tabs.find(t => t.id === pane.activeTabId)) {
          pane.activeTabId = pane.tabs[0]?.id || null
        }
      })
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

  async function fetchNote(id: string): Promise<NoteWithAttachments | null> {
    // Check cache first
    if (noteCache.value.has(id)) {
      return noteCache.value.get(id)!
    }

    try {
      const response = await notesApi.get(id)
      noteCache.value.set(id, response.data)
      return response.data
    } catch (error) {
      console.error('Failed to fetch note:', error)
      return null
    }
  }

  async function updateNote(id: string, data: Partial<Note>) {
    try {
      const response = await notesApi.update(id, data)
      // Update cache
      if (noteCache.value.has(id)) {
        const cached = noteCache.value.get(id)!
        noteCache.value.set(id, { ...cached, ...response.data })
      }
      // Update notes list
      const index = notes.value.findIndex(n => n.id === id)
      if (index !== -1) {
        notes.value[index] = { ...notes.value[index], ...response.data }
      }
      // Update tab title if needed
      if (data.title) {
        panes.value.forEach(pane => {
          const tab = pane.tabs.find(t => t.noteId === id)
          if (tab) {
            tab.title = data.title!
          }
        })
      }
      return response.data
    } catch (error) {
      console.error('Failed to update note:', error)
      throw error
    }
  }

  async function toggleNoteImportance(id: string) {
    const note = notes.value.find(n => n.id === id)
    if (!note) return
    try {
      const response = await notesApi.update(id, { is_important: !note.is_important })
      // Update cache
      if (noteCache.value.has(id)) {
        const cached = noteCache.value.get(id)!
        noteCache.value.set(id, { ...cached, ...response.data })
      }
      // Update notes list
      const index = notes.value.findIndex(n => n.id === id)
      if (index !== -1) {
        notes.value[index] = { ...notes.value[index], ...response.data }
      }
      return response.data
    } catch (error) {
      console.error('Failed to toggle note importance:', error)
      throw error
    }
  }

  function invalidateNoteCache(id: string) {
    noteCache.value.delete(id)
  }

  // Tab/Pane actions
  function openNote(noteId: string, paneId?: string) {
    const targetPaneId = paneId || activePaneId.value
    const pane = panes.value.find(p => p.id === targetPaneId)
    if (!pane) return

    // Check if already open in this pane
    const existingTab = pane.tabs.find(t => t.noteId === noteId)
    if (existingTab) {
      pane.activeTabId = existingTab.id
      activePaneId.value = targetPaneId
      return
    }

    // Find note title - check local notes first, then explorer store
    let title = 'Untitled'
    const note = notes.value.find(n => n.id === noteId)
    if (note?.title) {
      title = note.title
    } else {
      // Try to get from explorer store
        const explorerStore = useExplorerStore()
        const explorerNote = explorerStore.getNoteById(noteId)
      if (explorerNote?.title) {
        title = explorerNote.title
      }
    }

    // Create new tab
    const newTab: OpenTab = {
      id: `tab-${Date.now()}`,
      noteId,
      title,
      isDirty: false
    }

    pane.tabs.push(newTab)
    pane.activeTabId = newTab.id
    activePaneId.value = targetPaneId
  }

  function openNoteAtIndex(noteId: string, paneId: string, insertIndex: number) {
    const pane = panes.value.find(p => p.id === paneId)
    if (!pane) return

    // Check if already open in this pane
    const existingTabIndex = pane.tabs.findIndex(t => t.noteId === noteId)
    if (existingTabIndex !== -1) {
      // Move the existing tab to the new position
      const [existingTab] = pane.tabs.splice(existingTabIndex, 1)
      const actualIndex = insertIndex === -1 ? pane.tabs.length : Math.min(insertIndex, pane.tabs.length)
      pane.tabs.splice(actualIndex, 0, existingTab)
      pane.activeTabId = existingTab.id
      activePaneId.value = paneId
      return
    }

    // Find note title
    let title = 'Untitled'
    const note = notes.value.find(n => n.id === noteId)
    if (note?.title) {
      title = note.title
    } else {
      const explorerStore = useExplorerStore()
      const explorerNote = explorerStore.getNoteById(noteId)
      if (explorerNote?.title) {
        title = explorerNote.title
      }
    }

    // Create new tab
    const newTab: OpenTab = {
      id: `tab-${Date.now()}`,
      noteId,
      title,
      isDirty: false
    }

    // Insert at specified index
    const actualIndex = insertIndex === -1 ? pane.tabs.length : Math.min(insertIndex, pane.tabs.length)
    pane.tabs.splice(actualIndex, 0, newTab)
    pane.activeTabId = newTab.id
    activePaneId.value = paneId
  }

  function openNoteInNewSplit(noteId: string, direction: 'vertical' | 'horizontal' = 'vertical') {
    // First, create a split if we don't have one
    if (panes.value.length < 2) {
      const newPane: EditorPane = {
        id: `pane-${Date.now()}`,
        tabs: [],
        activeTabId: null
      }
      panes.value.push(newPane)
      splitDirection.value = direction
    }

    // Find the second pane
    const secondPane = panes.value[1]
    if (!secondPane) return

    // Open the note in the second pane
    openNote(noteId, secondPane.id)
  }

  function closeTab(tabId: string, paneId: string) {
    const pane = panes.value.find(p => p.id === paneId)
    if (!pane) return

    const tabIndex = pane.tabs.findIndex(t => t.id === tabId)
    if (tabIndex === -1) return

    pane.tabs.splice(tabIndex, 1)

    // Update active tab
    if (pane.activeTabId === tabId) {
      if (pane.tabs.length > 0) {
        // Select the tab before or after
        const newIndex = Math.min(tabIndex, pane.tabs.length - 1)
        pane.activeTabId = pane.tabs[newIndex].id
      } else {
        pane.activeTabId = null
      }
    }

    // If pane is empty and we have multiple panes, remove it
    if (pane.tabs.length === 0 && panes.value.length > 1) {
      const paneIndex = panes.value.findIndex(p => p.id === paneId)
      panes.value.splice(paneIndex, 1)
      if (activePaneId.value === paneId) {
        activePaneId.value = panes.value[0].id
      }
      splitDirection.value = null
    }
  }

  function setActiveTab(tabId: string, paneId: string) {
    const pane = panes.value.find(p => p.id === paneId)
    if (pane) {
      pane.activeTabId = tabId
      activePaneId.value = paneId
    }
  }

  function setTabDirty(tabId: string, isDirty: boolean) {
    panes.value.forEach(pane => {
      const tab = pane.tabs.find(t => t.id === tabId)
      if (tab) {
        tab.isDirty = isDirty
      }
    })
  }

  function splitPane(direction: 'horizontal' | 'vertical') {
    if (panes.value.length >= 2) return // Max 2 panes for now

    const activePane = panes.value.find(p => p.id === activePaneId.value)

    // If the active pane has more than one tab, move the focused tab into the new pane
    if (activePane && activePane.tabs.length > 1 && activePane.activeTabId) {
      const tabIndex = activePane.tabs.findIndex(t => t.id === activePane.activeTabId)
      if (tabIndex !== -1) {
        const [movedTab] = activePane.tabs.splice(tabIndex, 1)

        const newPane: EditorPane = {
          id: `pane-${Date.now()}`,
          tabs: [movedTab],
          activeTabId: movedTab.id
        }

        panes.value.push(newPane)
        splitDirection.value = direction

        // Set the newly created pane as active
        activePaneId.value = newPane.id
        return
      }
    }

    // If there's an active tab in the active pane, copy it to the new pane
    if (activePane && activePane.activeTabId) {
      const activeTab = activePane.tabs.find(t => t.id === activePane.activeTabId)
      if (activeTab) {
        const copiedTab: OpenTab = {
          id: `tab-${Date.now()}`,
          noteId: activeTab.noteId,
          title: activeTab.title,
          isDirty: false
        }

        const newPane: EditorPane = {
          id: `pane-${Date.now()}`,
          tabs: [copiedTab],
          activeTabId: copiedTab.id
        }

        panes.value.push(newPane)
        splitDirection.value = direction

        // Set the newly created pane as active
        activePaneId.value = newPane.id
        return
      }
    }

    // Fallback: create an empty pane (no active tab)
    const newPane: EditorPane = {
      id: `pane-${Date.now()}`,
      tabs: [],
      activeTabId: null
    }

    panes.value.push(newPane)
    splitDirection.value = direction
  }

  function moveTabToPane(tabId: string, fromPaneId: string, toPaneId: string) {
    const fromPane = panes.value.find(p => p.id === fromPaneId)
    const toPane = panes.value.find(p => p.id === toPaneId)
    if (!fromPane || !toPane) return

    const tabIndex = fromPane.tabs.findIndex(t => t.id === tabId)
    if (tabIndex === -1) return

    const [tab] = fromPane.tabs.splice(tabIndex, 1)
    toPane.tabs.push(tab)
    toPane.activeTabId = tab.id

    // Update active tab in source pane
    if (fromPane.activeTabId === tabId) {
      fromPane.activeTabId = fromPane.tabs[0]?.id || null
    }

    // Remove empty pane
    if (fromPane.tabs.length === 0 && panes.value.length > 1) {
      const paneIndex = panes.value.findIndex(p => p.id === fromPaneId)
      panes.value.splice(paneIndex, 1)
      if (activePaneId.value === fromPaneId) {
        activePaneId.value = toPaneId
      }
      splitDirection.value = null
    }

    activePaneId.value = toPaneId
  }

  function closeSplit() {
    if (panes.value.length <= 1) return

    // Merge all tabs into first pane
    const firstPane = panes.value[0]
    for (let i = 1; i < panes.value.length; i++) {
      firstPane.tabs.push(...panes.value[i].tabs)
    }

    panes.value = [firstPane]
    activePaneId.value = firstPane.id
    splitDirection.value = null
  }

  return {
    // State
    folders,
    notes,
    expandedFolders,
    selectedItemId,
    selectedItemType,
    panes,
    activePaneId,
    splitDirection,
    noteCache,

    // Computed
    activePane,
    hasSecondPane,

    // Actions
    fetchTree,
    toggleFolder,
    selectItem,
    createFolder,
    renameFolder,
    toggleFolderImportance,
    deleteFolder,
    moveFolder,
    createNote,
    deleteNote,
    moveNote,
    fetchNote,
    updateNote,
    toggleNoteImportance,
    invalidateNoteCache,
    openNote,
    openNoteAtIndex,
    openNoteInNewSplit,
    closeTab,
    setActiveTab,
    setTabDirty,
    splitPane,
    moveTabToPane,
    closeSplit
  }
})
