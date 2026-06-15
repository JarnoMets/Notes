// @vitest-environment happy-dom
import { describe, it, expect } from 'vitest'
import {
  bbcodeToHtml,
  toggleTodoById,
  wrapWithBBCode,
  insertBBCodeTag,
} from '../bbcodeFormatter'

describe('bbcodeFormatter', () => {
  describe('bbcodeToHtml', () => {
    it('renders bold tags', () => {
      expect(bbcodeToHtml('[b]hello[/b]')).toBe('<strong>hello</strong>')
    })

    it('renders nested formatting', () => {
      expect(bbcodeToHtml('[b][i]mix[/i][/b]')).toBe(
        '<strong><em>mix</em></strong>'
      )
    })

    it('escapes ampersands in plain text for print safety', () => {
      expect(bbcodeToHtml('Tom & Jerry')).toBe('Tom &amp; Jerry')
    })

    it('renders blockquote for print output', () => {
      expect(bbcodeToHtml('[quote]cited text[/quote]')).toBe(
        '<blockquote>cited text</blockquote>'
      )
    })
  })

  describe('wrapWithBBCode', () => {
    it('wraps text with a simple tag', () => {
      expect(wrapWithBBCode('hello', 'b')).toBe('[b]hello[/b]')
    })

    it('wraps text with a valued tag', () => {
      expect(wrapWithBBCode('click', 'url', 'https://example.com')).toBe(
        '[url=https://example.com]click[/url]'
      )
    })
  })

  describe('insertBBCodeTag', () => {
    it('inserts an opening tag at the cursor position', () => {
      expect(insertBBCodeTag('hello world', 5, 'b')).toBe('hello[b] world')
    })

    it('inserts a closing tag when requested', () => {
      expect(insertBBCodeTag('hello world', 5, 'b', undefined, true)).toBe(
        'hello[/b] world'
      )
    })
  })

  describe('toggleTodoById', () => {
    it('toggles a todo checkbox by id', () => {
      const input =
        '[todo id="t1" checked="0"]Buy milk[/todo]'
      const toggled = toggleTodoById(input, 't1')
      expect(toggled).toContain('checked="1"')
    })

    it('propagates parent checked state when all children are checked', () => {
      const input = [
        '[todo id="parent" checked="0"]',
        '[todo id="child1" checked="1"]one[/todo]',
        '[todo id="child2" checked="0"]two[/todo]',
        '[/todo]',
      ].join('')
      const toggled = toggleTodoById(input, 'child2')
      expect(toggled).toContain('id="parent" checked="1"')
      expect(toggled).toContain('id="child2" checked="1"')
    })
  })
})
