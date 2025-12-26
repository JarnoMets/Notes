import { useSyncStore } from '@/stores/sync'

export function useSync() {
  const syncStore = useSyncStore()

  /**
   * Wraps an async operation with sync status tracking and optimistic UI support.
   * 
   * @param action The async API call
   * @param optimisticUpdate Optional function to update local state immediately
   * @param rollback Optional function to revert local state on failure
   */
  async function withSync<T>(
    action: () => Promise<T>,
    optimisticUpdate?: () => void,
    rollback?: (error: any) => void
  ): Promise<T | undefined> {
    syncStore.startSync()
    
    if (optimisticUpdate) {
      optimisticUpdate()
    }

    try {
      const result = await action()
      return result
    } catch (error) {
      console.error('Sync failed:', error)
      if (rollback) {
        rollback(error)
      }
      // You might want to show a global error notification here
      return undefined
    } finally {
      syncStore.endSync()
    }
  }

  return {
    withSync,
    isSyncing: syncStore.isSyncing,
    pendingSyncs: syncStore.pendingSyncs
  }
}
