<template>
  <div class="calendar-grid week-style" :class="viewMode">
    <div class="week-view-container">
      <!-- Time column + Day headers -->
      <div class="week-header" :class="viewMode">
        <div class="time-gutter-header"></div>
        <div 
          v-for="day in calendarDays" 
          :key="day.date" 
          class="week-day-header"
          :class="{ 'today': day.isToday, 'selected': isSelectedDate(day.date) }"
          @click="$emit('select-date', day.date)"
          @contextmenu.prevent="$emit('open-context', $event, day)"
        >
          <span class="day-name">{{ getDayName(day.date) }}</span>
          <span class="day-number" :class="{ 'today-number': day.isToday, 'selected-number': isSelectedDate(day.date) && !day.isToday }">{{ day.dayNumber }}</span>
        </div>
      </div>

      <!-- All-day events row -->
      <div class="all-day-row" :class="viewMode">
        <div class="time-gutter all-day-label">All Day</div>
        <div 
          v-for="day in calendarDays" 
          :key="day.date + '-allday'" 
          class="all-day-cell"
          :class="{ 'today': day.isToday, 'selected': isSelectedDate(day.date) }"
          @contextmenu.prevent="$emit('open-context', $event, day)"
        >
          <div
            v-for="item in getAllDayItems(day)"
            :key="item.id"
            class="all-day-item"
            :class="[item.type, { overdue: item.isOverdue, done: item.type === 'card' && item.card && item.card.status === 'done' }]"
            :style="item.type === 'ics' ? { borderLeftColor: item.color, backgroundColor: item.color + '20' } : {}"
            @click.stop="$emit('item-click', item)"
          >
            {{ item.title }}
          </div>
        </div>
      </div>

      <!-- Scrollable time grid -->
      <div class="week-body" ref="weekBodyRef">
        <div class="time-grid" :class="viewMode">
          <!-- Time column -->
          <div class="time-gutter">
            <div v-for="hour in displayHours" :key="hour" class="time-slot-label">
              {{ formatHour(hour) }}
            </div>
          </div>

          <!-- Day columns -->
          <div 
            v-for="day in calendarDays" 
            :key="day.date + '-col'" 
            class="day-column"
            :class="{ 'today': day.isToday, 'selected': isSelectedDate(day.date) }"
            @contextmenu.prevent="$emit('open-context', $event, day)"
          >
            <!-- Hour grid lines -->
            <div v-for="hour in displayHours" :key="hour" class="hour-slot"></div>

            <!-- Time-based events -->
            <div
              v-for="item in getTimedItems(day)"
              :key="item.id"
              class="week-event"
              :class="[item.type, { overdue: item.isOverdue, done: item.type === 'card' && item.card && item.card.status === 'done' }]"
              :style="getEventStyle(item)"
              @click.stop="$emit('item-click', item)"
            >
              <span class="event-time">{{ item.time }}</span>
              <span class="event-title">{{ item.title }}</span>
              <span v-if="item.duration" class="event-duration">{{ item.duration }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { formatHour, getDayName } from '@/utils/calendar'

const props = defineProps<{
  viewMode: 'month' | 'week' | 'workweek' | 'day'
  calendarDays: any[]
  displayHours: number[]
  selectedDate: string
}>()

const emits = defineEmits(['select-date', 'open-context', 'item-click'])

const weekBodyRef = ref<HTMLElement | null>(null)

const defaultScrollHour = 7

onMounted(() => {
  // Scroll to default hour if not month view
  if (props.viewMode !== 'month' && weekBodyRef.value) {
    weekBodyRef.value.scrollTop = defaultScrollHour * 60
  }
})

function isSelectedDate(dateStr: string) {
  return props.selectedDate === dateStr
}

// getDayName and formatHour are provided by src/utils/calendar.ts

function getAllDayItems(day: any) {
  return day.items.filter((item: any) => item.isAllDay)
}

function getTimedItems(day: any) {
  return day.items.filter((item: any) => !item.isAllDay && item.time)
}

function getEventStyle(item: any): Record<string, string> {
  if (!item.time) return {}
  const startHour = item.date.getHours()
  const startMinutes = item.date.getMinutes()
  const top = (startHour * 60 + startMinutes)
  let height = 60
  if (item.endDate) {
    const diffMs = item.endDate.getTime() - item.date.getTime()
    const diffMins = Math.round(diffMs / 60000)
    height = Math.max(20, diffMins)
  }
  const bgColor = item.type === 'ics' ? item.color : 'var(--accent)'
  return {
    top: `${top}px`,
    height: `${height}px`,
    backgroundColor: item.type === 'ics' ? `${item.color}30` : 'rgba(var(--accent-rgb, 59, 130, 246), 0.2)',
    borderLeftColor: bgColor || 'var(--accent)'
  }
}

</script>

<style scoped>
/* presentational; reuse parent styles */
</style>
