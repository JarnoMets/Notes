<template>
  <div class="calendar-header">
    <div class="header-left">
      <button class="nav-btn" @click="$emit('previous')">
        <Icon name="chevron-left" />
      </button>
      <button class="nav-btn" @click="$emit('next')">
        <Icon name="chevron-right" />
      </button>
      <button class="today-btn" @click="$emit('goToToday')">Today</button>
      <h1 class="period-title">{{ periodTitle }}</h1>
    </div>
    <div class="header-right">
      <div class="view-toggle">
        <button :class="{ active: viewMode === 'day' }" @click="$emit('update:viewMode', 'day')">Day</button>
        <button :class="{ active: viewMode === 'workweek' }" @click="$emit('update:viewMode', 'workweek')">Work Week</button>
        <button :class="{ active: viewMode === 'week' }" @click="$emit('update:viewMode', 'week')">Week</button>
        <button :class="{ active: viewMode === 'month' }" @click="$emit('update:viewMode', 'month')">Month</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Icon from '@/components/ui/Icon.vue'
import { defineProps } from 'vue'

defineProps<{
  viewMode: 'month' | 'week' | 'workweek' | 'day'
  periodTitle: string
}>()
</script>

<style scoped>
/* Styles are inherited from the parent view's scoped styles */
</style>
