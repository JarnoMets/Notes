<template>
  <div class="calendar-view" :class="{ 'sidebar-collapsed': sidebarCollapsed }">
    <!-- Sidebar -->
    <aside class="calendar-sidebar" :class="{ collapsed: sidebarCollapsed }">
      <div class="sidebar-header">
        <h2 v-if="!sidebarCollapsed">Calendars</h2>
        <button class="collapse-btn" @click="sidebarCollapsed = !sidebarCollapsed" :title="sidebarCollapsed ? 'Expand sidebar' : 'Collapse sidebar'">
          <Icon :name="sidebarCollapsed ? 'chevron-right' : 'chevron-left'" />
        </button>
      </div>
      
      <div class="sidebar-content" v-if="!sidebarCollapsed">
        <!-- Mini Calendar Widget -->
        <div class="mini-calendar-widget">
          <div class="mini-calendar-header">
            <button class="mini-nav-btn" @click="miniCalendarPrevMonth">
              <Icon name="chevron-left" />
            </button>
            <span 
              class="mini-month-title" 
              @click="toggleMiniCalendarPicker"
              :class="{ 'picker-open': showMiniCalendarPicker }"
            >
              {{ miniCalendarTitle }}
              <Icon name="chevron-down" class="mini-chevron" />
            </span>
            <button class="mini-nav-btn" @click="miniCalendarNextMonth">
              <Icon name="chevron-right" />
            </button>
          </div>
          
          <!-- Year/Month Picker Dropdown -->
          <div v-if="showMiniCalendarPicker" class="mini-calendar-picker">
            <div class="picker-year-row">
              <button class="picker-nav-btn" @click="pickerYear--">
                <Icon name="chevron-left" />
              </button>
              <span class="picker-year">{{ pickerYear }}</span>
              <button class="picker-nav-btn" @click="pickerYear++">
                <Icon name="chevron-right" />
              </button>
            </div>
            <div class="picker-months-grid">
              <button 
                v-for="(month, index) in monthNames" 
                :key="index"
                class="picker-month"
                :class="{ 
                  'current': isCurrentMonth(index),
                  'selected': isSelectedMonth(index)
                }"
                @click="selectMonth(index)"
              >
                {{ month.slice(0, 3) }}
              </button>
            </div>
          </div>
          
          <div v-if="!showMiniCalendarPicker" class="mini-calendar-grid">
            <div class="mini-day-header" v-for="day in miniDayNames" :key="day">{{ day }}</div>
            <div 
              v-for="day in miniCalendarDays" 
              :key="day.date"
              class="mini-day"
              :class="{ 
                'other-month': day.isOtherMonth, 
                'today': day.isToday,
                'selected': day.isSelected
              }"
              @click="selectMiniCalendarDay(day)"
            >
              {{ day.dayNumber }}
            </div>
          </div>
        </div>

        <!-- Board Filters -->
        <div class="filter-section">
          <div class="section-header">
            <h3>Boards</h3>
            <button class="expand-toggle" @click="toggleBoardsList" :aria-expanded="showBoardsList" :title="showBoardsList ? 'Hide boards' : 'Show boards'">
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
                    @change="toggleBoard(board.id)"
                  />
                  <span class="filter-name">{{ board.name }}</span>
                </label>
              </div>
              <!-- lists removed from calendar sidebar for simplicity -->
            </div>
          </div>
        </div>

        <!-- ICS Calendars -->
        <div class="filter-section">
          <div class="section-header">
            <h3>External Calendars</h3>
            <button class="add-btn" @click="showAddCalendarModal = true" title="Add calendar">
              <Icon name="plus" />
            </button>
          </div>
          <div class="filter-list">
            <div v-for="cal in icsCalendars" :key="cal.id" class="filter-item ics-item">
              <label>
                <input
                  type="checkbox"
                  :checked="cal.enabled"
                  @change="toggleIcsCalendar(cal.id)"
                />
                <span class="color-dot" :style="{ backgroundColor: cal.color }"></span>
                <span class="filter-name">{{ cal.name }}</span>
              </label>
              <div class="ics-actions">
                <button class="icon-btn" @click="refreshCalendar(cal.id)" title="Refresh">
                  <Icon name="refresh" />
                </button>
                <button class="icon-btn" @click="editCalendar(cal)" title="Edit">
                  <Icon name="edit" />
                </button>
                <button class="icon-btn danger" @click="deleteCalendar(cal.id)" title="Delete">
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
            <button 
              :class="{ active: showOverdue }" 
              @click="showOverdue = !showOverdue"
            >
              Overdue
            </button>
            <button 
              :class="{ active: showToday }" 
              @click="showToday = !showToday"
            >
              Today
            </button>
            <button 
              :class="{ active: showUpcoming }" 
              @click="showUpcoming = !showUpcoming"
            >
              Upcoming
            </button>
          </div>
          <button v-if="hasActiveFilters" class="clear-filters" @click="clearFilters">
            Clear filters
          </button>
        </div>
      </div>
    </aside>

    <!-- Main Calendar -->
    <main class="calendar-main">
      <div class="calendar-header">
        <div class="header-left">
          <button class="nav-btn" @click="previousPeriod">
            <Icon name="chevron-left" />
          </button>
          <button class="nav-btn" @click="nextPeriod">
            <Icon name="chevron-right" />
          </button>
          <button class="today-btn" @click="goToToday">Today</button>
          <h1 class="period-title">{{ periodTitle }}</h1>
        </div>
        <div class="header-right">
          <div class="view-toggle">
            <button :class="{ active: viewMode === 'day' }" @click="viewMode = 'day'">Day</button>
            <button :class="{ active: viewMode === 'workweek' }" @click="viewMode = 'workweek'">Work Week</button>
            <button :class="{ active: viewMode === 'week' }" @click="viewMode = 'week'">Week</button>
            <button :class="{ active: viewMode === 'month' }" @click="viewMode = 'month'">Month</button>
          </div>
        </div>
      </div>

      <!-- Month View -->
      <div v-if="viewMode === 'month'" class="calendar-grid month">
        <!-- Day headers -->
        <div class="day-headers">
          <div v-for="day in orderedDayNames" :key="day" class="day-header">{{ day }}</div>
        </div>

        <!-- Calendar cells -->
        <div class="calendar-cells">
          <div
            v-for="day in calendarDays"
            :key="day.date"
            class="calendar-cell"
            :class="{
              'other-month': day.isOtherMonth,
              'today': day.isToday,
              'selected': isSelectedDate(day.date),
              'has-items': day.items.length > 0
            }"
            @click="selectDate(day.date)"
            @contextmenu.prevent="openContextMenu($event, day)"
          >
            <div class="cell-header">
              <span class="day-number">{{ day.dayNumber }}</span>
            </div>
            <div class="cell-items">
              <div
                v-for="item in day.items.slice(0, 3)"
                :key="item.id"
                class="calendar-item"
                :class="[item.type, { overdue: item.isOverdue }]"
                :style="item.type === 'ics' ? { borderLeftColor: item.color } : {}"
                @click="onItemClick(item)"
              >
                <span class="item-time" v-if="item.time">{{ item.time }}</span>
                <span class="item-title">{{ item.title }}</span>
              </div>
              <div v-if="day.items.length > 3" class="more-items" @click="showDayModal(day)">
                +{{ day.items.length - 3 }} more
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Week/Day/Workweek View with Hours -->
      <div v-else class="calendar-grid week-style" :class="viewMode">
        <div class="week-view-container">
          <!-- Time column + Day headers -->
          <div class="week-header" :class="viewMode">
            <div class="time-gutter-header"></div>
            <div 
              v-for="day in calendarDays" 
              :key="day.date" 
              class="week-day-header"
              :class="{ 'today': day.isToday, 'selected': isSelectedDate(day.date) }"
              @click="selectDate(day.date)"
              @contextmenu.prevent="openContextMenu($event, day)"
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
              @contextmenu.prevent="openContextMenu($event, day)"
            >
              <div
                v-for="item in getAllDayItems(day)"
                :key="item.id"
                class="all-day-item"
                :class="[item.type, { overdue: item.isOverdue }]"
                :style="item.type === 'ics' ? { borderLeftColor: item.color, backgroundColor: item.color + '20' } : {}"
                @click="onItemClick(item)"
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
                @contextmenu.prevent="openContextMenu($event, day)"
              >
                <!-- Hour grid lines -->
                <div v-for="hour in displayHours" :key="hour" class="hour-slot"></div>

                <!-- Time-based events -->
                <div
                  v-for="item in getTimedItems(day)"
                  :key="item.id"
                  class="week-event"
                  :class="[item.type, { overdue: item.isOverdue }]"
                  :style="getEventStyle(item)"
                  @click="onItemClick(item)"
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
    </main>

    <!-- Add/Edit Calendar Modal -->
    <div v-if="showAddCalendarModal || editingCalendar" class="modal-overlay" @click.self="closeCalendarModal">
      <div class="modal calendar-modal">
        <h2>{{ editingCalendar ? 'Edit Calendar' : 'Add External Calendar' }}</h2>
        <form @submit.prevent="saveCalendar">
          <div class="form-group">
            <label>Name</label>
            <input v-model="calendarForm.name" type="text" placeholder="My Calendar" required />
          </div>
          <div class="form-group">
            <label>ICS URL</label>
            <input v-model="calendarForm.url" type="url" placeholder="https://..." required />
            <small>Paste the ICS/iCal URL from Outlook, Google Calendar, etc.</small>
          </div>
          <div class="form-group">
            <label>Color</label>
            <div class="color-picker">
              <button
                v-for="color in calendarColors"
                :key="color"
                type="button"
                class="color-option"
                :class="{ selected: calendarForm.color === color }"
                :style="{ backgroundColor: color }"
                @click="calendarForm.color = color"
              ></button>
            </div>
          </div>
          <div class="modal-actions">
            <button type="button" class="btn-secondary" @click="closeCalendarModal">Cancel</button>
            <button type="submit" class="btn-primary" :disabled="calendarFormLoading">
              {{ calendarFormLoading ? 'Loading...' : (editingCalendar ? 'Save' : 'Add Calendar') }}
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Day Detail Modal -->
    <div v-if="selectedDay" class="modal-overlay" @click.self="selectedDay = null">
      <div class="modal day-modal">
        <h2>{{ formatDayModalTitle(selectedDay.date) }}</h2>
        <div class="day-items-list">
          <div
            v-for="item in selectedDay.items"
            :key="item.id"
            class="day-item"
            :class="[item.type, { overdue: item.isOverdue }]"
            :style="item.type === 'ics' ? { borderLeftColor: item.color } : {}"
            @click="onItemClick(item)"
          >
            <div class="item-info">
              <span class="item-title">{{ item.title }}</span>
              <span class="item-meta" v-if="item.boardName">{{ item.boardName }}</span>
              <span class="item-meta" v-if="item.calendarName">{{ item.calendarName }}</span>
            </div>
            <span class="item-time" v-if="item.time">{{ item.time }}</span>
          </div>
        </div>
        <button class="btn-secondary close-btn" @click="selectedDay = null">Close</button>
      </div>
    </div>

    <!-- Event Detail Modal (for ICS events) -->
    <div v-if="selectedEvent" class="modal-overlay" @click.self="selectedEvent = null">
      <div class="modal event-modal">
        <div class="event-modal-header">
          <div 
            class="event-color-bar" 
            :style="{ backgroundColor: selectedEvent.color }"
          ></div>
          <h2>{{ selectedEvent.title }}</h2>
        </div>
        
        <div class="event-details">
          <div class="event-detail-row">
            <Icon name="calendar" />
            <div class="detail-content">
              <span class="detail-primary">{{ formatEventDate(selectedEvent) }}</span>
              <span class="detail-secondary" v-if="selectedEvent.time">
                {{ selectedEvent.time }}
                <template v-if="selectedEvent.endTime"> - {{ selectedEvent.endTime }}</template>
                <template v-if="selectedEvent.duration"> ({{ selectedEvent.duration }})</template>
              </span>
              <span class="detail-secondary" v-else>All day</span>
            </div>
          </div>
          
          <div v-if="selectedEvent.calendarName" class="event-detail-row">
            <Icon name="folder" />
            <span class="detail-content">{{ selectedEvent.calendarName }}</span>
          </div>
          
          <div v-if="selectedEvent.location" class="event-detail-row">
            <Icon name="map" />
            <span class="detail-content">{{ selectedEvent.location }}</span>
          </div>
          
          <div v-if="selectedEvent.description" class="event-detail-row description">
            <Icon name="file" />
            <div class="detail-content description-text">{{ selectedEvent.description }}</div>
          </div>
        </div>
        
        <button class="btn-secondary close-btn" @click="selectedEvent = null">Close</button>
      </div>
    </div>

    <!-- Context Menu -->
    <div 
      v-if="contextMenu.visible" 
      class="context-menu"
      :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }"
      @click.self="closeContextMenu"
    >
      <div class="context-menu-content">
        <button class="context-menu-item" @click="openReminderDialog">
          <Icon name="bell" :size="16" />
          Set Reminder
        </button>
        <button class="context-menu-item" @click="openAddNoteDialog">
          <Icon name="plus" :size="16" />
          Add Note
        </button>
        <div class="context-menu-divider"></div>
        <button class="context-menu-item" @click="closeContextMenu">
          <Icon name="x" :size="16" />
          Cancel
        </button>
      </div>
    </div>

    <!-- Reminder Dialog -->
    <div v-if="showReminderDialog" class="modal-overlay" @click.self="showReminderDialog = false">
      <div class="modal reminder-modal">
        <h2>Set Reminder</h2>
        <div class="modal-body">
          <div class="form-group">
            <label>Date: {{ formatDate(contextMenu.selectedDate) }}</label>
          </div>
          <div class="form-group">
            <label>Time</label>
            <input v-model="reminderForm.time" type="time" />
          </div>
          <div class="form-group">
            <label>Title</label>
            <input v-model="reminderForm.title" type="text" placeholder="Reminder title" />
          </div>
          <div class="form-group">
            <label>Remind Before</label>
            <select v-model="reminderForm.remindBefore">
              <option value="0">At time</option>
              <option value="5">5 minutes before</option>
              <option value="15">15 minutes before</option>
              <option value="30">30 minutes before</option>
              <option value="60">1 hour before</option>
              <option value="1440">1 day before</option>
            </select>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn-secondary" @click="showReminderDialog = false">Cancel</button>
          <button class="btn-primary" @click="saveReminder">Save Reminder</button>
        </div>
      </div>
    </div>

    <!-- Add Note Dialog -->
    <div v-if="showAddNoteDialog" class="modal-overlay" @click.self="showAddNoteDialog = false">
      <div class="modal add-note-modal" style="max-width: 600px;">
        <h2>Add Note for {{ formatDate(contextMenu.selectedDate) }}</h2>
        <div class="modal-body">
          <div class="form-group">
            <label>Select or Create Note</label>
            <div class="note-tree-container">
              <note-tree-view
                :folders="explorerStore.folders"
                :notes="explorerStore.notes"
                @select-note="(note) => { selectNoteFromTree(note); showAddNoteDialog = false; }"
              />
            </div>
          </div>
          <div class="form-group">
            <label>Or create a new note:</label>
          </div>
          <div class="form-group">
            <label>Note Title</label>
            <input v-model="noteForm.title" type="text" placeholder="New note title" />
          </div>
          <div class="form-group">
            <label>Content</label>
            <textarea v-model="noteForm.content" placeholder="Note content..." rows="5"></textarea>
          </div>
        </div>
        <div class="modal-footer">
          <button class="btn-secondary" @click="showAddNoteDialog = false">Cancel</button>
          <button class="btn-primary" @click="saveNote" :disabled="!noteForm.title">Create New Note</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import Icon from '@/components/Icon.vue'
