<template>
  <div class="ribbon">
    <div class="ribbon-top">
      <button 
        class="ribbon-btn" 
        :class="{ active: activeTab === 'notes' }"
        @click="$emit('tab-change', 'notes')"
        title="Notes"
      >
        <Icon name="file" :size="20" />
      </button>
      <button 
        class="ribbon-btn" 
        :class="{ active: activeTab === 'boards' }"
        @click="$emit('tab-change', 'boards')"
        title="Boards"
      >
        <Icon name="board" :size="20" />
      </button>
      <button 
        class="ribbon-btn" 
        :class="{ active: activeTab === 'graphs' }"
        @click="$emit('tab-change', 'graphs')"
        title="Graphs"
      >
        <Icon name="share-2" :size="20" />
      </button>
      <button 
        class="ribbon-btn" 
        @click="$emit('open-calendar')"
        title="Calendar"
      >
        <Icon name="calendar" :size="20" />
      </button>
    </div>

    <div class="ribbon-middle">
      <div 
        class="sync-indicator" 
        :class="{ syncing: syncStore.isSyncing }"
        :title="syncStore.isSyncing ? 'Saving changes...' : 'All changes saved'"
      >
        <Icon :name="syncStore.isSyncing ? 'refresh' : 'check'" :size="16" />
      </div>
    </div>
    
    <div class="ribbon-bottom">
      <button class="ribbon-btn theme-btn" @click="$emit('open-theme-selector')" title="Change Theme">
        <div class="theme-icon-wrapper">
          <Icon name="palette" :size="18" />
          <div class="theme-dot"></div>
        </div>
      </button>
      <div class="user-avatar-container" @click="$emit('open-user-menu')" title="User Menu">
        <img 
          v-if="user?.avatar_url" 
          :src="user.avatar_url" 
          :alt="user.name"
          class="user-avatar"
        />
        <div v-else class="user-avatar-placeholder">
          {{ user?.name?.charAt(0).toUpperCase() }}
        </div>
      </div>
      <button class="ribbon-btn" @click="$emit('open-settings')" title="Settings">
        <Icon name="settings" :size="20" />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import Icon from '@/components/ui/Icon.vue'
import { useSyncStore } from '@/stores/sync'

const syncStore = useSyncStore()

defineProps<{
  activeTab: string
  user: any
}>()

defineEmits<{
  'tab-change': [tab: 'notes' | 'boards' | 'graphs']
  'open-calendar': []
  'open-theme-selector': []
  'open-settings': []
  'open-user-menu': []
}>()
</script>

<style scoped>
.ribbon {
  width: 48px;
  height: 100%;
  background: var(--bg-tertiary);
  border-right: 1px solid var(--border-primary);
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 8px 0;
  flex-shrink: 0;
}

/* Mobile */
@media (max-width: 768px) {
  .ribbon {
    width: 100%;
    height: 56px;
    flex-direction: row;
    border-right: none;
    border-top: 1px solid var(--border-primary);
    padding: 0 16px;
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 1000;
  }

  .ribbon-top, .ribbon-bottom {
    flex-direction: row;
    gap: 16px;
  }

  .ribbon-btn {
    width: 44px;
    height: 44px;
  }
}

.ribbon-top, .ribbon-bottom {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}

.ribbon-middle {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
  justify-content: center;
}

.sync-indicator {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  opacity: 0.5;
  transition: all 0.3s ease;
}

.sync-indicator.syncing {
  color: var(--accent);
  opacity: 1;
}

.sync-indicator.syncing .icon {
  animation: spin 2s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.ribbon-btn {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: transparent;
  border: none;
  border-radius: 6px;
  color: var(--text-muted);
  cursor: pointer;
  transition: all 0.2s;
}

.ribbon-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.ribbon-btn.active {
  color: var(--accent);
  background: var(--accent-light);
}

.theme-btn {
  position: relative;
}

.theme-icon-wrapper {
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.theme-dot {
  position: absolute;
  bottom: -2px;
  right: -2px;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--accent);
  border: 2px solid var(--bg-tertiary);
}

.user-avatar-container {
  width: 32px;
  height: 32px;
  cursor: pointer;
  margin-top: 4px;
}

.user-avatar {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  object-fit: cover;
}

.user-avatar-placeholder {
  width: 100%;
  height: 100%;
  border-radius: 50%;
  background: var(--accent);
  color: white;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 0.8rem;
}
</style>
