import { describe, expect, it } from 'vitest'
import {
  extractBBCodeReferences,
  htmlLinksTobbcode,
  insertBBCodeReference,
  parseBBCode,
  removeAttachmentReferences,
} from '../bbcode'

describe('bbcode util', () => {
  it('parseBBCode splits mixed text and note links', () => {
    const parts = parseBBCode('See [note]abc[/note] here')
    expect(parts).toEqual([
      { type: 'text', content: 'See ' },
      { type: 'note-link', content: 'abc', id: 'abc' },
      { type: 'text', content: ' here' },
    ])
  })

  it('parseBBCode returns plain text when no tags are present', () => {
    expect(parseBBCode('plain text')).toEqual([{ type: 'text', content: 'plain text' }])
  })

  it('extractBBCodeReferences collects note, board, and attachment ids', () => {
    const refs = extractBBCodeReferences(
      '[note]n1[/note] [board]b1[/board] [attachment]a1[/attachment]'
    )
    expect(refs).toEqual([
      { id: 'n1', type: 'note', display: '[n1]' },
      { id: 'b1', type: 'board', display: '[b1]' },
      { id: 'a1', type: 'attachment', display: '[a1]' },
    ])
  })

  it('insertBBCodeReference wraps ids in the correct tag', () => {
    expect(insertBBCodeReference('id-1', 'note')).toBe('[note]id-1[/note]')
    expect(insertBBCodeReference('id-2', 'board')).toBe('[board]id-2[/board]')
    expect(insertBBCodeReference('id-3', 'attachment')).toBe('[attachment]id-3[/attachment]')
  })

  it('htmlLinksTobbcode converts notes:// anchors', () => {
    expect(htmlLinksTobbcode('<a href="notes://id1">x</a>')).toBe('[note]id1[/note]')
  })

  it('removeAttachmentReferences strips only the matching attachment id', () => {
    const content = '[attachment]x[/attachment] keep [attachment]y[/attachment]'
    expect(removeAttachmentReferences(content, 'x')).toBe(' keep [attachment]y[/attachment]')
  })
})
