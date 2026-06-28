import { describe, it, expect } from 'vitest'
import {
  dateToString,
  stringToDate,
  getWeekStart,
  formatHour,
  calculateDuration,
  getDayName,
} from '../calendar'

describe('calendar util', () => {
  it('dateToString formats YYYY-MM-DD', () => {
    expect(dateToString(new Date(2025, 5, 15))).toBe('2025-06-15')
  })

  it('stringToDate parses date strings', () => {
    const d = stringToDate('2025-06-15')
    expect(d.getFullYear()).toBe(2025)
    expect(d.getMonth()).toBe(5)
    expect(d.getDate()).toBe(15)
  })

  it('getWeekStart returns Monday when weekStartsOnMonday is true', () => {
    const wed = new Date(2025, 5, 18) // Wed Jun 18 2025
    const start = getWeekStart(wed, true)
    expect(start.getDay()).toBe(1) // Monday
    expect(start.getDate()).toBe(16)
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
    const end45 = new Date(2025, 5, 15, 9, 45)
    expect(calculateDuration(start, end45, false)).toBe('45m')

    const end2h = new Date(2025, 5, 15, 11, 30)
    expect(calculateDuration(start, end2h, false)).toBe('2h 30m')
  })

  it('getDayName returns short weekday label', () => {
    expect(getDayName('2025-06-15')).toBe('Sun')
  })
})
