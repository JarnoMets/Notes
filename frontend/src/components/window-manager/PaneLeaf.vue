<template>
  <div 
    class="pane-leaf"
    :class="{ active: isActiveLeaf }"
    @click="activate"
    @dragover.prevent
    @drop="onDrop"
  >
    <!-- Tab Bar -->
    <div class="pane-tabs">
      <TabHeader
        v-for="tab in leaf.tabs"
        :key="tab.id"
        :tab="tab"
        :is-active="tab.id === leaf.activeTabId"
        @activate="activateTab(tab.id)"
        @close="closeTab(tab.id)"
        @dragstart="(e) => onTabDragStart(e, tab)"
      />
      <div class="tabs-spacer"></div>
      <div class="pane-actions">
        <button class="pane-action" @click="split('column')" title="Split Down">
          <Icon name="columns" :size="14" style="transform: rotate(90deg)" />
        </button>
        <button class="pane-action" @click="split('row')" title="Split Right">
          <Icon name="columns" :size="14" />
        </button>
        <button class="pane-action" @click="closeLeaf" title="Close Pane">
          <Icon name="x" :size="14" />
        </button>
      </div>
    </div>

    <!-- Content -->
    <div class="pane-content">
      <template v-if="activeTab">
        <NotePane 
          v-if="activeTab.type === 'note'" 
          :note-id="activeTab.entityId" 
          :pane-id="leaf.id"
          @update-title="(title) => updateTabTitle(activeTab!.id, title)"
        />
        <BoardPane 
          v-else-if="activeTab.type === 'board'" 
          :board-id="activeTab.entityId" 
          :pane-id="leaf.id"
        />
        <GraphPane 
          v-else-if="activeTab.type === 'graph'" 
          :graph-id="activeTab.entityId" 
          :pane-id="leaf.id"
        />
        <div v-else class="unknown-type">
          Unknown tab type: {{ activeTab.type }}
        </div>
      </template>
      <div v-else class="empty-pane">
        <div class="empty-message">
          <Icon name="layout" :size="48" />
          <p>Empty Pane</p>
          <p class="sub-text">Select an item from the sidebar to open it here</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useLayoutStore, type LayoutLeaf, type LayoutTab, type LayoutDirection } from '@/stores/layout'
import TabHeader from '../window-manager/TabHeader.vue'
import NotePane from '../panes/NotePane.vue'
import BoardPane from '../panes/BoardPane.vue'
import GraphPane from '../panes/GraphPane.vue'
import Icon from '../common/ui/Icon.vue'

const props = defineProps<{
  leaf: LayoutLeaf
}>()

const layoutStore = useLayoutStore()

const isActiveLeaf = computed(() => layoutStore.activeLeafId === props.leaf.id)
const activeTab = computed(() => props.leaf.tabs.find(t => t.id === props.leaf.activeTabId))

function activate() {
  layoutStore.setActiveLeaf(props.leaf.id)
}

function activateTab(tabId: string) {
  const tab = props.leaf.tabs.find(t => t.id === tabId)
  if (tab) {
    props.leaf.activeTabId = tabId
    activate()
  }
}

function closeTab(tabId: string) {
  layoutStore.closeTab(props.leaf.id, tabId)
}

function split(direction: LayoutDirection) {
  layoutStore.splitLeaf(props.leaf.id, direction)
}

function closeLeaf() {
  layoutStore.closeLeaf(props.leaf.id)
}

function updateTabTitle(tabId: string, title: string) {
  layoutStore.updateTab(props.leaf.id, tabId, { title })
}

function onTabDragStart(event: DragEvent, tab: LayoutTab) {
  if (event.dataTransfer) {
    event.dataTransfer.setData('application/x-layout-tab', JSON.stringify({
      tabId: tab.id,
      sourceLeafId: props.leaf.id
    }))
    event.dataTransfer.effectAllowed = 'move'
  }
}

function onDrop(event: DragEvent) {
  const tabData = event.dataTransfer?.getData('application/x-layout-tab')
  const explorerData = event.dataTransfer?.getData('application/x-explorer-item')

  if (tabData) {
    // Moving existing tab
    try {
      const { tabId, sourceLeafId } = JSON.parse(tabData)
      if (sourceLeafId === props.leaf.id) return // Same leaf, maybe reorder later

      const sourceLeaf = layoutStore.findNode(sourceLeafId) as LayoutLeaf
      const tab = sourceLeaf?.tabs.find(t => t.id === tabId)
      
      if (tab && sourceLeaf) {
        // Remove from source
        layoutStore.closeTab(sourceLeafId, tabId)
        // Add to this leaf
        layoutStore.addTab(props.leaf.id, {
          type: tab.type,
          entityId: tab.entityId,
          title: tab.title,
          icon: tab.icon
        })
      }
    } catch (e) {
      console.error('Failed to parse tab drop', e)
    }
  } else if (explorerData) {
    // Dropping from sidebar
    try {
      const item = JSON.parse(explorerData)
      // Map explorer types to layout types
      let type: LayoutTab['type'] | null = null
      if (item.type === 'note') type = 'note'
      else if (item.type === 'board') type = 'board'
      else if (item.type === 'graph') type = 'graph'
      
      if (type) {
        layoutStore.addTab(props.leaf.id, {
          type,
          entityId: item.id,
          title: item.name
        })
      }
    } catch (e) {
      console.error('Failed to parse explorer drop', e)
    }
  }
}
</script>

<style scoped>
.pane-leaf {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  border: 1px solid transparent;
}

.pane-leaf.active {
  border-color: var(--accent);
}

.pane-tabs {
  display: flex;
  height: 32px;
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
}

.tabs-spacer {
  flex: 1;
}

.pane-actions {
  display: flex;
  align-items: center;
  padding: 0 4px;
}

.pane-action {
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  border-radius: 4px;
}

.pane-action:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.pane-content {
  flex: 1;
  overflow: hidden;
  position: relative;
}

.empty-pane {
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
}

.empty-message {
  text-align: center;
}

.sub-text {
  font-size: 0.85rem;
  opacity: 0.7;
}
</style>
