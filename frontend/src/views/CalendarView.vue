<template>
  <div class="calendar-view" :class="{ 'sidebar-collapsed': sidebarCollapsed }">
    <!-- Sidebar -->
    <aside class="calendar-sidebar" :class="{ collapsed: sidebarCollapsed }">
      <div class="sidebar-content" v-if="!sidebarCollapsed">
        <CalendarSidebarFilters
          :boards="boards"
          :selectedBoardIds="selectedBoardIds"
          :showBoardsList="showBoardsList"
          :icsCalendars="icsCalendars"
          :showOverdue="showOverdue"
          :showDone="showDone"
          :showUpcoming="showUpcoming"
          :hasActiveFilters="hasActiveFilters"
          :selectedDate="selectedDate"
          :weekStartsOnMonday="weekStartsOnMonday"
          @toggle-boards-list="toggleBoardsList"
          @toggle-board="toggleBoard"
          @open-add-calendar="showAddCalendarModal = true"
          @toggle-ics="toggleIcsCalendar"
          @refresh-ics="refreshCalendar"
          @edit-ics="editCalendar"
          @delete-ics="deleteCalendar"
          @select-date="selectDate"
          @toggle-filter="(filter) => {
            if (filter === 'overdue') showOverdue = !showOverdue
            if (filter === 'done') showDone = !showDone
            if (filter === 'upcoming') showUpcoming = !showUpcoming
          }"
          @clear-filters="clearFilters"
        />
      </div>
    </aside>

    <!-- Main Calendar -->
    <main class="calendar-main">
      <CalendarHeader
        :viewMode="viewMode"
        :periodTitle="periodTitle"
        @previous="previousPeriod"
        @next="nextPeriod"
        @goToToday="goToToday"
        @update:viewMode="viewMode = $event"
      />

      <!-- Month View -->
      <MonthGrid
        v-if="viewMode === 'month'"
        :calendarDays="calendarDays"
        :orderedDayNames="orderedDayNames"
        :selectedDate="selectedDate"
        @select-date="selectDate"
        @open-context="openContextMenu"
        @item-click="onItemClick"
        @show-day-modal="showDayModal"
      />

      <WeekView
        v-else
        :viewMode="viewMode"
        :calendarDays="calendarDays"
        :displayHours="displayHours"
        :selectedDate="selectedDate"
        @select-date="selectDate"
        @open-context="openContextMenu"
        @item-click="onItemClick"
      />
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
            :class="[item.type, { overdue: item.isOverdue, done: item.type === 'card' && item.card && item.card.status === 'done' }]"
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
                @select-note="(note: any) => { selectNoteFromTree(note); showAddNoteDialog = false; }"
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

    <!-- Confirm Delete Calendar Modal -->
    <ConfirmModal
      :visible="deleteCalendarModal.visible"
      title="Delete Calendar"
      :message="deleteCalendarModal.message"
      confirm-text="Delete"
      variant="danger"
      @confirm="handleDeleteCalendar"
      @cancel="deleteCalendarModal.visible = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import { useRouter } from 'vue-router'
import Icon from '@/components/ui/Icon.vue'
import CalendarSidebarFilters from '@/components/calendar/CalendarSidebarFilters.vue'
import CalendarHeader from '@/components/calendar/CalendarHeader.vue'
import MonthGrid from '@/components/calendar/MonthGrid.vue'
import NoteTreeView from '@/components/ui/NoteTreeView.vue'
import WeekView from '@/components/calendar/WeekView.vue'
import ConfirmModal from '@/components/modals/ConfirmModal.vue'
import { boardsApi, listsApi, cardsApi, notesApi, remindersApi } from '@/api'
import { useSettingsStore } from '@/stores/settings'
import { useExplorerStore } from '@/stores/explorer'
import { useNotesStore } from '@/stores/notes'
import { useSync } from '@/composables/useSync'
import logger from '@/utils/logger'
import {
  dateToString,
  stringToDate,
  getWeekStart,
  formatTime,
  calculateDuration
} from '@/utils/calendar'
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

