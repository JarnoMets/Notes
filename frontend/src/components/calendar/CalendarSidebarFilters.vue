<template>
  <div>
    <!-- Mini Calendar Widget -->
    <MiniCalendar
      :miniCalendarDate="miniCalendarDate"
      :showPicker="showPicker"
      :pickerYear="pickerYear"
      :monthNames="monthNames"
      :miniDayNames="miniDayNames"
      :miniCalendarDays="miniCalendarDays"
      @prev="miniPrevMonth"
      @next="miniNextMonth"
      @select-day="onSelectDay"
      @select-month="onSelectMonth"
      @update:showPicker="showPicker = $event"
      @update:pickerYear="pickerYear = $event"
    />

    <!-- Board Filters -->
    <div class="filter-section">
      <div class="section-header">
        <h3>Boards</h3>
        <button class="expand-toggle" @click="$emit('toggle-boards-list')" :aria-expanded="showBoardsList" :title="showBoardsList ? 'Hide boards' : 'Show boards'">
          <Icon :name="showBoardsList ? 'chevron-down' : 'chevron-right'" />
        </button>
      </div>
      <div class="filter-list" v-if="showBoardsList">
        <div v-for="board in boards" :key="board.id" class="board-filter">
          <div class="board-header">
            <label class="filter-item">
              <input
                type="checkbox"
                :checked="selectedBoardIds.includes(board.id)"
                @change="$emit('toggle-board', board.id)"
              />
              <span class="filter-name">{{ board.name }}</span>
            </label>
          </div>
        </div>
      </div>
    </div>

    <!-- ICS Calendars -->
    <div class="filter-section">
      <div class="section-header">
        <h3>External Calendars</h3>
        <button class="add-btn" @click="$emit('open-add-calendar')" title="Add calendar">
          <Icon name="plus" />
        </button>
      </div>
      <div class="filter-list">
        <div v-for="cal in icsCalendars" :key="cal.id" class="filter-item ics-item">
          <label>
            <input
              type="checkbox"
              :checked="cal.enabled"
              @change="$emit('toggle-ics', cal.id)"
            />
            <span class="color-dot" :style="{ backgroundColor: cal.color }"></span>
            <span class="filter-name">{{ cal.name }}</span>
          </label>
          <div class="ics-actions">
            <button class="icon-btn" @click="$emit('refresh-ics', cal.id)" title="Refresh">
              <Icon name="refresh" />
            </button>
            <button class="icon-btn" @click="$emit('edit-ics', cal)" title="Edit">
              <Icon name="edit" />
            </button>
            <button class="icon-btn danger" @click="$emit('delete-ics', cal.id)" title="Delete">
              <Icon name="trash" />
            </button>
          </div>
        </div>
        <p v-if="icsCalendars.length === 0" class="empty-text">No external calendars</p>
      </div>
    </div>

    <!-- Quick Filters -->
    <div class="filter-section">
      <h3>Quick Filters</h3>
      <div class="quick-filters">
        <button :class="{ active: showOverdue }" @click="$emit('toggle-filter', 'overdue')">Overdue</button>
        <button :class="{ active: showDone }" @click="$emit('toggle-filter', 'done')">Done</button>
        <button :class="{ active: showUpcoming }" @click="$emit('toggle-filter', 'upcoming')">Upcoming</button>
      </div>
      <button v-if="hasActiveFilters" class="clear-filters" @click="$emit('clear-filters')">Clear filters</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import Icon from '@/components/common/ui/Icon.vue'
import MiniCalendar from '@/components/calendar/MiniCalendar.vue'
import type { Board } from '@/types/board'
import type { IcsCalendar } from '@/stores/settings'
import { ref, computed } from 'vue'
import { dateToString } from '@/utils/calendar'

type MiniCalendarDay = {
  date: string
  dayNumber: number
  isOtherMonth: boolean
  isToday: boolean
  isSelected: boolean
}

const props = defineProps<{
  boards: Board[]
  selectedBoardIds: string[]
  showBoardsList: boolean
  icsCalendars: IcsCalendar[]
  showOverdue: boolean
  showDone: boolean
  showUpcoming: boolean
  hasActiveFilters: boolean
  selectedDate?: string
  weekStartsOnMonday?: boolean
}>()

const emit = defineEmits([
  'toggle-boards-list', 'toggle-board', 'open-add-calendar',
  'toggle-ics', 'refresh-ics', 'edit-ics', 'delete-ics',
  'toggle-filter', 'clear-filters', 'select-date'
])

// Mini calendar local state
const miniCalendarDate = ref(new Date())
miniCalendarDate.value.setHours(0,0,0,0)
const showPicker = ref(false)
const pickerYear = ref(new Date().getFullYear())
const monthNames = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December']

const miniDayNames = computed(() => {
  if (props.weekStartsOnMonday) {
    return ['M', 'T', 'W', 'T', 'F', 'S', 'S']
  }
  return ['S', 'M', 'T', 'W', 'T', 'F', 'S']
})

const miniCalendarDays = computed((): MiniCalendarDay[] => {
  const days: MiniCalendarDay[] = []
  const today = new Date()
  today.setHours(0,0,0,0)
  const todayStr = dateToString(today)

  const year = miniCalendarDate.value.getFullYear()
  const month = miniCalendarDate.value.getMonth()

  let startDate = new Date(year, month, 1)
  const firstDayOffset = props.weekStartsOnMonday ? (startDate.getDay() + 6) % 7 : startDate.getDay()
  startDate.setDate(startDate.getDate() - firstDayOffset)

  let endDate = new Date(year, month + 1, 0)
  const lastDayOffset = props.weekStartsOnMonday ? (6 - (endDate.getDay() + 6) % 7) : (6 - endDate.getDay())
  endDate.setDate(endDate.getDate() + lastDayOffset)

  const current = new Date(startDate)
  while (current <= endDate) {
    const dateStr = dateToString(current)
    days.push({
      date: dateStr,
      dayNumber: current.getDate(),
      isOtherMonth: current.getMonth() !== month,
      isToday: dateStr === todayStr,
      isSelected: props.selectedDate ? dateStr === props.selectedDate : false
    })
    current.setDate(current.getDate() + 1)
  }

  return days
})

function miniPrevMonth() {
  const d = new Date(miniCalendarDate.value)
  d.setMonth(d.getMonth() - 1)
  miniCalendarDate.value = d
}

function miniNextMonth() {
  const d = new Date(miniCalendarDate.value)
  d.setMonth(d.getMonth() + 1)
  miniCalendarDate.value = d
}

function onSelectDay(day: MiniCalendarDay) {
  // Emit selected date string (yyyy-mm-dd) to parent
  miniCalendarDate.value = new Date(day.date)
  emit('select-date', day.date)
}

function onSelectMonth(monthIndex: number) {
  miniCalendarDate.value = new Date(pickerYear.value, monthIndex, 1)
  showPicker.value = false
}
</script>

<style scoped>
/* presentational: uses parent styles */
</style>
