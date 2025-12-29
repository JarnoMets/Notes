<template>
  <Teleport to="body">
    <div v-if="isOpen" class="modal-overlay" @click.self="close">
      <div class="modal modal-large">
        <div class="modal-header">
          <h2>Settings</h2>
          <button class="modal-close" @click="close">
            <Icon name="x" />
          </button>
        </div>

        <div class="modal-body">
          <!-- Calendar Settings -->
          <section class="settings-section">
            <h3>Calendar</h3>
            
            <label class="setting-item checkbox">
              <input 
                type="checkbox" 
                :checked="weekStartsOnMonday"
                @change="toggleWeekStart"
              />
              <span>Week starts on Monday</span>
            </label>
          </section>

          <!-- ICS Calendars -->
          <section class="settings-section">
            <div class="section-header">
              <h3>ICS Calendars</h3>
              <button class="btn btn-primary" @click="openCalendarModal()">
                <Icon name="plus" />
                Add Calendar
              </button>
            </div>

            <div v-if="icsCalendars.length === 0" class="empty-state">
              <p>No ICS calendars configured</p>
              <p class="hint">Add external calendars (Google, Outlook, etc.) by their ICS URL</p>
            </div>

            <div v-else class="calendar-list">
              <div 
                v-for="calendar in icsCalendars" 
                :key="calendar.id" 
                class="calendar-item"
              >
                <div class="calendar-color" :style="{ backgroundColor: calendar.color }"></div>
                <div class="calendar-info">
                  <span class="calendar-name">{{ calendar.name }}</span>
                  <span class="calendar-url">{{ calendar.url }}</span>
                </div>
                <label class="toggle">
                  <input 
                    type="checkbox" 
                    :checked="calendar.enabled"
                    @change="toggleCalendar(calendar.id)"
                  />
                  <span class="toggle-slider"></span>
                </label>
                <button class="btn-icon" @click="openCalendarModal(calendar)" title="Edit">
                  <Icon name="edit" />
                </button>
                <button class="btn-icon" @click="confirmDeleteCalendar(calendar)" title="Delete">
                  <Icon name="trash" />
                </button>
              </div>
            </div>
          </section>
        </div>
      </div>
    </div>

    <!-- Add/Edit Calendar Modal -->
    <div v-if="calendarModalOpen" class="modal-overlay" @click.self="closeCalendarModal">
      <div class="modal modal-container small">
        <div class="modal-header">
          <h2>{{ editingCalendar ? 'Edit Calendar' : 'Add ICS Calendar' }}</h2>
          <button class="modal-close" @click="closeCalendarModal">
            <Icon name="x" />
          </button>
        </div>

        <div class="modal-body">
          <div class="form-group">
            <label>Name</label>
            <input 
              v-model="calendarForm.name" 
              type="text" 
              placeholder="My Calendar"
              class="form-input"
            />
          </div>
          <div class="form-group">
            <label>ICS URL</label>
            <input 
              v-model="calendarForm.url" 
              type="url" 
              placeholder="https://calendar.google.com/..."
              class="form-input"
            />
          </div>
          <div class="form-group">
            <label>Color</label>
            <div class="color-picker">
              <button
                v-for="color in calendarColors"
                :key="color"
                class="color-option"
                :class="{ active: calendarForm.color === color }"
                :style="{ backgroundColor: color }"
                @click="calendarForm.color = color"
              ></button>
            </div>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn btn-secondary" @click="closeCalendarModal">Cancel</button>
          <button 
            class="btn btn-primary" 
            @click="saveCalendar"
            :disabled="!calendarForm.name || !calendarForm.url"
          >
            {{ editingCalendar ? 'Save' : 'Add' }}
          </button>
        </div>
      </div>
    </div>

    <!-- Confirm Delete Modal -->
    <ConfirmModal
      :visible="deleteConfirmOpen"
      title="Delete Calendar"
      :message="`Are you sure you want to delete '${calendarToDelete?.name}'?`"
      confirm-text="Delete"
      variant="danger"
      @confirm="deleteCalendar"
      @cancel="deleteConfirmOpen = false"
    />
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, reactive } from 'vue'
import { useSettingsStore, type IcsCalendar } from '@/stores/settings'
import Icon from '../ui/Icon.vue'
import ConfirmModal from './ConfirmModal.vue'

defineProps<{
  isOpen: boolean
}>()

const emit = defineEmits<{
  close: []
}>()

const settingsStore = useSettingsStore()

const weekStartsOnMonday = computed(() => settingsStore.weekStartsOnMonday)
const icsCalendars = computed(() => settingsStore.icsCalendars)

const calendarColors = [
  '#e74c3c', '#3498db', '#2ecc71', '#f39c12', '#9b59b6', 
  '#1abc9c', '#e67e22', '#34495e', '#fd79a8', '#00cec9'
]

