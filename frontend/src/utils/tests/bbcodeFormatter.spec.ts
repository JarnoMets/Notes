import { describe, it, expect } from 'vitest'
import { toggleTodoById } from '../bbcodeFormatter'

describe('toggleTodoById', () => {
  it('toggles a leaf todo checked state', () => {
    const bbcode = '[todo id="leaf" checked="0"]Buy milk[/todo]'
    const toggled = toggleTodoById(bbcode, 'leaf')
    expect(toggled).toContain('checked="1"')
    expect(toggled).not.toContain('checked="0"')
  })

  it('checks parent when all nested todos are checked', () => {
    const bbcode = [
      '[todo id="parent" checked="0"]',
      '[todo id="child-a" checked="1"]Task A[/todo]',
      '[todo id="child-b" checked="0"]Task B[/todo]',
      '[/todo]',
    ].join('')

    const toggled = toggleTodoById(bbcode, 'child-b')
    expect(toggled).toContain('id="child-b" checked="1"')
    expect(toggled).toContain('id="parent" checked="1"')
  })

  it('unchecks parent when any nested todo is unchecked', () => {
    const bbcode = [
      '[todo id="parent" checked="1"]',
      '[todo id="child-a" checked="1"]Task A[/todo]',
      '[todo id="child-b" checked="1"]Task B[/todo]',
      '[/todo]',
    ].join('')

    const toggled = toggleTodoById(bbcode, 'child-a')
    expect(toggled).toContain('id="child-a" checked="0"')
    expect(toggled).toContain('id="parent" checked="0"')
  })
})
