<template>
  <div 
    class="tab-header"
    :class="{ active: isActive }"
    @click="$emit('activate')"
    @contextmenu.prevent="openContextMenu($event)"
    @mousedown.middle.prevent="$emit('close')"
    draggable="true"
    @dragstart="onDragStart"
  >
    <div class="tab-icon">
      <Icon :name="iconName" :size="12" />
    </div>
    <span class="tab-title" :title="title">{{ title }}</span>
    <div v-if="isDirty" class="tab-dirty"></div>
    <button class="tab-close" @click.stop="$emit('close')">
      <Icon name="x" :size="10" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Icon from '../ui/Icon.vue'
import { openFloatingMenu } from '@/utils/floatingMenu'
import type { LayoutTab } from '@/stores/layout'

const props = defineProps<{
  tab: LayoutTab
  isActive: boolean
}>()

const emit = defineEmits<{
  'activate': []
  'close': []
  'close-others': []
  'close-left': []
  'close-right': []
  'split-down': []
  'split-right': []
  'dragstart': [event: DragEvent]
}>()

const title = computed(() => props.tab.title || 'Untitled')
const isDirty = computed(() => props.tab.isDirty)

const iconName = computed(() => {
  if (props.tab.icon) return props.tab.icon
  switch (props.tab.type) {
    case 'note': return 'file'
    case 'board': return 'board'
    case 'graph': return 'share-2'
    case 'calendar': return 'calendar'
    default: return 'file'
  }
})

function openContextMenu(event: MouseEvent) {
  const items = [
    { label: 'Close Tab', action: () => emit('close') },
    { label: 'Close Other Tabs', action: () => emit('close-others') },
    { label: 'Close Tabs to the Left', action: () => emit('close-left') },
    { label: 'Close Tabs to the Right', action: () => emit('close-right') },
    { divider: true },
    { label: 'Split Down', action: () => emit('split-down') },
    { label: 'Split Right', action: () => emit('split-right') }
  ]

  openFloatingMenu(items, event.clientX, event.clientY)
}

function onDragStart(event: DragEvent) {
  emit('dragstart', event)
}
</script>

<style scoped>
.tab-header {
  display: flex;
  align-items: center;
  padding: 0 8px 0 12px;
  height: 32px;
  max-width: 200px;
  min-width: 60px;
  background: var(--bg-secondary);
  cursor: pointer;
  user-select: none;
  font-size: 11px;
  font-weight: 500;
  color: var(--text-muted);
  position: relative;
  border-right: 1px solid var(--border-primary);
  transition: all 0.1s;
}

.tab-header:hover {
  background: var(--bg-hover);
  color: var(--text-secondary);
}

.tab-header.active {
  background: var(--bg-primary);
  color: var(--text-primary);
}

.tab-header.active::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--accent);
  z-index: 3;
}

.tab-header.active::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 0;
  right: 0;
  height: 1px;
  background: var(--bg-primary);
  z-index: 2;
}

.tab-icon {
  margin-right: 6px;
  display: flex;
  align-items: center;
  opacity: 0.6;
}

.active .tab-icon {
  opacity: 1;
  color: var(--accent);
}

.tab-title {
  flex: 1;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.tab-dirty {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--accent);
  margin-left: 6px;
}

.tab-close {
  margin-left: 4px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 18px;
  height: 18px;
  border-radius: 4px;
  opacity: 0;
  transition: all 0.1s;
}

.tab-header:hover .tab-close,
.tab-header.active .tab-close {
  opacity: 0.6;
}

.tab-close:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
  opacity: 1 !important;
}
</style>

