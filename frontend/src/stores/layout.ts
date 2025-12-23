import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { v4 as uuidv4 } from 'uuid'

export type LayoutDirection = 'row' | 'column'

export interface LayoutTab {
  id: string
  type: 'note' | 'board' | 'graph' | 'calendar'
  entityId?: string
  title: string
  isDirty?: boolean
  icon?: string
}

export interface LayoutLeaf {
  id: string
  type: 'leaf'
  tabs: LayoutTab[]
  activeTabId: string | null
}

export interface LayoutSplit {
  id: string
  type: 'split'
  direction: LayoutDirection
  children: LayoutNode[]
  sizes: number[] // Percentages for children
}

export type LayoutNode = LayoutLeaf | LayoutSplit

export const useLayoutStore = defineStore('layout', () => {
  // State
  const root = ref<LayoutNode>({
    id: 'root',
    type: 'leaf',
    tabs: [],
    activeTabId: null
  })

  const activeLeafId = ref<string | null>('root')

  // Actions
  function setRoot(node: LayoutNode) {
    root.value = node
  }

  function findNode(id: string, node: LayoutNode = root.value): LayoutNode | null {
    if (node.id === id) return node
    if (node.type === 'split') {
      for (const child of node.children) {
        const found = findNode(id, child)
        if (found) return found
      }
    }
    return null
  }

  function findParent(id: string, node: LayoutNode = root.value): LayoutSplit | null {
    if (node.type === 'leaf') return null
    if (node.children.some(c => c.id === id)) return node
    for (const child of node.children) {
      const found = findParent(id, child)
      if (found) return found
    }
    return null
  }

  function setActiveLeaf(leafId: string) {
    activeLeafId.value = leafId
  }

  function addTab(leafId: string, tab: Omit<LayoutTab, 'id'>) {
    const leaf = findNode(leafId) as LayoutLeaf
    if (!leaf || leaf.type !== 'leaf') return

    const newTab: LayoutTab = { ...tab, id: uuidv4() }
    leaf.tabs.push(newTab)
    leaf.activeTabId = newTab.id
    activeLeafId.value = leafId
  }

  function closeTab(leafId: string, tabId: string) {
    const leaf = findNode(leafId) as LayoutLeaf
    if (!leaf || leaf.type !== 'leaf') return

    const index = leaf.tabs.findIndex(t => t.id === tabId)
    if (index === -1) return

    leaf.tabs.splice(index, 1)
    
    if (leaf.activeTabId === tabId) {
      leaf.activeTabId = leaf.tabs.length > 0 
        ? leaf.tabs[Math.max(0, index - 1)].id 
        : null
    }
  }

  function updateTab(leafId: string, tabId: string, updates: Partial<LayoutTab>) {
    const leaf = findNode(leafId) as LayoutLeaf
    if (!leaf || leaf.type !== 'leaf') return

    const tab = leaf.tabs.find(t => t.id === tabId)
    if (tab) {
      Object.assign(tab, updates)
    }
  }

  function updateTabsByEntityId(entityId: string, updates: Partial<LayoutTab>) {
    function traverse(node: LayoutNode) {
      if (node.type === 'leaf') {
        node.tabs.forEach(tab => {
          if (tab.entityId === entityId) {
            Object.assign(tab, updates)
          }
        })
      } else {
        node.children.forEach(traverse)
      }
    }
    traverse(root.value)
  }

  function splitLeaf(leafId: string, direction: LayoutDirection) {
    const leaf = findNode(leafId) as LayoutLeaf
    if (!leaf || leaf.type !== 'leaf') return

    const parent = findParent(leafId)
    
    const newLeaf: LayoutLeaf = {
      id: uuidv4(),
      type: 'leaf',
      tabs: [],
      activeTabId: null
    }

    const newSplit: LayoutSplit = {
      id: uuidv4(),
      type: 'split',
      direction,
      children: [leaf, newLeaf],
      sizes: [50, 50]
    }

    if (!parent) {
      // Root is the leaf
      root.value = newSplit
    } else {
      const index = parent.children.findIndex(c => c.id === leafId)
      parent.children[index] = newSplit
    }
    
    activeLeafId.value = newLeaf.id
  }

  function closeLeaf(leafId: string) {
    const parent = findParent(leafId)
    if (!parent) return // Cannot close root if it's the only leaf

    const index = parent.children.findIndex(c => c.id === leafId)
    parent.children.splice(index, 1)
    parent.sizes.splice(index, 1)
    
    // Redistribute sizes
    const totalSize = parent.sizes.reduce((a, b) => a + b, 0)
    if (totalSize > 0) {
      parent.sizes = parent.sizes.map(s => (s / totalSize) * 100)
    }

    // If parent has only one child, replace parent with child
    if (parent.children.length === 1) {
      const grandParent = findParent(parent.id)
      const survivor = parent.children[0]
      
      if (!grandParent) {
        root.value = survivor
      } else {
        const pIndex = grandParent.children.findIndex(c => c.id === parent.id)
        grandParent.children[pIndex] = survivor
      }
    }
  }

  return {
    root,
    activeLeafId,
    setRoot,
    findNode,
    setActiveLeaf,
    addTab,
    closeTab,
    updateTab,
    updateTabsByEntityId,
    splitLeaf,
    closeLeaf
  }
})