// Date helpers moved to src/utils/calendar.ts

// Create today's date at midnight local time to avoid timezone offset issues
const today = new Date()
today.setHours(0, 0, 0, 0)
const todayStr = dateToString(today)

const currentDate = ref<Date>(new Date(today))
const selectedDate = ref<string>(todayStr)
const showOverdue = ref(false)
const showDone = ref(false)
const showUpcoming = ref(false)

// Persistence
const CALENDAR_VIEW_KEY = 'viewState.calendar'

function loadCalendarViewState() {
  try {
    const raw = localStorage.getItem(CALENDAR_VIEW_KEY)
    if (!raw) return
    const data = JSON.parse(raw)
    if (data.viewMode) viewMode.value = data.viewMode
    if (data.currentDate) currentDate.value = new Date(data.currentDate)
    if (data.selectedDate) selectedDate.value = data.selectedDate
    if (typeof data.showOverdue === 'boolean') showOverdue.value = data.showOverdue
    if (typeof data.showDone === 'boolean') showDone.value = data.showDone
    if (typeof data.showUpcoming === 'boolean') showUpcoming.value = data.showUpcoming
    if (typeof data.sidebarCollapsed === 'boolean') sidebarCollapsed.value = data.sidebarCollapsed
    if (typeof data.showBoardsList === 'boolean') showBoardsList.value = data.showBoardsList
    if (Array.isArray(data.selectedBoardIds)) selectedBoardIds.value = data.selectedBoardIds
  } catch (e) {
    // ignore
  }
}

function saveCalendarViewState() {
  try {
    const data = {
      viewMode: viewMode.value,
      currentDate: currentDate.value ? currentDate.value.toISOString() : null,
      selectedDate: selectedDate.value,
      showOverdue: showOverdue.value,
      showDone: showDone.value,
      showUpcoming: showUpcoming.value,
      sidebarCollapsed: sidebarCollapsed.value,
      showBoardsList: showBoardsList.value,
      selectedBoardIds: selectedBoardIds.value
    }
    localStorage.setItem(CALENDAR_VIEW_KEY, JSON.stringify(data))
  } catch (e) {
    // ignore
  }
}

// (mini calendar removed from this view; a compact calendar lives in the sidebar component)

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

