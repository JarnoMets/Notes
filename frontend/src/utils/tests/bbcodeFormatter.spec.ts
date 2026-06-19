import { describe, expect, it } from 'vitest'
import { bbcodeToHtml, toggleTodoById } from '../bbcodeFormatter'

describe('toggleTodoById', () => {
  it('toggles a leaf todo checked state', () => {
    const bbcode = '[todo id="task-1" checked="0"]Buy milk[/todo]'
    const updated = toggleTodoById(bbcode, 'task-1')
    expect(updated).toContain('checked="1"')
    expect(toggleTodoById(updated, 'task-1')).toContain('checked="0"')
  })

  it('checks parent when all nested todos are checked', () => {
    const bbcode = [
      '[todo id="parent" checked="0"]',
      '[todo id="child-a" checked="0"]First[/todo]',
      '[todo id="child-b" checked="0"]Second[/todo]',
      '[/todo]',
    ].join('')

    const afterFirst = toggleTodoById(bbcode, 'child-a')
    expect(afterFirst).toContain('id="child-a" checked="1"')
    expect(afterFirst).toContain('id="parent" checked="0"')

    const afterSecond = toggleTodoById(afterFirst, 'child-b')
    expect(afterSecond).toContain('id="child-b" checked="1"')
    expect(afterSecond).toContain('id="parent" checked="1"')
  })

  it('unchecks parent when any nested todo is unchecked', () => {
    const bbcode = [
      '[todo id="parent" checked="1"]',
      '[todo id="child-a" checked="1"]First[/todo]',
      '[todo id="child-b" checked="1"]Second[/todo]',
      '[/todo]',
    ].join('')

    const updated = toggleTodoById(bbcode, 'child-b')
    expect(updated).toContain('id="child-b" checked="0"')
    expect(updated).toContain('id="parent" checked="0"')
  })

  it('returns unchanged bbcode when todo id is missing', () => {
    const bbcode = '[todo id="task-1" checked="0"]Buy milk[/todo]'
    expect(toggleTodoById(bbcode, 'missing-id')).toBe(bbcode)
  })
})

describe('bbcodeToHtml', () => {
  it('renders basic inline formatting for print output', () => {
    const html = bbcodeToHtml('[b]Bold[/b] and [i]italic[/i] text')
    expect(html).toContain('<strong>Bold</strong>')
    expect(html).toContain('<em>italic</em>')
  })

  it('renders headings and links used in note print layouts', () => {
    const html = bbcodeToHtml(
      '[h2]Section[/h2][url=https://example.com]Read more[/url]'
    )
    expect(html).toContain('<h2')
    expect(html).toContain('Section')
    expect(html).toContain('href="https://example.com"')
    expect(html).toContain('Read more')
  })

  it('escapes raw html in text nodes to avoid injection in print preview', () => {
    const html = bbcodeToHtml('Hello <script>alert(1)</script> world')
    expect(html).not.toContain('<script>')
    expect(html).toContain('&lt;script&gt;')
  })

  it('renders todo checkboxes with stable ids for editor callbacks', () => {
    const html = bbcodeToHtml('[todo id="print-task" checked="0"]Review[/todo]', {
      paneId: 'pane-1',
    })
    expect(html).toContain('print-task')
    expect(html).toContain('type="checkbox"')
    expect(html).toContain('Review')
  })
})
