<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click.self="handleCancel">
      <div class="confirm-modal">
        <div class="modal-header">
          <h3>{{ title }}</h3>
          <button class="modal-close" @click="handleCancel">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
        <div class="modal-body">
          <p>{{ message }}</p>
        </div>
        <div class="modal-actions">
          <button class="btn btn-secondary" @click="handleCancel">Cancel</button>
          <button 
            class="btn" 
            :class="confirmClass" 
            @click="handleConfirm"
          >
            {{ confirmText }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  visible: boolean
  title?: string
  message: string
  confirmText?: string
  variant?: 'primary' | 'danger'
}>(), {
  title: 'Confirm',
  confirmText: 'Confirm',
  variant: 'primary'
})

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const confirmClass = computed(() => {
  return props.variant === 'danger' ? 'btn-danger' : 'btn-primary'
})

function handleConfirm() {
  emit('confirm')
}

function handleCancel() {
  emit('cancel')
}
</script>

<style scoped>
.modal-overlay {
  z-index: 2000;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.confirm-modal {
  animation: slideUp 0.2s ease-out;
}

@keyframes slideUp {
  from { 
    opacity: 0;
    transform: translateY(12px);
  }
  to { 
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-body {
  margin-bottom: 1.25rem;
}

.modal-body p {
  margin: 0;
  color: var(--text-secondary);
  line-height: 1.5;
}
</style>
