<template>
  <!-- Drop indicator above -->
  <div 
    v-if="dropPosition === 'before'"
    class="drop-indicator"
    :style="{ marginLeft: (depth * 16 + 8) + 'px' }"
  ></div>

  <div 
    class="tree-item"
    :class="{ 
      'is-folder': item.type === 'folder',
      'is-selected': isSelected,
      'drag-over': isDragOver && dropPosition === 'inside',
      'is-dragging': isDragging
    }"
    :style="{ paddingLeft: (depth * 16 + 8) + 'px' }"
    @click.stop="handleClick"
    @dblclick.stop="handleDoubleClick"
    @contextmenu.prevent="handleContextMenu"
    draggable="true"
    @dragstart="handleDragStart"
    @dragend="handleDragEnd"
    @dragover.prevent="handleDragOver"
    @dragleave="handleDragLeave"
    @drop.prevent="handleDrop"
  >
    <!-- Expand/Collapse Arrow -->
    <span 
      v-if="item.type === 'folder'" 
      class="tree-arrow"
      :class="{ expanded: item.isExpanded }"
      @click.stop="$emit('toggle', item)"
    >
      <Icon name="chevron-right" :size="12" />
    </span>
    <span v-else class="tree-arrow-placeholder"></span>

    <!-- Icon -->
    <span class="tree-icon">
      <Icon v-if="item.type === 'folder'" :name="item.isExpanded ? 'folder-open' : 'folder'" :size="14" />
      <Icon v-else name="file" :size="14" />
    </span>

    <!-- Name -->
    <span class="tree-name">{{ item.name }}</span>

    <!-- Inline Actions (visible on hover) -->
    <div class="tree-actions">
      <button 
        v-if="item.type === 'folder'"
        @click.stop="$emit('create-note', item.id)"
        title="New Note"
      ><Icon name="file" :size="12" /></button>
      <button 
        v-if="item.type === 'folder'"
        @click.stop="$emit('create-folder', item.id)"
        title="New Folder"
      ><Icon name="folder" :size="12" /></button>
      <button 
        @click.stop="$emit('rename', item)"
        title="Rename"
      ><Icon name="edit" :size="12" /></button>
      <button 
        @click.stop="$emit('delete', item)"
        title="Delete"
        class="delete-btn"
      ><Icon name="trash" :size="12" /></button>
    </div>
  </div>

  <!-- Drop indicator after (only for notes or collapsed folders) -->
  <div 
    v-if="dropPosition === 'after' && (item.type === 'note' || !item.isExpanded)"
    class="drop-indicator"
    :style="{ marginLeft: (depth * 16 + 8) + 'px' }"
  ></div>

  <!-- Children -->
  <template v-if="item.type === 'folder' && item.isExpanded">
    <FileTreeNode
      v-for="child in item.children"
      :key="child.id"
      :item="child"
      :depth="depth + 1"
      @select="$emit('select', $event)"
      @toggle="$emit('toggle', $event)"
      @create-note="$emit('create-note', $event)"
      @create-folder="$emit('create-folder', $event)"
      @rename="$emit('rename', $event)"
      @delete="$emit('delete', $event)"
      @drop="$emit('drop', $event)"
    />

    <!-- Empty folder drop zone -->
    <div 
      v-if="item.children.length === 0"
      class="tree-empty"
      :class="{ 'drag-over': isDragOver && dropPosition === 'inside' }"
      :style="{ paddingLeft: ((depth + 1) * 16 + 8) + 'px' }"
      @dragover.prevent="handleEmptyFolderDragOver"
      @dragleave="handleDragLeave"
      @drop.prevent="handleDrop"
    >
      <span class="empty-text">{{ isDragOver ? 'Drop here' : 'Empty' }}</span>
    </div>

    <!-- Drop indicator at end of folder children -->
    <div 
      v-if="dropPosition === 'after' && item.children.length > 0"
      class="drop-indicator"
      :style="{ marginLeft: ((depth + 1) * 16 + 8) + 'px' }"
    ></div>
  </template>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useNotesStore } from '@/stores/notes'
