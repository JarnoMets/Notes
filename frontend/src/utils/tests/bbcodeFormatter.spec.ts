import { describe, it, expect } from 'vitest'
import {
  bbcodeToHtml,
  toggleTodoById,
  wrapWithBBCode,
  insertBBCodeTag,
} from '../bbcodeFormatter'

describe('bbcodeFormatter', () => {
  describe('wrapWithBBCode', () => {
    it('wraps text with a simple tag', () => {
      expect(wrapWithBBCode('hello', 'b')).toBe('[b]hello[/b]')
    })

    it('wraps text with a tag that has a value', () => {
      expect(wrapWithBBCode('hello', 'color', 'red')).toBe('[color=red]hello[/color]')
    })
  })

  describe('insertBBCodeTag', () => {
    it('inserts an opening tag at the cursor position', () => {
      expect(insertBBCodeTag('hello world', 5, 'b')).toBe('hello[b] world')
    })

    it('inserts a closing tag when requested', () => {
      expect(insertBBCodeTag('hello world', 5, 'b', undefined, true)).toBe('hello[/b] world')
    })
  })

  describe('bbcodeToHtml', () => {
    it('renders basic formatting for print output', () => {
      const html = bbcodeToHtml('[b]Title[/b]\n[quote]Important note[/quote]')
      expect(html).toContain('<strong>Title</strong>')
      expect(html).toContain('<blockquote>Important note</blockquote>')
    })

    it('escapes plain text ampersands for safe print output', () => {
      const html = bbcodeToHtml('foo & bar')
      expect(html).toContain('foo &amp; bar')
    })

    it('escapes URL attributes in links', () => {
      const html = bbcodeToHtml('[url=https://example.com?q=1&r=2]link[/url]')
      expect(html).toContain('href="https://example.com?q=1&amp;r=2"')
      expect(html).toContain('link')
    })

    it('renders todo checkboxes with stable ids', () => {
      const html = bbcodeToHtml('[todo id="task-1" checked="0"]Buy milk[/todo]', {
        paneId: 'pane-42',
      })
      expect(html).toContain('data-todo-id="task-1"')
      expect(html).toContain('type="checkbox"')
      expect(html).toContain("window.__toggleTodo('pane-42', 'task-1')")
      expect(html).toContain('Buy milk')
    })
  })

  describe('toggleTodoById', () => {
    it('toggles the checked state for a todo by id', () => {
      const source =
        '[todo id="a" checked="0"]One[/todo][todo id="b" checked="1"]Two[/todo]'
      const toggled = toggleTodoById(source, 'a')
      expect(toggled).toContain('[todo id="a" checked="1"]One[/todo]')
      expect(toggled).toContain('[todo id="b" checked="1"]Two[/todo]')
    })

    it('propagates parent checked state when all child todos are checked', () => {
      const source = [
        '[todo id="parent" checked="0"]',
        '[todo id="child-a" checked="0"]A[/todo]',
        '[todo id="child-b" checked="0"]B[/todo]',
        '[/todo]',
      ].join('')

      const afterChildA = toggleTodoById(source, 'child-a')
      expect(afterChildA).toContain('[todo id="child-a" checked="1"]A[/todo]')
      expect(afterChildA).toContain('[todo id="parent" checked="0"]')

      const afterChildB = toggleTodoById(afterChildA, 'child-b')
      expect(afterChildB).toContain('[todo id="child-b" checked="1"]B[/todo]')
      expect(afterChildB).toContain('[todo id="parent" checked="1"]')
    })
  })
})
