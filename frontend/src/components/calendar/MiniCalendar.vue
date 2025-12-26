<template>
  <div class="mini-calendar-widget">
    <div class="mini-calendar-header">
      <button class="mini-nav-btn" @click="$emit('prev')">
        <Icon name="chevron-left" />
      </button>
      <span 
        class="mini-month-title" 
        @click="$emit('update:showPicker', !showPicker)"
        :class="{ 'picker-open': showPicker }"
      >
        {{ miniCalendarDate.toLocaleDateString('en-US', { month: 'short', year: 'numeric' }) }}
        <Icon name="chevron-down" class="mini-chevron" />
      </span>
      <button class="mini-nav-btn" @click="$emit('next')">
        <Icon name="chevron-right" />
      </button>
    </div>

    <!-- Year/Month Picker Dropdown -->
    <div v-if="showPicker" class="mini-calendar-picker">
      <div class="picker-year-row">
        <button class="picker-nav-btn" @click="$emit('update:pickerYear', pickerYear - 1)">
          <Icon name="chevron-left" />
        </button>
        <span class="picker-year">{{ pickerYear }}</span>
        <button class="picker-nav-btn" @click="$emit('update:pickerYear', pickerYear + 1)">
          <Icon name="chevron-right" />
        </button>
      </div>
      <div class="picker-months-grid">
        <button 
          v-for="(month, index) in monthNames" 
          :key="index"
          class="picker-month"
          :class="{ 'current': isCurrentMonth(index), 'selected': isSelectedMonth(index) }"
          @click="$emit('select-month', index)"
        >
          {{ month.slice(0,3) }}
        </button>
      </div>
    </div>

    <div v-else class="mini-calendar-grid">
      <div class="mini-day-header" v-for="day in miniDayNames" :key="day">{{ day }}</div>
      <div 
        v-for="day in miniCalendarDays" 
        :key="day.date"
        class="mini-day"
        :class="{ 'other-month': day.isOtherMonth, 'today': day.isToday, 'selected': day.isSelected }"
        @click="$emit('select-day', day)"
      >
        {{ day.dayNumber }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import Icon from '@/components/ui/Icon.vue'
import { defineProps } from 'vue'

type MiniCalendarDay = {
  date: string
  dayNumber: number
  isOtherMonth: boolean
  isToday: boolean
  isSelected: boolean
}

const props = defineProps<{
  miniCalendarDate: Date
  showPicker: boolean
  pickerYear: number
  monthNames: string[]
  miniDayNames: string[]
  miniCalendarDays: MiniCalendarDay[]
}>()

// Local helpers used in template via props
function isCurrentMonth(monthIndex: number) {
  const today = new Date()
  return today.getFullYear() === (props.pickerYear) && today.getMonth() === monthIndex
}

function isSelectedMonth(monthIndex: number) {
  return (props.miniCalendarDate.getFullYear() === props.pickerYear) && (props.miniCalendarDate.getMonth() === monthIndex)
}

// NOTE: pickerYear and showPicker are passed from parent and updated via emitted events
</script>

<style scoped>
/* Reuse parent styles; component is presentational */
</style>
