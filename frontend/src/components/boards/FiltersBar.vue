<template>
  <div class="filters-bar">
    <div class="filter-group">
      <label>Labels:</label>
      <div class="filter-labels">
        <button 
          v-for="label in labels" 
          :key="label.id" 
          class="filter-label"
          :class="{ selected: modelValue.labels.includes(label.id) }"
          :style="{ 
            backgroundColor: modelValue.labels.includes(label.id) ? label.color : 'transparent', 
            borderColor: label.color 
          }"
          @click="toggleLabel(label.id)"
        >
          {{ label.name }}
        </button>
      </div>
    </div>
    <div class="filter-group">
      <label>Due Date:</label>
      <select :value="modelValue.dueDate" @change="updateDueDate($event)">
        <option value="">All</option>
        <option value="overdue">Overdue</option>
        <option value="today">Due Today</option>
        <option value="week">Due This Week</option>
        <option value="none">No Due Date</option>
      </select>
    </div>
    <button class="btn btn-secondary btn-sm" @click="$emit('clear')">Clear Filters</button>
  </div>
</template>

<script setup lang="ts">
import type { BoardLabel } from '../../types'

export interface FilterState {
  labels: string[]
  dueDate: string
}

const props = defineProps<{
  labels: BoardLabel[]
  modelValue: FilterState
}>()

const emit = defineEmits<{
  'update:modelValue': [value: FilterState]
  clear: []
}>()

function toggleLabel(labelId: string) {
  const labels = [...props.modelValue.labels]
  const index = labels.indexOf(labelId)
  if (index === -1) {
    labels.push(labelId)
  } else {
    labels.splice(index, 1)
  }
  emit('update:modelValue', { ...props.modelValue, labels })
}

function updateDueDate(event: Event) {
  const target = event.target as HTMLSelectElement
  emit('update:modelValue', { ...props.modelValue, dueDate: target.value })
}
</script>

<style scoped>
.filters-bar {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  padding: 0.75rem 1.25rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
  flex-wrap: wrap;
}

.filter-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.filter-group label {
  font-weight: 500;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.filter-labels {
  display: flex;
  gap: 0.25rem;
  flex-wrap: wrap;
}

.filter-label {
  padding: 0.2rem 0.5rem;
  border-radius: 3px;
  font-size: 0.75rem;
  cursor: pointer;
  border: 2px solid;
  background: transparent;
  color: var(--text-primary);
  transition: all 0.2s;
}

.filter-label.selected {
  color: white;
}

.filter-group select {
  padding: 0.3rem 0.5rem;
  border-radius: 4px;
  border: 1px solid var(--border-primary);
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-size: 0.85rem;
}

@media (max-width: 768px) {
  .filters-bar {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
    padding: 0.625rem 0.75rem;
  }

  .filter-group {
    width: 100%;
    flex-wrap: wrap;
  }

  .filter-labels {
    width: 100%;
  }
}

@media (hover: none) and (pointer: coarse) {
  .filter-label {
    min-height: 44px;
    display: flex;
    align-items: center;
    justify-content: center;
  }
}
</style>
