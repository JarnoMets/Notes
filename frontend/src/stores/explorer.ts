import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Note, NoteFolder, Board, BoardFolder, Graph, GraphFolder } from '../types'
import { notesApi, foldersApi, boardsApi, boardFoldersApi, graphsApi, graphFoldersApi } from '../api'
import logger from '@/utils/logger'
import { useTreeState } from '@/utils/tree'

export interface ExplorerItem {
  id: string
  name: string
  type: 'folder' | 'note' | 'board' | 'board-folder' | 'graph' | 'graph-folder' | 'favorites-folder'
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
	const graphs = ref<Graph[]>([])
	const graphFolders = ref<GraphFolder[]>([])

	// UI state
	const expandedFolders = ref<Set<string>>(new Set())
	const expandedBoardFolders = ref<Set<string>>(new Set())
	const expandedGraphFolders = ref<Set<string>>(new Set())

	// Tree Managers
	const notesManager = useTreeState(notes, folders, expandedFolders, notesApi, foldersApi, { folderKey: 'folder_id', nameKey: 'title' })
	const boardsManager = useTreeState(boards, boardFolders, expandedBoardFolders, boardsApi, boardFoldersApi, { folderKey: 'folder_id', nameKey: 'name' })
	const graphsManager = useTreeState(graphs, graphFolders, expandedGraphFolders, graphsApi, graphFoldersApi, { folderKey: 'folder_id', nameKey: 'name' })

	const expandedSections = ref({
		notes: true,
		graphs: true,
		boards: true
	})
	const selectedItemId = ref<string | null>(null)
	const selectedItemType = ref<'note' | 'folder' | 'board' | 'board-folder' | 'graph' | 'graph-folder' | null>(null)
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
	
