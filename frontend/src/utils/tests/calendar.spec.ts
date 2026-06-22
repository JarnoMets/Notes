import { describe, expect, it } from 'vitest'
import {
  calculateDuration,
  dateToString,
  formatHour,
  getWeekStart,
  stringToDate,
} from '../calendar'

describe('calendar utils', () => {
  it('round-trips dates through string format', () => {
    const date = new Date(2025, 5, 15)
    expect(dateToString(date)).toBe('2025-06-15')
    expect(stringToDate('2025-06-15').getDate()).toBe(15)
  })

  it('finds week start on Monday by default', () => {
    // Wednesday 2025-06-18
    const wednesday = new Date(2025, 5, 18)
    const weekStart = getWeekStart(wednesday, true)
    expect(dateToString(weekStart)).toBe('2025-06-16')
  })

  it('formats hour labels for AM and PM', () => {
    expect(formatHour(0)).toBe('12 AM')
    expect(formatHour(12)).toBe('12 PM')
    expect(formatHour(15)).toBe('3 PM')
  })

  it('calculates duration between datetimes', () => {
    const start = new Date(2025, 5, 18, 10, 0)
    const end = new Date(2025, 5, 18, 11, 30)
    expect(calculateDuration(start, end, false)).toBe('1h 30m')
    expect(calculateDuration(start, end, true)).toBe('')
  })
})
