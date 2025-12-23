<template>
  <div class="board-header" v-if="board">
    <div class="header-left">
      <h2 :style="{ color: board.color || 'var(--text-primary)' }">
        {{ board.name }}
      </h2>
      <span class="board-description" v-if="board.description">
        {{ board.description }}
      </span>
    </div>
    <div class="header-actions">
      <button class="btn-icon" :class="{ active: hasActiveFilters }" @click="$emit('toggle-filters')" title="Filters">
        <Icon name="filter" :size="16" />
      </button>
      <button class="btn-icon" @click="$emit('open-labels')" title="Labels">
        <Icon name="tag" :size="16" />
      </button>
      <button class="btn-icon" @click="$emit('open-automations')" title="Automations">
        <Icon name="play" :size="16" />
      </button>
      <button class="btn-icon" @click="$emit('open-archive')" title="Archive">
        <Icon name="archive" :size="16" />
      </button>
      <button class="btn-icon" @click="$emit('edit-board')" title="Edit Board">
        <Icon name="edit" :size="16" />
      </button>
      <div class="menu-wrapper" ref="menuWrapper">
        <button class="btn-icon" @click="toggleBoardMenu" title="More options">
          <Icon name="more-vertical" :size="16" :fill="true" />
        </button>
        <div v-if="showBoardMenu" class="dropdown-menu">
          <button @click="$emit('add-list')">
            <Icon name="plus" :size="14" /> Add List
          </button>
          <div class="menu-divider"></div>
          <button class="danger" @click="$emit('delete-board')">
            <Icon name="trash" :size="14" /> Delete Board
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import Icon from '@/components/common/ui/Icon.vue'
import type { Board } from '@/types'

interface Props {
  board: Board | null
  hasActiveFilters: boolean
}

defineProps<Props>()

const emit = defineEmits<{
  'toggle-filters': []
  'open-labels': []
  'open-automations': []
  'open-archive': []
  'edit-board': []
  'add-list': []
  'delete-board': []
}>()

const showBoardMenu = ref(false)
const menuWrapper = ref<HTMLElement | null>(null)

function toggleBoardMenu() {
  showBoardMenu.value = !showBoardMenu.value
}

function closeBoardMenuOnClickOutside(event: MouseEvent) {
  if (menuWrapper.value && !menuWrapper.value.contains(event.target as Node)) {
    showBoardMenu.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', closeBoardMenuOnClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', closeBoardMenuOnClickOutside)
})
</script>

<style scoped>
.board-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1.25rem;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
}

.header-left {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
  min-width: 0;
}

.header-left h2 {
  margin: 0;
  font-size: 1.125rem;
  font-weight: 600;
  white-space: nowrap;
}

.board-description {
  font-size: 13px;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s;
}

.btn-icon:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.menu-wrapper {
  position: relative;
}

.dropdown-menu {
  position: absolute;
  top: 100%;
  right: 0;
  margin-top: 0.25rem;
  min-width: 160px;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  z-index: 100;
  overflow: hidden;
}

.dropdown-menu button {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  width: 100%;
  padding: 0.5rem 0.875rem;
  background: transparent;
  border: none;
  color: var(--text-primary);
  font-size: 13px;
  cursor: pointer;
  text-align: left;
}

.dropdown-menu button:hover {
  background: var(--bg-hover);
}

.dropdown-menu button.danger {
  color: var(--danger);
}

.dropdown-menu button.danger:hover {
  background: rgba(248, 81, 73, 0.1);
}

.menu-divider {
  height: 1px;
  background: var(--border-primary);
  margin: 0.25rem 0;
}

.btn-icon.active {
  background: var(--accent);
  color: white;
}
</style>