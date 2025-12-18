import { createApp } from 'vue'
import { createPinia } from 'pinia'
import router from './router'
import App from './App.vue'
import './style.css'
import './styles/index.css'
import { useAuthStore } from './stores/auth'

const app = createApp(App)

app.use(createPinia())
app.use(router)
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
