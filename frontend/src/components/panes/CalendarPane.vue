<template>
  <div class="calendar-pane">
    <CalendarView />
  </div>
</template>

<script setup lang="ts">
import CalendarView from '@/views/CalendarView.vue'
</script>

<style scoped>
.calendar-pane {
  height: 100%;
  width: 100%;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* We might need to override some styles of CalendarView to make it fit better in a pane */
:deep(.calendar-view) {
  height: 100%;
}
</style>
