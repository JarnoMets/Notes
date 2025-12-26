<template>
  <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal">
      <div class="modal-header">
        <h3>{{ isEdit ? 'Edit Board' : 'New Board' }}</h3>
        <button class="modal-close" @click="$emit('close')">
          <Icon name="x" :size="14" />
        </button>
      </div>
      <form @submit.prevent="handleSubmit">
        <div class="form-group">
          <label for="boardName">Name</label>
          <input 
            id="boardName" 
            v-model="localName" 
            type="text" 
            required 
            placeholder="Board name"
          />
        </div>
        <div class="form-group">
          <label for="boardDescription">Description (optional)</label>
          <textarea 
            id="boardDescription" 
            v-model="localDescription" 
            rows="3" 
            placeholder="What's this board for?"
          ></textarea>
        </div>
        <div class="form-group">
          <label>Color</label>
          <div class="color-picker">
            <button 
              v-for="color in colorPalette" 
              :key="color"
              type="button"
              class="color-option"
              :class="{ selected: localColor === color }"
              :style="{ backgroundColor: color }"
              @click="localColor = color"
            ></button>
          </div>
        </div>
        <div class="modal-actions">
          <button type="button" class="btn btn-secondary" @click="$emit('close')">Cancel</button>
          <button type="submit" class="btn btn-primary">{{ isEdit ? 'Save' : 'Create' }}</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import Icon from '../ui/Icon.vue'

const props = defineProps<{
  visible: boolean
  isEdit?: boolean
  name?: string
  description?: string
  color?: string
}>()

const emit = defineEmits<{
  close: []
  save: [data: { name: string; description: string; color: string }]
}>()

const colorPalette = ['#3498db', '#2ecc71', '#e74c3c', '#f39c12', '#9b59b6', '#1abc9c', '#e67e22', '#34495e']

const localName = ref('')
const localDescription = ref('')
const localColor = ref('#3498db')

// Reset form when modal opens
watch(() => props.visible, (visible) => {
  if (visible) {
    localName.value = props.name || ''
    localDescription.value = props.description || ''
    localColor.value = props.color || '#3498db'
  }
})

function handleSubmit() {
  emit('save', {
    name: localName.value,
    description: localDescription.value,
    color: localColor.value
  })
}
</script>

<style scoped>
/* Uses shared modal styles from components.css */
.color-picker {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
}

.color-option {
  width: 28px;
  height: 28px;
  border-radius: 6px;
  border: 2px solid transparent;
  cursor: pointer;
  transition: transform 0.15s;
}

.color-option:hover {
  transform: scale(1.1);
}

.color-option.selected {
  border-color: var(--text-primary);
}
</style>
