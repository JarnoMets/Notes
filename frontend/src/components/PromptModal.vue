<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click.self="handleCancel">
      <div class="prompt-modal">
        <div class="modal-header">
          <h3>{{ title }}</h3>
          <button class="modal-close" @click="handleCancel">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
        <form @submit.prevent="handleSubmit">
          <div class="modal-body">
            <label v-if="label" :for="inputId">{{ label }}</label>
            <input
              ref="inputRef"
              :id="inputId"
              v-model="inputValue"
              type="text"
              :placeholder="placeholder"
              @keydown.esc="handleCancel"
            />
          </div>
          <div class="modal-actions">
            <button type="button" class="btn btn-secondary" @click="handleCancel">Cancel</button>
            <button type="submit" class="btn btn-primary" :disabled="!inputValue.trim()">
              {{ confirmText }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'

const props = withDefaults(defineProps<{
  visible: boolean
  title?: string
  label?: string
  placeholder?: string
  confirmText?: string
  initialValue?: string
}>(), {
  title: 'Enter Value',
  placeholder: '',
  confirmText: 'OK',
  initialValue: ''
})

const emit = defineEmits<{
  submit: [value: string]
  cancel: []
}>()

const inputValue = ref(props.initialValue)
const inputRef = ref<HTMLInputElement | null>(null)
const inputId = `prompt-input-${Math.random().toString(36).slice(2)}`

watch(() => props.visible, async (isVisible) => {
  if (isVisible) {
    inputValue.value = props.initialValue
    await nextTick()
    inputRef.value?.focus()
    inputRef.value?.select()
  }
})

function handleSubmit() {
  if (inputValue.value.trim()) {
    emit('submit', inputValue.value.trim())
  }
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

.prompt-modal {
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  padding: 1.25rem;
  min-width: 380px;
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

.modal-body label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
}

.modal-body input {
  width: 100%;
  padding: 0.75rem 1rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 0.9375rem;
  transition: border-color 0.15s, box-shadow 0.15s;
}

.modal-body input::placeholder {
  color: var(--text-muted);
}

.modal-body input:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-light);
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

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: var(--accent-hover);
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
