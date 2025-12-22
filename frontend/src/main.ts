import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from './router'
import App from './App.vue'
import './style.css'
import './styles/index.css'
import { useAuthStore } from './stores/auth'
import { useThemeStore } from './stores/theme'

const app = createApp(App)

app.use(createPinia())
app.use(router)

// Initialize theme store early so CSS variables are applied before any view renders.
// This ensures pages that render before a user or settings are loaded (like /auth/*)
// will still receive the default theme (dark) immediately.
try {
	const theme = useThemeStore()
	// applyTheme is called on store init, but call explicitly to be safe
	theme.applyTheme()
} catch (e) {
	// If Pinia isn't ready yet, skip silently — theme will apply when store is used
	console.warn('Theme initialization skipped:', e)
}
app.mount('#app')

// Proactively validate session on startup, on window focus and periodically.
// This helps detect SSO/session desyncs and clears local auth when server returns 401.
try {
	const auth = useAuthStore()
	// Validate once at startup (if token exists)
	if (auth.token) {
		auth.fetchUser()
	}

	// Re-check on window focus (user may have signed out elsewhere)
	window.addEventListener('focus', () => {
		if (auth.token) auth.fetchUser()
	})

	// Periodic heartbeat to detect stale sessions (every 15 minutes)
	const HEARTBEAT_MS = 15 * 60 * 1000
	setInterval(() => {
		if (auth.token) auth.fetchUser()
	}, HEARTBEAT_MS)
} catch (e) {
	// If Pinia isn't ready yet (rare), skip proactive checks silently
	console.warn('Auth heartbeat initialization skipped:', e)
}
