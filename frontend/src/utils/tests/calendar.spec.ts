import { describe, it, expect } from 'vitest'
import {
  dateToString,
  stringToDate,
  getWeekStart,
  formatHour,
  getDayName,
  calculateDuration,
} from '../calendar'

describe('calendar util', () => {
  it('dateToString formats as YYYY-MM-DD', () => {
    expect(dateToString(new Date(2025, 5, 7))).toBe('2025-06-07')
  })

  it('stringToDate parses YYYY-MM-DD to local midnight', () => {
    const date = stringToDate('2025-06-07')
    expect(date.getFullYear()).toBe(2025)
    expect(date.getMonth()).toBe(5)
    expect(date.getDate()).toBe(7)
    expect(date.getHours()).toBe(0)
  })

  it('getWeekStart returns Monday when weekStartsOnMonday is true', () => {
    // Wednesday 2025-06-04 → Monday 2025-06-02
    const wednesday = new Date(2025, 5, 4)
    const weekStart = getWeekStart(wednesday, true)
    expect(dateToString(weekStart)).toBe('2025-06-02')
  })

  it('getWeekStart returns Sunday when weekStartsOnMonday is false', () => {
    // Wednesday 2025-06-04 → Sunday 2025-06-01
    const wednesday = new Date(2025, 5, 4)
    const weekStart = getWeekStart(wednesday, false)
    expect(dateToString(weekStart)).toBe('2025-06-01')
  })

  it('formatHour converts 24-hour clock to 12-hour labels', () => {
    expect(formatHour(0)).toBe('12 AM')
    expect(formatHour(12)).toBe('12 PM')
    expect(formatHour(9)).toBe('9 AM')
    expect(formatHour(15)).toBe('3 PM')
  })

  it('getDayName returns abbreviated weekday', () => {
    expect(getDayName('2025-06-07')).toBe('Sat')
    expect(getDayName('2025-06-02')).toBe('Mon')
  })

  it('calculateDuration returns empty string for all-day events', () => {
    const start = new Date(2025, 5, 7, 9, 0)
    const end = new Date(2025, 5, 7, 10, 30)
    expect(calculateDuration(start, end, true)).toBe('')
  })

  it('calculateDuration formats minutes under one hour', () => {
    const start = new Date(2025, 5, 7, 9, 0)
    const end = new Date(2025, 5, 7, 9, 45)
    expect(calculateDuration(start, end, false)).toBe('45m')
  })

  it('calculateDuration formats hours and minutes', () => {
    const start = new Date(2025, 5, 7, 9, 0)
    const end = new Date(2025, 5, 7, 11, 30)
    expect(calculateDuration(start, end, false)).toBe('2h 30m')
  })

  it('calculateDuration formats whole hours without minutes', () => {
    const start = new Date(2025, 5, 7, 9, 0)
    const end = new Date(2025, 5, 7, 11, 0)
    expect(calculateDuration(start, end, false)).toBe('2h')
  })
})
