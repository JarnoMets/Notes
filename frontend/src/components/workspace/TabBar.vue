<template>
  <div 
    class="tab-bar"
    @dragenter="handleContainerDragEnter"
    @dragover="handleContainerDragOver"
    @dragleave="handleContainerDragLeave"
    @drop="handleContainerDrop"
  >
    <div class="tabs-container" ref="tabsContainer">
      <div
        v-for="(tab, tabIndex) in tabs"
        :key="tab.id"
        class="tab"
        :class="{ 
          active: tab.id === activeTabId,
          'drop-left': dropTarget?.index === tabIndex && dropTarget?.position === 'left',
          'drop-right': dropTarget?.index === tabIndex && dropTarget?.position === 'right'
        }"
        @click.stop="$emit('selectTab', tab.id)"
        @mousedown.middle="$emit('closeTab', tab.id)"
        draggable="true"
        @dragstart="$emit('dragStart', $event, tab, tabIndex)"
        @dragend="$emit('dragEnd')"
        @dragover.prevent="handleDragOver($event, tabIndex)"
        @drop="$emit('drop', $event, tabIndex)"
      >
        <span class="tab-icon"><Icon name="file" :size="14" /></span>
        <span class="tab-title">{{ tab.title }}</span>
        <span v-if="tab.isDirty" class="tab-dirty">●</span>
        <button 
          class="tab-close"
          @click.stop="$emit('closeTab', tab.id)"
        ><Icon name="x" :size="12" /></button>
      </div>
      
      <!-- Drop zone at end of tabs for tree drags -->
      <div 
        class="tab-drop-zone"
        :class="{ 'drop-active': isEndDropActive }"
        @dragover="handleEndDropZoneDragOver"
        @dragleave="handleEndDropZoneDragLeave"
        @drop="handleEndDropZoneDrop"
      ></div>
    </div>
    
    <!-- Split drop zone (far right) -->
    <div 
      class="split-drop-zone"
      :class="{ 'drop-active': isSplitDropActive }"
      @dragover="handleSplitDragOver"
      @dragleave="handleSplitDragLeave"
      @drop="handleSplitDrop"
    >
      <Icon name="split-vertical" :size="14" />
    </div>
    
    <div class="tab-actions">
      <button 
        v-if="!hasSplit"
        @click="$emit('split', 'vertical')" 
        title="Split Right"
      ><Icon name="split-vertical" :size="14" /></button>
      <button 
        v-if="!hasSplit"
        @click="$emit('split', 'horizontal')" 
        title="Split Down"
      ><Icon name="split-horizontal" :size="14" /></button>
      <button 
        v-if="hasSplit"
        @click="$emit('closeSplit')" 
        title="Close Split"
      ><Icon name="x" :size="14" /></button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import Icon from '../common/ui/Icon.vue'

export interface Tab {
  id: string
  title: string
  noteId: string
  isDirty?: boolean
}

export interface DropTarget {
  index: number
  position: 'left' | 'right'
}

defineProps<{
  tabs: Tab[]
  activeTabId: string | null
  hasSplit: boolean
  dropTarget?: DropTarget | null
}>()

const emit = defineEmits<{
  selectTab: [tabId: string]
  closeTab: [tabId: string]
  split: [direction: 'horizontal' | 'vertical']
  closeSplit: []
  dragStart: [event: DragEvent, tab: Tab, index: number]
  dragEnd: []
  dragOver: [index: number, position: 'left' | 'right']
  drop: [event: DragEvent, index: number]
  treeDrop: [event: DragEvent, insertIndex: number]
  splitDrop: [event: DragEvent]
}>()

const tabsContainer = ref<HTMLElement | null>(null)
const isEndDropActive = ref(false)
const isSplitDropActive = ref(false)

function handleDragOver(event: DragEvent, tabIndex: number) {
  const rect = (event.currentTarget as HTMLElement).getBoundingClientRect()
  const x = event.clientX - rect.left
  const position = x < rect.width / 2 ? 'left' : 'right'
  emit('dragOver', tabIndex, position)
}

function handleContainerDragEnter(event: DragEvent) {
  // Check if this is an explorer tree item drag
  const isExplorerDrag = event.dataTransfer?.types?.includes?.('application/x-explorer-item')
  if (isExplorerDrag) {
    event.preventDefault()
    event.dataTransfer!.dropEffect = 'copy'
  }
}

function handleContainerDragOver(event: DragEvent) {
  // Check if this is an explorer tree item drag
  const isExplorerDrag = event.dataTransfer?.types?.includes?.('application/x-explorer-item')
  if (isExplorerDrag) {
    event.preventDefault()
    event.dataTransfer!.dropEffect = 'copy'
  }
}

function handleContainerDragLeave() {
  // Don't reset drop states here - let child zones handle it
}

function handleContainerDrop(event: DragEvent) {
  // Handle drop on the container (delegated to specific zones)
  // This is a fallback in case no specific zone handles it
  event.preventDefault()
}

function handleEndDropZoneDragOver(event: DragEvent) {
  const isExplorerDrag = event.dataTransfer?.types?.includes?.('application/x-explorer-item')
  if (isExplorerDrag) {
    event.preventDefault()
    isEndDropActive.value = true
    event.dataTransfer!.dropEffect = 'copy'
  }
}

function handleEndDropZoneDragLeave() {
  isEndDropActive.value = false
}

