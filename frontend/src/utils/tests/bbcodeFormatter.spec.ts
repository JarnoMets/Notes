import { describe, it, expect } from 'vitest'
import { bbcodeToHtml, toggleTodoById } from '../bbcodeFormatter'

describe('bbcodeFormatter', () => {
  it('bbcodeToHtml renders bold and italic tags', () => {
    const html = bbcodeToHtml('[b]bold[/b] and [i]italic[/i]')
    expect(html).toContain('<strong>bold</strong>')
    expect(html).toContain('<em>italic</em>')
  })

  it('bbcodeToHtml renders links with href', () => {
    const html = bbcodeToHtml('[url=https://example.com]Example[/url]')
    expect(html).toContain('href="https://example.com"')
    expect(html).toContain('Example')
  })

  it('toggleTodoById toggles a todo checkbox by id', () => {
    const source = '[todo id="t1" checked="0"]Buy milk[/todo]'
    const toggled = toggleTodoById(source, 't1')
    expect(toggled).toContain('checked="1"')
    expect(toggled).not.toContain('checked="0"')
  })

  it('toggleTodoById propagates parent state when all children are checked', () => {
    const source = [
      '[todo id="parent" checked="0"]',
      '[todo id="child" checked="0"]Task[/todo]',
      '[/todo]',
    ].join('')

    const toggled = toggleTodoById(source, 'child')
    expect(toggled).toContain('id="child" checked="1"')
    expect(toggled).toContain('id="parent" checked="1"')
  })
})
