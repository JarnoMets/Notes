import { describe, it, expect } from 'vitest'
import {
  dateToString,
  stringToDate,
  getWeekStart,
  formatHour,
  calculateDuration,
} from '../calendar'

describe('calendar util', () => {
  it('dateToString and stringToDate round-trip', () => {
    const date = new Date(2025, 5, 15)
    expect(dateToString(date)).toBe('2025-06-15')
    const parsed = stringToDate('2025-06-15')
    expect(parsed.getFullYear()).toBe(2025)
    expect(parsed.getMonth()).toBe(5)
    expect(parsed.getDate()).toBe(15)
  })

  it('getWeekStart returns Monday when weekStartsOnMonday is true', () => {
    const wednesday = new Date(2025, 5, 18)
    const weekStart = getWeekStart(wednesday, true)
    expect(weekStart.getDay()).toBe(1)
    expect(dateToString(weekStart)).toBe('2025-06-16')
  })

  it('getWeekStart returns Sunday when weekStartsOnMonday is false', () => {
    const wednesday = new Date(2025, 5, 18)
    const weekStart = getWeekStart(wednesday, false)
    expect(weekStart.getDay()).toBe(0)
    expect(dateToString(weekStart)).toBe('2025-06-15')
  })

  it('formatHour handles noon and midnight', () => {
    expect(formatHour(0)).toBe('12 AM')
    expect(formatHour(12)).toBe('12 PM')
    expect(formatHour(9)).toBe('9 AM')
    expect(formatHour(15)).toBe('3 PM')
  })

  it('calculateDuration formats minutes and hours', () => {
    const start = new Date(2025, 0, 1, 10, 0)
    expect(calculateDuration(start, new Date(2025, 0, 1, 10, 45), false)).toBe('45m')
    expect(calculateDuration(start, new Date(2025, 0, 1, 11, 0), false)).toBe('1h')
    expect(calculateDuration(start, new Date(2025, 0, 1, 11, 30), false)).toBe('1h 30m')
    expect(calculateDuration(start, new Date(2025, 0, 2, 10, 0), true)).toBe('')
  })
})
