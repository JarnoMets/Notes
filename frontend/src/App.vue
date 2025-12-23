<template>
  <div class="app" :class="['theme-' + themeStore.currentTheme]">
    <nav class="navbar" v-if="authStore.isAuthenticated">
      <div class="nav-brand" style="flex: 1; font-weight: bold; font-size: 1.2rem;">
        Notes
      </div>
      <div class="nav-right">
        <!-- Theme Switcher -->
        <div class="theme-switcher" ref="themeSwitcherRef">
          <button class="theme-btn" @click="showThemeMenu = !showThemeMenu" title="Change theme">
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="5"/>
              <line x1="12" y1="1" x2="12" y2="3"/>
              <line x1="12" y1="21" x2="12" y2="23"/>
              <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/>
              <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/>
              <line x1="1" y1="12" x2="3" y2="12"/>
              <line x1="21" y1="12" x2="23" y2="12"/>
              <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/>
              <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/>
            </svg>
          </button>
          <div v-if="showThemeMenu" class="theme-menu">
            <button 
              v-for="(t, key) in themes" 
              :key="key"
              class="theme-option"
              :class="{ active: themeStore.currentTheme === key }"
              @click="selectTheme(key as ThemeName)"
            >
              <span class="theme-preview" :style="getThemePreviewStyle(t)"></span>
              <span>{{ t.label }}</span>
              <svg v-if="themeStore.currentTheme === key" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <polyline points="20 6 9 17 4 12"/>
              </svg>
            </button>
          </div>
        </div>

        <!-- Settings Button -->
        <button class="theme-btn settings-btn" @click="showSettingsModal = true" title="Settings">
          <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
          </svg>
        </button>

        <!-- User Profile Dropdown -->
        <div class="user-menu" ref="userMenuRef">
          <button class="user-menu-trigger" @click="showUserMenu = !showUserMenu">
            <img 
              v-if="authStore.user?.avatar_url" 
              :src="authStore.user.avatar_url" 
              :alt="authStore.user.name"
              class="user-avatar"
            />
            <div v-else class="user-avatar-placeholder">
              {{ authStore.user?.name?.charAt(0).toUpperCase() }}
            </div>
            <span class="user-name">{{ authStore.user?.name }}</span>
            <svg class="chevron" :class="{ rotated: showUserMenu }" width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <polyline points="6 9 12 15 18 9"/>
            </svg>
          </button>
          <div v-if="showUserMenu" class="user-dropdown">
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
                  <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <ellipse cx="12" cy="5" rx="9" ry="3"/>
                    <path d="M21 12c0 1.66-4 3-9 3s-9-1.34-9-3"/>
                    <path d="M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5"/>
                  </svg>
                  <span>Storage</span>
                </div>
                <div class="storage-bar">
                  <div class="storage-used" :style="{ width: storagePercent + '%' }"></div>
                </div>
                <div class="storage-text">
                  {{ formatBytes(storageUsed) }} / {{ formatBytes(storageLimit) }}
                </div>
              </div>
            </div>

            <div class="dropdown-divider"></div>

            <button class="dropdown-item" @click="handleLogout">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/>
                <polyline points="16 17 21 12 16 7"/>
                <line x1="21" y1="12" x2="9" y2="12"/>
              </svg>
              <span>Sign out</span>
            </button>
          </div>
        </div>
      </div>
    </nav>
    <main class="main-content" :class="{ 'no-nav': !authStore.isAuthenticated }">
      <router-view />
    </main>
    
    <!-- Settings Modal -->
    <SettingsModal 
      :is-open="showSettingsModal" 
      @close="showSettingsModal = false" 
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watchEffect } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from './stores/auth'
import { useThemeStore, themes, type ThemeName, type Theme } from './stores/theme'
import { useSettingsStore } from './stores/settings'
import SettingsModal from './components/common/modals/SettingsModal.vue'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const themeStore = useThemeStore()
const settingsStore = useSettingsStore()

// Update document title when route changes
watchEffect(() => {
  if (route.path.startsWith('/notes')) document.title = 'Notes'
  else if (route.path.startsWith('/boards')) document.title = 'Boards'
  else if (route.path.startsWith('/calendar')) document.title = 'Calendar'
  else document.title = 'Notes'
})

