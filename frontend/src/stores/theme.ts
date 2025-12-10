import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { settingsApi } from '@/api/settings'

export type ThemeName = 'dark' | 'light' | 'midnight' | 'forest' | 'ocean' | 'sunset'

export interface Theme {
  name: ThemeName
  label: string
  colors: {
    // Base colors
    bgPrimary: string
    bgSecondary: string
    bgTertiary: string
    bgHover: string
    bgActive: string
    
    // Text colors
    textPrimary: string
    textSecondary: string
    textMuted: string
    
    // Border colors
    borderPrimary: string
    borderSecondary: string
    
    // Accent colors
    accent: string
    accentHover: string
    accentLight: string
    
    // Status colors
    success: string
    warning: string
    danger: string
    info: string
    
    // Editor specific
    editorBg: string
    editorText: string
    toolbarBg: string
    sidebarBg: string
    tabBg: string
    tabActiveBg: string
  }
}

export const themes: Record<ThemeName, Theme> = {
  dark: {
    name: 'dark',
    label: 'Dark',
    colors: {
      bgPrimary: '#1e1e1e',
      bgSecondary: '#252526',
      bgTertiary: '#2d2d30',
      bgHover: '#3c3c3c',
      bgActive: '#094771',
      textPrimary: '#d4d4d4',
      textSecondary: '#cccccc',
      textMuted: '#808080',
      borderPrimary: '#3c3c3c',
      borderSecondary: '#454545',
      accent: '#007acc',
      accentHover: '#0098ff',
      accentLight: '#094771',
      success: '#4ec9b0',
      warning: '#dcdcaa',
      danger: '#f14c4c',
      info: '#4fc1ff',
      editorBg: '#1e1e1e',
      editorText: '#d4d4d4',
      toolbarBg: '#252526',
      sidebarBg: '#252526',
      tabBg: '#2d2d30',
      tabActiveBg: '#1e1e1e',
    }
  },
  light: {
    name: 'light',
    label: 'Light',
    colors: {
      bgPrimary: '#ffffff',
      bgSecondary: '#f3f3f3',
      bgTertiary: '#ececec',
      bgHover: '#e8e8e8',
      bgActive: '#cce5ff',
      textPrimary: '#333333',
      textSecondary: '#555555',
      textMuted: '#888888',
      borderPrimary: '#e0e0e0',
      borderSecondary: '#d0d0d0',
      accent: '#0066cc',
      accentHover: '#0052a3',
      accentLight: '#e6f2ff',
      success: '#28a745',
      warning: '#ffc107',
      danger: '#dc3545',
      info: '#17a2b8',
      editorBg: '#ffffff',
      editorText: '#333333',
      toolbarBg: '#f8f8f8',
      sidebarBg: '#f3f3f3',
      tabBg: '#ececec',
      tabActiveBg: '#ffffff',
    }
  },
  midnight: {
    name: 'midnight',
    label: 'Midnight',
    colors: {
      bgPrimary: '#0d1117',
      bgSecondary: '#161b22',
      bgTertiary: '#21262d',
      bgHover: '#30363d',
      bgActive: '#1f6feb33',
      textPrimary: '#c9d1d9',
      textSecondary: '#8b949e',
      textMuted: '#6e7681',
      borderPrimary: '#30363d',
      borderSecondary: '#21262d',
      accent: '#58a6ff',
      accentHover: '#79b8ff',
      accentLight: '#1f6feb33',
      success: '#3fb950',
      warning: '#d29922',
      danger: '#f85149',
      info: '#58a6ff',
      editorBg: '#0d1117',
      editorText: '#c9d1d9',
      toolbarBg: '#161b22',
      sidebarBg: '#161b22',
      tabBg: '#21262d',
      tabActiveBg: '#0d1117',
    }
  },
  forest: {
    name: 'forest',
    label: 'Forest',
    colors: {
      bgPrimary: '#1a2421',
      bgSecondary: '#1f2d29',
      bgTertiary: '#263530',
      bgHover: '#2d403a',
      bgActive: '#2d5a4a',
      textPrimary: '#d4e5de',
      textSecondary: '#a8c5b8',
      textMuted: '#6b8f7d',
      borderPrimary: '#2d403a',
      borderSecondary: '#3a5249',
      accent: '#4ade80',
      accentHover: '#86efac',
      accentLight: '#2d5a4a',
      success: '#4ade80',
      warning: '#facc15',
      danger: '#f87171',
      info: '#38bdf8',
      editorBg: '#1a2421',
      editorText: '#d4e5de',
      toolbarBg: '#1f2d29',
      sidebarBg: '#1f2d29',
      tabBg: '#263530',
      tabActiveBg: '#1a2421',
    }
  },
  ocean: {
    name: 'ocean',
    label: 'Ocean',
    colors: {
      bgPrimary: '#0f172a',
      bgSecondary: '#1e293b',
      bgTertiary: '#334155',
      bgHover: '#475569',
      bgActive: '#0369a1',
      textPrimary: '#e2e8f0',
      textSecondary: '#cbd5e1',
      textMuted: '#64748b',
      borderPrimary: '#334155',
      borderSecondary: '#475569',
      accent: '#38bdf8',
      accentHover: '#7dd3fc',
      accentLight: '#0c4a6e',
      success: '#34d399',
      warning: '#fbbf24',
      danger: '#fb7185',
      info: '#38bdf8',
      editorBg: '#0f172a',
      editorText: '#e2e8f0',
      toolbarBg: '#1e293b',
      sidebarBg: '#1e293b',
      tabBg: '#334155',
      tabActiveBg: '#0f172a',
    }
  },
  sunset: {
    name: 'sunset',
    label: 'Sunset',
    colors: {
      bgPrimary: '#1c1917',
      bgSecondary: '#292524',
      bgTertiary: '#3f3a36',
      bgHover: '#57534e',
      bgActive: '#9a3412',
      textPrimary: '#fafaf9',
      textSecondary: '#e7e5e4',
      textMuted: '#a8a29e',
      borderPrimary: '#44403c',
      borderSecondary: '#57534e',
      accent: '#fb923c',
      accentHover: '#fdba74',
      accentLight: '#7c2d12',
      success: '#4ade80',
      warning: '#fbbf24',
      danger: '#f87171',
      info: '#38bdf8',
      editorBg: '#1c1917',
      editorText: '#fafaf9',
      toolbarBg: '#292524',
      sidebarBg: '#292524',
      tabBg: '#3f3a36',
      tabActiveBg: '#1c1917',
    }
  }
}

