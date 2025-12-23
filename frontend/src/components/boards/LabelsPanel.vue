<template>
  <div v-if="visible" class="side-panel-overlay" @click.self="$emit('close')">
    <div class="side-panel">
      <div class="side-panel-header">
        <h3>Labels</h3>
        <button class="modal-close" @click="$emit('close')">
          <Icon name="x" :size="14" />
        </button>
      </div>
      <div class="side-panel-content">
        <div class="labels-list-manage">
          <div 
            v-for="label in labels" 
            :key="label.id" 
            class="label-item"
          >
            <div class="label-preview" :style="{ backgroundColor: label.color }">
              {{ label.name }}
            </div>
            <div class="label-actions">
              <button class="btn btn-sm" @click="editLabel(label)">
                <Icon name="edit" :size="12" />
              </button>
              <button class="btn btn-sm btn-danger" @click="$emit('delete', label.id)">
                <Icon name="trash" :size="12" />
              </button>
            </div>
          </div>
        </div>
        <div class="create-label-form">
          <h4>Create Label</h4>
          <input v-model="newLabelName" placeholder="Label name" />
          <div class="color-picker">
            <button 
              v-for="color in colorPalette" 
              :key="color"
              type="button"
              class="color-option"
              :class="{ selected: newLabelColor === color }"
              :style="{ backgroundColor: color }"
              @click="newLabelColor = color"
            ></button>
          </div>
          <button 
            class="btn btn-primary" 
            @click="handleCreate" 
            :disabled="!newLabelName.trim()"
          >
            Create
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { BoardLabel } from '../../types'
import Icon from '../common/ui/Icon.vue'

defineProps<{
  visible: boolean
  labels: BoardLabel[]
}>()

const emit = defineEmits<{
  close: []
  create: [data: { name: string; color: string }]
  edit: [label: BoardLabel]
  delete: [labelId: string]
}>()

const colorPalette = ['#e74c3c', '#3498db', '#2ecc71', '#f39c12', '#9b59b6', '#1abc9c', '#e67e22', '#34495e', '#fd79a8', '#00cec9']

const newLabelName = ref('')
const newLabelColor = ref('#e74c3c')

function editLabel(label: BoardLabel) {
  const newName = prompt('Edit label name:', label.name)
  if (newName && newName.trim()) {
    emit('edit', { ...label, name: newName.trim() })
  }
}

function handleCreate() {
  if (!newLabelName.value.trim()) return
  emit('create', {
    name: newLabelName.value.trim(),
    color: newLabelColor.value
  })
  newLabelName.value = ''
}
</script>

<style scoped>
.labels-list-manage {
  margin-bottom: 1.5rem;
}

.label-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.label-preview {
  flex: 1;
  padding: 0.5rem;
  border-radius: 4px;
  color: white;
  font-weight: 500;
  font-size: 0.875rem;
}

.label-actions {
  display: flex;
  gap: 0.25rem;
  margin-left: 0.5rem;
}

.create-label-form {
  background: var(--bg-tertiary);
  padding: 1rem;
  border-radius: 8px;
}

.create-label-form h4 {
  margin: 0 0 0.75rem;
  font-size: 0.9rem;
  color: var(--text-primary);
}

.create-label-form input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-primary);
  border-radius: 4px;
  margin-bottom: 0.75rem;
  background: var(--bg-secondary);
  color: var(--text-primary);
}

.color-picker {
  display: flex;
  gap: 0.5rem;
  flex-wrap: wrap;
  margin-bottom: 0.75rem;
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

@media (max-width: 768px) {
  .label-item {
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .label-preview {
    flex: 1 1 100%;
  }

  .label-actions {
    margin-left: 0;
    width: 100%;
    justify-content: flex-end;
  }
}
</style>
