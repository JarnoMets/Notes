import { describe, it, expect } from 'vitest'
import { normalizeForInput, toApiIso } from '../dates'

describe('dates util', () => {
  it('normalizeForInput returns undefined for empty', () => {
    expect(normalizeForInput(undefined)).toBeUndefined()
  })

  it('normalizeForInput appends 18:00 for date-only strings', () => {
    expect(normalizeForInput('2025-12-01')).toBe('2025-12-01T18:00')
  })

  it('normalizeForInput replaces midnight with 18:00', () => {
    expect(normalizeForInput('2025-12-01T00:00')).toBe('2025-12-01T18:00')
  })

  it('normalizeForInput preserves valid datetime-local', () => {
    expect(normalizeForInput('2025-12-01T09:30')).toBe('2025-12-01T09:30')
  })

  it('toApiIso converts date-only to ISO for local 18:00 instant', () => {
    const iso = toApiIso('2025-12-01')
    expect(typeof iso).toBe('string')
    const parsed = new Date(iso as string)
    const expected = new Date(2025, 11, 1, 18, 0, 0)
    expect(parsed.getTime()).toBe(expected.getTime())
  })

  it('toApiIso converts datetime-local to matching local instant', () => {
    const iso = toApiIso('2025-12-01T09:30')
    expect(typeof iso).toBe('string')
    const parsed = new Date(iso as string)
    const expected = new Date(2025, 11, 1, 9, 30, 0)
    expect(parsed.getTime()).toBe(expected.getTime())
  })

  it('toApiIso parses Z-terminated ISO correctly', () => {
    const isoIn = '2025-12-01T09:30:00Z'
    const iso = toApiIso(isoIn)
    // Compare instants rather than exact string formatting (milliseconds may differ in formatting)
    expect(new Date(iso as string).getTime()).toBe(new Date(isoIn).getTime())
  })

  it('toApiIso returns undefined for invalid input', () => {
    expect(toApiIso('not-a-date')).toBeUndefined()
  })
})
