<template>
  <div class="calendar-grid month">
    <div class="day-headers">
      <div v-for="day in orderedDayNames" :key="day" class="day-header">{{ day }}</div>
    </div>

    <div class="calendar-cells">
      <div
        v-for="day in calendarDays"
        :key="day.date"
        class="calendar-cell"
        :class="{
          'other-month': day.isOtherMonth,
          'today': day.isToday,
          'selected': selectedDate === day.date,
          'has-items': day.items.length > 0
        }"
        @click="$emit('select-date', day.date)"
        @contextmenu.prevent="$emit('open-context', $event, day)"
      >
        <div class="cell-header">
          <span class="day-number">{{ day.dayNumber }}</span>
        </div>
        <div class="cell-items">
          <div
            v-for="item in day.items.slice(0, 3)"
            :key="item.id"
            class="calendar-item"
            :class="[item.type, { overdue: item.isOverdue, done: item.type === 'card' && item.card && item.card.status === 'done' }]"
            :style="item.type === 'ics' ? { borderLeftColor: item.color } : {}"
            @click.stop="$emit('item-click', item)"
          >
            <span class="item-time" v-if="item.time">{{ item.time }}</span>
            <span class="item-title">{{ item.title }}</span>
          </div>
          <div v-if="day.items.length > 3" class="more-items" @click.stop="$emit('show-day-modal', day)">
            +{{ day.items.length - 3 }} more
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue'

type CalendarItem = {
  id: string
  title: string
  date: Date
  endDate?: Date
  time?: string
  endTime?: string
  duration?: string
  type: 'card' | 'ics' | 'note'
  isOverdue: boolean
  isAllDay?: boolean
  boardName?: string
  calendarName?: string
  color?: string
  card?: any
}

type CalendarDay = {
  date: string
  dayNumber: number
  isOtherMonth: boolean
  isToday: boolean
  items: CalendarItem[]
}

defineProps<{
  calendarDays: CalendarDay[]
  orderedDayNames: string[]
  selectedDate: string
}>()
</script>

<style scoped>
/* Presentational styles come from parent scoped CSS; this file intentionally doesn't duplicate them */
</style>