function handleEndDropZoneDrop(event: DragEvent) {
  event.preventDefault()
  isEndDropActive.value = false
  const isExplorerDrag = event.dataTransfer?.types?.includes?.('application/x-explorer-item')
  if (isExplorerDrag) {
    // Calculate insert index based on mouse position relative to tabs
    let insertIndex = -1 // default: append at end
    
    if (tabsContainer.value) {
      const childRects = Array.from(tabsContainer.value.querySelectorAll('.tab')).map(el => 
        (el as HTMLElement).getBoundingClientRect()
      )
      
      // Find which tab's position we're closest to
      for (let i = 0; i < childRects.length; i++) {
        const tabRect = childRects[i]
        if (event.clientX < tabRect.right) {
          insertIndex = i
          break
        }
      }
    }
    
    emit('treeDrop', event, insertIndex)
  }
}

function handleSplitDragOver(event: DragEvent) {
  const isExplorerDrag = event.dataTransfer?.types?.includes?.('application/x-explorer-item')
  if (isExplorerDrag) {
    event.preventDefault()
    isSplitDropActive.value = true
    event.dataTransfer!.dropEffect = 'copy'
  }
}

function handleSplitDragLeave() {
  isSplitDropActive.value = false
}

function handleSplitDrop(event: DragEvent) {
  event.preventDefault()
  isSplitDropActive.value = false
  const isExplorerDrag = event.dataTransfer?.types?.includes?.('application/x-explorer-item')
  if (isExplorerDrag) {
    emit('splitDrop', event)
  }
}
</script>

<style scoped>
.tab-bar {
  display: flex;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
  min-height: 36px;
}

.tabs-container {
  flex: 1;
  display: flex;
  overflow-x: auto;
  overflow-y: hidden;
}

.tabs-container::-webkit-scrollbar {
  height: 3px;
}

.tabs-container::-webkit-scrollbar-thumb {
  background: var(--border-secondary);
}

.tab {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: var(--tab-bg);
  border-right: 1px solid var(--border-primary);
  cursor: pointer;
  white-space: nowrap;
  font-size: 13px;
  min-width: 100px;
  max-width: 200px;
  transition: background 0.15s;
  position: relative;
}

.tab:hover {
  background: var(--bg-hover);
}

.tab.active {
  background: var(--tab-active-bg);
  border-bottom: 2px solid var(--accent);
}

.tab.drop-left::before,
.tab.drop-right::after {
  content: '';
  position: absolute;
  top: 4px;
  bottom: 4px;
  width: 2px;
  background: var(--accent);
  border-radius: 1px;
}

.tab.drop-left::before {
  left: -1px;
}

.tab.drop-right::after {
  right: -1px;
}

.tab-icon {
  font-size: 14px;
}

.tab-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  color: var(--text-primary);
}

.tab-dirty {
  color: var(--warning);
  font-size: 10px;
}

.tab-close {
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0 4px;
  font-size: 16px;
  opacity: 0;
  border-radius: 3px;
  transition: all 0.15s;
}

.tab:hover .tab-close,
.tab.active .tab-close {
  opacity: 0.7;
}

.tab-close:hover {
  opacity: 1;
  background: var(--bg-hover);
}

/* Drop zone at end of tabs */
.tab-drop-zone {
  min-width: 40px;
  flex-shrink: 0;
  transition: all 0.15s;
}

.tab-drop-zone.drop-active {
  background: color-mix(in srgb, var(--accent) 20%, transparent);
  min-width: 60px;
}

/* Split drop zone */
.split-drop-zone {
  display: flex;
  align-items: center;
  justify-content: center;
  /* Wider hit area so splitting is easier */
  width: 64px;
  margin-right: 8px;
  border-radius: 6px;
  color: var(--text-muted);
  transition: all 0.12s;
  opacity: 0.3;
  cursor: pointer;
}

.split-drop-zone.drop-active {
  background: color-mix(in srgb, var(--accent) 30%, transparent);
  color: var(--accent);
  opacity: 1;
}

/* Make split zone less prominent when split controls are hidden (smaller opacity) */
.tab-bar:not(:has(.tab-actions button[title="Split Right"])) .split-drop-zone {
  opacity: 0.15;
}

.tab-actions {
  display: flex;
  align-items: center;
  padding: 0 0.5rem;
  gap: 0.25rem;
}

.tab-actions button {
  background: transparent;
  border: none;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0.375rem;
  font-size: 14px;
  border-radius: 4px;
  transition: all 0.15s;
}

.tab-actions button:hover {
  color: var(--text-primary);
  background: var(--bg-hover);
}

/* Responsive */
@media (max-width: 768px) {
  .tab-bar {
    min-height: 40px;
  }

  .tabs-container {
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
  }

  .tab {
    min-width: 80px;
    max-width: 150px;
    padding: 0.5rem 0.75rem;
    font-size: 12px;
  }

  .tab-icon {
    font-size: 12px;
  }

  .tab-close {
    opacity: 1;
    padding: 0.25rem;
  }

  .tab-actions button[title="Split Right"],
  .tab-actions button[title="Split Down"] {
    display: none;
  }
  
  .split-drop-zone {
    display: none;
  }
}

@media (hover: none) and (pointer: coarse) {
  .tab-close {
    opacity: 1;
    min-width: 36px;
    min-height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .tab-actions button {
    min-width: 44px;
    min-height: 44px;
  }
}
</style>
