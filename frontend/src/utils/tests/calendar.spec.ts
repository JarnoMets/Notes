import { describe, it, expect } from 'vitest'
import { getWeekStart, calculateDuration } from '../calendar'

describe('calendar util', () => {
  it('getWeekStart returns Monday for a Wednesday when week starts on Monday', () => {
    const wednesday = new Date(2025, 5, 18) // 2025-06-18
    const weekStart = getWeekStart(wednesday, true)
    expect(weekStart.getDay()).toBe(1)
    expect(weekStart.getDate()).toBe(16)
  })

  it('getWeekStart returns Sunday when week starts on Sunday', () => {
    const wednesday = new Date(2025, 5, 18)
    const weekStart = getWeekStart(wednesday, false)
    expect(weekStart.getDay()).toBe(0)
    expect(weekStart.getDate()).toBe(15)
  })

  it('calculateDuration returns empty string for all-day events', () => {
    const start = new Date(2025, 5, 18, 9, 0)
    const end = new Date(2025, 5, 18, 10, 30)
    expect(calculateDuration(start, end, true)).toBe('')
  })

  it('calculateDuration formats minutes and hours', () => {
    const start = new Date(2025, 5, 18, 9, 0)
    expect(calculateDuration(start, new Date(2025, 5, 18, 9, 45), false)).toBe('45m')
    expect(calculateDuration(start, new Date(2025, 5, 18, 11, 0), false)).toBe('2h')
    expect(calculateDuration(start, new Date(2025, 5, 18, 10, 30), false)).toBe('1h 30m')
  })
})