const showThemeMenu = ref(false)
const showUserMenu = ref(false)
const showSettingsModal = ref(false)
const themeSwitcherRef = ref<HTMLElement | null>(null)
const userMenuRef = ref<HTMLElement | null>(null)

// Storage info (placeholder - will be replaced with API data)
const storageUsed = computed(() => authStore.storageInfo.used)
const storageLimit = computed(() => authStore.storageInfo.limit)

const storagePercent = computed(() => {
  if (storageLimit.value === 0) return 0
  return Math.min(100, (storageUsed.value / storageLimit.value) * 100)
})

function formatBytes(bytes: number): string {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

function handleLogout() {
  showUserMenu.value = false
  authStore.logout()
  router.push('/login')
}

function selectTheme(themeName: ThemeName) {
  themeStore.setTheme(themeName)
  showThemeMenu.value = false
}

function getThemePreviewStyle(theme: Theme) {
  return {
    background: `linear-gradient(135deg, ${theme.colors.bgPrimary} 50%, ${theme.colors.accent} 50%)`
  }
}

function handleClickOutside(event: MouseEvent) {
  if (themeSwitcherRef.value && !themeSwitcherRef.value.contains(event.target as Node)) {
    showThemeMenu.value = false
  }
  if (userMenuRef.value && !userMenuRef.value.contains(event.target as Node)) {
    showUserMenu.value = false
  }
}

// Global handlers for BBCode links are now managed by MainLayout.vue
/*
;(window as any).__openNote = (noteId: string) => {
  router.push('/notes')
  notesStore.openNote(noteId)
}

;(window as any).__openBoard = (boardId: string) => {
  router.push({ path: '/boards', query: { board: boardId } })
}

;(window as any).__openGraph = (graphId: string) => {
  router.push({ path: '/graphs', query: { graph: graphId } })
}
*/

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
  themeStore.applyTheme()
  
  // Initialize settings and calendars when app loads
  if (authStore.isAuthenticated) {
    // Load settings from server (theme and calendars)
    themeStore.loadFromServer()
    settingsStore.loadFromServer().then(() => {
      // Initialize ICS calendars after settings are loaded
      settingsStore.initializeCalendars()
    })
    // Fetch storage info
    authStore.fetchStorageInfo()
  }
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
.app {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

.navbar {
  background: var(--bg-secondary);
  padding: 0 1.5rem;
  display: flex;
  align-items: center;
  height: 56px;
  border-bottom: 1px solid var(--border-primary);
  gap: 1rem;
}

.nav-links {
  display: flex;
  gap: 0.25rem;
  flex: 1;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  color: var(--text-secondary);
  text-decoration: none;
  font-weight: 500;
  font-size: 0.9375rem;
  padding: 0.625rem 1.25rem;
  border-radius: 8px;
  transition: all 0.2s;
}

.nav-link span {
  display: inline;
}

.nav-link:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.nav-link.router-link-active {
  background: var(--accent-light);
  color: var(--accent);
}

.nav-right {
  display: flex;
  align-items: center;
  gap: 1rem;
}

/* Theme Switcher */
.theme-switcher {
  position: relative;
  display: flex;
  align-items: center;
}

.theme-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.2s;
}

.theme-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.theme-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.5rem;
  min-width: 160px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
  overflow: hidden;
}

.theme-option {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  padding: 0.625rem 1rem;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 0.875rem;
  cursor: pointer;
  text-align: left;
  transition: background 0.15s;
}

.theme-option:hover {
  background: var(--bg-hover);
}

.theme-option.active {
  background: var(--accent-light);
}

.theme-option svg {
  margin-left: auto;
  color: var(--accent);
}

.theme-preview {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  border: 2px solid var(--border-primary);
}

/* User Menu */
.user-menu {
  position: relative;
}

.user-menu-trigger {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0.5rem 0.25rem 0.25rem;
  background: transparent;
  border: 1px solid transparent;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.user-menu-trigger:hover {
  background: var(--bg-hover);
  border-color: var(--border-primary);
}

.user-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  object-fit: cover;
}

.user-avatar-placeholder {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--accent);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 0.875rem;
}

.user-name {
  color: var(--text-secondary);
  font-size: 0.875rem;
  font-weight: 500;
}

.chevron {
  color: var(--text-muted);
  transition: transform 0.2s;
}

