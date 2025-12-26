<script setup lang="ts">
import { onMounted, watchEffect } from 'vue'
import { useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { useThemeStore } from '@/stores/theme'

const route = useRoute()
const authStore = useAuthStore()
const themeStore = useThemeStore()

// Update document title when route changes
watchEffect(() => {
  if (route.path.startsWith('/notes')) document.title = 'Notes'
  else if (route.path.startsWith('/boards')) document.title = 'Boards'
  else if (route.path.startsWith('/calendar')) document.title = 'Calendar'
  else if (route.path === '/login') document.title = 'Login - Notes'
  else if (route.path === '/register') document.title = 'Register - Notes'
  else document.title = 'Notes'
})

onMounted(() => {
  themeStore.applyTheme()
  console.log('App mounted, auth status:', authStore.isAuthenticated)
})
</script>

<template>
  <div class="app" :class="['theme-' + themeStore.currentTheme]">
    <main class="main-content" :class="{ 'no-nav': !authStore.isAuthenticated }">
      <router-view />
    </main>
  </div>
</template>

<style>
/* Global styles moved to style.css and styles/index.css */
.app {
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

.main-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  min-height: 0;
  overflow: hidden;
}

.main-content.no-nav {
  height: 100vh;
}
</style>

