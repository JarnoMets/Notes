import { describe, expect, it } from 'vitest'
import { BBCodeParser, bbcodeToHtml, toggleTodoById } from '../bbcodeFormatter'

describe('BBCodeParser', () => {
  it('parses nested formatting tags', () => {
    const nodes = new BBCodeParser('[b][i]hello[/i][/b]').parse()
    expect(nodes).toHaveLength(1)
    expect(nodes[0].type).toBe('b')
    expect(Array.isArray(nodes[0].content)).toBe(true)
    const inner = nodes[0].content as Array<{ type: string }>
    expect(inner[0].type).toBe('i')
  })

  it('parses todo tags with named attributes', () => {
    const nodes = new BBCodeParser('[todo id="abc-123" checked="0"]Buy milk[/todo]').parse()
    expect(nodes).toHaveLength(1)
    expect(nodes[0].type).toBe('todo')
    expect(nodes[0].attrs?.id).toBe('abc-123')
    expect(nodes[0].attrs?.checked).toBe('0')
  })
})

describe('bbcodeToHtml', () => {
  it('renders bold text with escaped HTML', () => {
    const html = bbcodeToHtml('[b]<script>alert(1)</script>[/b]')
    expect(html).toContain('<strong>')
    expect(html).not.toContain('<script>')
    expect(html).toContain('&lt;script&gt;')
  })

  it('renders todo checkbox with stable id', () => {
    const html = bbcodeToHtml('[todo id="todo-1" checked="1"]Done[/todo]', { paneId: 'pane-a' })
    expect(html).toContain('data-todo-id="todo-1"')
    expect(html).toContain('checked')
    expect(html).toContain("__toggleTodo('pane-a'")
  })
})

describe('toggleTodoById', () => {
  it('toggles a todo checked state by id', () => {
    const input = '[todo id="a" checked="0"]Task A[/todo]'
    const toggled = toggleTodoById(input, 'a')
    expect(toggled).toContain('checked="1"')

    const restored = toggleTodoById(toggled, 'a')
    expect(restored).toContain('checked="0"')
  })

  it('propagates parent checked when all nested todos are checked', () => {
    const input = [
      '[todo id="parent" checked="0"]',
      '[todo id="child-a" checked="0"]A[/todo]',
      '[todo id="child-b" checked="0"]B[/todo]',
      '[/todo]',
    ].join('')

    const afterChildA = toggleTodoById(input, 'child-a')
    expect(afterChildA).toContain('id="child-a" checked="1"')
    expect(afterChildA).toContain('id="parent" checked="0"')

    const afterChildB = toggleTodoById(afterChildA, 'child-b')
    expect(afterChildB).toContain('id="child-b" checked="1"')
    expect(afterChildB).toContain('id="parent" checked="1"')
  })

  it('unchecks parent when any nested todo is unchecked', () => {
    const input = [
      '[todo id="parent" checked="1"]',
      '[todo id="child-a" checked="1"]A[/todo]',
      '[todo id="child-b" checked="1"]B[/todo]',
      '[/todo]',
    ].join('')

    const toggled = toggleTodoById(input, 'child-b')
    expect(toggled).toContain('id="child-b" checked="0"')
    expect(toggled).toContain('id="parent" checked="0"')
  })
})
