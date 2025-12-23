<template>
  <svg
    :width="size"
    :height="size"
    viewBox="0 0 24 24"
    fill="none"
    :stroke="stroke ? 'currentColor' : 'none'"
    :stroke-width="strokeWidth"
    stroke-linecap="round"
    stroke-linejoin="round"
  >
    <!-- File/Document -->
    <template v-if="name === 'file'">
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" :fill="fill ? 'currentColor' : 'none'"/>
      <polyline points="14 2 14 8 20 8"/>
    </template>

    <!-- Folder -->
    <template v-else-if="name === 'folder'">
      <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" :fill="fill ? 'currentColor' : 'none'"/>
    </template>

    <!-- Folder Open -->
    <template v-else-if="name === 'folder-open'">
      <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2v1" :fill="fill ? 'currentColor' : 'none'"/>
      <path d="M3 9h18l-2 10H5L3 9z"/>
    </template>

    <!-- Trash/Delete -->
    <template v-else-if="name === 'trash'">
      <polyline points="3 6 5 6 21 6"/>
      <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"/>
      <line x1="10" y1="11" x2="10" y2="17"/>
      <line x1="14" y1="11" x2="14" y2="17"/>
    </template>

    <!-- Edit/Pencil -->
    <template v-else-if="name === 'edit'">
      <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"/>
      <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"/>
    </template>

    <!-- Refresh -->
    <template v-else-if="name === 'refresh'">
      <polyline points="23 4 23 10 17 10"/>
      <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10"/>
    </template>

    <!-- Paperclip/Attachment -->
    <template v-else-if="name === 'paperclip'">
      <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"/>
    </template>

    <!-- Link -->
    <template v-else-if="name === 'link'">
      <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/>
      <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>
    </template>

    <!-- Copy/Duplicate -->
    <template v-else-if="name === 'copy'">
      <rect x="9" y="9" width="13" height="13" rx="2" ry="2"/>
      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"/>
    </template>

    <!-- Download -->
    <template v-else-if="name === 'download'">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
      <polyline points="7 10 12 15 17 10"/>
      <line x1="12" y1="15" x2="12" y2="3"/>
    </template>

    <!-- Plus -->
    <template v-else-if="name === 'plus'">
      <line x1="12" y1="5" x2="12" y2="19"/>
      <line x1="5" y1="12" x2="19" y2="12"/>
    </template>

    <!-- X/Close -->
    <template v-else-if="name === 'x'">
      <line x1="18" y1="6" x2="6" y2="18"/>
      <line x1="6" y1="6" x2="18" y2="18"/>
    </template>

    <!-- Check -->
    <template v-else-if="name === 'check'">
      <polyline points="20 6 9 17 4 12"/>
    </template>

    <!-- More Vertical (3 dots) -->
    <template v-else-if="name === 'more-vertical'">
      <circle cx="12" cy="5" r="1" :fill="fill ? 'currentColor' : 'none'"/>
      <circle cx="12" cy="12" r="1" :fill="fill ? 'currentColor' : 'none'"/>
      <circle cx="12" cy="19" r="1" :fill="fill ? 'currentColor' : 'none'"/>
    </template>

    <!-- Image -->
    <template v-else-if="name === 'image'">
      <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
      <circle cx="8.5" cy="8.5" r="1.5"/>
      <polyline points="21 15 16 10 5 21"/>
    </template>

    <!-- Video -->
    <template v-else-if="name === 'video'">
      <rect x="2" y="5" width="20" height="14" rx="2" ry="2"/>
      <polygon points="10 9 15 12 10 15 10 9" :fill="fill ? 'currentColor' : 'none'"/>
    </template>

    <!-- Music/Audio -->
    <template v-else-if="name === 'music'">
      <path d="M9 18V5l12-2v13"/>
      <circle cx="6" cy="18" r="3"/>
      <circle cx="18" cy="16" r="3"/>
    </template>

    <!-- File Text -->
    <template v-else-if="name === 'file-text'">
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
      <polyline points="14 2 14 8 20 8"/>
      <line x1="16" y1="13" x2="8" y2="13"/>
      <line x1="16" y1="17" x2="8" y2="17"/>
      <polyline points="10 9 9 9 8 9"/>
    </template>

    <!-- Spreadsheet/Table -->
    <template v-else-if="name === 'table'">
      <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
      <line x1="3" y1="9" x2="21" y2="9"/>
      <line x1="3" y1="15" x2="21" y2="15"/>
      <line x1="9" y1="3" x2="9" y2="21"/>
      <line x1="15" y1="3" x2="15" y2="21"/>
    </template>

    <!-- Archive/Zip -->
    <template v-else-if="name === 'archive'">
      <polyline points="21 8 21 21 3 21 3 8"/>
      <rect x="1" y="3" width="22" height="5"/>
      <line x1="10" y1="12" x2="14" y2="12"/>
    </template>

    <!-- Note -->
    <template v-else-if="name === 'note'">
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/>
      <polyline points="14 2 14 8 20 8"/>
      <line x1="16" y1="13" x2="8" y2="13"/>
      <line x1="16" y1="17" x2="8" y2="17"/>
    </template>

    <!-- Board/Kanban -->
    <template v-else-if="name === 'board'">
      <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
      <line x1="9" y1="3" x2="9" y2="21"/>
      <line x1="15" y1="3" x2="15" y2="21"/>
    </template>

    <!-- Split Vertical -->
    <template v-else-if="name === 'split-vertical'">
      <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
      <line x1="12" y1="3" x2="12" y2="21"/>
    </template>

    <!-- Split Horizontal -->
    <template v-else-if="name === 'split-horizontal'">
      <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
      <line x1="3" y1="12" x2="21" y2="12"/>
    </template>

    <!-- Insert -->
    <template v-else-if="name === 'insert'">
      <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/>
      <polyline points="17 8 12 3 7 8"/>
      <line x1="12" y1="3" x2="12" y2="15"/>
    </template>

    <!-- Chevron Right -->
    <template v-else-if="name === 'chevron-right'">
      <polyline points="9 18 15 12 9 6"/>
    </template>

    <!-- Chevron Left -->
    <template v-else-if="name === 'chevron-left'">
      <polyline points="15 18 9 12 15 6"/>
    </template>

    <!-- Chevron Down -->
    <template v-else-if="name === 'chevron-down'">
      <polyline points="6 9 12 15 18 9"/>
    </template>

    <!-- Chevron Up -->
    <template v-else-if="name === 'chevron-up'">
      <polyline points="18 15 12 9 6 15"/>
    </template>

    <!-- List -->
    <template v-else-if="name === 'list'">
      <line x1="8" y1="6" x2="21" y2="6"/>
      <line x1="8" y1="12" x2="21" y2="12"/>
      <line x1="8" y1="18" x2="21" y2="18"/>
      <line x1="3" y1="6" x2="3.01" y2="6"/>
      <line x1="3" y1="12" x2="3.01" y2="12"/>
      <line x1="3" y1="18" x2="3.01" y2="18"/>
    </template>

    <!-- External Link -->
    <template v-else-if="name === 'external-link'">
      <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>
      <polyline points="15 3 21 3 21 9"/>
      <line x1="10" y1="14" x2="21" y2="3"/>
    </template>

    <!-- Calendar -->
    <template v-else-if="name === 'calendar'">
      <rect x="3" y="4" width="18" height="18" rx="2" ry="2"/>
      <line x1="16" y1="2" x2="16" y2="6"/>
      <line x1="8" y1="2" x2="8" y2="6"/>
      <line x1="3" y1="10" x2="21" y2="10"/>
    </template>

    <!-- Filter -->
    <template v-else-if="name === 'filter'">
      <polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/>
    </template>

    <!-- Tag -->
    <template v-else-if="name === 'tag'">
      <path d="M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"/>
      <line x1="7" y1="7" x2="7.01" y2="7"/>
    </template>

    <!-- Play -->
    <template v-else-if="name === 'play'">
      <polygon points="5 3 19 12 5 21 5 3" :fill="fill ? 'currentColor' : 'none'"/>
    </template>

    <!-- Settings/Gear -->
    <template v-else-if="name === 'settings'">
      <circle cx="12" cy="12" r="3"/>
      <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"/>
    </template>

    <!-- Menu (Hamburger) -->
    <template v-else-if="name === 'menu'">
      <line x1="3" y1="6" x2="21" y2="6"/>
      <line x1="3" y1="12" x2="21" y2="12"/>
      <line x1="3" y1="18" x2="21" y2="18"/>
    </template>

    <!-- Sidebar -->
    <template v-else-if="name === 'sidebar'">
      <rect x="3" y="3" width="18" height="18" rx="2" ry="2"/>
      <line x1="9" y1="3" x2="9" y2="21"/>
    </template>

    <!-- Alert Circle (Exclamation) -->
    <template v-else-if="name === 'alert-circle'">
      <circle cx="12" cy="12" r="10"/>
      <line x1="12" y1="8" x2="12" y2="12"/>
      <line x1="12" y1="16" x2="12.01" y2="16"/>
    </template>

    <!-- Star -->
    <template v-else-if="name === 'star'">
      <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" :fill="fill ? 'currentColor' : 'none'"/>
    </template>

    <!-- Star Outline (for favorites) -->
    <template v-else-if="name === 'star-outline'">
      <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" fill="none"/>
    </template>

    <!-- Alert Triangle (urgent) -->
    <template v-else-if="name === 'alert-triangle'">
      <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" :fill="fill ? 'currentColor' : 'none'"/>
      <line x1="12" y1="9" x2="12" y2="13" :stroke="fill ? 'var(--bg-primary, #1a1a1a)' : 'currentColor'" :stroke-width="fill ? 2.5 : strokeWidth"/>
      <line x1="12" y1="17" x2="12.01" y2="17" :stroke="fill ? 'var(--bg-primary, #1a1a1a)' : 'currentColor'" :stroke-width="fill ? 2.5 : strokeWidth"/>
    </template>

    <!-- Heart (for favorites folder) -->
    <template v-else-if="name === 'heart'">
      <path d="M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z" :fill="fill ? 'currentColor' : 'none'"/>
    </template>

    <!-- Map Pin/Location -->
    <template v-else-if="name === 'map'">
      <path d="M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z"/>
      <circle cx="12" cy="10" r="3"/>
    </template>

    <!-- Share-2 (Graph) -->
    <template v-else-if="name === 'share-2'">
      <circle cx="18" cy="5" r="3"/>
      <circle cx="6" cy="12" r="3"/>
      <circle cx="18" cy="19" r="3"/>
      <line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/>
      <line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/>
    </template>
  </svg>
</template>

<script setup lang="ts">
withDefaults(defineProps<{
  name: string
  size?: number | string
  strokeWidth?: number | string
  stroke?: boolean
  fill?: boolean
}>(), {
  size: 16,
  strokeWidth: 2,
  stroke: true,
  fill: false
})
</script>
