import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export const useSyncStore = defineStore('sync', () => {
  const pendingSyncs = ref(0)
  const isSyncing = computed(() => pendingSyncs.value > 0)
  const lastSyncTime = ref<number | null>(null)

  function startSync() {
    pendingSyncs.value++
  }

  function endSync() {
    setTimeout(() => {
      pendingSyncs.value = Math.max(0, pendingSyncs.value - 1)
      if (pendingSyncs.value === 0) {
        lastSyncTime.value = Date.now()
      }
    }, 500) // Keep the "Saving..." state visible for a moment
  }

  return {
    pendingSyncs,
    isSyncing,
    lastSyncTime,
    startSync,
    endSync
  }
})
