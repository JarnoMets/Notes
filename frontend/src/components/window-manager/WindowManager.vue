<template>
  <div class="window-manager">
    <div v-if="isMobile" class="mobile-window-manager">
      <PaneLeaf v-if="activeLeaf" :leaf="activeLeaf" />
      <div v-else class="empty-state">
        <p>No active leaf</p>
      </div>
    </div>
    <PaneContainer v-else :node="layoutStore.root" />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useLayoutStore, type LayoutLeaf } from '@/stores/layout'
import { useBreakpoints, breakpointsTailwind } from '@vueuse/core'
import PaneContainer from './PaneContainer.vue'
import PaneLeaf from './PaneLeaf.vue'

const layoutStore = useLayoutStore()
const breakpoints = useBreakpoints(breakpointsTailwind)
const isMobile = breakpoints.smaller('md')

const activeLeaf = computed(() => {
  if (!layoutStore.activeLeafId) return null
  return layoutStore.findNode(layoutStore.activeLeafId) as LayoutLeaf
})
</script>

<style scoped>
.window-manager {
  width: 100%;
  height: 100%;
  background: var(--bg-primary);
}

.mobile-window-manager {
  width: 100%;
  height: 100%;
}

.empty-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: var(--text-muted);
}
</style>
