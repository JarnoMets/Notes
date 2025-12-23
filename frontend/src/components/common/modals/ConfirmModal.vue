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
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  animation: fadeIn 0.15s ease-out;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.confirm-modal {
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  padding: 1.25rem;
  min-width: 360px;
  max-width: 480px;
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
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

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.modal-header h3 {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.modal-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  border-radius: 6px;
  transition: all 0.15s;
}

.modal-close:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.modal-body {
  margin-bottom: 1.25rem;
}

.modal-body p {
  margin: 0;
  color: var(--text-secondary);
  line-height: 1.5;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
}

.btn {
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.875rem;
  font-weight: 500;
  transition: all 0.15s;
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover {
  background: var(--accent-hover);
}

.btn-danger {
  background: var(--danger);
  color: white;
}

.btn-danger:hover {
  filter: brightness(1.1);
}

.btn-secondary {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-primary);
}

.btn-secondary:hover {
  background: var(--bg-hover);
}
</style>
