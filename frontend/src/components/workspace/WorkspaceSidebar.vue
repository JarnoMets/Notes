<template>
  <div class="workspace-sidebar" :class="{ open: isOpen }" :style="{ width: width + 'px' }">
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  width: number
  isOpen?: boolean
}>()
</script>

<style scoped>
.workspace-sidebar {
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  min-width: 150px;
  max-width: 500px;
  border-right: 1px solid var(--border-primary);
  height: 100%;
  overflow: hidden;
}

/* Mobile */
@media (max-width: 768px) {
  .workspace-sidebar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 56px;
    width: 100% !important;
    max-width: 100% !important;
    height: calc(100vh - 56px);
    height: calc(100dvh - 56px);
    z-index: 100;
    transform: translateX(-100%);
    transition: transform 0.3s ease;
    border-right: none;
  }

  .workspace-sidebar.open {
    transform: translateX(0);
  }
}
</style>

