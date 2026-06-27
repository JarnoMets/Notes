import { describe, it, expect } from 'vitest'
import { bbcodeToHtml, toggleTodoById } from '../bbcodeFormatter'

describe('bbcodeFormatter util', () => {
  it('bbcodeToHtml escapes HTML in text nodes', () => {
    const html = bbcodeToHtml('[b]<script>alert(1)</script>[/b]')
    expect(html).toContain('&lt;script&gt;')
    expect(html).not.toContain('<script>')
  })

  it('toggleTodoById toggles checked state by id', () => {
    const bbcode = '[todo id="t1" checked="0"]Item[/todo]'
    const toggled = toggleTodoById(bbcode, 't1')
    expect(toggled).toContain('checked="1"')
  })

  it('toggleTodoById propagates parent checked when all children checked', () => {
    const bbcode =
      '[todo id="parent" checked="0"][todo id="child" checked="0"]Child[/todo][/todo]'
    const toggled = toggleTodoById(bbcode, 'child')
    expect(toggled).toContain('id="child" checked="1"')
    expect(toggled).toContain('id="parent" checked="1"')
  })

  it('toggleTodoById unchecks parent when a child is unchecked', () => {
    const bbcode =
      '[todo id="parent" checked="1"][todo id="child1" checked="1"]A[/todo][todo id="child2" checked="1"]B[/todo][/todo]'
    const toggled = toggleTodoById(bbcode, 'child2')
    expect(toggled).toContain('id="child2" checked="0"')
    expect(toggled).toContain('id="parent" checked="0"')
  })
})
