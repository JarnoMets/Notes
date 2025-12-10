<template>
  <div class="workspace-sidebar" :class="{ open: isOpen }" :style="{ width: width + 'px' }">
    <div class="sidebar-header">
      <span class="sidebar-title">EXPLORER</span>
      <div class="sidebar-actions">
        <button @click="$emit('refresh')" title="Refresh">
          <Icon name="refresh" :size="14" />
        </button>
      </div>
    </div>
    
    <slot></slot>
  </div>
</template>

<script setup lang="ts">
import Icon from '../Icon.vue'

defineProps<{
  width: number
  isOpen?: boolean
}>()

defineEmits<{
  refresh: []
}>()
</script>

<style scoped>
.workspace-sidebar {
  display: flex;
  flex-direction: column;
  background: var(--sidebar-bg);
  min-width: 150px;
  max-width: 500px;
  border-right: 1px solid var(--border-primary);
}

.sidebar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  font-size: 11px;
  font-weight: 600;
  letter-spacing: 0.5px;
  text-transform: uppercase;
  color: var(--text-muted);
  border-bottom: 1px solid var(--border-primary);
}

.sidebar-actions {
  display: flex;
  gap: 0.25rem;
}

.sidebar-actions button {
  background: transparent;
  border: none;
  padding: 0.375rem;
  cursor: pointer;
  color: var(--text-muted);
  font-size: 14px;
  border-radius: 4px;
  transition: all 0.15s;
}

.sidebar-actions button:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

/* Mobile */
@media (max-width: 768px) {
  .workspace-sidebar {
    position: fixed;
    top: 56px;
    left: 0;
    right: 0;
    bottom: 0;
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

@media (hover: none) and (pointer: coarse) {
  .sidebar-actions button {
    min-width: 44px;
    min-height: 44px;
  }
}
</style>
