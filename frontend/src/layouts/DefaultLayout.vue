<template>
  <div class="main-layout">
    <!-- Ribbon (Far Left) -->
    <Ribbon
      :active-tab="activeTab"
      :user="authStore.user"
      @tab-change="handleTabChange"
      @open-calendar="handleOpenCalendar"
      @open-theme-selector="toggleThemeSelector"
      @open-settings="showSettingsModal = true"
      @open-user-menu="toggleUserMenu"
    />

    <!-- User Menu Dropdown (Floating) -->
    <div v-if="showUserMenu" class="user-dropdown-floating" ref="userMenuRef">
      <div class="dropdown-header">
        <div class="dropdown-user-info">
          <img 
            v-if="authStore.user?.avatar_url" 
            :src="authStore.user.avatar_url" 
            :alt="authStore.user.name"
            class="dropdown-avatar"
          />
          <div v-else class="dropdown-avatar-placeholder">
            {{ authStore.user?.name?.charAt(0).toUpperCase() }}
          </div>
          <div class="dropdown-user-details">
            <span class="dropdown-user-name">{{ authStore.user?.name }}</span>
            <span class="dropdown-user-email">{{ authStore.user?.email }}</span>
          </div>
        </div>
      </div>
      
      <div class="dropdown-section">
        <div class="storage-info">
          <div class="storage-header">
            <Icon name="database" :size="14" />
            <span>Storage</span>
          </div>
          <div class="storage-bar">
            <div class="storage-used" :style="{ width: storagePercent + '%' }"></div>
          </div>
          <div class="storage-text">
            {{ formatBytes(authStore.storageInfo.used) }} / {{ formatBytes(authStore.storageInfo.limit) }}
          </div>
        </div>
      </div>

      <div class="dropdown-divider"></div>

      <button class="dropdown-item" @click="handleLogout">
        <Icon name="log-out" :size="16" />
        <span>Sign out</span>
      </button>
    </div>

    <!-- Theme Selector Dropdown (Floating) -->
    <div v-if="showThemeSelector" class="theme-dropdown-floating" ref="themeMenuRef">
      <div class="dropdown-header">
        <h4 class="dropdown-title">Choose Theme</h4>
      </div>
      
      <div class="theme-options">
        <button 
          v-for="themeOption in themeStore.themes" 
          :key="themeOption.name"
          class="theme-option"
          :class="{ active: themeStore.currentTheme === themeOption.name }"
          @click="selectTheme(themeOption.name)"
        >
          <div class="theme-swatch" :style="{ background: themeOption.colors.bgPrimary }">
            <div class="swatch-accent" :style="{ background: themeOption.colors.accent }"></div>
          </div>
          <div class="theme-info">
            <span class="theme-label">{{ themeOption.label }}</span>
            <div class="theme-colors-preview">
              <div class="color-dot" :style="{ background: themeOption.colors.accent }"></div>
              <div class="color-dot" :style="{ background: themeOption.colors.success }"></div>
              <div class="color-dot" :style="{ background: themeOption.colors.danger }"></div>
            </div>
          </div>
          <Icon v-if="themeStore.currentTheme === themeOption.name" name="check" :size="14" class="active-check" />
        </button>
      </div>
    </div>

    <!-- Mobile Sidebar Toggle -->
    <MobileSidebarToggle
      :is-open="isMobileSidebarOpen"
      @toggle="toggleMobileSidebar"
    />

    <!-- Mobile Sidebar Overlay -->
    <MobileOverlay
      :is-open="isMobileSidebarOpen"
      @close="isMobileSidebarOpen = false"
    />

    <!-- Sidebar -->
    <Sidebar
      :width="sidebarWidth"
      :is-open="isMobileSidebarOpen"
      @refresh="refreshTree"
    >
      <ExplorerTree
        :default-tab="activeTab"
        @select="handleSelect"
        @create-note="handleCreateNote"
        @create-folder="handleCreateFolder"
        @create-board="handleCreateBoard"
        @create-board-folder="handleCreateBoardFolder"
        @create-graph="handleCreateGraph"
        @create-graph-folder="handleCreateGraphFolder"
        @rename="handleRename"
        @delete="handleDelete"
        @open-note="openNote"
        @open-board="openBoard"
        @open-graph="openGraph"
        @drop="handleDrop"
      />
    </Sidebar>

    <!-- Resize Handle -->
    <ResizeHandle
      direction="vertical"
      @resize-start="startSidebarResize"
    />

    <!-- Window Manager -->
    <div class="content-area">
      <WindowManager />
    </div>

    <!-- Modals (Global) -->
    <PromptModal
      :visible="promptModal.visible"
      :title="promptModal.title"
      :placeholder="promptModal.placeholder"
      :initial-value="promptModal.initialValue"
      :confirm-text="promptModal.confirmText"
      @submit="handlePromptSubmit"
      @cancel="promptModal.visible = false"
    />

    <ConfirmModal
      :visible="confirmModal.visible"
      :title="confirmModal.title"
      :message="confirmModal.message"
      :confirm-text="confirmModal.confirmText"
      :variant="confirmModal.variant"
      @confirm="handleConfirmDelete"
      @cancel="confirmModal.visible = false"
    />

    <SettingsModal 
      :is-open="showSettingsModal" 
      @close="showSettingsModal = false" 
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useLayoutStore } from '@/stores/layout'
import { useExplorerStore, type ExplorerItem } from '@/stores/explorer'
import { useAuthStore } from '@/stores/auth'
import { useThemeStore } from '@/stores/theme'
import { Sidebar, MobileSidebarToggle, MobileOverlay, ResizeHandle, Ribbon } from '@/components/layout'
import ExplorerTree from '@/components/ui/ExplorerTree.vue'
import WindowManager from '@/components/window-manager/WindowManager.vue'
import PromptModal from '@/components/modals/PromptModal.vue'
import ConfirmModal from '@/components/modals/ConfirmModal.vue'
import Icon from '@/components/ui/Icon.vue'
import SettingsModal from '@/components/modals/SettingsModal.vue'

