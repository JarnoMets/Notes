import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { settingsApi } from '@/api/settings'
import logger from '@/utils/logger'
import { useSyncStore } from '@/stores/sync'

export interface IcsCalendar {
  id: string
  name: string
  url: string
  color: string
  enabled: boolean
}

export interface CalendarEvent {
  id: string
  calendarId: string
  calendarName: string
  calendarColor: string
  title: string
  description?: string
  start: Date
  end?: Date
  allDay: boolean
  location?: string
}

export interface UserSettings {
  icsCalendars: IcsCalendar[]
  weekStartsOnMonday: boolean
}

const SETTINGS_KEY = 'notes_user_settings'

// Windows timezone to IANA timezone mapping
// Maps Windows timezone names to their IANA equivalents
const WINDOWS_TO_IANA_TIMEZONES: Record<string, string> = {
  // Romance Standard Time (Central European Time)
  'Romance Standard Time': 'Europe/Paris',
  // W. Europe Standard Time (Western European Time)
  'W. Europe Standard Time': 'Europe/London',
  // GMT Standard Time
  'GMT Standard Time': 'Europe/London',
  // Greenwich Standard Time
  'Greenwich Standard Time': 'Etc/GMT',
  // Eastern Standard Time
  'Eastern Standard Time': 'America/New_York',
  // Central Standard Time
  'Central Standard Time': 'America/Chicago',
  // Mountain Standard Time
  'Mountain Standard Time': 'America/Denver',
  // Pacific Standard Time
  'Pacific Standard Time': 'America/Los_Angeles',
  // AUS Eastern Standard Time
  'AUS Eastern Standard Time': 'Australia/Sydney',
  // Central Standard Time (Mexico)
  'Central Standard Time (Mexico)': 'America/Mexico_City',
  // Canada Central Standard Time
  'Canada Central Standard Time': 'America/Edmonton',
  // Atlantic Standard Time
  'Atlantic Standard Time': 'America/Halifax',
  // Newfoundland Standard Time
  'Newfoundland Standard Time': 'America/St_Johns',
}

// Default color palette for calendars
const calendarColors = [
  '#e74c3c', '#3498db', '#2ecc71', '#f39c12', '#9b59b6', 
  '#1abc9c', '#e67e22', '#34495e', '#fd79a8', '#00cec9'
]

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<UserSettings>(loadSettings())
  const syncStore = useSyncStore()
  const calendarEvents = ref<CalendarEvent[]>([])
  const loadingCalendars = ref(false)
  const calendarErrors = ref<Map<string, string>>(new Map())
  
  // Caching and deduplication
  const lastFetchTime = ref<Map<string, number>>(new Map())
  const fetchingCalendars = ref<Set<string>>(new Set())
  const initialized = ref(false)
  const serverInitialized = ref(false)
  const CACHE_DURATION = 5 * 60 * 1000 // 5 minutes

  function loadSettings(): UserSettings {
    try {
      const stored = localStorage.getItem(SETTINGS_KEY)
      if (stored) {
        return JSON.parse(stored)
      }
    } catch (e) {
      logger.error('Failed to load settings:', e)
    }
    return { icsCalendars: [], weekStartsOnMonday: true }
  }

  function saveSettings() {
    localStorage.setItem(SETTINGS_KEY, JSON.stringify(settings.value))
  }

  async function withSync<R>(action: () => Promise<R>): Promise<R> {
    syncStore.startSync()
    try {
      return await action()
    } finally {
      syncStore.endSync()
    }
  }

  // Save settings to server
  async function saveToServer() {
    const token = localStorage.getItem('notes_auth_token')
    if (!token) return
    
    return withSync(async () => {
      try {
        await settingsApi.update({
          week_starts_on_monday: settings.value.weekStartsOnMonday,
          ics_calendars: settings.value.icsCalendars.map(c => ({
            id: c.id,
            name: c.name,
            url: c.url,
            color: c.color,
            enabled: c.enabled
          }))
        })
      } catch (e) {
        logger.error('Failed to save settings to server:', e)
        throw e
      }
    })
  }

  // Load settings from server
  async function loadFromServer() {
    const token = localStorage.getItem('notes_auth_token')
    if (!token || serverInitialized.value) return
    
    return withSync(async () => {
      try {
        const response = await settingsApi.get()
        const data = response.data
        
        // Merge server settings with local settings
        settings.value.weekStartsOnMonday = data.week_starts_on_monday
        
        // Only use server calendars if we have them
        if (data.ics_calendars && data.ics_calendars.length > 0) {
          settings.value.icsCalendars = data.ics_calendars.map(c => ({
            id: c.id,
            name: c.name,
            url: c.url,
            color: c.color,
            enabled: c.enabled
          }))
        }
        
        saveSettings()
        serverInitialized.value = true
      } catch (e) {
        logger.error('Failed to load settings from server:', e)
        throw e
      }
    })
  }

  async function addIcsCalendar(name: string, url: string, color?: string): Promise<IcsCalendar> {
    const newCalendar: IcsCalendar = {
      id: `ics-${Date.now()}`,
      name,
      url,
      color: color || calendarColors[settings.value.icsCalendars.length % calendarColors.length],
      enabled: true
    }
    settings.value.icsCalendars.push(newCalendar)
    saveSettings()
    await saveToServer()
    // Fetch events for the new calendar
    fetchIcsCalendar(newCalendar)
    return newCalendar
  }

  async function updateIcsCalendar(id: string, updates: Partial<Omit<IcsCalendar, 'id'>>) {
    const calendar = settings.value.icsCalendars.find(c => c.id === id)
    if (calendar) {
      Object.assign(calendar, updates)
      saveSettings()
      await saveToServer()
      // Re-fetch if URL changed or if enabled
      if (updates.url || updates.enabled) {
        fetchIcsCalendar(calendar)
      }
    }
  }

  async function removeIcsCalendar(id: string) {
    settings.value.icsCalendars = settings.value.icsCalendars.filter(c => c.id !== id)
    calendarEvents.value = calendarEvents.value.filter(e => e.calendarId !== id)
    calendarErrors.value.delete(id)
    saveSettings()
    await saveToServer()
  }

  async function toggleIcsCalendar(id: string) {
    const calendar = settings.value.icsCalendars.find(c => c.id === id)
    if (calendar) {
      calendar.enabled = !calendar.enabled
      saveSettings()
      await saveToServer()
      // Only fetch if enabling and not already cached
      if (calendar.enabled) {
        const lastFetch = lastFetchTime.value.get(id)
        // Only fetch if no cache exists
        if (!lastFetch) {
          fetchIcsCalendar(calendar)
        }
      }
      // Don't remove events when disabling - they're filtered out in the getters
    }
  }
