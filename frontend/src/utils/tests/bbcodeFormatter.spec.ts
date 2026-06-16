import { describe, expect, it } from 'vitest'
import { toggleTodoById } from '../bbcodeFormatter'

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
