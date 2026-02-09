<template>
  <div class="explorer-section">
    <div class="content-header">
      <div class="content-actions">
        <button v-for="action in actions" :key="action.title" @click="action.handler" :title="action.title">
          <Icon :name="action.icon" :size="14" />
        </button>
        <button @click="$emit('refresh')" title="Refresh">
          <Icon name="refresh" :size="14" />
        </button>
      </div>
    </div>
    
    <div 
      class="tree-content"
      @dragenter="onDragEnter"
      @dragover.prevent="onDragOver"
      @dragleave="onDragLeave"
      @drop.prevent="onDrop"
      @contextmenu.prevent="onContextMenu"
      :class="{ 'root-drop-active': isRootDropTarget }"
    >
      <!-- Favorites Section -->
      <div v-if="favorites.length > 0" class="favorites-section">
        <div class="section-header" @click="$emit('toggle-favorites')">
          <span class="expand-toggle" :class="{ 'is-expanded': favoritesExpanded }">
            <Icon name="chevron-right" :size="12" />
          </span>
          <span class="section-title">Favorites</span>
          <span class="section-count">{{ favorites.length }}</span>
        </div>
        <div v-if="favoritesExpanded" class="favorites-items">
          <ExplorerTreeNode
            v-for="item in favorites"
            :key="'fav-' + item.id"
            :item="item"
            :depth="1"
            :in-favorites="true"
            @select="$emit('select', $event)"
            @toggle="$emit('toggle', $event)"
            @dblclick="$emit('dblclick', $event)"
            @create-note="$emit('create-note', $event)"
            @create-folder="$emit('create-folder', $event)"
            @rename="$emit('rename', $event)"
            @delete="$emit('delete', $event)"
            @toggle-importance="$emit('toggle-importance', $event)"
            @toggle-urgent="$emit('toggle-urgent', $event)"
            @move-item="$emit('move-item', $event)"
          />
        </div>
      </div>
      
      <div class="main-section">
        <div v-if="showHeader" class="section-header" @click="$emit('toggle-section')">
          <span class="expand-toggle" :class="{ 'is-expanded': sectionExpanded }">
            <Icon name="chevron-right" :size="12" />
          </span>
          <span class="section-title">{{ title }}</span>
          <span class="section-count">{{ items.length }}</span>
        </div>

        <div v-if="!showHeader || sectionExpanded">
          <template v-if="items.length > 0">
            <ExplorerTreeNode
              v-for="item in items"
              :key="item.id"
              :item="item"
              :depth="0"
              @select="$emit('select', $event)"
              @toggle="$emit('toggle', $event)"
              @dblclick="$emit('dblclick', $event)"
              @create-note="$emit('create-note', $event)"
              @create-folder="$emit('create-folder', $event)"
              @rename="$emit('rename', $event)"
              @delete="$emit('delete', $event)"
              @toggle-importance="$emit('toggle-importance', $event)"
              @toggle-urgent="$emit('toggle-urgent', $event)"
              @move-item="$emit('move-item', $event)"
            />
            <div v-if="isRootDropTarget" class="root-drop-indicator">Drop here to move to root</div>
          </template>
          <div v-else class="tree-empty">
            <Icon :name="emptyIcon" :size="32" />
            <span>{{ emptyText }}</span>
            <button class="btn btn-primary btn-sm" @click="onEmptyAction">{{ emptyActionText }}</button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import type { ExplorerItem } from '@/stores/explorer'
import ExplorerTreeNode from './ExplorerTreeNode.vue'
import Icon from './Icon.vue'

const props = defineProps<{
  title: string
  items: ExplorerItem[]
  favorites: ExplorerItem[]
  favoritesExpanded: boolean
  sectionExpanded: boolean
  showHeader: boolean
  emptyIcon: string
  emptyText: string
  emptyActionText: string
  actions: Array<{ title: string, icon: string, handler: () => void }>
  allowedDragTypes: string[]
}>()

const emit = defineEmits<{
  'select': [item: ExplorerItem]
  'toggle': [item: ExplorerItem]
  'dblclick': [item: ExplorerItem]
  'create-note': [folderId: string | null]
  'create-folder': [parentId: string | null]
  'rename': [item: ExplorerItem]
  'delete': [item: ExplorerItem]
  'toggle-importance': [item: ExplorerItem]
  'toggle-urgent': [item: ExplorerItem]
  'move-item': [data: any]
  'refresh': []
  'toggle-favorites': []
  'toggle-section': []
  'contextmenu': [event: MouseEvent]
  'empty-action': []
}>()