const layoutStore = useLayoutStore()
const explorerStore = useExplorerStore()
const authStore = useAuthStore()
const themeStore = useThemeStore()
const router = useRouter()

// Sidebar state
const sidebarWidth = ref(240)
const isMobileSidebarOpen = ref(false)
const isResizingSidebar = ref(false)
const activeTab = ref<'notes' | 'boards' | 'graphs'>('notes')

// User Menu & Settings
const showUserMenu = ref(false)
const showSettingsModal = ref(false)
const userMenuRef = ref<HTMLElement | null>(null)

// Theme Selector
const showThemeSelector = ref(false)
const themeMenuRef = ref<HTMLElement | null>(null)

const storagePercent = computed(() => {
  if (authStore.storageInfo.limit === 0) return 0
  return Math.min(100, (authStore.storageInfo.used / authStore.storageInfo.limit) * 100)
})

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function toggleUserMenu() {
  showUserMenu.value = !showUserMenu.value
}

function toggleThemeSelector() {
  showThemeSelector.value = !showThemeSelector.value
}

function selectTheme(themeName: string) {
  themeStore.setTheme(themeName as any)
  showThemeSelector.value = false
}

function handleLogout() {
  showUserMenu.value = false
  authStore.logout()
  router.push('/login')
}

function handleTabChange(tab: 'notes' | 'boards' | 'graphs') {
  activeTab.value = tab
}

function handleOpenCalendar() {
  // Check if calendar tab already exists and focus it
  const existingCalendarTab = findExistingCalendarTab()
  if (existingCalendarTab) {
    layoutStore.setActiveLeaf(existingCalendarTab.leafId)
    const leaf = layoutStore.findNode(existingCalendarTab.leafId) as any
    if (leaf && leaf.type === 'leaf') {
      leaf.activeTabId = existingCalendarTab.tabId
    }
    return
  }

  // Create new calendar tab if none exists
  if (layoutStore.activeLeafId) {
    layoutStore.addTab(layoutStore.activeLeafId, {
      type: 'calendar',
      entityId: 'calendar',
      title: 'Calendar'
    })
  }
}

function findExistingCalendarTab(): { leafId: string; tabId: string } | null {
  function traverse(node: any): { leafId: string; tabId: string } | null {
    if (node.type === 'leaf') {
      const calendarTab = node.tabs.find((tab: any) => tab.type === 'calendar')
      if (calendarTab) {
        return { leafId: node.id, tabId: calendarTab.id }
      }
    } else if (node.type === 'split') {
      for (const child of node.children) {
        const result = traverse(child)
        if (result) return result
      }
    }
    return null
  }
  return traverse(layoutStore.root)
}

