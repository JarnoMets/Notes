<template>
  <div v-if="visible" class="modal-overlay" @click.self="$emit('close')">
    <div class="modal link-modal">
      <div class="modal-header">
        <h3>{{ type === 'note' ? 'Link to Note' : 'Link to Board' }}</h3>
        <button class="modal-close" @click="$emit('close')">&times;</button>
      </div>
      <div class="modal-body">
        <input
          v-model="search"
          type="text"
          class="search-input"
          :placeholder="type === 'note' ? 'Search notes...' : 'Search boards...'"
        />
        <div class="link-results">
          <div
            v-for="item in filteredItems"
            :key="item.id"
            class="link-result-item"
            @click="$emit('select', item)"
          >
            <span class="link-icon">
              <Icon :name="type === 'note' ? 'note' : 'board'" :size="14" />
            </span>
            <span class="link-name">{{ getItemName(item) }}</span>
          </div>
          <div v-if="filteredItems.length === 0" class="link-empty">
            No {{ type === 'note' ? 'notes' : 'boards' }} found
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Note, Board } from '../../types'
import Icon from '../Icon.vue'

const props = defineProps<{
  visible: boolean
  type: 'note' | 'board'
  items: (Note | Board)[]
}>()

defineEmits<{
  close: []
  select: [item: Note | Board]
}>()

const search = ref('')

const filteredItems = computed(() => {
  const searchLower = search.value.toLowerCase()
  return props.items.filter(item => {
    const name = getItemName(item).toLowerCase()
    return name.includes(searchLower)
  })
})

function getItemName(item: Note | Board): string {
  return 'title' in item ? item.title : item.name
}
</script>

<style scoped>
.link-modal {
  width: 400px;
}

.search-input {
  width: 100%;
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 0.875rem;
  margin-bottom: 0.75rem;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent);
}

.search-input::placeholder {
  color: var(--text-muted);
}

.link-results {
  max-height: 300px;
  overflow-y: auto;
}

.link-result-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.625rem 0.75rem;
  border-radius: 6px;
  cursor: pointer;
  transition: background 0.15s;
}

.link-result-item:hover {
  background: var(--bg-hover);
}

.link-icon {
  font-size: 16px;
}

.link-name {
  color: var(--text-primary);
  font-size: 0.875rem;
}

.link-empty {
  text-align: center;
  padding: 2rem;
  color: var(--text-muted);
  font-size: 0.875rem;
}

/* Responsive */
@media (max-width: 768px) {
  .link-modal {
    width: 100%;
    max-width: 100%;
    margin: 0;
    border-radius: 16px 16px 0 0;
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    max-height: 70vh;
  }

  .search-input {
    font-size: 16px; /* Prevents zoom on iOS */
  }

  .link-results {
    max-height: 40vh;
  }

  .link-result-item {
    padding: 0.875rem;
  }
}

@media (hover: none) and (pointer: coarse) {
  .link-result-item {
    min-height: 48px;
  }
}
</style>
