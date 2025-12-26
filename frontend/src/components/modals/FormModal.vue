<template>
  <div v-if="visible" class="modal-overlay" @click="handleOverlayClick">
    <div class="modal" @click.stop>
      <div class="modal-header">
        <h3>{{ title }}</h3>
        <button @click="$emit('close')" class="modal-close">&times;</button>
      </div>
      <div class="modal-body">
        <form @submit.prevent="handleSubmit">
          <template v-for="field in fields" :key="field.name">
            <div v-if="!field.showIf || field.showIf(formData)" class="form-field">
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
          </template>
          <div class="modal-actions">
            <button type="button" @click="$emit('close')" class="btn btn-secondary">Cancel</button>
            <button type="submit" class="btn btn-primary">{{ submitLabel }}</button>
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
  showIf?: (data: Record<string, string>) => boolean
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
.form-field {
  margin-bottom: 1rem;
}

.form-field label {
  display: block;
  margin-bottom: 0.5rem;
  color: var(--text-primary);
  font-weight: 500;
}

.form-input,
.form-textarea,
.form-select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-family: inherit;
}

.form-textarea {
  min-height: 100px;
  resize: vertical;
}
</style>