function findExistingEntityTab(type: 'note' | 'board' | 'graph', entityId: string): { leafId: string; tabId: string } | null {
  function traverse(node: any): { leafId: string; tabId: string } | null {
    if (node.type === 'leaf') {
      const entityTab = node.tabs.find((tab: any) => tab.type === type && tab.entityId === entityId)
      if (entityTab) {
        return { leafId: node.id, tabId: entityTab.id }
      }
    } else if (node.type === 'split') {
      for (const child of node.children) {
        const result = traverse(child)
        if (result) return result
      }
    }
    return null
  }
  return traverse(layoutStore.root)
}

function handleClickOutside(event: MouseEvent) {
  if (userMenuRef.value && !userMenuRef.value.contains(event.target as Node)) {
    // Check if the click was on the ribbon avatar to avoid double toggle
    const ribbonAvatar = document.querySelector('.user-avatar-container')
    if (ribbonAvatar && ribbonAvatar.contains(event.target as Node)) return
    showUserMenu.value = false
  }
  
  if (themeMenuRef.value && !themeMenuRef.value.contains(event.target as Node)) {
    // Check if the click was on the ribbon theme button to avoid double toggle
    const ribbonThemeBtn = document.querySelector('.ribbon-btn[title="Change Theme"]')
    if (ribbonThemeBtn && ribbonThemeBtn.contains(event.target as Node)) return
    showThemeSelector.value = false
  }
}

// Modals
const promptModal = ref({
  visible: false,
  title: '',
  placeholder: '',
  action: '',
  parentId: null as string | null,
  initialValue: '',
  confirmText: 'Create',
  item: null as ExplorerItem | null
})

const confirmModal = ref({
  visible: false,
  title: '',
  message: '',
  confirmText: 'Confirm',
  variant: 'primary' as 'primary' | 'danger',
  item: null as ExplorerItem | null
})

// Actions
function openNote(noteId: string) {
  // Check if note tab already exists and focus it
  const existingTab = findExistingEntityTab('note', noteId)
  if (existingTab) {
    layoutStore.setActiveLeaf(existingTab.leafId)
    const leaf = layoutStore.findNode(existingTab.leafId) as any
    if (leaf && leaf.type === 'leaf') {
      leaf.activeTabId = existingTab.tabId
    }
    isMobileSidebarOpen.value = false
    return
  }

  // Create new note tab if none exists
  const note = explorerStore.notes.find(n => n.id === noteId)
  if (layoutStore.activeLeafId) {
    layoutStore.addTab(layoutStore.activeLeafId, {
      type: 'note',
      entityId: noteId,
      title: note?.title || 'Untitled Note'
    })
  }
  isMobileSidebarOpen.value = false
}

function openBoard(boardId: string) {
  // Check if board tab already exists and focus it
  const existingTab = findExistingEntityTab('board', boardId)
  if (existingTab) {
    layoutStore.setActiveLeaf(existingTab.leafId)
    const leaf = layoutStore.findNode(existingTab.leafId) as any
    if (leaf && leaf.type === 'leaf') {
      leaf.activeTabId = existingTab.tabId
    }
    isMobileSidebarOpen.value = false
    return
  }

  // Create new board tab if none exists
  const board = explorerStore.boards.find(b => b.id === boardId)
  if (layoutStore.activeLeafId) {
    layoutStore.addTab(layoutStore.activeLeafId, {
      type: 'board',
      entityId: boardId,
      title: board?.name || 'Untitled Board'
    })
  }
  isMobileSidebarOpen.value = false
}

function openGraph(graphId: string) {
  // Check if graph tab already exists and focus it
  const existingTab = findExistingEntityTab('graph', graphId)
  if (existingTab) {
    layoutStore.setActiveLeaf(existingTab.leafId)
    const leaf = layoutStore.findNode(existingTab.leafId) as any
    if (leaf && leaf.type === 'leaf') {
      leaf.activeTabId = existingTab.tabId
    }
    isMobileSidebarOpen.value = false
    return
  }

  // Create new graph tab if none exists
  const graph = explorerStore.graphs.find(g => g.id === graphId)
  if (layoutStore.activeLeafId) {
    layoutStore.addTab(layoutStore.activeLeafId, {
      type: 'graph',
      entityId: graphId,
      title: graph?.name || 'Untitled Graph'
    })
  }
  isMobileSidebarOpen.value = false
}

// Explorer Handlers
function handleSelect(item: ExplorerItem) {
  explorerStore.selectItem(item.id, item.type as any)
}

function refreshTree() {
  explorerStore.fetchAll()
}

