import { describe, it, expect } from 'vitest'
import {
  parseBBCode,
  extractBBCodeReferences,
  insertBBCodeReference,
  bbcodeToHtmlLinks,
  removeAttachmentReferences,
} from '../bbcode'

describe('bbcode util', () => {
  it('parseBBCode splits text and reference tags', () => {
    const content = 'See [note]abc-123[/note] and [board]board-1[/board]'
    const parsed = parseBBCode(content)
    expect(parsed).toHaveLength(4)
    expect(parsed[0]).toEqual({ type: 'text', content: 'See ' })
    expect(parsed[1]).toEqual({ type: 'note-link', content: 'abc-123', id: 'abc-123' })
    expect(parsed[2]).toEqual({ type: 'text', content: ' and ' })
    expect(parsed[3]).toEqual({ type: 'board-link', content: 'board-1', id: 'board-1' })
  })

  it('extractBBCodeReferences collects note, board, and attachment refs', () => {
    const content =
      '[note]n1[/note][board]b1[/board][attachment]att-1[/attachment]'
    const refs = extractBBCodeReferences(content)
    expect(refs).toHaveLength(3)
    expect(refs.map(r => r.type)).toEqual(['note', 'board', 'attachment'])
  })

  it('insertBBCodeReference wraps id in correct tag', () => {
    expect(insertBBCodeReference('id-1', 'note')).toBe('[note]id-1[/note]')
    expect(insertBBCodeReference('id-2', 'attachment')).toBe('[attachment]id-2[/attachment]')
  })

  it('bbcodeToHtmlLinks converts note references to internal links', () => {
    const html = bbcodeToHtmlLinks('See [note]abc-123[/note]')
    expect(html).toContain('href="notes://abc-123"')
    expect(html).toContain('class="internal-link note-link"')
  })

  it('removeAttachmentReferences strips only the matching attachment id', () => {
    const content = 'before [attachment]att-1[/attachment] after [attachment]att-2[/attachment]'
    expect(removeAttachmentReferences(content, 'att-1')).toBe('before  after [attachment]att-2[/attachment]')
  })
})
