<template>
  <div class="tree-node">
    <div
      class="node-row"
      :class="{
        'is-selected': isSelected,
        'is-folder': isFolder,
        'is-important': item.isImportant,
        'is-urgent': item.isUrgent,
        'has-urgent-descendant': item.hasUrgentDescendant,
        'drop-target': isDropTarget,
        'drop-above': dropPosition === 'above',
        'drop-below': dropPosition === 'below',
        'drop-inside': dropPosition === 'inside'
      }"
      :style="{ paddingLeft: `${depth * 12 + 8}px` }"
      :draggable="!inFavorites"
      @click="handleClick"
      @dblclick="handleDoubleClick"
      @contextmenu.prevent="handleContextMenu"
      @dragstart="onDragStart"
      @dragend="onDragEnd"
      @dragover.prevent="onDragOver"
      @dragleave="onDragLeave"
      @drop.prevent="onDrop"
    >
      <span
        v-if="isFolder"
        class="expand-toggle"
        @click.stop="handleToggle"
      >
        <Icon :name="item.isExpanded ? 'chevron-down' : 'chevron-right'" :size="10" />
      </span>
      <span v-else class="expand-placeholder"></span>

      <Icon
        v-if="isFolder"
        :name="item.isExpanded ? 'folder-open' : 'folder'"
        :size="14"
        class="node-icon folder-icon"
      />
      <Icon
        v-else-if="item.type === 'board'"
        name="board"
        :size="14"
        class="node-icon board-icon"
        :style="item.color ? { color: item.color } : {}"
      />
      <Icon
        v-else-if="item.type === 'graph'"
        name="share-2"
        :size="14"
        class="node-icon graph-icon"
      />
      <Icon
        v-else
        name="file"
        :size="14"
        class="node-icon file-icon"
      />

      <span class="node-name" :title="item.name">{{ item.name }}</span>

      <!-- Action icons container -->
      <div class="node-actions">
        <!-- Urgent indicator (exclamation) -->
        <button 
          v-if="canBeUrgent"
          class="action-btn urgent-btn"
          :class="{ 'is-urgent': item.isUrgent, 'has-urgent-child': item.hasUrgentDescendant }"
          @click.stop="$emit('toggle-urgent', item)"
          :title="item.isUrgent ? 'Remove urgent' : 'Mark as urgent'"
        >
          <Icon name="alert-triangle" :size="10" :fill="item.isUrgent || item.hasUrgentDescendant" />
        </button>
        
        <!-- Star (favorite) indicator -->
        <button 
          v-if="canBeFavorited"
          class="action-btn star-btn"
          :class="{ 'is-starred': item.isImportant }"
          @click.stop="$emit('toggle-importance', item)"
          :title="item.isImportant ? 'Remove from favorites' : 'Add to favorites'"
        >
          <Icon :name="item.isImportant ? 'star' : 'star-outline'" :size="10" :fill="item.isImportant" />
        </button>
      </div>
    </div>

    <div v-if="isFolder && item.isExpanded && !inFavorites" class="node-children">
      <ExplorerTreeNode
        v-for="child in item.children"
        :key="child.id"
        :item="child"
        :depth="depth + 1"
        :selected-id="selectedId"
        @select="$emit('select', $event)"
        @toggle="$emit('toggle', $event)"
        @dblclick="$emit('dblclick', $event)"
        @create-note="$emit('create-note', $event)"
        @create-board="$emit('create-board', $event)"
        @create-graph="$emit('create-graph', $event)"
        @create-folder="$emit('create-folder', $event)"
        @rename="$emit('rename', $event)"
        @delete="$emit('delete', $event)"
        @toggle-importance="$emit('toggle-importance', $event)"
        @toggle-urgent="$emit('toggle-urgent', $event)"
        @move-item="$emit('move-item', $event)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue';
import type { ExplorerItem } from '@/stores/explorer';
import Icon from './Icon.vue';
import logger from '@/utils/logger'
import { openFloatingMenu } from '@/utils/floatingMenu'

const props = defineProps<{
  item: ExplorerItem;
  depth?: number;
  selectedId?: string | null;
  inFavorites?: boolean;
}>();

const emit = defineEmits<{
  (e: 'select', item: ExplorerItem): void;
  (e: 'toggle', item: ExplorerItem): void;
  (e: 'dblclick', item: ExplorerItem): void;
  (e: 'create-note', folderId: string | null): void;
  (e: 'create-board', folderId: string | null): void;
  (e: 'create-graph', folderId: string | null): void;
  (e: 'create-folder', parentId: string | null): void;
  (e: 'rename', item: ExplorerItem): void;
  (e: 'delete', item: ExplorerItem): void;
  (e: 'toggle-importance', item: ExplorerItem): void;
  (e: 'toggle-urgent', item: ExplorerItem): void;
  (e: 'move-item', data: MoveItemData): void;
}>();

