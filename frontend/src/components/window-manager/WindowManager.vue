<template>
  <div class="window-manager">
    <PaneContainer :node="layoutStore.root" />
  </div>
</template>

<script setup lang="ts">
import { useLayoutStore } from '@/stores/layout'
import PaneContainer from './PaneContainer.vue'

const layoutStore = useLayoutStore()
</script>

<style scoped>
.window-manager {
  width: 100%;
  height: 100%;
  background: var(--bg-tertiary);
}
</style>