import NoteTreeView from '@/components/NoteTreeView.vue'
import { boardsApi, listsApi, cardsApi, notesApi, remindersApi } from '@/api'
import { useSettingsStore } from '@/stores/settings'
import { useExplorerStore } from '@/stores/explorer'
import { useNotesStore } from '@/stores/notes'
import type { Board, List } from '@/types/board'
import type { Card } from '@/types/card'
import type { IcsCalendar } from '@/stores/settings'

const router = useRouter()
const settingsStore = useSettingsStore()
const explorerStore = useExplorerStore()
const notesStore = useNotesStore()

// Extended card with board info
interface CardWithBoard extends Card {
  boardId: string
  boardName: string
}

// State
const boards = ref<Board[]>([])
const cards = ref<CardWithBoard[]>([])
const selectedBoardIds = ref<string[]>([])
// Lists per board (boardId -> List[])
const boardLists = ref<Record<string, List[]>>({})
// Per-board expand state removed for Calendar view — show boards only (no per-board dropdowns)
const sidebarCollapsed = ref(false)
// Single spoiler-like toggle to show/hide boards section in the sidebar (persisted)
const BOARDS_LIST_KEY = 'calendar.showBoardsList'
const showBoardsList = ref<boolean>(true)
function loadShowBoardsList() {
  try {
    const v = localStorage.getItem(BOARDS_LIST_KEY)
    showBoardsList.value = v === null ? true : (v === '1' || v === 'true')
  } catch (e) {
    showBoardsList.value = true
  }
}
function saveShowBoardsList() {
  try { localStorage.setItem(BOARDS_LIST_KEY, showBoardsList.value ? '1' : '0') } catch (e) {}
}
function toggleBoardsList() {
  showBoardsList.value = !showBoardsList.value
  saveShowBoardsList()
}
loadShowBoardsList()
const viewMode = ref<'month' | 'week' | 'workweek' | 'day'>('month')