	// Helper to check if a graph folder has urgent descendants
	function checkGraphUrgentDescendants(folderId: string): boolean {
		// Check graphs in this folder
		const folderGraphs = graphs.value.filter(g => g.folder_id === folderId)
		if (folderGraphs.some(g => g.is_urgent)) {
			return true
		}
		// Check subfolders
		const subFolders = graphFolders.value.filter(f => f.parent_id === folderId)
		for (const sub of subFolders) {
			if (sub.is_urgent || checkGraphUrgentDescendants(sub.id)) {
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
		
		// Add starred graphs
		const starredGraphs = graphs.value.filter(g => g.is_important)
		for (const graph of starredGraphs) {
			items.push({
				id: graph.id,
				name: graph.name,
				type: 'graph',
				parentId: '__favorites__',
				position: graph.position || 0,
				isExpanded: false,
				isImportant: true,
				isUrgent: graph.is_urgent,
				hasUrgentDescendant: false,
				children: []
			})
		}
		
		// Add starred graph folders
		const starredGraphFolders = graphFolders.value.filter(f => f.is_important)
		for (const folder of starredGraphFolders) {
			items.push({
				id: folder.id,
				name: folder.name,
				type: 'graph-folder',
				parentId: '__favorites__',
				position: folder.position,
				isExpanded: false,
				isImportant: true,
				isUrgent: folder.is_urgent,
				hasUrgentDescendant: checkGraphUrgentDescendants(folder.id),
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
	
	// Build tree for graphs section
	const graphsList = computed<ExplorerItem[]>(() => {
		const buildGraphTree = (parentId: string | null): ExplorerItem[] => {
			const items: ExplorerItem[] = []

			// Add graph folders
			const childFolders = graphFolders.value
				.filter(f => f.parent_id === parentId)
				.sort((a, b) => a.position - b.position)

			for (const folder of childFolders) {
				const children = buildGraphTree(folder.id)
				const isUrgent = folder.is_urgent
				const hasUrgentDescendant = !isUrgent && checkGraphUrgentDescendants(folder.id)
				items.push({
					id: folder.id,
					name: folder.name,
					type: 'graph-folder',
					parentId: folder.parent_id,
					position: folder.position,
					isExpanded: expandedGraphFolders.value.has(folder.id),
					isImportant: folder.is_important,
					isUrgent,
					hasUrgentDescendant,
					children
				})
			}

			// Add graphs
			const childGraphs = graphs.value
				.filter(g => (g.folder_id || null) === parentId)
				.sort((a, b) => (a.position || 0) - (b.position || 0))

			for (const graph of childGraphs) {
				items.push({
					id: graph.id,
					name: graph.name,
					type: 'graph',
					parentId: graph.folder_id || null,
					position: graph.position || 0,
					isExpanded: false,
					isImportant: graph.is_important,
					isUrgent: graph.is_urgent,
					hasUrgentDescendant: false,
					children: []
				})
			}

			return items
		}

		return buildGraphTree(null)
	})
	
	// Fetch all data
	async function fetchAll() {
		await Promise.all([fetchNotesTree(), fetchBoardsTree(), fetchGraphsTree()])
	}
	
	async function fetchNotesTree() {
		try {
			const response = await notesApi.getTree()
			folders.value = response.data.folders
			notes.value = response.data.notes
		} catch (error) {
			logger.error('Failed to fetch notes tree:', error)
		}
	}
	
	async function fetchBoardsTree() {
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
				logger.error('Failed to fetch boards:', e)
			}
		}
	}
	
	async function fetchGraphsTree() {
		try {
			const response = await graphsApi.getTree()
			graphFolders.value = response.data.folders || []
			graphs.value = response.data.graphs || []
		} catch (error) {
			// Fallback to simple getAll if tree endpoint doesn't exist yet
			try {
				const response = await graphsApi.getAll()
				graphs.value = response.data
				graphFolders.value = []
			} catch (e) {
				logger.error('Failed to fetch graphs:', e)
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
	
	function toggleGraphFolder(folderId: string) {
		if (expandedGraphFolders.value.has(folderId)) {
			expandedGraphFolders.value.delete(folderId)
		} else {
			expandedGraphFolders.value.add(folderId)
		}
	}
	
	function toggleSection(section: 'notes' | 'graphs' | 'boards') {
		expandedSections.value[section] = !expandedSections.value[section]
	}
	
	// Selection
	function selectItem(id: string, type: 'note' | 'folder' | 'board' | 'board-folder' | 'graph' | 'graph-folder') {
		selectedItemId.value = id
		selectedItemType.value = type
	}

	function clearSelection() {
		selectedItemId.value = null
		selectedItemType.value = null
	}

	// Getters
	function getNoteById(id: string): Note | undefined {
		return notes.value.find(n => n.id === id)
	}

	function getBoardById(id: string): Board | undefined {
		return boards.value.find(b => b.id === id)
	}

	function getGraphById(id: string): Graph | undefined {
		return graphs.value.find(g => g.id === id)
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
		graphs,
		graphFolders,
		expandedFolders,
		expandedBoardFolders,
		expandedGraphFolders,
		expandedSections,
		selectedItemId,
		selectedItemType,
		favoritesFolderExpanded,

		// Computed
		notesTree,
		boardsList,
		graphsList,
		favoritesList,

		// Actions
		fetchAll,
		fetchNotesTree,
		fetchBoardsTree,
		fetchGraphsTree,
		toggleFolder,
		toggleBoardFolder,
		toggleGraphFolder,
		toggleSection,
		selectItem,
		clearSelection,
		getNoteById,
		getBoardById,
		getGraphById,

		// Note/Folder Actions
		createFolder: notesManager.createFolder,
		renameFolder: notesManager.renameFolder,
		deleteFolder: notesManager.deleteFolder,
		moveFolder: notesManager.moveFolder,
		createNote: notesManager.createItem,
		updateNote: notesManager.updateItem,
		deleteNote: notesManager.deleteItem,
		moveNote: notesManager.moveItem,
		toggleFolderImportance: notesManager.toggleFolderImportance,
		toggleNoteImportance: notesManager.toggleItemImportance,
		toggleNoteUrgent: notesManager.toggleItemUrgent,
		toggleFolderUrgent: notesManager.toggleFolderUrgent,

		// Board Actions
		createBoard: boardsManager.createItem,
		updateBoard: boardsManager.updateItem,
		deleteBoard: boardsManager.deleteItem,
		moveBoard: boardsManager.moveItem,
		createBoardFolder: boardsManager.createFolder,
		renameBoardFolder: boardsManager.renameFolder,
		deleteBoardFolder: boardsManager.deleteFolder,
		moveBoardFolder: boardsManager.moveFolder,
		toggleBoardUrgent: boardsManager.toggleItemUrgent,
		toggleBoardFolderUrgent: boardsManager.toggleFolderUrgent,
		toggleBoardImportance: boardsManager.toggleItemImportance,
		toggleBoardFolderImportance: boardsManager.toggleFolderImportance,

		// Graph Actions
		createGraph: graphsManager.createItem,
		updateGraph: graphsManager.updateItem,
		deleteGraph: graphsManager.deleteItem,
		moveGraph: graphsManager.moveItem,
		createGraphFolder: graphsManager.createFolder,
		renameGraphFolder: graphsManager.renameFolder,
		deleteGraphFolder: graphsManager.deleteFolder,
		moveGraphFolder: graphsManager.moveFolder,
		toggleGraphImportance: graphsManager.toggleItemImportance,
		toggleGraphFolderImportance: graphsManager.toggleFolderImportance,
		toggleGraphUrgent: graphsManager.toggleItemUrgent,
		toggleGraphFolderUrgent: graphsManager.toggleFolderUrgent,

		toggleFavoritesFolder
	}
})
