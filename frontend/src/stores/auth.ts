import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User, AuthResponse, LoginRequest, RegisterRequest } from '../types'
import api from '../api'

const TOKEN_KEY = 'notes_auth_token'
const USER_KEY = 'notes_user'

export interface StorageInfo {
  used: number
  limit: number
}

export const useAuthStore = defineStore('auth', () => {
  const token = ref<string | null>(localStorage.getItem(TOKEN_KEY))
  const user = ref<User | null>(JSON.parse(localStorage.getItem(USER_KEY) || 'null'))
  const loading = ref(false)
  const error = ref<string | null>(null)
  const storageInfo = ref<StorageInfo>({ used: 0, limit: 1073741824 })

  const isAuthenticated = computed(() => !!token.value && !!user.value)

  function setAuth(authResponse: AuthResponse) {
    token.value = authResponse.token
    user.value = authResponse.user
    localStorage.setItem(TOKEN_KEY, authResponse.token)
    localStorage.setItem(USER_KEY, JSON.stringify(authResponse.user))
    // Set default auth header
    api.defaults.headers.common['Authorization'] = `Bearer ${authResponse.token}`
  }

  function clearAuth() {
    token.value = null
    user.value = null
    localStorage.removeItem(TOKEN_KEY)
    localStorage.removeItem(USER_KEY)
    delete api.defaults.headers.common['Authorization']
  }

  async function login(credentials: LoginRequest) {
    loading.value = true
    error.value = null
    try {
      const response = await api.post<AuthResponse>('/auth/login', credentials)
      setAuth(response.data)
      return true
    } catch (e: any) {
      error.value = e.response?.data?.error || 'Login failed'
      return false
    } finally {
      loading.value = false
    }
  }

  async function register(data: RegisterRequest) {
    loading.value = true
    error.value = null
    try {
      const response = await api.post<AuthResponse>('/auth/register', data)
      setAuth(response.data)
      return true
    } catch (e: any) {
      error.value = e.response?.data?.error || 'Registration failed'
      return false
    } finally {
      loading.value = false
    }
  }

  async function loginWithGoogle(code: string) {
    loading.value = true
    error.value = null
    try {
      const response = await api.post<AuthResponse>('/auth/google', { code })
      setAuth(response.data)
      return true
    } catch (e: any) {
      error.value = e.response?.data?.error || 'Google login failed'
      return false
    } finally {
      loading.value = false
    }
  }

  async function getGoogleAuthUrl(): Promise<string | null> {
    try {
      const response = await api.get<{ url: string }>('/auth/google/url')
      return response.data.url
    } catch (e) {
      console.error('Failed to get Google auth URL:', e)
      return null
    }
  }

  async function fetchUser() {
    if (!token.value) return false
    
    try {
      const response = await api.get<User>('/auth/me')
      user.value = response.data
      localStorage.setItem(USER_KEY, JSON.stringify(response.data))
      // Also fetch storage info
      await fetchStorageInfo()
      return true
    } catch (e) {
      clearAuth()
      return false
    }
  }

  async function fetchStorageInfo() {
    if (!token.value) return
    
    try {
      const response = await api.get<StorageInfo>('/auth/storage')
      storageInfo.value = response.data
    } catch (e) {
      console.error('Failed to fetch storage info:', e)
    }
  }

  function logout() {
    clearAuth()
  }

  // Listen for global forced logout events (dispatched by axios interceptor)
  // This keeps the reactive store in sync when client clears localStorage directly.
  if (typeof window !== 'undefined') {
    window.addEventListener('auth:force-logout', () => {
      clearAuth()
    })
  }

  // Initialize auth header if token exists
  if (token.value) {
    api.defaults.headers.common['Authorization'] = `Bearer ${token.value}`
  }

  return {
    token,
    user,
    loading,
    error,
    storageInfo,
    isAuthenticated,
    login,
    register,
    loginWithGoogle,
    getGoogleAuthUrl,
    fetchUser,
    fetchStorageInfo,
    logout
  }
})