interface MoveItemData {
  itemId: string;
  itemType: 'note' | 'folder' | 'board' | 'board-folder';
  targetFolderId: string | null;
  insertBeforeId: string | null;
}

const depth = computed(() => props.depth ?? 0);
const isFolder = computed(() => props.item.type === 'folder' || props.item.type === 'board-folder' || props.item.type === 'graph-folder');
const isSelected = computed(() => props.selectedId === props.item.id);
const canBeFavorited = computed(() => props.item.type === 'note' || props.item.type === 'folder' || props.item.type === 'board' || props.item.type === 'board-folder' || props.item.type === 'graph' || props.item.type === 'graph-folder');
const canBeUrgent = computed(() => props.item.type !== 'favorites-folder');

const isDropTarget = ref(false);
const dropPosition = ref<'above' | 'below' | 'inside' | null>(null);
let dragCounter = 0;

function handleClick() {
  emit('select', props.item);
}

function handleDoubleClick() {
  emit('dblclick', props.item);
}

function handleToggle() {
  emit('toggle', props.item);
}

function handleContextMenu(event: MouseEvent) {
  event.stopPropagation();
  emit('select', props.item);
  try {
    const menuItems: import('@/utils/floatingMenu').FloatingMenuItem[] = []

    if (isFolder.value) {
      if (props.item.type === 'folder') {
        menuItems.push({ label: 'New Note', action: () => emit('create-note', props.item.id) })
        menuItems.push({ label: 'New Folder', action: () => emit('create-folder', props.item.id) })
      } else if (props.item.type === 'board-folder') {
        menuItems.push({ label: 'New Board', action: () => emit('create-board', props.item.id) })
        menuItems.push({ label: 'New Folder', action: () => emit('create-folder', props.item.id) })
      } else if (props.item.type === 'graph-folder') {
        menuItems.push({ label: 'New Graph', action: () => emit('create-graph', props.item.id) })
        menuItems.push({ label: 'New Folder', action: () => emit('create-folder', props.item.id) })
      }
      menuItems.push({ divider: true })
    }

    menuItems.push({ label: 'Rename', action: () => emit('rename', props.item) })

    if (props.item.type === 'note' || props.item.type === 'folder') {
      menuItems.push({ label: props.item.isImportant ? 'Remove Star' : 'Star', action: () => emit('toggle-importance', props.item) })
    }

    menuItems.push({ label: props.item.isUrgent ? 'Remove Urgent' : 'Mark Urgent', action: () => emit('toggle-urgent', props.item) })
    menuItems.push({ divider: true })
    menuItems.push({ label: 'Delete', action: () => emit('delete', props.item), danger: true })

    openFloatingMenu(menuItems, event.clientX, event.clientY)
  } catch (e) {
    logger.error('handleContextMenu failed', e)
  }
}

function onDragStart(event: DragEvent) {
  if (props.inFavorites) {
    // Don't allow dragging items from the favorites section (they're copies)
    event.preventDefault();
    return;
  }
  const dragData = JSON.stringify({
    id: props.item.id,
    type: props.item.type,
    parentId: props.item.parentId,
    name: props.item.name
  })
  event.dataTransfer?.setData('application/x-explorer-item', dragData);
  event.dataTransfer?.setData('text/plain', props.item.id);
  event.dataTransfer!.effectAllowed = 'copyMove';
  
  // Add a class to the body to indicate dragging is in progress
  document.body.classList.add('explorer-dragging');
}

function onDragEnd() {
  isDropTarget.value = false;
  dropPosition.value = null;
  dragCounter = 0;
  
  // Remove dragging class from body
  document.body.classList.remove('explorer-dragging');
  
  // Clear all drop indicators in case any got stuck
  document.querySelectorAll('.drop-target, .drop-above, .drop-below, .drop-inside').forEach(el => {
    el.classList.remove('drop-target', 'drop-above', 'drop-below', 'drop-inside');
  });
}

function onDragOver(event: DragEvent) {
  const hasExplorerType = event.dataTransfer?.types?.includes?.('application/x-explorer-item') || false;
  if (!hasExplorerType) return;

  dragCounter++;
  isDropTarget.value = true;

  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect();
  const y = event.clientY - rect.top;
  const height = rect.height;

  if (isFolder.value) {
    if (y < height * 0.25) {
      dropPosition.value = 'above';
    } else if (y > height * 0.75) {
      dropPosition.value = 'below';
    } else {
      dropPosition.value = 'inside';
    }
  } else {
    dropPosition.value = y < height / 2 ? 'above' : 'below';
  }

  event.dataTransfer!.dropEffect = 'move';
}