// Helper function to convert date to YYYY-MM-DD format in local time
function dateToString(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

// Helper function to parse YYYY-MM-DD string to Date in local time
function stringToDate(dateStr: string): Date {
  const [year, month, day] = dateStr.split('-').map(Number)
  return new Date(year, month - 1, day)
}

// Create today's date at midnight local time to avoid timezone offset issues
const today = new Date()
today.setHours(0, 0, 0, 0)
const todayStr = dateToString(today)

const currentDate = ref<Date>(new Date(today))
const selectedDate = ref<string>(todayStr)
const showOverdue = ref(false)
const showToday = ref(false)
const showUpcoming = ref(false)

// Mini calendar state
const miniCalendarDate = ref(new Date(today))
const showMiniCalendarPicker = ref(false)
const pickerYear = ref(new Date().getFullYear())
const monthNames = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December']

// Modal state
const showAddCalendarModal = ref(false)
const editingCalendar = ref<IcsCalendar | null>(null)
const calendarFormLoading = ref(false)
const calendarForm = ref({
  name: '',
  url: '',
  color: '#3b82f6'
})
const selectedDay = ref<CalendarDay | null>(null)
const selectedEvent = ref<CalendarItem | null>(null)

// Context menu state
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  selectedDate: '',
  clickedHour: null as number | null
})
const showReminderDialog = ref(false)
const showAddNoteDialog = ref(false)
const reminderForm = ref({
  title: '',
  time: '09:00',
  remindBefore: '0'
})
const noteForm = ref({
  title: '',
  content: ''
})

// Note reminders added to calendar (date -> [note ids])
interface NoteReminder {
  noteId: string
  noteName: string
  date: string
}
const noteReminders = ref<NoteReminder[]>([])

// Calendar colors for picker
const calendarColors = [
  '#ef4444', '#f97316', '#eab308', '#22c55e', '#14b8a6',
  '#3b82f6', '#6366f1', '#8b5cf6', '#ec4899', '#64748b'
]

// Hours for week view - show all 24 hours but scroll to 7 AM by default
const defaultScrollHour = 7

// Display all 24 hours (0-23)
const displayHours = computed(() => {
  return Array.from({ length: 24 }, (_, i) => i)
})

// Ref for the week body to allow scrolling
const weekBodyRef = ref<HTMLElement | null>(null)

// Types
interface CalendarItem {
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
  card?: CardWithBoard
  description?: string
  location?: string
  noteId?: string
}

interface CalendarDay {
  date: string
  dayNumber: number
  isOtherMonth: boolean
  isToday: boolean
  items: CalendarItem[]
}

// Computed
const icsCalendars = computed(() => settingsStore.icsCalendars)
const calendarEvents = computed(() => settingsStore.calendarEvents)
const weekStartsOnMonday = computed(() => settingsStore.weekStartsOnMonday)

const dayNames = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']

// Mini calendar computed
const miniDayNames = computed(() => {
  if (weekStartsOnMonday.value) {
    return ['M', 'T', 'W', 'T', 'F', 'S', 'S']
  }
  return ['S', 'M', 'T', 'W', 'T', 'F', 'S']
})

const miniCalendarTitle = computed(() => {
  return miniCalendarDate.value.toLocaleDateString('en-US', { month: 'short', year: 'numeric' })
})

interface MiniCalendarDay {
  date: string
  dayNumber: number
  isOtherMonth: boolean
  isToday: boolean
  isSelected: boolean
}

const miniCalendarDays = computed((): MiniCalendarDay[] => {
  const days: MiniCalendarDay[] = []
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  const todayStr = dateToString(today)
  
  const year = miniCalendarDate.value.getFullYear()
  const month = miniCalendarDate.value.getMonth()
  
  let startDate = new Date(year, month, 1)
  const firstDayOffset = weekStartsOnMonday.value 
    ? (startDate.getDay() + 6) % 7 
    : startDate.getDay()
  startDate.setDate(startDate.getDate() - firstDayOffset)
  
  let endDate = new Date(year, month + 1, 0)
  const lastDayOffset = weekStartsOnMonday.value
    ? (6 - (endDate.getDay() + 6) % 7)
    : (6 - endDate.getDay())
  endDate.setDate(endDate.getDate() + lastDayOffset)
  
  const current = new Date(startDate)
  const selectedDateStr = dateToString(currentDate.value)
  
  while (current <= endDate) {
    const dateStr = dateToString(current)
    days.push({
      date: dateStr,
      dayNumber: current.getDate(),
      isOtherMonth: current.getMonth() !== month,
      isToday: dateStr === todayStr,
      isSelected: dateStr === selectedDateStr
    })
    current.setDate(current.getDate() + 1)
  }
  
  return days
})

// Ordered day names based on week start setting
const orderedDayNames = computed(() => {
  if (weekStartsOnMonday.value) {
    return ['Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat', 'Sun']
  }
  return dayNames
})

const periodTitle = computed(() => {
  const options: Intl.DateTimeFormatOptions = { month: 'long', year: 'numeric' }
  if (viewMode.value === 'day') {
    return currentDate.value.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric', year: 'numeric' })
  }
  if (viewMode.value === 'workweek') {
    let start = getWeekStart(currentDate.value)
    if (!weekStartsOnMonday.value) {
      start = new Date(start)
      start.setDate(start.getDate() + 1)
    }
    const end = new Date(start)
    end.setDate(end.getDate() + 4)
    if (start.getMonth() === end.getMonth()) {
      return `${start.toLocaleDateString('en-US', { month: 'long', day: 'numeric' })} - ${end.getDate()}, ${end.getFullYear()}`
    }
    return `${start.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })} - ${end.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })}`
  }
  if (viewMode.value === 'week') {
    const start = getWeekStart(currentDate.value)
    const end = new Date(start)
    end.setDate(end.getDate() + 6)
    if (start.getMonth() === end.getMonth()) {
      return `${start.toLocaleDateString('en-US', { month: 'long', day: 'numeric' })} - ${end.getDate()}, ${end.getFullYear()}`
    }
    return `${start.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })} - ${end.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' })}`
  }
  return currentDate.value.toLocaleDateString('en-US', options)
})

const hasActiveFilters = computed(() => {
  return showOverdue.value || showToday.value || showUpcoming.value || selectedBoardIds.value.length < boards.value.length
})

