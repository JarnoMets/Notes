<template>
  <div 
    v-if="isOpen"
    class="mobile-overlay"
    @click="$emit('close')"
  ></div>
</template>

<script setup lang="ts">
defineProps<{
  isOpen: boolean
}>()

defineEmits<{
  close: []
}>()
</script>

<style scoped>
.mobile-overlay {
  display: none;
  position: fixed;
  top: 56px;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  z-index: 99;
  animation: fadeIn 0.2s ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@media (max-width: 768px) {
  .mobile-overlay {
    display: block;
  }
}
</style>