function onDragLeave(event: DragEvent) {
  // Only handle if we're leaving the current element, not entering a child
  const relatedTarget = event.relatedTarget as Node | null;
  const currentTarget = event.currentTarget as HTMLElement;
  
  if (relatedTarget && currentTarget.contains(relatedTarget)) {
    return; // Moving to a child element, don't reset
  }
  
  dragCounter = Math.max(0, dragCounter - 1);
  if (dragCounter === 0) {
    isDropTarget.value = false;
    dropPosition.value = null;
  }
}

function onDrop(event: DragEvent) {
  isDropTarget.value = false;
  const position = dropPosition.value;
  dropPosition.value = null;
  dragCounter = 0;
  
  // Clean up any lingering drop indicators
  document.body.classList.remove('explorer-dragging');

  const data = event.dataTransfer?.getData('application/x-explorer-item');
  if (!data) return;

  try {
    const { id, type } = JSON.parse(data);

    if (id === props.item.id) return;

    let targetFolderId: string | null;
    let insertBeforeId: string | null;

    if (position === 'inside' && isFolder.value) {
      // Drop inside folder - append at the end of the folder's children
      targetFolderId = props.item.id;
      insertBeforeId = null;
    } else if (position === 'above') {
      // Drop above this item - insert before this item
      targetFolderId = props.item.parentId;
      insertBeforeId = props.item.id;
    } else {
      // Drop below this item - need to find the next sibling to insert before
      // If no next sibling, insertBeforeId = null means append at end
      targetFolderId = props.item.parentId;
      
      // Get the next sibling by emitting a special event or using a provided prop
      // For now, we'll emit with a special flag that tells the parent to calculate the next sibling
      insertBeforeId = '__AFTER__' + props.item.id;
    }

    emit('move-item', {
      itemId: id,
      itemType: type,
      targetFolderId,
      insertBeforeId
    });
  } catch (e) {
    logger.error('Failed to parse drag data:', e);
  }
}
</script>

<style scoped>
.tree-node {
  user-select: none;
}

.node-row {
  display: flex;
  align-items: center;
  height: 24px;
  padding-right: 8px;
  gap: 4px;
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.1s;
  position: relative;
  margin: 1px 4px;
}

.node-row:hover {
  background: var(--bg-hover);
}

.node-row.is-selected {
  background: var(--accent-light);
  color: var(--accent);
}

.node-row.is-selected .node-name {
  font-weight: 600;
  color: var(--accent);
}

.node-row.is-important .node-name {
  font-weight: 600;
}

.node-row.is-urgent .node-name,
.node-row.has-urgent-descendant .node-name {
  color: var(--danger);
}

.node-row.drop-target {
  background: var(--accent-light);
}

.node-row.drop-above::before {
  content: '';
  position: absolute;
  top: 0;
  left: 4px;
  right: 4px;
  height: 2px;
  background: var(--accent);
  border-radius: 1px;
}

.node-row.drop-below::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 4px;
  right: 4px;
  height: 2px;
  background: var(--accent);
  border-radius: 1px;
}

.node-row.drop-inside {
  outline: 1px solid var(--accent);
  outline-offset: -1px;
}

.expand-toggle {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  color: var(--text-muted);
  flex-shrink: 0;
  border-radius: 3px;
  transition: background 0.1s, color 0.1s;
}

.expand-toggle:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

.expand-placeholder {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
}

.node-icon {
  flex-shrink: 0;
  color: var(--text-muted);
}

.is-selected .node-icon {
  color: var(--accent);
}

.node-icon.folder-icon {
  color: var(--text-muted);
}

.node-icon.board-icon {
  color: var(--text-muted);
}

.node-name {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12px;
  color: var(--text-secondary);
}

/* Node action buttons */
.node-actions {
  display: flex;
  align-items: center;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.15s;
}

.node-row:hover .node-actions {
  opacity: 1;
}

/* Show actions if any are active */
.node-row .node-actions:has(.is-starred),
.node-row .node-actions:has(.is-urgent),
.node-row .node-actions:has(.has-urgent-child) {
  opacity: 1;
}

.action-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  background: transparent;
  border: none;
  border-radius: 3px;
  color: var(--text-muted);
  cursor: pointer;
  transition: all 0.15s;
  padding: 0;
}

.action-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* Star button */
.star-btn {
  color: var(--text-muted);
}

.star-btn:hover {
  color: var(--warning);
}

.star-btn.is-starred {
  color: var(--warning);
}

/* Urgent button */
.urgent-btn {
  color: var(--text-muted);
}

.urgent-btn:hover {
  color: var(--danger);
}

.urgent-btn.is-urgent {
  color: var(--danger);
}

.urgent-btn.has-urgent-child {
  color: var(--danger);
  opacity: 0.6;
}

.importance-icon {
  flex-shrink: 0;
  color: var(--warning);
}

.node-children {
  /* Children are indented via paddingLeft on node-row */
}
</style>

