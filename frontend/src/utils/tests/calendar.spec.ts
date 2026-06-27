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
    const roundTrip = stringToDate('2025-06-15')
    expect(roundTrip.getFullYear()).toBe(2025)
    expect(roundTrip.getMonth()).toBe(5)
    expect(roundTrip.getDate()).toBe(15)
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

  it('calculateDuration returns empty for all-day events', () => {
    const start = new Date(2025, 5, 15, 9, 0)
    const end = new Date(2025, 5, 15, 10, 30)
    expect(calculateDuration(start, end, true)).toBe('')
  })

  it('calculateDuration formats minutes and hours', () => {
    const start = new Date(2025, 5, 15, 9, 0)
    expect(calculateDuration(start, new Date(2025, 5, 15, 9, 45), false)).toBe('45m')
    expect(calculateDuration(start, new Date(2025, 5, 15, 11, 0), false)).toBe('2h')
    expect(calculateDuration(start, new Date(2025, 5, 15, 10, 30), false)).toBe('1h 30m')
  })
})