export const useThemeStore = defineStore('theme', () => {
  const currentTheme = ref<ThemeName>(
    (localStorage.getItem('notes_theme') as ThemeName) || 'dark'
  )
  
  const theme = ref<Theme>(themes[currentTheme.value])
  const initialized = ref(false)

  // Helper function to apply theme
  function kebabCase(str: string): string {
    return str.replace(/([a-z])([A-Z])/g, '$1-$2').toLowerCase()
  }

  function applyThemeInternal(t: Theme) {
    const colors = t.colors
    const root = document.documentElement

    Object.entries(colors).forEach(([key, value]) => {
      root.style.setProperty(`--${kebabCase(key)}`, value)
    })
  }

  // Apply theme immediately on store initialization
  applyThemeInternal(theme.value)

  async function setTheme(themeName: ThemeName) {
    currentTheme.value = themeName
    theme.value = themes[themeName]
    localStorage.setItem('notes_theme', themeName)
    applyTheme()
    
    // Save to server if authenticated
    const token = localStorage.getItem('notes_auth_token')
    if (token) {
      try {
        await settingsApi.update({ theme: themeName })
      } catch (e) {
        console.error('Failed to save theme to server:', e)
      }
    }
  }

  function applyTheme() {
    applyThemeInternal(theme.value)
  }

  // Load theme from server
  async function loadFromServer() {
    const token = localStorage.getItem('notes_auth_token')
    if (!token || initialized.value) return
    
    try {
      const response = await settingsApi.get()
      const serverTheme = response.data.theme as ThemeName
      if (serverTheme && themes[serverTheme]) {
        currentTheme.value = serverTheme
        theme.value = themes[serverTheme]
        localStorage.setItem('notes_theme', serverTheme)
        applyTheme()
      }
      initialized.value = true
    } catch (e) {
      console.error('Failed to load theme from server:', e)
    }
  }

  // Apply theme on load
  watch(theme, applyTheme, { immediate: true })

  return {
    currentTheme,
    theme,
    themes,
    setTheme,
    applyTheme,
    loadFromServer,
    initialized
  }
})