.chevron.rotated {
  transform: rotate(180deg);
}

.user-dropdown {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.5rem;
  min-width: 280px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
  overflow: hidden;
}

.dropdown-header {
  padding: 1rem;
  border-bottom: 1px solid var(--border-primary);
}

.dropdown-user-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.dropdown-avatar {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  object-fit: cover;
}

.dropdown-avatar-placeholder {
  width: 40px;
  height: 40px;
  border-radius: 50%;
  background: var(--accent);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 1rem;
}

.dropdown-user-details {
  display: flex;
  flex-direction: column;
  gap: 0.125rem;
}

.dropdown-user-name {
  font-weight: 600;
  font-size: 0.875rem;
  color: var(--text-primary);
}

.dropdown-user-email {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.dropdown-section {
  padding: 0.75rem 1rem;
}

.storage-info {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.storage-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.storage-bar {
  height: 6px;
  background: var(--bg-primary);
  border-radius: 3px;
  overflow: hidden;
}

.storage-used {
  height: 100%;
  background: var(--accent);
  border-radius: 3px;
  transition: width 0.3s;
}

.storage-text {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.dropdown-divider {
  height: 1px;
  background: var(--border-primary);
}

.dropdown-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  width: 100%;
  padding: 0.75rem 1rem;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 0.875rem;
  cursor: pointer;
  text-align: left;
  transition: background 0.15s;
}

.dropdown-item:hover {
  background: var(--bg-hover);
}

.dropdown-item svg {
  color: var(--text-muted);
}

.dropdown-item:last-child {
  border-radius: 0 0 12px 12px;
}

.dropdown-item:hover svg {
  color: var(--danger);
}

.dropdown-item:hover {
  color: var(--danger);
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.main-content.no-nav {
  padding: 0;
}

/* ========================================
   RESPONSIVE DESIGN - TABLET (max-width: 1024px)
   ======================================== */
@media (max-width: 1024px) {
  .navbar {
    padding: 0 1rem;
    gap: 0.75rem;
  }

  .nav-link {
    padding: 0.5rem 1rem;
  }

  .user-name {
    display: none;
  }
  
  .chevron {
    display: none;
  }
}

/* ========================================
   RESPONSIVE DESIGN - SMARTPHONE (max-width: 768px)
   ======================================== */
@media (max-width: 768px) {
  .navbar {
    padding: 0 0.75rem;
    gap: 0.5rem;
    height: 52px;
  }

  .nav-links {
    gap: 0.125rem;
  }

  .nav-link {
    padding: 0.5rem 0.75rem;
    font-size: 0.8rem;
  }

  .nav-link svg {
    width: 18px;
    height: 18px;
  }

  .nav-right {
    gap: 0.5rem;
  }

  .theme-btn {
    width: 40px;
    height: 40px;
  }

  .user-avatar,
  .user-avatar-placeholder {
    width: 28px;
    height: 28px;
    font-size: 0.75rem;
  }
  
  .user-menu-trigger {
    padding: 0.25rem;
  }

  /* Theme menu mobile */
  .theme-menu,
  .user-dropdown {
    position: fixed;
    left: 1rem;
    right: 1rem;
    top: auto;
    bottom: 1rem;
    border-radius: 12px;
  }
  
  .user-dropdown {
    min-width: auto;
  }

  .theme-option,
  .dropdown-item {
    padding: 0.875rem 1rem;
  }
}

/* ========================================
   RESPONSIVE DESIGN - SMALL PHONES (max-width: 480px)
   ======================================== */
@media (max-width: 480px) {
  .navbar {
    height: 48px;
    padding: 0 0.5rem;
  }

  .nav-link {
    padding: 0.375rem 0.5rem;
  }

  .nav-link span {
    display: none;
  }

  .theme-btn {
    width: 36px;
    height: 36px;
  }

  .theme-btn svg {
    width: 16px;
    height: 16px;
  }
}

/* ========================================
   TOUCH DEVICE OPTIMIZATIONS
   ======================================== */
@media (hover: none) and (pointer: coarse) {
  .nav-link {
    min-height: 44px;
    display: flex;
    align-items: center;
  }

  .theme-btn {
    min-width: 44px;
    min-height: 44px;
  }

  .theme-option,
  .dropdown-item {
    min-height: 48px;
  }
}
</style>
