<template>
  <div class="list-header">
    <h3>{{ list.name }}</h3>
    <span class="card-count">{{ cardCount }}</span>
    <button class="list-menu-btn" @click="$emit('archive')" title="Archive List">
      <Icon name="archive" :size="14" />
    </button>
    <button class="list-menu-btn" @click="$emit('delete')" title="Delete List">
      <Icon name="x" :size="14" />
    </button>
  </div>
</template>

<script setup lang="ts">
import type { List } from '../../types'
import Icon from '../ui/Icon.vue'

defineProps<{
  list: List
  cardCount: number
}>()

defineEmits<{
  archive: []
  delete: []
}>()
</script>

<style scoped>
.list-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0 0 0.75rem 0;
  margin-bottom: 0.5rem;
}

.list-header h3 {
  flex: 1;
  margin: 0;
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
  letter-spacing: -0.01em;
}

.card-count {
  background: var(--bg-tertiary);
  color: var(--text-muted);
  font-size: 11px;
  padding: 0.125rem 0.5rem;
  border-radius: 10px;
}

.list-menu-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  border-radius: 4px;
  opacity: 0;
  transition: all 0.15s;
}

.list-header:hover .list-menu-btn {
  opacity: 1;
}

.list-menu-btn:hover {
  background: var(--bg-hover);
  color: var(--danger);
}

@media (max-width: 768px) {
  .list-header {
    padding: 0 0.125rem 0.5rem;
  }

  .list-header h3 {
    font-size: 0.8rem;
  }

  .list-menu-btn {
    opacity: 1;
    width: 28px;
    height: 28px;
  }
}
</style>