// ...existing code...
  function setWeekStartsOnMonday(value: boolean) {
    settings.value.weekStartsOnMonday = value
    saveSettings()
    saveToServer()
  }

  // Parse ICS file content
  function parseIcs(icsContent: string, calendar: IcsCalendar): CalendarEvent[] {
    const events: CalendarEvent[] = []
    const lines = icsContent.split(/\r?\n/)
    
    let currentEvent: Partial<CalendarEvent> | null = null

    for (let i = 0; i < lines.length; i++) {
      let line = lines[i]
      
      // Handle line folding (lines starting with space or tab are continuations)
      while (i + 1 < lines.length && (lines[i + 1].startsWith(' ') || lines[i + 1].startsWith('\t'))) {
        i++
        line += lines[i].substring(1)
      }

      if (line.startsWith('BEGIN:VEVENT')) {
        currentEvent = {
          id: '',
          calendarId: calendar.id,
          calendarName: calendar.name,
          calendarColor: calendar.color,
          title: '',
          allDay: false
        }
      } else if (line.startsWith('END:VEVENT') && currentEvent) {
        if (currentEvent.title && currentEvent.start) {
          currentEvent.id = `${calendar.id}-${currentEvent.start.getTime()}-${events.length}`
          events.push(currentEvent as CalendarEvent)
        }
        currentEvent = null
      } else if (currentEvent) {
        const colonIndex = line.indexOf(':')
        if (colonIndex > -1) {
          const keyPart = line.substring(0, colonIndex)
          const value = line.substring(colonIndex + 1)
          
          // Extract the key without parameters
          const key = keyPart.split(';')[0]
          
          switch (key) {
            case 'SUMMARY':
              currentEvent.title = unescapeIcsText(value)
              break
            case 'DESCRIPTION':
              currentEvent.description = unescapeIcsText(value)
              break
            case 'LOCATION':
              currentEvent.location = unescapeIcsText(value)
              break
            case 'UID':
              currentEvent.id = `${calendar.id}-${value}`
              break
            case 'DTSTART':
              const startResult = parseIcsDate(value, keyPart)
              currentEvent.start = startResult.date
              currentEvent.allDay = startResult.allDay
              break
            case 'DTEND':
              const endResult = parseIcsDate(value, keyPart)
              currentEvent.end = endResult.date
              break
          }
        }
      }
    }

    return events
  }

  function parseIcsDate(value: string, keyPart: string): { date: Date; allDay: boolean } {
    let allDay = false
    
    // Check if it's a date-only value (all day event)
    if (keyPart.includes('VALUE=DATE') || value.length === 8) {
      allDay = true
      // Parse YYYYMMDD
      const year = parseInt(value.substring(0, 4))
      const month = parseInt(value.substring(4, 6)) - 1
      const day = parseInt(value.substring(6, 8))
      return { date: new Date(year, month, day), allDay }
    }
    
    // Parse YYYYMMDDTHHMMSS or YYYYMMDDTHHMMSSZ
    const year = parseInt(value.substring(0, 4))
    const month = parseInt(value.substring(4, 6)) - 1
    const day = parseInt(value.substring(6, 8))
    const hour = parseInt(value.substring(9, 11)) || 0
    const minute = parseInt(value.substring(11, 13)) || 0
    const second = parseInt(value.substring(13, 15)) || 0
    
    if (value.endsWith('Z')) {
      // UTC time - convert to local
      return { date: new Date(Date.UTC(year, month, day, hour, minute, second)), allDay }
    }
    
    // Check for TZID parameter (e.g., DTSTART;TZID=America/New_York:20231215T140000)
    const tzidMatch = keyPart.match(/TZID=([^;:]+)/i)
    if (tzidMatch) {
      try {
        let tzName = tzidMatch[1]

        // Convert Windows timezone names to IANA timezone names
        if (tzName in WINDOWS_TO_IANA_TIMEZONES) {
          tzName = WINDOWS_TO_IANA_TIMEZONES[tzName as keyof typeof WINDOWS_TO_IANA_TIMEZONES]
        }

        // Use Intl.DateTimeFormat.formatToParts to robustly compute the offset
        // Create a UTC timestamp for the provided components
        const utcForGiven = Date.UTC(year, month, day, hour, minute, second)

        // Formatter that will output the date/time parts for the target timezone
        const fmt = new Intl.DateTimeFormat('en-US', {
          timeZone: tzName,
          year: 'numeric',
          month: '2-digit',
          day: '2-digit',
          hour: '2-digit',
          minute: '2-digit',
          second: '2-digit',
          hour12: false
        })

        const parts = fmt.formatToParts(new Date(utcForGiven))
        const partMap: Record<string, string> = {}
        for (const p of parts) {
          if (p.type && p.value) partMap[p.type] = p.value
        }

        const tzYear = Number(partMap.year)
        const tzMonth = Number(partMap.month)
        const tzDay = Number(partMap.day)
        const tzHour = Number(partMap.hour)
        const tzMinute = Number(partMap.minute)
        const tzSecond = Number(partMap.second)

        // Build a UTC timestamp from the timezone-formatted parts
        const tzAsUtc = Date.UTC(tzYear, tzMonth - 1, tzDay, tzHour, tzMinute, tzSecond)

        // The tzAsUtc is the UTC timestamp corresponding to the formatted parts we got
        // (i.e. the UTC instant that, when formatted in the target timezone, yields those parts).
        // To get the correct UTC instant for the original local components we started with
        // we compute: target = 2*utcForGiven - tzAsUtc. This derives from correcting the
        // initial UTC guess by the difference between the guessed and actual timezone mapping.
        const targetTimestamp = 2 * utcForGiven - tzAsUtc
        return { date: new Date(targetTimestamp), allDay }
      } catch (e) {
        // If timezone parsing fails, treat as local time
        logger.warn(`Failed to parse timezone ${tzidMatch[1]}, using local time`)
        return { date: new Date(year, month, day, hour, minute, second), allDay }
      }
    }
    
    // Local time (no timezone specified)
    return { date: new Date(year, month, day, hour, minute, second), allDay }
  }

  function unescapeIcsText(text: string): string {
    return text
      .replace(/\\n/g, '\n')
      .replace(/\\,/g, ',')
      .replace(/\\;/g, ';')
      .replace(/\\\\/g, '\\')
  }

  // List of CORS proxies to try
  const corsProxies = [
    (url: string) => `https://corsproxy.io/?${encodeURIComponent(url)}`,
    (url: string) => `https://api.codetabs.com/v1/proxy?quest=${encodeURIComponent(url)}`,
    (url: string) => `https://proxy.cors.sh/${url}`,
  ]

  async function fetchWithCorsProxy(url: string): Promise<Response> {
    // Try direct fetch first
    try {
      const response = await fetch(url, { mode: 'cors' })
      if (response.ok) return response
    } catch (e) {
      // Direct fetch failed, try proxies
    }

    // Try each proxy in order
    for (const proxyFn of corsProxies) {
      try {
        const proxyUrl = proxyFn(url)
        const response = await fetch(proxyUrl)
        if (response.ok) return response
      } catch (e) {
        // This proxy failed, try next
        continue
      }
    }

    throw new Error('All CORS proxies failed')
  }

  async function fetchIcsCalendar(calendar: IcsCalendar, force = false) {
    if (!calendar.enabled) return
    
    // Prevent duplicate fetches
    if (fetchingCalendars.value.has(calendar.id)) {
      return
    }
    
    // Check cache
    const lastFetch = lastFetchTime.value.get(calendar.id)
    if (!force && lastFetch && Date.now() - lastFetch < CACHE_DURATION) {
      return // Use cached data
    }

    calendarErrors.value.delete(calendar.id)
    fetchingCalendars.value.add(calendar.id)
    
    try {
      const response = await fetchWithCorsProxy(calendar.url)
      
      if (!response.ok) {
        throw new Error(`Failed to fetch calendar: ${response.statusText}`)
      }
      
      const icsContent = await response.text()
      const events = parseIcs(icsContent, calendar)
      
      // Remove old events from this calendar and add new ones
      calendarEvents.value = [
        ...calendarEvents.value.filter(e => e.calendarId !== calendar.id),
        ...events
      ]
      
      // Update cache time
      lastFetchTime.value.set(calendar.id, Date.now())
    } catch (error) {
      logger.error(`Failed to fetch ICS calendar ${calendar.name}:`, error)
      calendarErrors.value.set(calendar.id, error instanceof Error ? error.message : 'Failed to fetch calendar')
    } finally {
      fetchingCalendars.value.delete(calendar.id)
    }
  }

  async function fetchAllIcsCalendars(force = false) {
    // Prevent duplicate initialization
    if (!force && initialized.value && loadingCalendars.value) {
      return
    }
    
    loadingCalendars.value = true
    try {
      await Promise.all(
        settings.value.icsCalendars
          .filter(c => c.enabled)
          .map(c => fetchIcsCalendar(c, force))
      )
      initialized.value = true
    } finally {
      loadingCalendars.value = false
    }
  }
  
  // Initialize calendars in background (call from App.vue)
  function initializeCalendars() {
    if (!initialized.value) {
      fetchAllIcsCalendars()
    }
  }

  // Get enabled calendar IDs for filtering
  const enabledCalendarIds = computed(() => 
    new Set(settings.value.icsCalendars.filter(c => c.enabled).map(c => c.id))
  )

  // Get events for a specific date range (only from enabled calendars)
  const getEventsForDateRange = computed(() => {
    return (startDate: Date, endDate: Date) => {
      return calendarEvents.value.filter(event => {
        if (!enabledCalendarIds.value.has(event.calendarId)) return false
        const eventStart = new Date(event.start)
        const eventEnd = event.end ? new Date(event.end) : eventStart
        return eventStart <= endDate && eventEnd >= startDate
      })
    }
  })

  // Get events for a specific day (only from enabled calendars)
  const getEventsForDay = computed(() => {
    return (date: Date) => {
      const dayStart = new Date(date.getFullYear(), date.getMonth(), date.getDate())
      const dayEnd = new Date(date.getFullYear(), date.getMonth(), date.getDate(), 23, 59, 59)
      
      return calendarEvents.value.filter(event => {
        if (!enabledCalendarIds.value.has(event.calendarId)) return false
        const eventStart = new Date(event.start)
        const eventEnd = event.end ? new Date(event.end) : eventStart
        
        // For all-day events, compare just the dates
        if (event.allDay) {
          const eventStartDate = new Date(eventStart.getFullYear(), eventStart.getMonth(), eventStart.getDate())
          const eventEndDate = event.end 
            ? new Date(eventEnd.getFullYear(), eventEnd.getMonth(), eventEnd.getDate())
            : eventStartDate
          return eventStartDate <= dayEnd && eventEndDate >= dayStart
        }
        
        return eventStart <= dayEnd && eventEnd >= dayStart
      })
    }
  })

  // Computed for easy access to ICS calendars
  const icsCalendars = computed(() => settings.value.icsCalendars)
  
  // Week start setting
  const weekStartsOnMonday = computed(() => settings.value.weekStartsOnMonday)
  
  async function setWeekStartsOnMonday(value: boolean) {
    settings.value.weekStartsOnMonday = value
    saveSettings()
    await saveToServer()
  }

  return {
    settings,
    icsCalendars,
    weekStartsOnMonday,
    calendarEvents,
    loadingCalendars,
    calendarErrors,
    initialized,
    serverInitialized,
    addIcsCalendar,
    updateIcsCalendar,
    removeIcsCalendar,
    toggleIcsCalendar,
    fetchIcsCalendar,
    fetchAllIcsCalendars,
    initializeCalendars,
    loadFromServer,
    getEventsForDateRange,
    getEventsForDay,
    setWeekStartsOnMonday
  }
})