import { useExplorerStore } from '@/stores/explorer'
import { storeToRefs } from 'pinia'
import Icon from './Icon.vue'
import logger from '@/utils/logger'

interface TreeItem {
  id: string
  name: string
  type: 'folder' | 'note'
  parentId: string | null
  position: number
  isExpanded: boolean
  children: TreeItem[]
}

const props = defineProps<{
  item: TreeItem
  depth: number
}>()

const emit = defineEmits<{
  select: [item: TreeItem]
  toggle: [item: TreeItem]
  'create-note': [folderId: string | null]
  'create-folder': [parentId: string | null]
  rename: [item: TreeItem]
  delete: [item: TreeItem]
  drop: [data: { draggedId: string; draggedType: 'note' | 'folder'; targetId: string | null; targetType: 'folder' | 'note' | null; position: number; dropPosition: 'before' | 'after' | 'inside' }]
}>()

const notesStore = useNotesStore()
const explorerStore = useExplorerStore()
const { selectedItemId, selectedItemType } = storeToRefs(explorerStore)

const isDragOver = ref(false)
const isDragging = ref(false)
const dropPosition = ref<'before' | 'after' | 'inside' | null>(null)

const isSelected = computed(() => {
  return selectedItemId.value === props.item.id && selectedItemType.value === props.item.type
})

function handleClick() {
  emit('select', props.item)
}

function handleDoubleClick() {
  if (props.item.type === 'folder') {
    emit('toggle', props.item)
  }
}

function handleContextMenu(_event: MouseEvent) {
  emit('select', props.item)
  // Context menu is handled by parent
}

function handleDragStart(event: DragEvent) {
  isDragging.value = true
  if (event.dataTransfer) {
    event.dataTransfer.setData('application/x-notes-tree', JSON.stringify({
      id: props.item.id,
      type: props.item.type,
      parentId: props.item.parentId,
      name: props.item.name
    }))
    event.dataTransfer.effectAllowed = 'move'
    
    // Set a custom drag image (optional)
    const dragGhost = document.createElement('div')
    dragGhost.className = 'drag-ghost'
    dragGhost.textContent = props.item.name
    dragGhost.style.cssText = 'position: absolute; top: -1000px; left: -1000px; background: var(--bg-tertiary); padding: 4px 8px; border-radius: 4px; font-size: 13px;'
    document.body.appendChild(dragGhost)
    event.dataTransfer.setDragImage(dragGhost, 0, 0)
    setTimeout(() => document.body.removeChild(dragGhost), 0)
  }
}

function handleDragEnd() {
  isDragging.value = false
  isDragOver.value = false
  dropPosition.value = null
}

function handleDragOver(event: DragEvent) {
  // Check if this is a tree item drag
  if (!event.dataTransfer?.types.includes('application/x-notes-tree')) {
    return
  }

  isDragOver.value = true
  event.dataTransfer.dropEffect = 'move'

  // Determine drop position based on mouse position within the element
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  const y = event.clientY - rect.top
  const height = rect.height

  if (props.item.type === 'folder') {
    // Folders can accept drops inside, before, or after
    if (y < height * 0.25) {
      dropPosition.value = 'before'
    } else if (y > height * 0.75) {
      dropPosition.value = 'after'
    } else {
      dropPosition.value = 'inside'
    }
  } else {
    // Notes only accept before or after
    if (y < height * 0.5) {
      dropPosition.value = 'before'
    } else {
      dropPosition.value = 'after'
    }
  }
}

function handleEmptyFolderDragOver(event: DragEvent) {
  if (!event.dataTransfer?.types.includes('application/x-notes-tree')) {
    return
  }
  isDragOver.value = true
  dropPosition.value = 'inside'
  event.dataTransfer.dropEffect = 'move'
}

function handleDragLeave(event: DragEvent) {
  // Only reset if we're actually leaving this element
  const relatedTarget = event.relatedTarget as HTMLElement
  const currentTarget = event.currentTarget as HTMLElement
  if (!currentTarget.contains(relatedTarget)) {
    isDragOver.value = false
    dropPosition.value = null
  }
}

