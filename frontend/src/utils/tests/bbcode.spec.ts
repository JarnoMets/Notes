import { describe, expect, it } from 'vitest'
import {
  bbcodeToHtmlLinks,
  extractBBCodeReferences,
  htmlLinksTobbcode,
  insertBBCodeReference,
  parseBBCode,
  removeAttachmentReferences,
} from '../bbcode'

describe('parseBBCode', () => {
  it('splits text and note references', () => {
    const parts = parseBBCode('See [note]abc-123[/note] for details')
    expect(parts).toEqual([
      { type: 'text', content: 'See ' },
      { type: 'note-link', content: 'abc-123', id: 'abc-123' },
      { type: 'text', content: ' for details' },
    ])
  })

  it('returns plain text when no tags are present', () => {
    expect(parseBBCode('hello')).toEqual([{ type: 'text', content: 'hello' }])
  })
})

describe('extractBBCodeReferences', () => {
  it('collects note, board, and attachment references', () => {
    const refs = extractBBCodeReferences(
      '[note]n1[/note] [board]b1[/board] [attachment]a1[/attachment]'
    )
    expect(refs).toEqual([
      { id: 'n1', type: 'note', display: '[n1]' },
      { id: 'b1', type: 'board', display: '[b1]' },
      { id: 'a1', type: 'attachment', display: '[a1]' },
    ])
  })
})

describe('insertBBCodeReference', () => {
  it('builds correctly tagged references', () => {
    expect(insertBBCodeReference('id-1', 'note')).toBe('[note]id-1[/note]')
    expect(insertBBCodeReference('id-2', 'board')).toBe('[board]id-2[/board]')
    expect(insertBBCodeReference('id-3', 'attachment')).toBe('[attachment]id-3[/attachment]')
  })
})

describe('htmlLinksTobbcode', () => {
  it('converts internal HTML links to BBCode', () => {
    const html =
      '<a href="notes://note-1">Note</a> and <a href="boards://board-1">Board</a>'
    expect(htmlLinksTobbcode(html)).toBe('[note]note-1[/note] and [board]board-1[/board]')
  })
})

describe('bbcodeToHtmlLinks', () => {
  it('converts BBCode references to internal HTML links', () => {
    const html = bbcodeToHtmlLinks('[note]note-1[/note]')
    expect(html).toContain('href="notes://note-1"')
    expect(html).toContain('[note-1]')
  })
})

describe('removeAttachmentReferences', () => {
  it('removes only the matching attachment tag', () => {
    const content = 'Keep [attachment]a1[/attachment] and [attachment]a2[/attachment]'
    expect(removeAttachmentReferences(content, 'a1')).toBe('Keep  and [attachment]a2[/attachment]')
  })

  it('escapes regex metacharacters in attachment ids', () => {
    const content = 'Ref [attachment]file.v2[/attachment] here'
    expect(removeAttachmentReferences(content, 'file.v2')).toBe('Ref  here')
  })
})