// Delete calendar confirmation
const deleteCalendarModal = ref({
  visible: false,
  calendarId: '',
  message: ''
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

// Hours for week view - show all 24 hours

// Display all 24 hours (0-23)
const displayHours = computed(() => {
  return Array.from({ length: 24 }, (_, i) => i)
})

// weekBodyRef removed; week scrolling handled inside WeekView component

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

// (mini calendar day names moved into sidebar component)
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
  return showOverdue.value || showDone.value || showUpcoming.value || selectedBoardIds.value.length < boards.value.length
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
      // Treat cards marked as done as not overdue so they don't render with the red overdue style
      isOverdue: dueDate < today && !card.archived && card.status !== 'done',
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
    if (showOverdue.value || showDone.value || showUpcoming.value) {
      filteredItems = dayItems.filter(item => {
        if (showOverdue.value && item.isOverdue) return true
        // When 'Done' filter is active, include cards with status 'done'
        if (showDone.value && item.type === 'card' && item.card && item.card.status === 'done') return true
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

// Methods (week start logic moved to utils/calendar.ts)

// Mini calendar methods
// Mini calendar helpers removed — sidebar component handles its own picker and state

// Date selection methods

function selectDate(dateStr: string) {
  selectedDate.value = dateStr
  const newDate = stringToDate(dateStr)
  currentDate.value = newDate
}

// Duration and time formatting moved to utils/calendar.ts

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
  selectedDate.value = dateToString(today)
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
  showDone.value = false
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

const { withSync } = useSync()

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
    return withSync(async () => {
      try {
        await remindersApi.create({
          note_id: note.id,
          title: note.name || note.title,
          date: contextMenu.value.selectedDate,
          time: '09:00',
          remind_before: '0'
        })
      } catch (error) {
        logger.error('Failed to save note link to calendar:', error)
        // Remove from local array if save fails
        const index = noteReminders.value.indexOf(reminder)
        if (index > -1) {
          noteReminders.value.splice(index, 1)
        }
        alert('Failed to save note to calendar')
      }
    })
  }
  
  showAddNoteDialog.value = false
}

async function saveReminder() {
  if (!reminderForm.value.title) {
    alert('Please enter a reminder title')
    return
  }
  
  return withSync(async () => {
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
      logger.error('Failed to save reminder:', error)
      alert('Failed to save reminder')
    }
  })
}

async function saveNote() {
  if (!noteForm.value.title) {
    alert('Please enter a note title')
    return
  }
  
  return withSync(async () => {
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
      logger.error('Failed to save note:', error)
      alert('Failed to create note')
    }
  })
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
  return withSync(async () => {
    try {
      if (editingCalendar.value) {
        await settingsStore.updateIcsCalendar(editingCalendar.value.id, {
          name: calendarForm.value.name,
          url: calendarForm.value.url,
          color: calendarForm.value.color
        })
      } else {
        await settingsStore.addIcsCalendar(
          calendarForm.value.name,
          calendarForm.value.url,
          calendarForm.value.color
        )
      }
      closeCalendarModal()
    } catch (err) {
      logger.error('Failed to save calendar:', err)
      alert('Failed to save calendar. Please check the URL and try again.')
    } finally {
      calendarFormLoading.value = false
    }
  })
}

async function refreshCalendar(calendarId: string) {
  const calendar = icsCalendars.value.find(c => c.id === calendarId)
  if (calendar) {
    return withSync(async () => {
      try {
        await settingsStore.fetchIcsCalendar(calendar)
      } catch (err) {
        logger.error('Failed to refresh calendar:', err)
      }
    })
  }
}

function deleteCalendar(calendarId: string) {
  const calendar = icsCalendars.value.find(c => c.id === calendarId)
  if (calendar) {
    deleteCalendarModal.value = {
      visible: true,
      calendarId,
      message: `Are you sure you want to delete the calendar "${calendar.name}"?`
    }
  }
}

async function handleDeleteCalendar() {
  if (deleteCalendarModal.value.calendarId) {
    await withSync(async () => {
      await settingsStore.removeIcsCalendar(deleteCalendarModal.value.calendarId)
    })
  }
  deleteCalendarModal.value.visible = false
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
    logger.error('Failed to load reminders:', err)
  }
}

// Lifecycle
onMounted(async () => {
  return withSync(async () => {
    try {
      // Restore UI state saved from previous visit
      loadCalendarViewState()

      // Fetch boards
      const boardsRes = await boardsApi.getAll()
      boards.value = boardsRes.data
      // If no selection was restored, default to all boards
      if (!selectedBoardIds.value || selectedBoardIds.value.length === 0) {
        selectedBoardIds.value = boards.value.map(b => b.id)
      }

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
              logger.error(`Failed to load lists for board ${board.id}:`, err)
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
                logger.error(`Failed to load cards for list ${list.id}:`, err)
                return [] as CardWithBoard[]
              })
            cardFetchPromises.push(p)
          }
        }

        const cardsArrays = await Promise.all(cardFetchPromises)
        cards.value = cardsArrays.flat()
      } catch (err) {
        logger.error('Failed to load cards for calendar:', err)
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
      
      // WeekView handles its own scroll position; no action needed here
      
      // Add click listener to close context menu
      document.addEventListener('click', closeContextMenu)
    } catch (err) {
      logger.error('Failed to load calendar data:', err)
    }
  })
})

onUnmounted(() => {
  // Persist calendar UI state
  saveCalendarViewState()
  document.removeEventListener('click', closeContextMenu)
})

// scrollToDefaultHour removed; WeekView handles scrolling internally

// Watch for view mode changes
watch(viewMode, () => {
  // Reset to today when switching views
  currentDate.value = new Date()
})

</script>

<style>
/* Component-specific overrides if needed */
</style>
