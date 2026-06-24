import { describe, expect, it } from 'vitest'
import {
  calculateDuration,
  dateToString,
  formatHour,
  getWeekStart,
  stringToDate,
} from '../calendar'

describe('calendar util', () => {
  it('dateToString and stringToDate round-trip calendar days', () => {
    const date = new Date(2025, 11, 3)
    expect(dateToString(date)).toBe('2025-12-03')
    expect(stringToDate('2025-12-03')).toEqual(new Date(2025, 11, 3))
  })

  it('getWeekStart respects weekStartsOnMonday', () => {
    const wednesday = new Date(2025, 11, 3)
    const weekStart = getWeekStart(wednesday, true)
    expect(weekStart.getDay()).toBe(1)
    expect(dateToString(weekStart)).toBe('2025-12-01')
  })

  it('getWeekStart can start weeks on Sunday', () => {
    const wednesday = new Date(2025, 11, 3)
    const weekStart = getWeekStart(wednesday, false)
    expect(weekStart.getDay()).toBe(0)
    expect(dateToString(weekStart)).toBe('2025-11-30')
  })

  it('calculateDuration formats hours and minutes', () => {
    const start = new Date(2025, 0, 1, 9, 0)
    const end = new Date(2025, 0, 1, 10, 30)
    expect(calculateDuration(start, end, false)).toBe('1h 30m')
  })

  it('calculateDuration returns empty string for all-day events', () => {
    const start = new Date(2025, 0, 1, 9, 0)
    const end = new Date(2025, 0, 1, 10, 0)
    expect(calculateDuration(start, end, true)).toBe('')
  })

  it('formatHour renders 12-hour clock labels', () => {
    expect(formatHour(0)).toBe('12 AM')
    expect(formatHour(12)).toBe('12 PM')
    expect(formatHour(9)).toBe('9 AM')
    expect(formatHour(15)).toBe('3 PM')
  })
})