// Get all calendar items (cards + ICS events)
const allCalendarItems = computed((): CalendarItem[] => {
  const items: CalendarItem[] = []
  const today = new Date()
  today.setHours(0, 0, 0, 0)

  // Add cards with due dates
  for (const card of cards.value) {
    if (!card.due_date) continue
    // If no boards are selected, don't show any cards.
    if (selectedBoardIds.value.length === 0) continue
    if (!selectedBoardIds.value.includes(card.boardId)) continue

    const dueDate = new Date(card.due_date)

    items.push({
      id: `card-${card.id}`,
      title: card.title,
      date: dueDate,
      type: 'card',
      isOverdue: dueDate < today && !card.archived,
      isAllDay: true,
      boardName: card.boardName,
      card
    })
  }

  // Add ICS events
  for (const event of calendarEvents.value) {
    const calendar = icsCalendars.value.find(c => c.id === event.calendarId)
    if (!calendar?.enabled) continue

    const startDate = new Date(event.start)
    const endDate = event.end ? new Date(event.end) : startDate
    const duration = calculateDuration(startDate, endDate, event.allDay)

    items.push({
      id: event.id,
      title: event.title,
      date: startDate,
      endDate: endDate,
      time: !event.allDay ? formatTime(startDate) : undefined,
      endTime: !event.allDay && event.end ? formatTime(endDate) : undefined,
      duration: duration,
      type: 'ics',
      isOverdue: false,
      isAllDay: event.allDay,
      calendarName: event.calendarName,
      color: event.calendarColor,
      description: event.description,
      location: event.location
    })
  }

  // Add note reminders
  for (const reminder of noteReminders.value) {
    const reminderDate = new Date(reminder.date)
    items.push({
      id: `note-${reminder.noteId}`,
      title: reminder.noteName,
      date: reminderDate,
      type: 'note',
      isOverdue: reminderDate < today,
      isAllDay: true,
      noteId: reminder.noteId,
      color: '#8b5cf6'
    })
  }

  return items
})

const calendarDays = computed((): CalendarDay[] => {
  const days: CalendarDay[] = []
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  const todayStr = dateToString(today)

  let startDate: Date
  let endDate: Date

  if (viewMode.value === 'day') {
    // Day view - single day
    startDate = new Date(currentDate.value)
    startDate.setHours(0, 0, 0, 0)
    endDate = new Date(startDate)
  } else if (viewMode.value === 'workweek') {
    // Work week view - Monday to Friday
    startDate = getWeekStart(currentDate.value)
    // If week starts on Sunday, move to Monday
    if (!weekStartsOnMonday.value) {
      startDate.setDate(startDate.getDate() + 1)
    }
    endDate = new Date(startDate)
    endDate.setDate(endDate.getDate() + 4) // Mon-Fri = 5 days
  } else if (viewMode.value === 'week') {
    startDate = getWeekStart(currentDate.value)
    endDate = new Date(startDate)
    endDate.setDate(endDate.getDate() + 6)
  } else {
    // Month view
    const year = currentDate.value.getFullYear()
    const month = currentDate.value.getMonth()
    
    startDate = new Date(year, month, 1)
    // Adjust start for week start preference
    const firstDayOffset = weekStartsOnMonday.value 
      ? (startDate.getDay() + 6) % 7 
      : startDate.getDay()
    startDate.setDate(startDate.getDate() - firstDayOffset)
    
    endDate = new Date(year, month + 1, 0)
    // Adjust end for week start preference
    const lastDayOffset = weekStartsOnMonday.value
      ? (6 - (endDate.getDay() + 6) % 7)
      : (6 - endDate.getDay())
    endDate.setDate(endDate.getDate() + lastDayOffset)
  }

  const current = new Date(startDate)
  while (current <= endDate) {
    const dateStr = dateToString(current)
    const isOtherMonth = current.getMonth() !== currentDate.value.getMonth()
    const isToday = dateStr === todayStr

    // Get items for this day
    const dayItems = allCalendarItems.value.filter(item => {
      const itemDate = new Date(item.date)
      itemDate.setHours(0, 0, 0, 0)
      return dateToString(itemDate) === dateStr
    })

    // Apply quick filters
    let filteredItems = dayItems
    if (showOverdue.value || showToday.value || showUpcoming.value) {
      filteredItems = dayItems.filter(item => {
        if (showOverdue.value && item.isOverdue) return true
        if (showToday.value && isToday) return true
        if (showUpcoming.value && item.date > today) return true
        return false
      })
    }

    days.push({
      date: dateStr,
      dayNumber: current.getDate(),
      isOtherMonth,
      isToday,
      items: filteredItems.sort((a, b) => a.date.getTime() - b.date.getTime())
    })

    current.setDate(current.getDate() + 1)
  }

  return days
})

// (removed list-level counts — lists are not shown in calendar sidebar)

// Per-board expand helpers removed — calendar displays only board checkboxes

// Methods
function getWeekStart(date: Date): Date {
  const d = new Date(date)
  d.setHours(0, 0, 0, 0)
  const dayOfWeek = d.getDay()
  const offset = weekStartsOnMonday.value 
    ? (dayOfWeek + 6) % 7 
    : dayOfWeek
  d.setDate(d.getDate() - offset)
  return d
}

function getDayName(dateStr: string): string {
  const date = new Date(dateStr)
  return dayNames[date.getDay()]
}

function formatHour(hour: number): string {
  if (hour === 0) return '12 AM'
  if (hour === 12) return '12 PM'
  if (hour < 12) return `${hour} AM`
  return `${hour - 12} PM`
}

// Mini calendar methods
function miniCalendarPrevMonth() {
  const d = new Date(miniCalendarDate.value)
  d.setMonth(d.getMonth() - 1)
  miniCalendarDate.value = d
}

function miniCalendarNextMonth() {
  const d = new Date(miniCalendarDate.value)
  d.setMonth(d.getMonth() + 1)
  miniCalendarDate.value = d
}

function selectMiniCalendarDay(day: MiniCalendarDay) {
  const newDate = stringToDate(day.date)
  selectedDate.value = day.date
  currentDate.value = newDate
  miniCalendarDate.value = new Date(newDate.getFullYear(), newDate.getMonth(), 1)
}

// Mini calendar picker methods
function toggleMiniCalendarPicker() {
  showMiniCalendarPicker.value = !showMiniCalendarPicker.value
  if (showMiniCalendarPicker.value) {
    pickerYear.value = miniCalendarDate.value.getFullYear()
  }
}

function isCurrentMonth(monthIndex: number): boolean {
  const today = new Date()
  return today.getFullYear() === pickerYear.value && today.getMonth() === monthIndex
}

function isSelectedMonth(monthIndex: number): boolean {
  return miniCalendarDate.value.getFullYear() === pickerYear.value && 
         miniCalendarDate.value.getMonth() === monthIndex
}

function selectMonth(monthIndex: number) {
  miniCalendarDate.value = new Date(pickerYear.value, monthIndex, 1)
  currentDate.value = new Date(pickerYear.value, monthIndex, 1)
  showMiniCalendarPicker.value = false
}

// Date selection methods
function isSelectedDate(dateStr: string): boolean {
  return selectedDate.value === dateStr
}

function selectDate(dateStr: string) {
  selectedDate.value = dateStr
  const newDate = stringToDate(dateStr)
  currentDate.value = newDate
  miniCalendarDate.value = new Date(newDate.getFullYear(), newDate.getMonth(), 1)
}

function calculateDuration(start: Date, end: Date, isAllDay: boolean): string {
  if (isAllDay) return ''
  const diffMs = end.getTime() - start.getTime()
  const diffMins = Math.round(diffMs / 60000)
  if (diffMins < 60) return `${diffMins}m`
  const hours = Math.floor(diffMins / 60)
  const mins = diffMins % 60
  if (mins === 0) return `${hours}h`
  return `${hours}h ${mins}m`
}

function getAllDayItems(day: CalendarDay): CalendarItem[] {
  return day.items.filter(item => item.isAllDay)
}

function getTimedItems(day: CalendarDay): CalendarItem[] {
  return day.items.filter(item => !item.isAllDay && item.time)
}

