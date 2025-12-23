<template>
  <div v-if="visible" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal-content" @click.stop>
      <div class="modal-header">
        <h3>{{ title }}</h3>
        <button @click="$emit('close')" class="close-button">&times;</button>
      </div>
      <div class="modal-body">
        <form @submit.prevent="handleSubmit">
          <div v-for="field in fields" :key="field.name" class="form-field">
            <label :for="field.name">{{ field.label }}</label>
            <input
              v-if="field.type === 'text' || field.type === 'url'"
              :id="field.name"
              v-model="formData[field.name]"
              :type="field.type"
              :placeholder="field.placeholder"
              :required="field.required"
              class="form-input"
            />
            <textarea
              v-else-if="field.type === 'textarea'"
              :id="field.name"
              v-model="formData[field.name]"
              :placeholder="field.placeholder"
              :required="field.required"
              class="form-textarea"
            ></textarea>
            <select
              v-else-if="field.type === 'select'"
              :id="field.name"
              v-model="formData[field.name]"
              :required="field.required"
              class="form-select"
            >
              <option v-for="option in field.options" :key="option.value" :value="option.value">
                {{ option.label }}
              </option>
            </select>
          </div>
          <div class="modal-actions">
            <button type="button" @click="$emit('close')" class="cancel-button">Cancel</button>
            <button type="submit" class="submit-button">{{ submitLabel }}</button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

interface FormField {
  name: string
  label: string
  type: 'text' | 'url' | 'textarea' | 'select'
  placeholder?: string
  required?: boolean
  options?: { value: string; label: string }[]
}

interface Props {
  visible: boolean
  title: string
  fields: FormField[]
  submitLabel?: string
}

interface Emits {
  (e: 'close'): void
  (e: 'submit', data: Record<string, string>): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const formData = ref<Record<string, string>>({})

// Reset form data when modal becomes visible
watch(() => props.visible, (newVisible) => {
  if (newVisible) {
    formData.value = {}
    props.fields.forEach(field => {
      formData.value[field.name] = ''
    })
  }
})

function handleOverlayClick() {
  emit('close')
}

function handleSubmit() {
  emit('submit', { ...formData.value })
  emit('close')
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.modal-content {
  background: var(--bg-color);
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  max-width: 500px;
  width: 90%;
  max-height: 80vh;
  overflow-y: auto;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  border-bottom: 1px solid var(--border-color);
}

.modal-header h3 {
  margin: 0;
  color: var(--text-color);
}

.close-button {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: var(--text-color);
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.modal-body {
  padding: 1rem;
}

.form-field {
  margin-bottom: 1rem;
}

.form-field label {
  display: block;
  margin-bottom: 0.5rem;
  color: var(--text-color);
  font-weight: 500;
}

.form-input,
.form-textarea,
.form-select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 4px;
  background: var(--input-bg);
  color: var(--text-color);
  font-family: inherit;
}

.form-textarea {
  min-height: 100px;
  resize: vertical;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.5rem;
  margin-top: 1rem;
}

.cancel-button,
.submit-button {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-family: inherit;
}

.cancel-button {
  background: var(--button-secondary-bg);
  color: var(--button-secondary-text);
}

.submit-button {
  background: var(--button-primary-bg);
  color: var(--button-primary-text);
}

.cancel-button:hover {
  background: var(--button-secondary-hover-bg);
}

.submit-button:hover {
  background: var(--button-primary-hover-bg);
}
</style>