/**
 * @vitest-environment jsdom
 */
import { describe, it, expect } from 'vitest'
import { bbcodeToHtml, toggleTodoById } from '../bbcodeFormatter'

describe('bbcodeFormatter', () => {
  it('bbcodeToHtml renders bold and url tags', () => {
    const html = bbcodeToHtml('[b]Hello[/b] [url=https://example.com]Link[/url]')
    expect(html).toContain('<strong>Hello</strong>')
    expect(html).toContain('href="https://example.com"')
    expect(html).toContain('Link')
  })

  it('toggleTodoById toggles checked state by id', () => {
    const input = '[todo id="t1" checked="0"]Item[/todo]'
    const toggled = toggleTodoById(input, 't1')
    expect(toggled).toContain('checked="1"')
    const restored = toggleTodoById(toggled, 't1')
    expect(restored).toContain('checked="0"')
  })

  it('toggleTodoById propagates parent checked when all children checked', () => {
    const input =
      '[todo id="parent" checked="0"][todo id="child1" checked="0"]A[/todo][todo id="child2" checked="0"]B[/todo][/todo]'
    let result = toggleTodoById(input, 'child1')
    result = toggleTodoById(result, 'child2')
    expect(result).toMatch(/id="parent" checked="1"/)
  })

  it('bbcodeToHtml handles malformed input without throwing', () => {
    expect(() => bbcodeToHtml('[b]unclosed')).not.toThrow()
    expect(bbcodeToHtml('[b]unclosed')).toContain('unclosed')
  })
})
