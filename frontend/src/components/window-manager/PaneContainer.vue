<template>
  <div class="pane-container">
    <SplitView 
      v-if="node.type === 'split'" 
      :split="node" 
    />
    <PaneLeaf 
      v-else 
      :leaf="node" 
    />
  </div>
</template>

<script setup lang="ts">
import type { LayoutNode } from '@/stores/layout'
import SplitView from './SplitView.vue'
import PaneLeaf from './PaneLeaf.vue'

defineProps<{
  node: LayoutNode
}>()
</script>

<style scoped>
.pane-container {
  width: 100%;
  height: 100%;
}
</style>
