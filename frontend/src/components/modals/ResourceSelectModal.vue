<template>
  <Teleport to="body">
    <div v-if="visible" class="modal-overlay" @click.self="handleCancel">
      <div class="modal resource-select-modal">
        <div class="modal-header">
          <h3>{{ title }}</h3>
          <button class="modal-close" @click="handleCancel">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line x1="18" y1="6" x2="6" y2="18"/>
              <line x1="6" y1="6" x2="18" y2="18"/>
            </svg>
          </button>
        </div>
        <div class="modal-body">
          <div class="search-container">
            <input 
              v-model="searchQuery" 
              type="text" 
              placeholder="Search..." 
              class="search-input"
              ref="searchRef"
            />
          </div>
          <div class="tree-container">
            <ExplorerTree 
              :default-tab="type === 'note' ? 'notes' : 'boards'"
              :show-boards-header="false"
              @select="handleSelect"
              @dblclick="handleDoubleClick"
            />
          </div>
        </div>
        <div class="modal-actions">
          <button type="button" class="btn btn-secondary" @click="handleCancel">Cancel</button>
          <button 
            type="button" 
            class="btn btn-primary" 
            :disabled="!selectedItem"
            @click="handleConfirm"
          >
            Add {{ type === 'note' ? 'Note' : 'Board' }}
          </button>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, watch, nextTick } from 'vue'
import ExplorerTree from '@/components/ui/ExplorerTree.vue'
import { useExplorerStore, type ExplorerItem } from '@/stores/explorer'

const props = defineProps<{
  visible: boolean
  type: 'note' | 'board'
  title: string
}>()

const emit = defineEmits<{
  select: [itemId: string]
  cancel: []
}>()

const explorerStore = useExplorerStore()
const searchQuery = ref('')
const selectedItem = ref<ExplorerItem | null>(null)
const searchRef = ref<HTMLInputElement | null>(null)

watch(() => props.visible, async (isVisible) => {
  if (isVisible) {
    searchQuery.value = ''
    selectedItem.value = null
    explorerStore.fetchAll()
    await nextTick()
    searchRef.value?.focus()
  }
})

function handleSelect(item: ExplorerItem) {
  if (item.type === props.type) {
    selectedItem.value = item
  } else {
    selectedItem.value = null
  }
}

function handleDoubleClick(item: ExplorerItem) {
  if (item.type === props.type) {
    selectedItem.value = item
    handleConfirm()
  }
}

function handleConfirm() {
  if (selectedItem.value) {
    emit('select', selectedItem.value.id)
  }
}

function handleCancel() {
  emit('cancel')
}
</script>

<style scoped>
.modal-overlay {
  z-index: 2000;
}

.resource-select-modal {
  width: 450px;
  max-width: 90vw;
  height: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
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

.modal-body {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  padding: 0;
}

.search-container {
  padding: 1rem;
  border-bottom: 1px solid var(--border-primary);
}

.search-input {
  width: 100%;
  padding: 0.6rem 0.8rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  font-size: 0.9rem;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent);
}

.tree-container {
  flex: 1;
  overflow-y: auto;
  background: var(--bg-secondary);
}

.modal-actions {
  padding: 1rem;
  border-top: 1px solid var(--border-primary);
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
}

:deep(.explorer-tree) {
  background: transparent;
}

:deep(.explorer-section) {
  border: none;
}
</style>
