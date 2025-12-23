<template>
  <div v-if="visible" class="side-panel-overlay" @click.self="$emit('close')">
    <div class="side-panel">
      <div class="side-panel-header">
        <h3>Archived Items</h3>
        <button class="modal-close" @click="$emit('close')">
          <Icon name="x" :size="14" />
        </button>
      </div>
      <div class="side-panel-content">
        <div class="archive-section">
          <h4>Archived Lists</h4>
          <div v-if="archivedLists.length === 0" class="empty-state">No archived lists</div>
          <div v-for="list in archivedLists" :key="list.id" class="archived-item">
            <span>{{ list.name }}</span>
            <button class="btn btn-sm" @click="$emit('restoreList', list.id)">Restore</button>
          </div>
        </div>
        <div class="archive-section">
          <h4>Archived Cards</h4>
          <div v-if="archivedCards.length === 0" class="empty-state">No archived cards</div>
          <div v-for="card in archivedCards" :key="card.id" class="archived-item">
            <span>{{ card.title }}</span>
            <button class="btn btn-sm" @click="$emit('restoreCard', card.id)">Restore</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { List, Card } from '../../types'
import Icon from '../common/ui/Icon.vue'

defineProps<{
  visible: boolean
  archivedLists: List[]
  archivedCards: Card[]
}>()

defineEmits<{
  close: []
  restoreList: [listId: string]
  restoreCard: [cardId: string]
}>()
</script>

<style scoped>
.archive-section {
  margin-bottom: 1.5rem;
}

.archive-section h4 {
  margin: 0 0 0.75rem;
  color: var(--text-muted);
  font-size: 0.85rem;
}

.archived-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background: var(--bg-tertiary);
  border-radius: 4px;
  margin-bottom: 0.5rem;
}

.archived-item span {
  font-size: 0.875rem;
  color: var(--text-primary);
}

.empty-state {
  color: var(--text-muted);
  font-size: 0.85rem;
  padding: 0.5rem;
}

@media (max-width: 768px) {
  .archived-item {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .archived-item button {
    align-self: flex-end;
  }
}
</style>
