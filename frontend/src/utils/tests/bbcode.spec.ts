import { describe, it, expect } from 'vitest'
import {
  parseBBCode,
  extractBBCodeReferences,
  insertBBCodeReference,
} from '../bbcode'

describe('bbcode util', () => {
  it('parseBBCode extracts note, board, and attachment links', () => {
    const content = 'See [note]abc-123[/note] and [board]board-1[/board] plus [attachment]file.pdf[/attachment]'
    const parsed = parseBBCode(content)
    expect(parsed).toEqual([
      { type: 'text', content: 'See ' },
      { type: 'note-link', content: 'abc-123', id: 'abc-123' },
      { type: 'text', content: ' and ' },
      { type: 'board-link', content: 'board-1', id: 'board-1' },
      { type: 'text', content: ' plus ' },
      { type: 'attachment-link', content: 'file.pdf', id: 'file.pdf' },
    ])
  })

  it('parseBBCode returns plain text when no tags present', () => {
    expect(parseBBCode('plain text')).toEqual([{ type: 'text', content: 'plain text' }])
  })

  it('extractBBCodeReferences collects all reference types', () => {
    const refs = extractBBCodeReferences(
      '[note]n1[/note][board]b1[/board][attachment]a1[/attachment]'
    )
    expect(refs).toHaveLength(3)
    expect(refs.map((r) => r.type)).toEqual(['note', 'board', 'attachment'])
    expect(refs.map((r) => r.id)).toEqual(['n1', 'b1', 'a1'])
  })

  it('insertBBCodeReference wraps id in correct tag', () => {
    expect(insertBBCodeReference('id-1', 'note')).toBe('[note]id-1[/note]')
    expect(insertBBCodeReference('id-2', 'board')).toBe('[board]id-2[/board]')
    expect(insertBBCodeReference('id-3', 'attachment')).toBe('[attachment]id-3[/attachment]')
  })
})
