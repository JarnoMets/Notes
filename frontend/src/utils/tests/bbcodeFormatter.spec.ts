import { describe, expect, it } from 'vitest'
import { bbcodeToHtml, toggleTodoById } from '../bbcodeFormatter'

describe('bbcodeFormatter util', () => {
  it('toggleTodoById flips checked for matching id', () => {
    const source = '[todo id="a" checked="0"]One[/todo]'
    expect(toggleTodoById(source, 'a')).toContain('checked="1"')
    expect(toggleTodoById(toggleTodoById(source, 'a'), 'a')).toContain('checked="0"')
  })

  it('toggleTodoById propagates parent checked when all children are checked', () => {
    const source =
      '[todo id="parent" checked="0"][todo id="child-a" checked="0"]A[/todo][todo id="child-b" checked="0"]B[/todo][/todo]'
    const afterChildA = toggleTodoById(source, 'child-a')
    expect(afterChildA).toContain('id="child-a" checked="1"')
    expect(afterChildA).toContain('id="parent" checked="0"')

    const afterChildB = toggleTodoById(afterChildA, 'child-b')
    expect(afterChildB).toContain('id="child-b" checked="1"')
    expect(afterChildB).toContain('id="parent" checked="1"')
  })

  it('bbcodeToHtml escapes ampersands in plain text', () => {
    expect(bbcodeToHtml('a & b')).toContain('&amp;')
  })

  it('bbcodeToHtml renders bold tags', () => {
    expect(bbcodeToHtml('[b]bold[/b]')).toContain('<strong>bold</strong>')
  })
})