const isRootDropTarget = ref(false)
let dragCounter = 0

function onDragEnter(event: DragEvent) {
  const hasAllowedType = props.allowedDragTypes.some(type => 
    event.dataTransfer?.types.includes(type)
  ) || event.dataTransfer?.types.includes('application/x-explorer-item')
  
  if (!hasAllowedType) return
  
  dragCounter++
}

function onDragOver(event: DragEvent) {
  const hasAllowedType = props.allowedDragTypes.some(type => 
    event.dataTransfer?.types.includes(type)
  ) || event.dataTransfer?.types.includes('application/x-explorer-item')
  
  if (!hasAllowedType) return
  
  const target = event.currentTarget as HTMLElement
  const rect = target.getBoundingClientRect()
  const y = event.clientY - rect.top
  const scrollHeight = target.scrollHeight
  
  if (y > scrollHeight - 40 || props.items.length === 0) {
    isRootDropTarget.value = true
    event.dataTransfer!.dropEffect = 'move'
    event.preventDefault()
  } else {
    isRootDropTarget.value = false
  }
}

function onDragLeave(event: DragEvent) {
  const relatedTarget = event.relatedTarget as Node | null
  const currentTarget = event.currentTarget as HTMLElement
  
  if (relatedTarget && currentTarget.contains(relatedTarget)) {
    return
  }
  
  dragCounter = Math.max(0, dragCounter - 1)
  if (dragCounter === 0) {
    isRootDropTarget.value = false
  }
}

function onDrop(event: DragEvent) {
  if (!isRootDropTarget.value) return
  isRootDropTarget.value = false
  dragCounter = 0
  
  const data = event.dataTransfer?.getData('application/x-explorer-item')
  if (!data) return
  
  try {
    const { id, type } = JSON.parse(data)
    if (!props.allowedDragTypes.includes(type) && type !== 'note' && type !== 'folder') {
        // Basic check, might need more specific logic per section
    }
    
    emit('move-item', {
      itemId: id,
      itemType: type,
      targetFolderId: null,
      insertBeforeId: null
    })
  } catch (e) {
    // ignore
  }
}

function onContextMenu(event: MouseEvent) {
  emit('contextmenu', event)
}

function onEmptyAction() {
  emit('empty-action')
}
</script>

<style scoped>
.explorer-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.content-header {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  padding: 4px 8px;
}

.content-actions {
  display: flex;
  gap: 2px;
}

.content-actions button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: transparent;
  border: none;
  border-radius: 4px;
  color: var(--text-muted);
  cursor: pointer;
  transition: all 0.1s;
}

.content-actions button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.tree-content {
  display: flex;
  flex-direction: column;
  gap: 0;
  position: relative;
  flex: 1;
  padding: 4px 0;
  overflow-y: auto;
}

.section-header {
  display: flex;
  align-items: center;
  cursor: pointer;
  padding: 4px 8px;
  margin: 0 4px;
  border-radius: 4px;
  transition: all 0.1s;
  color: var(--text-muted);
  text-transform: uppercase;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.05em;
}

.section-header:hover {
  background-color: var(--bg-hover);
  color: var(--text-secondary);
}

.section-title {
  flex: 1;
  margin-left: 4px;
}

.section-count {
  font-size: 9px;
  opacity: 0.6;
}

.expand-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  color: var(--text-muted);
  transition: transform 0.15s ease;
}

.expand-toggle.is-expanded {
  transform: rotate(90deg);
}

.expand-toggle .icon {
  display: block;
}

.favorites-section, .main-section {
  margin-bottom: 8px;
}

.tree-empty {
  text-align: center;
  color: var(--text-muted);
  padding: 32px 16px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 12px;
}

.tree-empty span {
  font-size: 0.8rem;
}

.root-drop-active {
  background-color: var(--accent-light);
}

.root-drop-indicator {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  text-align: center;
  color: var(--accent);
  font-size: 0.7rem;
  font-weight: 500;
  padding: 4px;
  border: 1px dashed var(--accent);
  background-color: var(--accent-light);
  border-radius: 4px;
  pointer-events: none;
  z-index: 10;
}
</style>