function handleDrop(event: DragEvent) {
  isDragOver.value = false
  const currentDropPosition = dropPosition.value
  dropPosition.value = null
  
  const data = event.dataTransfer?.getData('application/x-notes-tree')
  if (!data) return

  try {
    const { id, type, parentId: draggedParentId } = JSON.parse(data)
    
    // Don't drop on self
    if (id === props.item.id) return
    
    // Don't drop folder into its own descendants
    if (type === 'folder' && currentDropPosition === 'inside') {
      if (isDescendant(id, props.item.id)) return
    }

    // Calculate target folder and position
    let targetId: string | null
    let position: number

    if (currentDropPosition === 'inside') {
      // Dropping inside a folder
      targetId = props.item.id
      position = 0 // First position in the folder
    } else if (currentDropPosition === 'before') {
      // Dropping before this item - same parent as this item
      targetId = props.item.parentId
      position = props.item.position
    } else {
      // Dropping after this item - same parent as this item
      targetId = props.item.parentId
      position = props.item.position + 1
    }

    // Adjust position if moving within the same folder
    if (draggedParentId === targetId && currentDropPosition === 'after') {
      // Item will be removed first, so adjust position
    }

    emit('drop', {
      draggedId: id,
      draggedType: type,
      targetId,
      targetType: props.item.type,
      position,
      dropPosition: currentDropPosition || 'after'
    })
  } catch (e) {
    logger.error('Failed to parse drag data:', e)
  }
}

// Helper to check if targetId is a descendant of folderId
function isDescendant(folderId: string, targetId: string): boolean {
  const folders = explorerStore.folders
  let currentId: string | null = targetId
  
  while (currentId) {
    if (currentId === folderId) return true
    const folder = folders.find(f => f.id === currentId)
    currentId = folder?.parent_id || null
  }
  
  return false
}
</script>

<style scoped>
.tree-item {
  display: flex;
  align-items: center;
  height: 22px;
  cursor: pointer;
  user-select: none;
  position: relative;
  gap: 4px;
  transition: background 0.1s, opacity 0.1s;
}

.tree-item:hover {
  background: var(--bg-hover);
}

.tree-item.is-selected {
  background: var(--accent-light);
}

.tree-item.drag-over {
  background: color-mix(in srgb, var(--accent) 30%, transparent);
}

.tree-item.is-dragging {
  opacity: 0.5;
}

.drop-indicator {
  height: 2px;
  background: var(--accent);
  border-radius: 1px;
  margin: 0 8px;
  animation: pulse 0.5s ease-in-out infinite alternate;
}

@keyframes pulse {
  from { opacity: 0.7; }
  to { opacity: 1; }
}

.tree-arrow {
  width: 16px;
  text-align: center;
  font-size: 12px;
  color: var(--text-muted);
  transition: transform 0.1s;
  flex-shrink: 0;
}

.tree-arrow.expanded {
  transform: rotate(90deg);
}

.tree-arrow-placeholder {
  width: 16px;
  flex-shrink: 0;
}

.tree-icon {
  font-size: 14px;
  flex-shrink: 0;
}

.tree-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 13px;
  color: var(--text-primary);
}

.tree-actions {
  display: none;
  gap: 2px;
  padding-right: 8px;
}

.tree-item:hover .tree-actions {
  display: flex;
}

.tree-actions button {
  background: transparent;
  border: none;
  padding: 2px 4px;
  cursor: pointer;
  font-size: 11px;
  opacity: 0.7;
  border-radius: 3px;
}

.tree-actions button:hover {
  opacity: 1;
  background: var(--bg-hover);
}

.tree-actions .delete-btn:hover {
  background: color-mix(in srgb, var(--danger) 20%, transparent);
}

.tree-empty {
  height: 22px;
  display: flex;
  align-items: center;
  transition: background 0.1s;
}

.tree-empty.drag-over {
  background: color-mix(in srgb, var(--accent) 20%, transparent);
}

.empty-text {
  font-size: 12px;
  color: var(--text-muted);
  font-style: italic;
}

/* Drag ghost styles (applied via JS) */
:global(.drag-ghost) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
  padding: 4px 12px;
  border-radius: 4px;
  font-size: 13px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-primary);
}
</style>
