<template>
  <div v-if="showFiltersPanel && currentBoard" class="filters-bar">
    <div class="filter-group">
      <label>Labels:</label>
      <div class="filter-labels">
        <button
          v-for="label in currentBoard.labels"
          :key="label.id"
          class="filter-label"
          :class="{ selected: selectedLabelFilters.includes(label.id) }"
          :style="{ backgroundColor: selectedLabelFilters.includes(label.id) ? label.color : 'transparent', borderColor: label.color }"
          @click="toggleLabelFilter(label.id)"
        >
          {{ label.name }}
        </button>
      </div>
    </div>
    <div class="filter-group">
      <label>Due Date:</label>
      <select v-model="dueDateFilterModel">
        <option value="">All</option>
        <option value="overdue">Overdue</option>
        <option value="today">Due Today</option>
        <option value="week">Due This Week</option>
        <option value="none">No Due Date</option>
      </select>
    </div>
    <div class="filter-group">
      <label>Hide Done:</label>
      <input type="checkbox" v-model="hideDoneCardsModel" />
    </div>
    <button class="btn btn-secondary btn-sm" @click="clearFilters">Clear Filters</button>
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, computed } from 'vue'
import type { BoardWithLists } from '@/types'

interface Props {
  showFiltersPanel: boolean
  currentBoard: BoardWithLists | null
  selectedLabelFilters: string[]
  dueDateFilter: string
  hideDoneCards: boolean
}

interface Emits {
  (e: 'toggle-label-filter', labelId: string): void
  (e: 'update:due-date-filter', value: string): void
  (e: 'update:hide-done-cards', value: boolean): void
  (e: 'clear-filters'): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const dueDateFilterModel = computed({
  get: () => props.dueDateFilter,
  set: (value: string) => emit('update:due-date-filter', value)
})

const hideDoneCardsModel = computed({
  get: () => props.hideDoneCards,
  set: (value: boolean) => emit('update:hide-done-cards', value)
})

function toggleLabelFilter(labelId: string) {
  emit('toggle-label-filter', labelId)
}

function clearFilters() {
  emit('clear-filters')
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

.btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.375rem;
  padding: 0.625rem 1.25rem;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.15s;
}

.btn-secondary {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-primary);
}

.btn-secondary:hover {
  background: var(--bg-hover);
}

.btn-sm {
  padding: 0.5rem 1rem;
  font-size: 12px;
}
</style>