function getEventStyle(item: CalendarItem): Record<string, string> {
  if (!item.time) return {}
  
  const startHour = item.date.getHours()
  const startMinutes = item.date.getMinutes()
  
  // Calculate position - now showing all 24 hours starting from 0
  const top = (startHour * 60 + startMinutes) // pixels from top
  
  let height = 60 // Default 1 hour
  if (item.endDate) {
    const diffMs = item.endDate.getTime() - item.date.getTime()
    const diffMins = Math.round(diffMs / 60000)
    height = Math.max(20, diffMins) // Minimum 20px
  }
  
  const bgColor = item.type === 'ics' ? item.color : 'var(--accent)'
  
  return {
    top: `${top}px`,
    height: `${height}px`,
    backgroundColor: item.type === 'ics' ? `${item.color}30` : 'rgba(var(--accent-rgb, 59, 130, 246), 0.2)',
    borderLeftColor: bgColor || 'var(--accent)'
  }
}

function formatTime(date: Date): string {
  return date.toLocaleTimeString('en-US', { hour: 'numeric', minute: '2-digit' })
}

function formatDayModalTitle(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric', year: 'numeric' })
}

function formatEventDate(item: CalendarItem): string {
  return item.date.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric', year: 'numeric' })
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric', year: 'numeric' })
}

function previousPeriod() {
  const d = new Date(currentDate.value)
  if (viewMode.value === 'day') {
    d.setDate(d.getDate() - 1)
  } else if (viewMode.value === 'workweek' || viewMode.value === 'week') {
    d.setDate(d.getDate() - 7)
  } else {
    d.setMonth(d.getMonth() - 1)
  }
  currentDate.value = d
}

function nextPeriod() {
  const d = new Date(currentDate.value)
  if (viewMode.value === 'day') {
    d.setDate(d.getDate() + 1)
  } else if (viewMode.value === 'workweek' || viewMode.value === 'week') {
    d.setDate(d.getDate() + 7)
  } else {
    d.setMonth(d.getMonth() + 1)
  }
  currentDate.value = d
}

function goToToday() {
  const today = new Date()
  currentDate.value = today
  miniCalendarDate.value = today
  selectedDate.value = today.toISOString().split('T')[0]
}

function toggleBoard(boardId: string) {
  const idx = selectedBoardIds.value.indexOf(boardId)
  if (idx >= 0) {
    selectedBoardIds.value.splice(idx, 1)
  } else {
    selectedBoardIds.value.push(boardId)
  }
}

function toggleIcsCalendar(calendarId: string) {
  settingsStore.toggleIcsCalendar(calendarId)
}

function clearFilters() {
  showOverdue.value = false
  showToday.value = false
  showUpcoming.value = false
  selectedBoardIds.value = boards.value.map(b => b.id)
}

function showDayModal(day: CalendarDay) {
  selectedDay.value = day
}

function onItemClick(item: CalendarItem) {
  if (item.type === 'card' && item.card) {
    // Navigate to boards and include both board and card query params so the boards view
    // can open the correct board and highlight/open the card immediately.
    router.push(`/boards?board=${item.card.boardId}&card=${item.card.id}`)
  } else if (item.type === 'ics') {
    selectedEvent.value = item
  } else if (item.type === 'note' && item.noteId) {
    // Navigate to notes and open the note in the editor
    router.push('/notes')
    notesStore.openNote(item.noteId)
  }
}

// Context menu methods
function openContextMenu(event: MouseEvent, day: CalendarDay) {
  let clickedHour: number | null = null
  
  // Try to determine which hour slot was clicked
  // Check if we're in the week-body (which has the hour grid)
  const weekBody = (event.target as HTMLElement)?.closest('.week-body')
  if (weekBody) {
    const timeGutter = weekBody.querySelector('.time-gutter') as HTMLElement
    if (timeGutter) {
      const timeSlotHeight = timeGutter.clientHeight / displayHours.value.length
      const weekBodyRect = weekBody.getBoundingClientRect()
      const clickY = event.clientY - weekBodyRect.top + weekBody.scrollTop
      clickedHour = Math.floor(clickY / timeSlotHeight)
      // Clamp to valid hour range
      if (clickedHour < 0) clickedHour = 0
      if (clickedHour >= 24) clickedHour = 23
    }
  }
  
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    selectedDate: day.date,
    clickedHour
  }
}

function closeContextMenu() {
  contextMenu.value.visible = false
}

function openReminderDialog() {
  closeContextMenu()
  showReminderDialog.value = true
  
  // Set default time based on clicked hour in week view, otherwise use 09:00
  let defaultTime = '09:00'
  if (contextMenu.value.clickedHour !== null) {
    const hour = String(contextMenu.value.clickedHour).padStart(2, '0')
    defaultTime = `${hour}:00`
  }
  
  // Reset form
  reminderForm.value = {
    title: '',
    time: defaultTime,
    remindBefore: '0'
  }
}

function openAddNoteDialog() {
  closeContextMenu()
  showAddNoteDialog.value = true
  // Reset form
  noteForm.value = {
    title: '',
    content: ''
  }
  // Ensure explorer data is loaded
  if (explorerStore.notes.length === 0) {
    explorerStore.fetchAll()
  }
}

async function selectNoteFromTree(note: any) {
  // Add note reminder to calendar and save to backend
  const reminder: NoteReminder = {
    noteId: note.id,
    noteName: note.name || note.title,
    date: contextMenu.value.selectedDate
  }
  
  // Check if already exists locally
  const exists = noteReminders.value.find(r => r.noteId === note.id && r.date === contextMenu.value.selectedDate)
  if (!exists) {
    noteReminders.value.push(reminder)
    
    // Save to backend
    try {
      await remindersApi.create({
        note_id: note.id,
        title: note.name || note.title,
        date: contextMenu.value.selectedDate,
        time: '09:00',
        remind_before: '0'
      })
    } catch (error) {
      console.error('Failed to save note link to calendar:', error)
      // Remove from local array if save fails
      const index = noteReminders.value.indexOf(reminder)
      if (index > -1) {
        noteReminders.value.splice(index, 1)
      }
      alert('Failed to save note to calendar')
    }
  }
  
  showAddNoteDialog.value = false
}

async function saveReminder() {
  if (!reminderForm.value.title) {
    alert('Please enter a reminder title')
    return
  }
  
  try {
    // Reminders without a specific note are stored as reminders
    // For now, we'll save them as-is to the backend
    await remindersApi.create({
      note_id: '', // Empty note_id for standalone reminders
      title: reminderForm.value.title,
      date: contextMenu.value.selectedDate,
      time: reminderForm.value.time,
      remind_before: reminderForm.value.remindBefore
    })
    
    showReminderDialog.value = false
    
    // Refresh reminders list
    await fetchReminders()
  } catch (error) {
    console.error('Failed to save reminder:', error)
    alert('Failed to save reminder')
  }
}

async function saveNote() {
  if (!noteForm.value.title) {
    alert('Please enter a note title')
    return
  }
  
  try {
    // Create note via API
    const response = await notesApi.create({
      title: noteForm.value.title,
      content: noteForm.value.content || ''
    })
    
    const newNote = response.data
    
    // Navigate to the new note
    router.push(`/notes?note=${newNote.id}`)
    
    showAddNoteDialog.value = false
  } catch (error) {
    console.error('Failed to save note:', error)
    alert('Failed to create note')
  }
}

// Calendar modal methods
function closeCalendarModal() {
  showAddCalendarModal.value = false
  editingCalendar.value = null
  calendarForm.value = { name: '', url: '', color: '#3b82f6' }
}

function editCalendar(cal: IcsCalendar) {
  editingCalendar.value = cal
  calendarForm.value = {
    name: cal.name,
    url: cal.url,
    color: cal.color
  }
}

async function saveCalendar() {
  calendarFormLoading.value = true
  try {
    if (editingCalendar.value) {
      settingsStore.updateIcsCalendar(editingCalendar.value.id, {
        name: calendarForm.value.name,
        url: calendarForm.value.url,
        color: calendarForm.value.color
      })
    } else {
      settingsStore.addIcsCalendar(
        calendarForm.value.name,
        calendarForm.value.url,
        calendarForm.value.color
      )
    }
    closeCalendarModal()
  } catch (err) {
    console.error('Failed to save calendar:', err)
    alert('Failed to save calendar. Please check the URL and try again.')
  } finally {
    calendarFormLoading.value = false
  }
}

