<template>
  <div 
    class="resize-handle"
    :class="direction"
    @mousedown="$emit('start')"
  ></div>
</template>

<script setup lang="ts">
defineProps<{
  direction: 'vertical' | 'horizontal'
}>()

defineEmits<{
  start: []
}>()
</script>

<style scoped>
.resize-handle {
  background: var(--border-primary);
  flex-shrink: 0;
  transition: background 0.15s;
}

.resize-handle.vertical {
  width: 4px;
  cursor: col-resize;
}

.resize-handle.horizontal {
  height: 4px;
  cursor: row-resize;
}

.resize-handle:hover {
  background: var(--accent);
}

@media (max-width: 768px) {
  .resize-handle.vertical {
    display: none;
  }
}
</style>