function toggleMobileSidebar() {
  isMobileSidebarOpen.value = !isMobileSidebarOpen.value
}

// Sidebar Resize
function startSidebarResize() {
  isResizingSidebar.value = true
  document.addEventListener('mousemove', onSidebarResize)
  document.addEventListener('mouseup', stopSidebarResize)
}

function onSidebarResize(event: MouseEvent) {
  if (isResizingSidebar.value) {
    sidebarWidth.value = Math.max(150, Math.min(500, event.clientX - 48)) // Subtract ribbon width
  }
}

function stopSidebarResize() {
  isResizingSidebar.value = false
  document.removeEventListener('mousemove', onSidebarResize)
  document.removeEventListener('mouseup', stopSidebarResize)
}

// CRUD Handlers (Simplified for now)
function handleCreateNote(folderId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Note',
    placeholder: 'Note Name',
    action: 'note',
    parentId: folderId,
    initialValue: '',
    confirmText: 'Create',
    item: null
  }
}

function handleCreateFolder(parentId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Folder',
    placeholder: 'Folder Name',
    action: 'folder',
    parentId: parentId,
    initialValue: '',
    confirmText: 'Create',
    item: null
  }
}

function handleCreateBoard(folderId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Board',
    placeholder: 'Board Name',
    action: 'board',
    parentId: folderId,
    initialValue: '',
    confirmText: 'Create',
    item: null
  }
}

function handleCreateBoardFolder(parentId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Board Folder',
    placeholder: 'Folder Name',
    action: 'board-folder',
    parentId: parentId,
    initialValue: '',
    confirmText: 'Create',
    item: null
  }
}

function handleCreateGraph(folderId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Graph',
    placeholder: 'Graph Name',
    action: 'graph',
    parentId: folderId,
    initialValue: '',
    confirmText: 'Create',
    item: null
  }
}

function handleCreateGraphFolder(parentId: string | null) {
  promptModal.value = {
    visible: true,
    title: 'New Graph Folder',
    placeholder: 'Folder Name',
    action: 'graph-folder',
    parentId: parentId,
    initialValue: '',
    confirmText: 'Create',
    item: null
  }
}

function handleRename(item: ExplorerItem) {
  promptModal.value = {
    visible: true,
    title: 'Rename',
    placeholder: 'New Name',
    action: 'rename',
    parentId: null,
    initialValue: item.name,
    confirmText: 'Rename',
    item: item
  }
}

function handleDelete(item: ExplorerItem) {
  confirmModal.value = {
    visible: true,
    title: 'Delete Item',
    message: `Are you sure you want to delete "${item.name}"? This action cannot be undone.`,
    confirmText: 'Delete',
    variant: 'danger',
    item: item
  }
}

function handleConfirmDelete() {
  const item = confirmModal.value.item
  if (!item) return

  if (item.type === 'note') explorerStore.deleteNote(item.id)
  else if (item.type === 'folder') explorerStore.deleteFolder(item.id)
  else if (item.type === 'board') explorerStore.deleteBoard(item.id)
  else if (item.type === 'board-folder') explorerStore.deleteBoardFolder(item.id)
  else if (item.type === 'graph') explorerStore.deleteGraph(item.id)
  else if (item.type === 'graph-folder') explorerStore.deleteGraphFolder(item.id)

  confirmModal.value.visible = false
}

function handleDrop() {
  // TODO: Handle move
}

async function handlePromptSubmit(name: string) {
  const action = promptModal.value.action
  const parentId = promptModal.value.parentId
  const item = promptModal.value.item
  
  promptModal.value.visible = false

  if (action === 'note') {
    const note = await explorerStore.createNote(name, parentId)
    if (note) openNote(note.id)
  } else if (action === 'folder') {
    await explorerStore.createFolder(name, parentId)
  } else if (action === 'board') {
    const board = await explorerStore.createBoard(name, parentId)
    if (board) openBoard(board.id)
  } else if (action === 'board-folder') {
    await explorerStore.createBoardFolder(name, parentId)
  } else if (action === 'graph') {
    const graph = await explorerStore.createGraph(name, parentId)
    if (graph) openGraph(graph.id)
  } else if (action === 'graph-folder') {
    await explorerStore.createGraphFolder(name, parentId)
  } else if (action === 'rename' && item) {
    if (name === item.name) return

    if (item.type === 'note') {
      explorerStore.updateNote(item.id, { title: name })
      layoutStore.updateTabsByEntityId(item.id, { title: name })
    }
    else if (item.type === 'folder') explorerStore.renameFolder(item.id, name)
    else if (item.type === 'board') {
      explorerStore.updateBoard(item.id, { name: name })
      layoutStore.updateTabsByEntityId(item.id, { title: name })
    }
    else if (item.type === 'board-folder') explorerStore.renameBoardFolder(item.id, name)
    else if (item.type === 'graph') {
      explorerStore.updateGraph(item.id, { name: name })
      layoutStore.updateTabsByEntityId(item.id, { title: name })
    }
    else if (item.type === 'graph-folder') explorerStore.renameGraphFolder(item.id, name)
  }
}