async function refreshCalendar(calendarId: string) {
  const calendar = icsCalendars.value.find(c => c.id === calendarId)
  if (calendar) {
    try {
      await settingsStore.fetchIcsCalendar(calendar)
    } catch (err) {
      console.error('Failed to refresh calendar:', err)
    }
  }
}

function deleteCalendar(calendarId: string) {
  if (confirm('Are you sure you want to delete this calendar?')) {
    settingsStore.removeIcsCalendar(calendarId)
  }
}

// Fetch reminders from backend
async function fetchReminders() {
  try {
    const response = await remindersApi.getAll()
    const reminders = response.data
    
    // Convert API reminders to noteReminders format for display
    // Filter to only note-type reminders (those with note_id)
    noteReminders.value = reminders
      .filter((r: any) => r.note_id)
      .map((r: any) => ({
        noteId: r.note_id,
        noteName: r.title,
        date: r.date
      }))
  } catch (err) {
    console.error('Failed to load reminders:', err)
  }
}

// Lifecycle
onMounted(async () => {
  try {
    // Fetch boards
    const boardsRes = await boardsApi.getAll()
    boards.value = boardsRes.data
    selectedBoardIds.value = boards.value.map(b => b.id)

    // Fetch cards for all boards through lists (parallelized)
    try {
      // Fetch lists for all boards in parallel and store them in boardLists
      const listsPerBoard = await Promise.all(
        boards.value.map(async (board) => {
          try {
            const listsRes = await listsApi.getAll(board.id)
            // store lists
            boardLists.value[board.id] = listsRes.data as List[]
            return { board, lists: listsRes.data as List[] }
          } catch (err) {
            console.error(`Failed to load lists for board ${board.id}:`, err)
            boardLists.value[board.id] = []
            return { board, lists: [] as List[] }
          }
        })
      )

      // Fetch cards for all lists in parallel (flattened)
      const cardFetchPromises: Promise<CardWithBoard[]>[] = []
      for (const bwl of listsPerBoard) {
        for (const list of bwl.lists) {
          const p = cardsApi.getAll(list.id)
            .then(res => (res.data as Card[]).map(card => ({ ...card, boardId: bwl.board.id, boardName: bwl.board.name })))
            .catch(err => {
              console.error(`Failed to load cards for list ${list.id}:`, err)
              return [] as CardWithBoard[]
            })
          cardFetchPromises.push(p)
        }
      }

      const cardsArrays = await Promise.all(cardFetchPromises)
      cards.value = cardsArrays.flat()
    } catch (err) {
      console.error('Failed to load cards for calendar:', err)
      cards.value = []
    }

    // ICS calendars are pre-loaded from App.vue, but ensure they're loaded
    if (!settingsStore.initialized) {
      await settingsStore.fetchAllIcsCalendars()
    }

    // Load notes and folders for the tree view in Add Note dialog
    await explorerStore.fetchAll()

    // Fetch reminders from backend
    await fetchReminders()
    
    // Scroll to 7 AM in week views after DOM is ready
    nextTick(() => {
      scrollToDefaultHour()
    })
    
    // Add click listener to close context menu
    document.addEventListener('click', closeContextMenu)
  } catch (err) {
    console.error('Failed to load calendar data:', err)
  }
})

// Scroll to default hour (7 AM) in week views
function scrollToDefaultHour() {
  if (viewMode.value !== 'month' && weekBodyRef.value) {
    // Each hour slot is 60px tall, scroll to 7 AM
    const scrollTop = defaultScrollHour * 60
    weekBodyRef.value.scrollTop = scrollTop
  }
}

// Watch for view mode changes
watch(viewMode, () => {
  // Reset to today when switching views
  currentDate.value = new Date()
  
  // Scroll to 7 AM in week views after DOM updates
  if (viewMode.value !== 'month') {
    nextTick(() => {
      scrollToDefaultHour()
    })
  }
})
</script>

<style scoped>
.calendar-view {
  display: flex;
  flex: 1;
  height: 100%;
  min-height: 0;
  background: var(--bg-primary);
  color: var(--text-primary);
  overflow: hidden;
}

/* Sidebar */
.calendar-sidebar {
  width: 280px;
  border-right: 1px solid var(--border-primary);
  display: flex;
  flex-direction: column;
  background: var(--bg-secondary);
  transition: width 0.2s ease;
}

.calendar-sidebar.collapsed {
  width: 48px;
}

.sidebar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem;
  border-bottom: 1px solid var(--border-primary);
}

.sidebar-header h2 {
  font-size: 1rem;
  font-weight: 600;
  margin: 0;
  color: var(--text-primary);
}

.collapse-btn {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
  color: var(--text-muted);
  border-radius: 4px;
}

.collapse-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.boards-spoiler-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  background: none;
  border: 1px solid transparent;
  border-radius: 6px;
  cursor: pointer;
  color: var(--text-muted);
}

.boards-spoiler-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
  border-color: var(--border-primary);
}

.boards-spoiler-btn Icon {
  pointer-events: none;
}

.sidebar-content {
  flex: 1;
  overflow-y: auto;
  padding: 1rem;
}

/* Mini Calendar Widget */
.mini-calendar-widget {
  margin-bottom: 1.5rem;
  background: var(--bg-tertiary);
  border-radius: 8px;
  padding: 0.75rem;
}

.mini-calendar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.75rem;
}

.mini-month-title {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-primary);
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  transition: background 0.15s;
}

.mini-month-title:hover {
  background: var(--bg-hover);
}

.mini-month-title .mini-chevron {
  width: 14px;
  height: 14px;
  transition: transform 0.2s;
}

.mini-month-title.picker-open .mini-chevron {
  transform: rotate(180deg);
}

.mini-nav-btn {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
  color: var(--text-muted);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.mini-nav-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

/* Mini Calendar Picker */
.mini-calendar-picker {
  padding: 0.5rem 0;
}

.picker-year-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.75rem;
}

.picker-year {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-primary);
}

.picker-nav-btn {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
  color: var(--text-muted);
  border-radius: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.picker-nav-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.picker-months-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 4px;
}

.picker-month {
  padding: 0.5rem 0.25rem;
  background: none;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.75rem;
  font-weight: 500;
  color: var(--text-primary);
  transition: all 0.15s;
}

.picker-month:hover {
  background: var(--bg-hover);
}

.picker-month.current {
  color: var(--accent);
  font-weight: 600;
}

.picker-month.selected {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.2);
  color: var(--accent);
}

.mini-calendar-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 2px;
}

.mini-day-header {
  font-size: 0.65rem;
  font-weight: 600;
  color: var(--text-muted);
  text-align: center;
  padding: 0.25rem 0;
}

.mini-day {
  font-size: 0.75rem;
  text-align: center;
  padding: 0.25rem;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.15s;
  color: var(--text-primary);
}

.mini-day:hover {
  background: var(--bg-hover);
}

.mini-day.other-month {
  color: var(--text-muted);
}

.mini-day.today {
  background: var(--accent);
  color: white;
  font-weight: 600;
}

.mini-day.selected:not(.today) {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.25);
  color: var(--accent);
  font-weight: 600;
}

.mini-day.today.selected {
  background: var(--accent);
}

.filter-section {
  margin-bottom: 1.5rem;
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.5rem;
}

.section-header h3 {
  margin: 0;
}

.filter-section h3 {
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  color: var(--text-muted);
  margin-bottom: 0.5rem;
  letter-spacing: 0.05em;
}

.filter-list {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.filter-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.375rem 0.5rem;
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.15s;
}

.filter-item:hover {
  background: var(--bg-hover);
}

.filter-item input[type="checkbox"] {
  cursor: pointer;
  accent-color: var(--accent);
}

.filter-name {
  font-size: 0.875rem;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-primary);
}

.ics-item {
  flex-wrap: wrap;
}

.ics-item label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  flex: 1;
  cursor: pointer;
}

.color-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
  box-shadow: 0 0 0 1px rgba(0,0,0,0.1);
}

.ics-actions {
  display: flex;
  gap: 0.25rem;
  opacity: 0;
  transition: opacity 0.15s;
}

