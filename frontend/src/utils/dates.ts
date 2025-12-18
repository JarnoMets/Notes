// Utility helpers for handling date/time values for inputs and API
export function pad(n: number) {
  return n < 10 ? `0${n}` : `${n}`
}

// Returns a string suitable for <input type="datetime-local">: YYYY-MM-DDTHH:mm
export function toDatetimeLocal(date: Date): string {
  const y = date.getFullYear()
  const m = pad(date.getMonth() + 1)
  const d = pad(date.getDate())
  const hh = pad(date.getHours())
  const mm = pad(date.getMinutes())
  return `${y}-${m}-${d}T${hh}:${mm}`
}

// Ensure a date string used for the datetime-local input has a sensible default time.
// If the incoming value is a date-only string (YYYY-MM-DD) we append 18:00.
// If it's an ISO with timezone, convert to local datetime-local format.
export function normalizeForInput(value?: string): string | undefined {
  if (!value) return undefined

  // Date-only (YYYY-MM-DD)
  if (/^\d{4}-\d{2}-\d{2}$/.test(value)) {
    return `${value}T18:00`
  }

  // If it's already in datetime-local like format (YYYY-MM-DDTHH:mm) keep, but if time is 00:00 default to 18:00
  const dtMatch = value.match(/^(\d{4}-\d{2}-\d{2})T(\d{2}):(\d{2})/) 
  if (dtMatch) {
    const hour = parseInt(dtMatch[2], 10)
    const min = parseInt(dtMatch[3], 10)
    if (hour === 0 && min === 0) {
      return `${dtMatch[1]}T18:00`
    }
    // If it's already a local-like string, just trim seconds/offset if present
    return `${dtMatch[1]}T${dtMatch[2]}:${dtMatch[3]}`
  }

  // Otherwise, try parsing as ISO and convert to local datetime-local
  const parsed = new Date(value)
  if (isNaN(parsed.getTime())) return undefined
  return toDatetimeLocal(parsed)
}

// Convert the datetime-local (or other) input value to a full ISO string with timezone
// suitable for the backend DateTime<Utc> parsing. Returns undefined if input is falsy.
export function toApiIso(value?: string): string | undefined {
  if (!value) return undefined
  // Date-only -> append time
  if (/^\d{4}-\d{2}-\d{2}$/.test(value)) {
    value = `${value}T18:00`
  }

  // If value looks like YYYY-MM-DDTHH:mm (no offset), Date will interpret as local time — that's desired.
  const d = new Date(value)
  if (isNaN(d.getTime())) return undefined
  return d.toISOString()
}
