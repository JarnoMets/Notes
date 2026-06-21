import { describe, it, expect } from 'vitest'
import { bbcodeToHtml, toggleTodoById } from '../bbcodeFormatter'

describe('bbcodeToHtml', () => {
  it('renders bold and italic tags', () => {
    const html = bbcodeToHtml('[b]Hello[/b] [i]world[/i]')
    expect(html).toContain('<strong>Hello</strong>')
    expect(html).toContain('<em>world</em>')
  })

  it('renders todo checkboxes with checked state', () => {
    const html = bbcodeToHtml('[todo id="t1" checked="1"]Buy milk[/todo]')
    expect(html).toContain('type="checkbox"')
    expect(html).toContain('checked')
    expect(html).toContain('Buy milk')
  })

  it('escapes HTML in plain text nodes', () => {
    const html = bbcodeToHtml('<script>alert(1)</script>')
    expect(html).not.toContain('<script>')
    expect(html).toContain('&lt;script&gt;')
  })
})

describe('toggleTodoById', () => {
  it('toggles a todo checked state by id', () => {
    const input = '[todo id="a" checked="0"]Task A[/todo]'
    const toggled = toggleTodoById(input, 'a')
    expect(toggled).toContain('checked="1"')
  })

  it('propagates parent checked when all children are checked', () => {
    const input = [
      '[todo id="parent" checked="0"]',
      '[todo id="child1" checked="0"]Sub 1[/todo]',
      '[todo id="child2" checked="0"]Sub 2[/todo]',
      '[/todo]',
    ].join('')

    const afterChild1 = toggleTodoById(input, 'child1')
    expect(afterChild1).toContain('id="child1" checked="1"')
    expect(afterChild1).toContain('id="parent" checked="0"')

    const afterChild2 = toggleTodoById(afterChild1, 'child2')
    expect(afterChild2).toContain('id="child2" checked="1"')
    expect(afterChild2).toContain('id="parent" checked="1"')
  })

  it('unchecks parent when any child is unchecked', () => {
    const input = [
      '[todo id="parent" checked="1"]',
      '[todo id="child1" checked="1"]Sub 1[/todo]',
      '[todo id="child2" checked="1"]Sub 2[/todo]',
      '[/todo]',
    ].join('')

    const toggled = toggleTodoById(input, 'child1')
    expect(toggled).toContain('id="child1" checked="0"')
    expect(toggled).toContain('id="parent" checked="0"')
  })
})
