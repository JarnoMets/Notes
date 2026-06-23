import { describe, it, expect } from 'vitest'
import {
  parseBBCode,
  extractBBCodeReferences,
  removeAttachmentReferences,
} from '../bbcode'

describe('bbcode reference parsing', () => {
  it('parseBBCode splits text and note links', () => {
    const parts = parseBBCode('See [note]abc-123[/note] for details')
    expect(parts).toEqual([
      { type: 'text', content: 'See ' },
      { type: 'note-link', content: 'abc-123', id: 'abc-123' },
      { type: 'text', content: ' for details' },
    ])
  })

  it('extractBBCodeReferences collects note, board, and attachment refs', () => {
    const refs = extractBBCodeReferences(
      'Note [note]n1[/note], board [board]b1[/board], file [attachment]a1[/attachment]'
    )
    expect(refs).toEqual([
      { id: 'n1', type: 'note', display: '[n1]' },
      { id: 'b1', type: 'board', display: '[b1]' },
      { id: 'a1', type: 'attachment', display: '[a1]' },
    ])
  })

  it('removeAttachmentReferences strips only the matching attachment tag', () => {
    const content = 'Keep [attachment]keep-me[/attachment] and drop [attachment]drop-me[/attachment]'
    const cleaned = removeAttachmentReferences(content, 'drop-me')
    expect(cleaned).toContain('[attachment]keep-me[/attachment]')
    expect(cleaned).not.toContain('drop-me')
  })
})
