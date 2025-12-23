<template>
  <div 
    class="tab-header"
    :class="{ active: isActive }"
    @click="$emit('activate')"
    @mousedown.middle.prevent="$emit('close')"
    draggable="true"
    @dragstart="onDragStart"
  >
    <div class="tab-icon">
      <Icon :name="iconName" :size="14" />
    </div>
    <span class="tab-title" :title="title">{{ title }}</span>
    <div v-if="isDirty" class="tab-dirty"></div>
    <button class="tab-close" @click.stop="$emit('close')">
      <Icon name="x" :size="12" />
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Icon from '../common/ui/Icon.vue'
import type { LayoutTab } from '@/stores/layout'

const props = defineProps<{
  tab: LayoutTab
  isActive: boolean
}>()

const emit = defineEmits<{
  'activate': []
  'close': []
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

function onDragStart(event: DragEvent) {
  emit('dragstart', event)
}
</script>

<style scoped>
.tab-header {
  display: flex;
  align-items: center;
  padding: 0 10px;
  height: 32px;
  max-width: 200px;
  min-width: 80px;
  background: var(--bg-secondary);
  border-right: 1px solid var(--border-primary);
  cursor: pointer;
  user-select: none;
  font-size: 13px;
  color: var(--text-secondary);
  position: relative;
}

.tab-header:hover {
  background: var(--bg-hover);
}

.tab-header.active {
  background: var(--bg-primary);
  color: var(--text-primary);
  border-top: 2px solid var(--accent);
}

.tab-icon {
  margin-right: 6px;
  display: flex;
  align-items: center;
  opacity: 0.7;
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
  background: var(--text-muted);
  margin-left: 6px;
}

.tab-close {
  margin-left: 6px;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  border-radius: 2px;
  opacity: 0;
}

.tab-header:hover .tab-close {
  opacity: 1;
}

.tab-close:hover {
  background: rgba(0, 0, 0, 0.1);
  color: var(--text-primary);
}
</style>