onMounted(() => {
  explorerStore.fetchAll()
  document.addEventListener('click', handleClickOutside)
  
  // Global handlers for BBCode links (override App.vue ones)
  ;(window as any).__openNote = openNote
  ;(window as any).__openBoard = openBoard
  ;(window as any).__openGraph = openGraph
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

@media (max-width: 768px) {
  .main-layout {
    flex-direction: column;
  }
}

.content-area {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

@media (max-width: 768px) {
  .content-area {
    height: calc(100vh - 56px);
    height: calc(100dvh - 56px);
  }
}

/* User Dropdown Floating */
.user-dropdown-floating {
  position: absolute;
  bottom: 16px;
  left: 56px;
  min-width: 240px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 1000;
  overflow: hidden;
}

@media (max-width: 768px) {
  .user-dropdown-floating {
    bottom: 64px;
    left: 16px;
    right: 16px;
    min-width: 0;
  }
}

.dropdown-header {
  padding: 12px;
  border-bottom: 1px solid var(--border-primary);
}

.dropdown-user-info {
  display: flex;
  align-items: center;
  gap: 10px;
}

.dropdown-avatar {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  object-fit: cover;
}

.dropdown-avatar-placeholder {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: var(--accent);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 0.9rem;
}

.dropdown-user-details {
  display: flex;
  flex-direction: column;
}

.dropdown-user-name {
  font-weight: 600;
  font-size: 0.85rem;
}

.dropdown-user-email {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.dropdown-section {
  padding: 12px;
}

.storage-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.storage-header {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.storage-bar {
  height: 4px;
  background: var(--bg-primary);
  border-radius: 2px;
  overflow: hidden;
}

.storage-used {
  height: 100%;
  background: var(--accent);
  transition: width 0.3s;
}

.storage-text {
  font-size: 0.7rem;
  color: var(--text-muted);
}

.dropdown-divider {
  height: 1px;
  background: var(--border-primary);
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 10px;
  width: 100%;
  padding: 10px 12px;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 0.85rem;
  cursor: pointer;
  text-align: left;
  transition: background 0.15s;
}

.dropdown-item:hover {
  background: var(--bg-hover);
}

.dropdown-item:hover span {
  color: var(--danger);
}

/* Theme Selector Dropdown */
.theme-dropdown-floating {
  position: absolute;
  bottom: 16px;
  left: 56px;
  min-width: 200px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 1000;
  overflow: hidden;
}

@media (max-width: 768px) {
  .theme-dropdown-floating {
    bottom: 64px;
    left: 16px;
    right: 16px;
    min-width: 0;
  }
}

.theme-dropdown-floating .dropdown-header {
  padding: 12px;
  border-bottom: 1px solid var(--border-primary);
}

.theme-dropdown-floating .dropdown-title {
  margin: 0;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
}

.theme-options {
  padding: 8px;
}

.theme-option {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
  padding: 10px 12px;
  background: transparent;
  border: none;
  border-radius: 8px;
  color: var(--text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
  text-align: left;
  margin-bottom: 4px;
}

.theme-option:hover {
  background: var(--bg-hover);
  transform: translateX(4px);
}

.theme-option.active {
  background: var(--accent-light);
  color: var(--accent);
}

.theme-swatch {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  border: 1px solid var(--border-primary);
  position: relative;
  overflow: hidden;
  flex-shrink: 0;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.swatch-accent {
  position: absolute;
  bottom: 0;
  right: 0;
  width: 50%;
  height: 50%;
  border-top-left-radius: 4px;
}

.theme-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.theme-label {
  font-size: 0.85rem;
  font-weight: 600;
}

.theme-colors-preview {
  display: flex;
  gap: 4px;
}

.color-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
}

.active-check {
  color: var(--accent);
}
</style>

