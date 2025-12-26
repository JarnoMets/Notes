<template>
  <button 
    class="mobile-toggle"
    @click="$emit('toggle')"
    :class="{ 'sidebar-open': isOpen }"
  >
    <Icon :name="isOpen ? 'x' : 'menu'" :size="20" />
  </button>
</template>

<script setup lang="ts">
import Icon from '../ui/Icon.vue'

defineProps<{
  isOpen: boolean
}>()

defineEmits<{
  toggle: []
}>()
</script>

<style scoped>
.mobile-toggle {
  display: none;
  position: fixed;
  bottom: 1rem;
  left: 1rem;
  z-index: 150;
  width: 48px;
  height: 48px;
  border-radius: 50%;
  background: var(--accent);
  color: white;
  border: none;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  cursor: pointer;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
}

.mobile-toggle:active {
  transform: scale(0.95);
}

.mobile-toggle.sidebar-open {
  background: var(--danger);
}

@media (max-width: 768px) {
  .mobile-toggle {
    display: flex;
  }
}
</style>
