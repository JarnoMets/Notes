import axios from 'axios'

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || '/api',
  headers: {
    'Content-Type': 'application/json'
  }
})

// Add auth token to all requests
api.interceptors.request.use((config) => {
  const token = localStorage.getItem('notes_auth_token')
  if (token) {
    config.headers.Authorization = `Bearer ${token}`
  }
  return config
})

// Global response interceptor: on 401 for application-protected endpoints,
// clear local auth state and notify the app to force-logout the user.
api.interceptors.response.use(
  (response) => response,
  (error) => {
    const status = error?.response?.status
    const url: string = error?.config?.url || ''

    if (status === 401) {
      // Only force logout for typical user-content endpoints. Some 401s (e.g. admin
      // routes) should be handled by the caller/UI instead.
      const protectedPaths = [
        '/notes',
        '/folders',
        '/boards',
        '/attachments',
        '/lists',
        '/cards',
        '/labels',
        '/reminders',
        '/settings',
        '/sync',
        '/auth/me'
      ]

      const isProtected = protectedPaths.some((p) => url.includes(p))

      if (isProtected) {
        // Clear local storage and default auth header
        localStorage.removeItem('notes_auth_token')
        localStorage.removeItem('notes_user')
        if (api.defaults.headers && api.defaults.headers.common) {
          delete api.defaults.headers.common['Authorization']
        }

        // Broadcast a custom event so stores/components can reactively clear state
        try {
          window.dispatchEvent(new CustomEvent('auth:force-logout'))
        } catch (e) {
          // ignore
        }
      }
    }

    return Promise.reject(error)
  }
)

export default api