// Calendar modal state
const calendarModalOpen = ref(false)
const editingCalendar = ref<IcsCalendar | null>(null)
const calendarForm = reactive({
  name: '',
  url: '',
  color: calendarColors[0]
})

// Delete confirmation
const deleteConfirmOpen = ref(false)
const calendarToDelete = ref<IcsCalendar | null>(null)

function close() {
  emit('close')
}

async function toggleWeekStart() {
  await settingsStore.setWeekStartsOnMonday(!weekStartsOnMonday.value)
}

async function toggleCalendar(calendarId: string) {
  await settingsStore.toggleIcsCalendar(calendarId)
}

function openCalendarModal(calendar?: IcsCalendar) {
  if (calendar) {
    editingCalendar.value = calendar
    calendarForm.name = calendar.name
    calendarForm.url = calendar.url
    calendarForm.color = calendar.color
  } else {
    editingCalendar.value = null
    calendarForm.name = ''
    calendarForm.url = ''
    calendarForm.color = calendarColors[icsCalendars.value.length % calendarColors.length]
  }
  calendarModalOpen.value = true
}

function closeCalendarModal() {
  calendarModalOpen.value = false
  editingCalendar.value = null
}

async function saveCalendar() {
  if (editingCalendar.value) {
    await settingsStore.updateIcsCalendar(editingCalendar.value.id, {
      name: calendarForm.name,
      url: calendarForm.url,
      color: calendarForm.color
    })
  } else {
    await settingsStore.addIcsCalendar(
      calendarForm.name,
      calendarForm.url,
      calendarForm.color
    )
  }
  closeCalendarModal()
}

function confirmDeleteCalendar(calendar: IcsCalendar) {
  calendarToDelete.value = calendar
  deleteConfirmOpen.value = true
}

async function deleteCalendar() {
  if (calendarToDelete.value) {
    await settingsStore.removeIcsCalendar(calendarToDelete.value.id)
  }
  deleteConfirmOpen.value = false
  calendarToDelete.value = null
}
</script>

<style scoped>
.modal-overlay {
  z-index: 1000;
}

.modal-container {
  width: 90%;
  max-width: 600px;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
}

.modal-container.small {
  max-width: 450px;
}

.modal-header h2 {
  font-size: 1.25rem;
  font-weight: 600;
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--border-primary);
}

.settings-section {
  margin-bottom: 2rem;
}

.settings-section:last-child {
  margin-bottom: 0;
}

.settings-section h3 {
  margin: 0 0 1rem;
  font-size: 0.9rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-secondary);
}

.section-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1rem;
}

.section-header h3 {
  margin: 0;
}

.setting-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 8px;
  cursor: pointer;
}

.setting-item:hover {
  background: var(--bg-hover);
}

.setting-item input[type="checkbox"] {
  width: 18px;
  height: 18px;
  accent-color: var(--accent);
}

.empty-state {
  text-align: center;
  padding: 2rem;
  background: var(--bg-tertiary);
  border-radius: 8px;
}

.empty-state p {
  margin: 0;
  color: var(--text-secondary);
}

.empty-state .hint {
  margin-top: 0.5rem;
  font-size: 0.875rem;
  opacity: 0.7;
}

.calendar-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.calendar-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem;
  background: var(--bg-tertiary);
  border-radius: 8px;
}

.calendar-color {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  flex-shrink: 0;
}

.calendar-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.calendar-name {
  font-weight: 500;
}

.calendar-url {
  font-size: 0.75rem;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.toggle {
  position: relative;
  display: inline-block;
  width: 40px;
  height: 22px;
  flex-shrink: 0;
}

.toggle input {
  opacity: 0;
  width: 0;
  height: 0;
}

.toggle-slider {
  position: absolute;
  cursor: pointer;
  inset: 0;
  background-color: var(--bg-hover);
  border-radius: 22px;
  transition: 0.2s;
}

.toggle-slider::before {
  position: absolute;
  content: "";
  height: 16px;
  width: 16px;
  left: 3px;
  bottom: 3px;
  background-color: white;
  border-radius: 50%;
  transition: 0.2s;
}

.toggle input:checked + .toggle-slider {
  background-color: var(--accent);
}

.toggle input:checked + .toggle-slider::before {
  transform: translateX(18px);
}

/* Form styles */
.form-group {
  margin-bottom: 1rem;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  font-size: 0.875rem;
}

.form-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--border-primary);
  border-radius: 6px;
  background: var(--bg-tertiary);
  color: var(--text-primary);
  font-size: 0.875rem;
}

.form-input:focus {
  outline: none;
  border-color: var(--accent);
}

.color-picker {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.color-option {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  border: 3px solid transparent;
  cursor: pointer;
  transition: transform 0.1s;
}

.color-option:hover {
  transform: scale(1.1);
}

.color-option.active {
  border-color: var(--text-primary);
}
</style>