.ics-item:hover .ics-actions {
  opacity: 1;
}

.icon-btn {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
  color: var(--text-muted);
  border-radius: 4px;
}

.icon-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.icon-btn.danger:hover {
  color: var(--danger);
}

.add-btn {
  background: none;
  border: none;
  padding: 0.25rem;
  cursor: pointer;
  color: var(--text-muted);
  border-radius: 4px;
}

.add-btn:hover {
  background: var(--bg-hover);
  color: var(--accent);
}

.empty-text {
  font-size: 0.8rem;
  color: var(--text-muted);
  font-style: italic;
  padding: 0.5rem;
}

.quick-filters {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.quick-filters button {
  font-size: 0.75rem;
  padding: 0.375rem 0.625rem;
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all 0.15s;
  font-weight: 500;
}

.quick-filters button:hover {
  border-color: var(--accent);
  color: var(--accent);
}

.quick-filters button.active {
  background: var(--accent);
  border-color: var(--accent);
  color: white;
}

.clear-filters {
  margin-top: 0.75rem;
  font-size: 0.75rem;
  color: var(--text-muted);
  background: none;
  border: none;
  cursor: pointer;
  padding: 0;
}

.clear-filters:hover {
  color: var(--accent);
  text-decoration: underline;
}

.setting-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.375rem 0.5rem;
  cursor: pointer;
  border-radius: 4px;
  transition: background 0.15s;
  font-size: 0.875rem;
  color: var(--text-primary);
}

.setting-item:hover {
  background: var(--bg-hover);
}

.setting-item input[type="checkbox"] {
  cursor: pointer;
  accent-color: var(--accent);
}

/* Main Calendar */
.calendar-main {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
  background: var(--bg-primary);
}

.calendar-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
  flex-shrink: 0;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.period-title {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0;
  margin-left: 0.5rem;
  color: var(--text-primary);
}

.nav-btn,
.today-btn {
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  padding: 0.5rem;
  cursor: pointer;
  color: var(--text-primary);
  transition: all 0.15s;
}

.nav-btn:hover,
.today-btn:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
}

.today-btn {
  padding: 0.5rem 1rem;
  font-weight: 500;
}

.view-toggle {
  display: flex;
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  overflow: hidden;
  background: var(--bg-tertiary);
}

.view-toggle button {
  background: none;
  border: none;
  padding: 0.5rem 1rem;
  cursor: pointer;
  color: var(--text-secondary);
  font-weight: 500;
  transition: all 0.15s;
}

.view-toggle button:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.view-toggle button.active {
  background: var(--accent);
  color: white;
}

/* Calendar Grid */
.calendar-grid {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.calendar-grid.month {
  min-height: 0;
}

.calendar-grid.week-style {
  min-height: 0;
}

.day-headers {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
  flex-shrink: 0;
}

.day-header {
  padding: 0.5rem 0.25rem;
  text-align: center;
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.calendar-cells {
  flex: 1;
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  grid-template-rows: repeat(6, 1fr);
  overflow: hidden;
  min-height: 0;
}

.calendar-cell {
  border-right: 1px solid var(--border-primary);
  border-bottom: 1px solid var(--border-primary);
  padding: 0.25rem;
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  transition: background 0.15s;
  overflow: hidden;
  min-height: 0;
}

.calendar-cell:nth-child(7n) {
  border-right: none;
}

.calendar-cell.other-month {
  background: var(--bg-secondary);
}

.calendar-cell.other-month .day-number {
  color: var(--text-muted);
}

.calendar-cell.today {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.08);
}

.calendar-cell.today .day-number {
  background: var(--accent);
  color: white;
  border-radius: 50%;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
}

.calendar-cell.selected:not(.today) {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.12);
}

.calendar-cell.selected:not(.today) .day-number {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.3);
  color: var(--accent);
  border-radius: 50%;
  width: 24px;
  height: 24px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
}

.calendar-cell:hover {
  background: var(--bg-hover);
  cursor: pointer;
}

.cell-header {
  display: flex;
  justify-content: flex-end;
  padding: 0.125rem;
  margin-bottom: 0.125rem;
  flex-shrink: 0;
}

.day-number {
  font-size: 0.8rem;
  font-weight: 500;
  color: var(--text-primary);
}

.cell-items {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  overflow: hidden;
  min-height: 0;
}

.calendar-item {
  font-size: 0.7rem;
  padding: 2px 4px;
  border-radius: 3px;
  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  display: flex;
  align-items: center;
  gap: 3px;
  font-weight: 500;
  transition: opacity 0.15s, transform 0.15s;
  flex-shrink: 0;
}

.calendar-item:hover {
  opacity: 0.85;
  transform: translateX(2px);
}

.calendar-item.card {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.15);
  border-left: 3px solid var(--accent);
  color: var(--text-primary);
}

.calendar-item.card.overdue {
  background: rgba(239, 68, 68, 0.15);
  border-left-color: #ef4444;
  color: #dc2626;
}

.calendar-item.ics {
  background: var(--bg-tertiary);
  border-left: 3px solid;
  color: var(--text-primary);
}

.item-time {
  color: var(--text-muted);
  font-size: 0.65rem;
  font-weight: 400;
  flex-shrink: 0;
}

.item-title {
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
}

.more-items {
  font-size: 0.65rem;
  color: var(--text-muted);
  cursor: pointer;
  padding: 2px 4px;
  font-weight: 500;
  border-radius: 3px;
  transition: all 0.15s;
  flex-shrink: 0;
}

.more-items:hover {
  color: var(--accent);
  background: var(--bg-hover);
}

/* Week View Styles */
.week-view-container {
  flex: 1;
  display: flex;
  flex-direction: column;
  overflow: hidden;
  min-height: 0;
}

.week-header {
  display: grid;
  grid-template-columns: 60px repeat(7, 1fr);
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
  flex-shrink: 0;
}

.week-header.day {
  grid-template-columns: 60px 1fr;
}

.week-header.workweek {
  grid-template-columns: 60px repeat(5, 1fr);
}

.week-header.week {
  grid-template-columns: 60px repeat(7, 1fr);
}

.time-gutter-header {
  border-right: 1px solid var(--border-primary);
}

.week-day-header {
  padding: 0.75rem 0.5rem;
  text-align: center;
  border-right: 1px solid var(--border-primary);
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 0.25rem;
  cursor: pointer;
  transition: background 0.15s;
}

.week-day-header:last-child {
  border-right: none;
}

.week-day-header:hover {
  background: var(--bg-hover);
}

.week-day-header.today {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.08);
}

.week-day-header.selected:not(.today) {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.12);
}

.week-day-header .day-name {
  font-size: 0.7rem;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.week-day-header .day-number {
  font-size: 1.25rem;
  font-weight: 600;
  color: var(--text-primary);
}

.week-day-header .day-number.today-number {
  background: var(--accent);
  color: white;
  border-radius: 50%;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.week-day-header .day-number.selected-number {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.3);
  color: var(--accent);
  border-radius: 50%;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
}

/* All-day events row */
.all-day-row {
  display: grid;
  grid-template-columns: 60px repeat(7, 1fr);
  border-bottom: 1px solid var(--border-primary);
  min-height: 40px;
  max-height: 80px;
  overflow: hidden;
  background: var(--bg-secondary);
  flex-shrink: 0;
}

.all-day-row.day {
  grid-template-columns: 60px 1fr;
}

.all-day-row.workweek {
  grid-template-columns: 60px repeat(5, 1fr);
}

.all-day-row.week {
  grid-template-columns: 60px repeat(7, 1fr);
}

.all-day-label {
  font-size: 0.7rem;
  color: var(--text-muted);
  display: flex;
  align-items: center;
  justify-content: center;
  border-right: 1px solid var(--border-primary);
}

.all-day-cell {
  padding: 4px;
  border-right: 1px solid var(--border-primary);
  display: flex;
  flex-direction: column;
  gap: 2px;
  overflow: hidden;
}

.all-day-cell:last-child {
  border-right: none;
}

.all-day-cell.today {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.08);
}

.all-day-cell.selected:not(.today) {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.12);
}

