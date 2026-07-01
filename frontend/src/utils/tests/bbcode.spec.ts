import { describe, it, expect } from 'vitest'
import {
  parseBBCode,
  extractBBCodeReferences,
  htmlLinksTobbcode,
  removeAttachmentReferences,
} from '../bbcode'

describe('bbcode util', () => {
  it('parseBBCode splits note and board references', () => {
    const parsed = parseBBCode('See [note]abc-123[/note] and [board]board-1[/board]')
    expect(parsed).toHaveLength(4)
    expect(parsed[1]).toMatchObject({ type: 'note-link', id: 'abc-123' })
    expect(parsed[3]).toMatchObject({ type: 'board-link', id: 'board-1' })
  })

  it('extractBBCodeReferences collects all reference types', () => {
    const refs = extractBBCodeReferences(
      '[note]n1[/note] [board]b1[/board] [attachment]a1[/attachment]'
    )
    expect(refs).toHaveLength(3)
    expect(refs.map(r => r.type)).toEqual(['note', 'board', 'attachment'])
  })

  it('htmlLinksTobbcode converts internal anchor hrefs', () => {
    const html =
      '<a href="notes://note-id">Note</a> <a href="boards://board-id">Board</a> <a href="attachments://file-id">File</a>'
    const bbcode = htmlLinksTobbcode(html)
    expect(bbcode).toContain('[note]note-id[/note]')
    expect(bbcode).toContain('[board]board-id[/board]')
    expect(bbcode).toContain('[attachment]file-id[/attachment]')
  })

  it('removeAttachmentReferences escapes regex special characters in ids', () => {
    const content = 'Before [attachment]file.id+1[/attachment] after'
    expect(removeAttachmentReferences(content, 'file.id+1')).toBe('Before  after')
  })
})
