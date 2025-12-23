<template>
  <div 
    class="split-view"
    :class="split.direction"
    ref="containerRef"
  >
    <template v-for="(child, index) in split.children" :key="child.id">
      <div 
        class="split-child"
        :style="{ flex: split.sizes[index] }"
      >
        <PaneContainer :node="child" />
      </div>
      
      <!-- Splitter -->
      <div 
        v-if="index < split.children.length - 1"
        class="splitter"
        :class="split.direction"
        @mousedown.prevent="startResize($event, index)"
      ></div>
    </template>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { LayoutSplit } from '@/stores/layout'
import PaneContainer from './PaneContainer.vue'

const props = defineProps<{
  split: LayoutSplit
}>()

const containerRef = ref<HTMLElement | null>(null)
const isResizing = ref(false)
const resizeIndex = ref(-1)

function startResize(event: MouseEvent, index: number) {
  isResizing.value = true
  resizeIndex.value = index
  document.addEventListener('mousemove', onResize)
  document.addEventListener('mouseup', stopResize)
}

function onResize(event: MouseEvent) {
  if (!isResizing.value || !containerRef.value) return
  
  const rect = containerRef.value.getBoundingClientRect()
  const totalSize = props.split.direction === 'row' ? rect.width : rect.height
  const mousePos = props.split.direction === 'row' 
    ? event.clientX - rect.left 
    : event.clientY - rect.top
  
  // Calculate percentage position
  const percentage = (mousePos / totalSize) * 100
  
  // We are resizing the boundary between child[index] and child[index+1]
  // We need to adjust sizes[index] and sizes[index+1]
  // But sizes are relative to the whole container.
  
  // Simpler approach:
  // Calculate the new size of the left/top element (index)
  // The sum of sizes[index] + sizes[index+1] should remain constant?
  // Actually, flex-grow works best if we just update the ratios.
  
  // Let's assume sizes are percentages that sum to 100 (roughly).
  // We need to find the cumulative size up to index.
  let prevSize = 0
  for (let i = 0; i < resizeIndex.value; i++) {
    prevSize += props.split.sizes[i]
  }
  
  const newSize = Math.max(5, Math.min(95, percentage - prevSize))
  const delta = newSize - props.split.sizes[resizeIndex.value]
  
  // Adjust current and next
  if (props.split.sizes[resizeIndex.value + 1] - delta < 5) return // Min size check
  
  props.split.sizes[resizeIndex.value] += delta
  props.split.sizes[resizeIndex.value + 1] -= delta
}

function stopResize() {
  isResizing.value = false
  resizeIndex.value = -1
  document.removeEventListener('mousemove', onResize)
  document.removeEventListener('mouseup', stopResize)
}
</script>

<style scoped>
.split-view {
  display: flex;
  width: 100%;
  height: 100%;
}

.split-view.row {
  flex-direction: row;
}

.split-view.column {
  flex-direction: column;
}

.split-child {
  overflow: hidden;
  min-width: 0;
  min-height: 0;
}

.splitter {
  background: var(--border-primary);
  z-index: 10;
}

.splitter.row {
  width: 4px;
  cursor: col-resize;
  margin: 0 -2px; /* Overlap to make it easier to grab */
  position: relative;
  z-index: 100;
}

.splitter.column {
  height: 4px;
  cursor: row-resize;
  margin: -2px 0;
  position: relative;
  z-index: 100;
}

.splitter:hover, .splitter:active {
  background: var(--accent);
}
</style>