.all-day-item {
  font-size: 0.7rem;
  padding: 2px 6px;
  border-radius: 3px;
  cursor: pointer;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: 500;
  border-left: 3px solid;
  background: var(--bg-tertiary);
}

.all-day-item:hover {
  opacity: 0.85;
}

.all-day-item.card {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.15);
  border-left-color: var(--accent);
}

/* Week body - scrollable time grid */
.week-body {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  min-height: 0;
}

.time-grid {
  display: grid;
  grid-template-columns: 60px repeat(7, 1fr);
  position: relative;
  /* 24 hours * 60px per hour = 1440px total height */
  height: 1440px;
  min-height: 1440px;
}

.time-grid.day {
  grid-template-columns: 60px 1fr;
}

.time-grid.workweek {
  grid-template-columns: 60px repeat(5, 1fr);
}

.time-grid.week {
  grid-template-columns: 60px repeat(7, 1fr);
}

.time-gutter {
  border-right: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.time-slot-label {
  height: 60px;
  padding: 4px 8px;
  font-size: 0.7rem;
  color: var(--text-muted);
  text-align: right;
  border-bottom: 1px solid var(--border-primary);
  display: flex;
  align-items: flex-start;
  justify-content: flex-end;
}

.day-column {
  position: relative;
  border-right: 1px solid var(--border-primary);
  background: var(--bg-primary);
}

.day-column:last-child {
  border-right: none;
}

.day-column.today {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.05);
}

.day-column.selected:not(.today) {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.08);
}

.hour-slot {
  height: 60px;
  border-bottom: 1px solid var(--border-primary);
}

.week-event {
  position: absolute;
  left: 2px;
  right: 2px;
  border-radius: 4px;
  padding: 4px 6px;
  font-size: 0.7rem;
  cursor: pointer;
  overflow: hidden;
  border-left: 3px solid;
  z-index: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.week-event:hover {
  opacity: 0.9;
  z-index: 2;
}

.week-event .event-time {
  font-weight: 600;
  color: var(--text-primary);
}

.week-event .event-title {
  font-weight: 500;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.week-event .event-duration {
  font-size: 0.65rem;
  color: var(--text-muted);
}

.week-event.card {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.2);
  border-left-color: var(--accent);
}

.week-event.card.overdue {
  background: rgba(239, 68, 68, 0.2);
  border-left-color: #ef4444;
}

/* Modals */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(2px);
}

.modal {
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  padding: 1.5rem;
  max-width: 90vw;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
}

.calendar-modal {
  width: 420px;
}

.calendar-modal h2 {
  margin: 0 0 1.25rem;
  color: var(--text-primary);
  font-size: 1.125rem;
}

.form-group {
  margin-bottom: 1.25rem;
}

.form-group label {
  display: block;
  font-size: 0.875rem;
  font-weight: 500;
  margin-bottom: 0.375rem;
  color: var(--text-primary);
}

.form-group input {
  width: 100%;
  padding: 0.625rem 0.75rem;
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-size: 0.875rem;
  transition: border-color 0.15s;
}

.form-group input:focus {
  outline: none;
  border-color: var(--accent);
}

.form-group input::placeholder {
  color: var(--text-muted);
}

.form-group small {
  display: block;
  margin-top: 0.375rem;
  font-size: 0.75rem;
  color: var(--text-muted);
}

.color-picker {
  display: flex;
  gap: 0.625rem;
  flex-wrap: wrap;
}

.color-option {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 3px solid transparent;
  cursor: pointer;
  transition: transform 0.15s, border-color 0.15s;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.color-option:hover {
  transform: scale(1.1);
}

.color-option.selected {
  border-color: var(--text-primary);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  margin-top: 1.5rem;
}

.btn-primary,
.btn-secondary {
  padding: 0.625rem 1.25rem;
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  border: none;
  transition: all 0.15s;
}

.btn-primary {
  background: var(--accent);
  color: white;
}

.btn-primary:hover {
  opacity: 0.9;
  transform: translateY(-1px);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  transform: none;
}

.btn-secondary {
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  color: var(--text-primary);
}

.btn-secondary:hover {
  background: var(--bg-hover);
}

/* Day Modal */
.day-modal {
  width: 420px;
}

.day-modal h2 {
  margin: 0 0 1.25rem;
  color: var(--text-primary);
  font-size: 1.125rem;
}

.day-items-list {
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
  max-height: 400px;
  overflow-y: auto;
}

.day-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.875rem;
  border-radius: 8px;
  cursor: pointer;
  transition: opacity 0.15s, transform 0.15s;
}

.day-item:hover {
  opacity: 0.85;
  transform: translateX(4px);
}

.day-item.card {
  background: rgba(var(--accent-rgb, 59, 130, 246), 0.15);
  border-left: 4px solid var(--accent);
}

.day-item.card.overdue {
  background: rgba(239, 68, 68, 0.15);
  border-left-color: #ef4444;
}

.day-item.ics {
  background: var(--bg-tertiary);
  border-left: 4px solid;
}

.item-info {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.item-info .item-title {
  font-weight: 600;
  color: var(--text-primary);
}

.item-meta {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.close-btn {
  margin-top: 1.25rem;
  width: 100%;
}

/* Event Detail Modal */
.event-modal {
  width: 400px;
}

.event-modal-header {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.event-color-bar {
  width: 6px;
  height: 100%;
  min-height: 32px;
  border-radius: 3px;
  flex-shrink: 0;
}

.event-modal-header h2 {
  margin: 0;
  font-size: 1.25rem;
  line-height: 1.4;
  color: var(--text-primary);
}

.event-details {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.event-detail-row {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
}

.event-detail-row svg {
  flex-shrink: 0;
  color: var(--text-muted);
  margin-top: 2px;
}

.detail-content {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.detail-primary {
  font-weight: 500;
  color: var(--text-primary);
}

.detail-secondary {
  font-size: 0.875rem;
  color: var(--text-secondary);
}

.event-detail-row.description {
  align-items: flex-start;
}

.description-text {
  font-size: 0.875rem;
  color: var(--text-secondary);
  white-space: pre-wrap;
  word-break: break-word;
  max-height: 200px;
  overflow-y: auto;
}

/* Context Menu */
.context-menu {
  position: fixed;
  z-index: 1000;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 8px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
  min-width: 200px;
}

.context-menu-content {
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

.context-menu-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  background: none;
  border: none;
  color: var(--text-primary);
  font-size: 0.875rem;
  cursor: pointer;
  text-align: left;
  transition: background 0.15s;
}

.context-menu-item:hover {
  background: var(--bg-hover);
}

.context-menu-item svg {
  flex-shrink: 0;
  opacity: 0.7;
}

.context-menu-divider {
  height: 1px;
  background: var(--border-primary);
  margin: 0.25rem 0;
}

/* Modal Overlays */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}

.modal {
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: 12px;
  padding: 2rem;
  max-width: 500px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
}

.modal h2 {
  margin: 0 0 1.5rem 0;
  font-size: 1.25rem;
  color: var(--text-primary);
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
  margin-bottom: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group label {
  font-size: 0.875rem;
  font-weight: 500;
  color: var(--text-primary);
}

.form-group input,
.form-group textarea,
.form-group select {
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  color: var(--text-primary);
  font-family: inherit;
  font-size: 0.875rem;
}

.form-group input:focus,
.form-group textarea:focus,
.form-group select:focus {
  outline: none;
  border-color: var(--accent);
  box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.1);
}

.form-group textarea {
  resize: vertical;
  min-height: 100px;
}

.modal-footer {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
}

.btn-primary {
  padding: 0.625rem 1.25rem;
  background: var(--accent);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-primary:hover {
  background: var(--accent-hover);
}

.btn-primary:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-secondary {
  padding: 0.625rem 1.25rem;
  background: var(--bg-tertiary);
  color: var(--text-primary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: background 0.2s;
}

.btn-secondary:hover {
  background: var(--bg-hover);
}

.note-tree-container {
  margin-bottom: 1rem;
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  max-height: 300px;
  overflow-y: auto;
}

.note-tree-container :deep(.tree-node) {
  padding: 0.25rem 0;
}
</style>
