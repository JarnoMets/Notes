<template>
  <div class="virtual-list" ref="container">
    <div
      class="virtual-list-spacer"
      :style="{ height: totalHeight + 'px' }"
    ></div>
    <div
      class="virtual-list-content"
      :style="{ transform: `translateY(${offsetY}px)` }"
    >
      <div
        v-for="item in visibleItems"
        :key="item.id"
        class="virtual-list-item"
        :style="{ top: item.index * itemHeight + 'px' }"
      >
        <slot :item="item" :index="item.index"></slot>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'

interface Props {
  items: any[]
  itemHeight: number
  containerHeight?: number
}

const props = withDefaults(defineProps<Props>(), {
  containerHeight: 400
})

const container = ref<HTMLElement>()
const scrollTop = ref(0)

const totalHeight = computed(() => props.items.length * props.itemHeight)

const visibleRange = computed(() => {
  const start = Math.floor(scrollTop.value / props.itemHeight)
  const end = Math.min(
    start + Math.ceil(props.containerHeight / props.itemHeight) + 2, // +2 for buffer
    props.items.length
  )
  return { start: Math.max(0, start - 1), end } // -1 and +1 for buffer
})

const visibleItems = computed(() => {
  const { start, end } = visibleRange.value
  return props.items.slice(start, end).map((item, index) => ({
    ...item,
    index: start + index
  }))
})

const offsetY = computed(() => visibleRange.value.start * props.itemHeight)

function handleScroll() {
  if (container.value) {
    scrollTop.value = container.value.scrollTop
  }
}

onMounted(() => {
  if (container.value) {
    container.value.addEventListener('scroll', handleScroll)
  }
})

onUnmounted(() => {
  if (container.value) {
    container.value.removeEventListener('scroll', handleScroll)
  }
})

watch(() => props.items, () => {
  // Reset scroll when items change
  scrollTop.value = 0
  if (container.value) {
    container.value.scrollTop = 0
  }
})
</script>

<style scoped>
.virtual-list {
  height: v-bind('containerHeight + "px"');
  overflow-y: auto;
  position: relative;
}

.virtual-list-spacer {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
}

.virtual-list-content {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
}

.virtual-list-item {
  position: absolute;
  left: 0;
  right: 0;
  height: v-bind('itemHeight + "px"');
}
</style>