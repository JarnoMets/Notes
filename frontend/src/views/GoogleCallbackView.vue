<template>
  <div class="auth-container" :class="['theme-' + themeStore.currentTheme]">
    <div class="auth-card">
      <div v-if="loading" class="loading">
        <p>Completing sign in...</p>
      </div>
      <div v-else-if="error" class="error">
        <p>{{ error }}</p>
        <router-link to="/login" class="btn btn-primary">Back to Login</router-link>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useThemeStore } from '../stores/theme'

const router = useRouter()
const route = useRoute()
const authStore = useAuthStore()
const themeStore = useThemeStore()

const loading = ref(true)
const error = ref<string | null>(null)

onMounted(async () => {
  const code = route.query.code as string
  
  if (!code) {
    error.value = 'No authorization code received'
    loading.value = false
    return
  }

  const success = await authStore.loginWithGoogle(code)
  
  if (success) {
    router.push('/notes')
  } else {
    error.value = authStore.error || 'Failed to complete Google sign in'
    loading.value = false
  }
})
</script>

<style scoped>
.auth-container {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-primary);
  padding: 1rem;
}

.auth-card {
  background: var(--bg-secondary);
  border-radius: 12px;
  padding: 2rem;
  width: 100%;
  max-width: 400px;
  box-shadow: 0 10px 40px rgba(0,0,0,0.2);
  border: 1px solid var(--border-primary);
  text-align: center;
}

.loading p {
  color: var(--text-secondary);
  font-size: 1.1rem;
}

.error p {
  color: var(--danger);
  margin-bottom: 1rem;
}
</style>
