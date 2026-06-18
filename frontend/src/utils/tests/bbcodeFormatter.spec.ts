import { beforeAll, describe, expect, it } from 'vitest'
import { bbcodeToHtml, toggleTodoById } from '../bbcodeFormatter'

beforeAll(() => {
  if (typeof document === 'undefined') {
    ;(globalThis as typeof globalThis & { document: Document }).document = {
      createElement: () => {
        let text = ''
        return {
          set textContent(value: string) {
            text = value
          },
          get innerHTML() {
            return text
              .replace(/&/g, '&amp;')
              .replace(/</g, '&lt;')
              .replace(/>/g, '&gt;')
              .replace(/"/g, '&quot;')
          },
        }
      },
    } as unknown as Document
  }
})

describe('bbcodeFormatter', () => {
  it('bbcodeToHtml renders basic formatting tags', () => {
    const html = bbcodeToHtml('[b]bold[/b] and [i]italic[/i]')
    expect(html).toContain('<strong>bold</strong>')
    expect(html).toContain('<em>italic</em>')
  })

  it('bbcodeToHtml escapes raw HTML in text nodes', () => {
    const html = bbcodeToHtml('[b]<script>alert(1)</script>[/b]')
    expect(html).not.toContain('<script>')
    expect(html).toContain('&lt;script&gt;alert(1)&lt;/script&gt;')
  })

  it('bbcodeToHtml renders todo checkboxes with stable ids', () => {
    const html = bbcodeToHtml('[todo id="task-1" checked="0"]Buy milk[/todo]', {
      paneId: 'pane-a',
    })
    expect(html).toContain('data-todo-id="task-1"')
    expect(html).toContain('type="checkbox"')
    expect(html).toContain("window.__toggleTodo('pane-a', 'task-1')")
    expect(html).toContain('Buy milk')
  })

  it('toggleTodoById flips checked state and propagates parent todo', () => {
    const source =
      '[todo id="parent" checked="1"][todo id="child-a" checked="1"]Done[/todo][todo id="child-b" checked="1"]Also[/todo][/todo]'
    const toggled = toggleTodoById(source, 'child-a')

    expect(toggled).toContain('id="child-a" checked="0"')
    expect(toggled).toContain('id="parent" checked="0"')
  })
})
