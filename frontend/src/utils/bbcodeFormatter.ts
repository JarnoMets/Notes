/**
 * Comprehensive BBCode Parser and Formatter
 * Supports all formatting: bold, italic, underline, strikethrough, colors, alignment, lists, tables, links, images, etc.
 */

export interface BBCodeNode {
  type: string
  content?: string | BBCodeNode[]
  attrs?: Record<string, string>
}

/**
 * BBCode Lexer - tokenizes BBCode text
 */
class BBCodeLexer {
  private text: string
  private pos: number

  constructor(text: string) {
    this.text = text
    this.pos = 0
  }

  private peek(offset = 0): string {
    return this.text[this.pos + offset] || ''
  }

  private advance(count = 1): void {
    this.pos += count
  }

  private readUntil(char: string): string {
    let result = ''
    while (this.pos < this.text.length && this.peek() !== char) {
      result += this.peek()
      this.advance()
    }
    return result
  }

  private parseTag(): { name: string; attrs: Record<string, string> } | null {
    if (this.peek() !== '[') return null

    this.advance() // skip [
    // Read tag name until we hit ] or space
    let tagName = ''
    while (this.pos < this.text.length && this.peek() !== ']' && this.peek() !== ' ' && this.peek() !== '=') {
      tagName += this.peek()
      this.advance()
    }

    const attrs: Record<string, string> = {}

    // Parse named attributes until closing ]
    // Supports forms:
    // [tag]
    // [tag=value]
    // [tag key="value" other=val]
    // Quoted values with " or ' are supported
    // Skip spaces
    while (this.peek() === ' ') this.advance()

    // If next char is '=' then this is the old-style [tag=value]
    if (this.peek() === '=') {
      this.advance()
      const value = this.readUntil(']')
      attrs['value'] = value
      if (this.peek() === ']') this.advance()
      return { name: tagName, attrs }
    }

    // Parse key=value pairs
    while (this.pos < this.text.length && this.peek() !== ']') {
      // skip spaces
      while (this.peek() === ' ') this.advance()
      if (this.peek() === ']') break

      // read key
      let key = ''
      while (this.pos < this.text.length && /[A-Za-z0-9_-]/.test(this.peek())) {
        key += this.peek()
        this.advance()
      }

      // skip spaces
      while (this.peek() === ' ') this.advance()

      let value = ''
      if (this.peek() === '=') {
        this.advance()
        // skip spaces
        while (this.peek() === ' ') this.advance()
        const q = this.peek()
        if (q === '"' || q === "'") {
          this.advance()
          // read until matching quote
          while (this.pos < this.text.length && this.peek() !== q) {
            value += this.peek()
            this.advance()
          }
          if (this.peek() === q) this.advance()
        } else {
          // unquoted value until space or ]
          while (this.pos < this.text.length && this.peek() !== ' ' && this.peek() !== ']') {
            value += this.peek()
            this.advance()
          }
        }
      }

      if (key) attrs[key] = value
      // skip spaces
      while (this.peek() === ' ') this.advance()
    }

    if (this.peek() === ']') this.advance()

    return { name: tagName, attrs }
  }

  tokenize(): Array<{ type: 'text' | 'tag' | 'close'; content: string; name?: string; attrs?: Record<string, string> }> {
    const tokens: Array<{ type: 'text' | 'tag' | 'close'; content: string; name?: string; attrs?: Record<string, string> }> = []

    while (this.pos < this.text.length) {
      if (this.peek() === '[') {
        if (this.peek(1) === '/') {
          // Closing tag
          this.advance(2) // [/
          const tagName = this.readUntil(']')
          this.advance() // ]
          tokens.push({ type: 'close', name: tagName, content: '' })
        } else {
          // Opening tag
          const tag = this.parseTag()
          if (tag) {
            tokens.push({ type: 'tag', name: tag.name, attrs: tag.attrs, content: tag.name })
          }
        }
      } else {
        // Text
        let text = ''
        while (this.pos < this.text.length && this.peek() !== '[') {
          text += this.peek()
          this.advance()
        }
        if (text) tokens.push({ type: 'text', content: text })
      }
    }

    return tokens
  }
}

/**
 * BBCode Parser - builds tree from tokens
 */
export class BBCodeParser {
  private tokens: Array<{ type: 'text' | 'tag' | 'close'; content: string; name?: string; attrs?: Record<string, string> }>
  private pos: number

  constructor(text: string) {
    const lexer = new BBCodeLexer(text)
    this.tokens = lexer.tokenize()
    this.pos = 0
  }

  private peek(): typeof this.tokens[0] | null {
    return this.tokens[this.pos] || null
  }

  private advance(): void {
    this.pos++
  }

  private parseNodes(untilTag?: string): BBCodeNode[] {
    const nodes: BBCodeNode[] = []

    while (this.pos < this.tokens.length) {
      const token = this.peek()
      if (!token) break

      if (token.type === 'text') {
        nodes.push({ type: 'text', content: token.content })
        this.advance()
      } else if (token.type === 'tag') {
        const children = this.parseTag(token.name!)
        if (children) {
          nodes.push(children)
        }
      } else if (token.type === 'close') {
        if (untilTag && token.name === untilTag) {
          this.advance()
          break
        }
        this.advance()
      }
    }

    return nodes
  }

  private parseTag(tagName: string): BBCodeNode | null {
    const token = this.peek()
    if (!token) return null

    this.advance()

    const children = this.parseNodes(tagName)

    return {
      type: tagName.toLowerCase(),
      content: children,
      attrs: token.attrs || {},
    }
  }

  parse(): BBCodeNode[] {
    return this.parseNodes()
  }
}

/**
 * BBCode to HTML Renderer
 */
export class BBCodeRenderer {
  private nodeMap: Record<string, (node: BBCodeNode) => string>
  // todoCounter was used for index-based approach; with id-based todos it's unused

  constructor() {
    this.nodeMap = {
      text: (node) => {
        const escaped = this.escapeHtml(node.content as string)
        // Convert newlines to <br> tags
        return escaped.replace(/\n/g, '<br />')
      },
      b: (node) => `<strong>${this.renderContent(node.content)}</strong>`,
      bold: (node) => `<strong>${this.renderContent(node.content)}</strong>`,
      i: (node) => `<em>${this.renderContent(node.content)}</em>`,
      italic: (node) => `<em>${this.renderContent(node.content)}</em>`,
      u: (node) => `<u>${this.renderContent(node.content)}</u>`,
      underline: (node) => `<u>${this.renderContent(node.content)}</u>`,
      s: (node) => `<s>${this.renderContent(node.content)}</s>`,
      strike: (node) => `<s>${this.renderContent(node.content)}</s>`,
      h1: (node) => `<h1>${this.renderContent(node.content)}</h1>`,
      h2: (node) => `<h2>${this.renderContent(node.content)}</h2>`,
      h3: (node) => `<h3>${this.renderContent(node.content)}</h3>`,
      color: (node) =>
        `<span style="color: ${node.attrs?.value || 'inherit'}">${this.renderContent(node.content)}</span>`,
      bg: (node) =>
        `<span style="background-color: ${node.attrs?.value || 'inherit'}">${this.renderContent(node.content)}</span>`,
      highlight: (node) =>
        `<span style="background-color: ${node.attrs?.value || 'yellow'}">${this.renderContent(node.content)}</span>`,
      left: (node) => `<div style="text-align: left">${this.renderContent(node.content)}</div>`,
      center: (node) => `<div style="text-align: center">${this.renderContent(node.content)}</div>`,
      right: (node) => `<div style="text-align: right">${this.renderContent(node.content)}</div>`,
      justify: (node) => `<div style="text-align: justify">${this.renderContent(node.content)}</div>`,
      quote: (node) => `<blockquote>${this.renderContent(node.content)}</blockquote>`,
      code: (node) => {
        const content = typeof node.content === 'string' ? node.content : ''
        return `<code>${this.escapeHtml(content)}</code>`
      },
      pre: (node) => {
        const content = typeof node.content === 'string' ? node.content : ''
        return `<pre><code>${this.escapeHtml(content)}</code></pre>`
      },
      url: (node) => {
        const href = node.attrs?.value || '#'
        const text = this.renderContent(node.content)
        return `<a href="${this.escapeHtml(href)}" class="bbcode-url-link" target="_blank">${text}</a>`
      },
      link: (node) => {
        const href = node.attrs?.value || '#'
        const text = this.renderContent(node.content)
        return `<a href="${this.escapeHtml(href)}" class="bbcode-url-link" target="_blank">${text}</a>`
      },
      img: (node) => {
        const src = node.attrs?.value || ''
        const alt = typeof node.content === 'string' ? node.content : 'image'
        return `<img src="${this.escapeHtml(src)}" alt="${this.escapeHtml(alt)}" class="bbcode-image" style="max-width: 100%; height: auto; border-radius: 4px;" />`
      },
      note: (node) => {
        const noteId = node.attrs?.value || ''
        // Extract text content from children nodes
        const noteTitle = this.renderContent(node.content).replace(/<[^>]*>/g, '') // Strip HTML tags
        const displayText = noteTitle ? `[${noteTitle}]` : `[Note: ${noteId}]`
        return `<a href="javascript:void(0)" class="bbcode-link bbcode-note-link" data-note-id="${noteId}" onclick="window.__openNote?.('${noteId}'); return false">${displayText}</a>`
      },
      board: (node) => {
        const boardId = node.attrs?.value || ''
        // Extract text content from children nodes
        const boardTitle = this.renderContent(node.content).replace(/<[^>]*>/g, '') // Strip HTML tags
        const displayText = boardTitle ? `[${boardTitle}]` : `[Board: ${boardId}]`
        return `<a href="javascript:void(0)" class="bbcode-link bbcode-board-link" data-board-id="${boardId}" onclick="window.__openBoard?.('${boardId}'); return false">${displayText}</a>`
      },
      attachment: (node) => {
        const attachmentId = node.attrs?.value || ''
        // Extract text content from children nodes
        const attachmentTitle = this.renderContent(node.content).replace(/<[^>]*>/g, '') // Strip HTML tags
        const displayText = attachmentTitle ? `[${attachmentTitle}]` : `[Attachment: ${attachmentId}]`
        return `<a href="javascript:void(0)" class="bbcode-link bbcode-attachment-link" data-attachment-id="${attachmentId}" onclick="window.__openAttachment?.('${attachmentId}'); return false">${displayText}</a>`
      },
      todo: (node) => {
        // todo nodes must have an id attribute in the new syntax
        const todoId = node.attrs?.id || ''
        // Determine checked state from attrs.checked which should be '1' or '0'
        const rawChecked = node.attrs?.checked || '0'
        const checked = /^\s*1\s*$/.test(rawChecked)

        const inner = this.renderContent(node.content)
        const paneId = (this as any).paneId || ''

        // Render with stable todo id and call toggle by id
        return `<ul class="task-list"><li class="task-item" data-checked="${checked ? 'true' : 'false'}" data-todo-id="${this.escapeHtml(todoId)}"><label><input type="checkbox" ${checked ? 'checked' : ''} onclick="window.__toggleTodo && window.__toggleTodo('${paneId}', '${this.escapeHtml(todoId)}'); event.stopPropagation();" /></label><div>${inner}</div></li></ul>`
      },
      ul: (node) => `<ul class="bbcode-list">${this.renderContent(node.content)}</ul>`,
      ol: (node) => `<ol class="bbcode-list">${this.renderContent(node.content)}</ol>`,
      li: (node) => `<li>${this.renderContent(node.content)}</li>`,
      table: (node) => `<table class="bbcode-table"><tbody>${this.renderContent(node.content)}</tbody></table>`,
      tr: (node) => `<tr>${this.renderContent(node.content)}</tr>`,
      td: (node) => `<td>${this.renderContent(node.content)}</td>`,
      th: (node) => `<th>${this.renderContent(node.content)}</th>`,
      hr: () => '<hr />',
      br: () => '<br />',
    }
  }

  private escapeHtml(text: string): string {
    const div = document.createElement('div')
    div.textContent = text
    return div.innerHTML
  }

  private renderContent(content: string | BBCodeNode[] | undefined): string {
    if (!content) return ''
    if (typeof content === 'string') return this.escapeHtml(content)
    if (Array.isArray(content)) {
      return content.map((node) => this.renderNode(node)).join('')
    }
    return ''
  }

  private renderNode(node: BBCodeNode): string {
    const renderer = this.nodeMap[node.type]
    if (renderer) {
      return renderer(node)
    }
    // Unknown tag, render as text
    return this.renderContent(node.content)
  }

  render(nodes: BBCodeNode[]): string {
    // id-based todos: nothing to reset here
    return nodes.map((node) => this.renderNode(node)).join('')
  }
}

/**
 * Convert BBCode string to HTML
 */
export function bbcodeToHtml(bbcode: string, options?: { paneId?: string }): string {
  const parser = new BBCodeParser(bbcode)
  const nodes = parser.parse()
  const renderer = new BBCodeRenderer()
  // Pass paneId so todo checkboxes can call back to the correct editor instance
  ;(renderer as any).paneId = options?.paneId
  return renderer.render(nodes)
}

/**
 * Toggle the Nth todo (preorder) in the BBCode and propagate parent states.
 * Returns the new BBCode string.
 */
export function toggleTodoById(bbcode: string, todoId: string): string {
  const parser = new BBCodeParser(bbcode)
  const nodes = parser.parse()

  // Find node with attrs.id === todoId and toggle its attrs.checked between '0' and '1'
  function toggleNode(list: BBCodeNode[]): boolean {
    for (const node of list) {
      if (node.type === 'todo' && node.attrs?.id === todoId) {
        node.attrs = node.attrs || {}
        node.attrs.checked = node.attrs.checked === '1' ? '0' : '1'
        return true
      }
      if (Array.isArray(node.content)) {
        const found = toggleNode(node.content)
        if (found) return true
      }
    }
    return false
  }

  toggleNode(nodes)

  // After toggling, propagate parent check states: a todo is checked if ALL child todo nodes are checked
  function propagate(list: BBCodeNode[]) {
    for (const node of list) {
      if (Array.isArray(node.content)) {
        propagate(node.content)
        if (node.type === 'todo') {
          // collect all descendant todos
          const childTodos: BBCodeNode[] = []
          function collect(nList: BBCodeNode[]) {
            for (const n of nList) {
              if (n.type === 'todo') childTodos.push(n)
              if (Array.isArray(n.content)) collect(n.content)
            }
          }
          collect(node.content)
          if (childTodos.length > 0) {
            const allChecked = childTodos.every(t => (t.attrs?.checked || '0') === '1')
            node.attrs = node.attrs || {}
            node.attrs.checked = allChecked ? '1' : '0'
          }
        }
      }
    }
  }

  propagate(nodes)

  // Convert nodes back to BBCode with named attributes as key="value"
  function nodesToBBCode(list: BBCodeNode[]): string {
    return list.map(node => nodeToBBCode(node)).join('')
  }

  function escapeAttr(v: string): string {
    return v.replace(/"/g, '\\"')
  }

  function nodeToBBCode(node: BBCodeNode): string {
    if (node.type === 'text') return (node.content as string) || ''
    const attrs = node.attrs ? ' ' + Object.entries(node.attrs).map(([k, v]) => `${k}="${escapeAttr(v || '')}"`).join(' ') : ''
    const inner = Array.isArray(node.content) ? nodesToBBCode(node.content) : (node.content as string || '')
    return `[${node.type}${attrs}]${inner}[/${node.type}]`
  }

  return nodesToBBCode(nodes)
}

/**
 * Wrap selected text with BBCode tags
 */
export function wrapWithBBCode(text: string, tag: string, value?: string): string {
  if (value) {
    return `[${tag}=${value}]${text}[/${tag}]`
  }
  return `[${tag}]${text}[/${tag}]`
}

/**
 * Insert BBCode tag at position
 */
export function insertBBCodeTag(text: string, position: number, tag: string, value?: string, isClosing = false): string {
  const tagStr = isClosing ? `[/${tag}]` : value ? `[${tag}=${value}]` : `[${tag}]`
  return text.slice(0, position) + tagStr + text.slice(position)
}

/**
 * Get selected text range info
 */
export function getTextSelection(textarea: HTMLTextAreaElement): { start: number; end: number; text: string } {
  return {
    start: textarea.selectionStart,
    end: textarea.selectionEnd,
    text: textarea.value.substring(textarea.selectionStart, textarea.selectionEnd),
  }
}

/**
 * Apply BBCode formatting to selected text
 */
export function applyBBCodeFormat(
  textarea: HTMLTextAreaElement,
  tag: string,
  value?: string
): void {
  const selection = getTextSelection(textarea)
  const openTag = value ? `[${tag}=${value}]` : `[${tag}]`
  const closeTag = `[/${tag}]`

  if (selection.start === selection.end) {
    // No selection, insert both open and close tags
    textarea.value = textarea.value.slice(0, selection.start) + openTag + closeTag + textarea.value.slice(selection.start)
    // Position cursor between tags
    textarea.selectionStart = selection.start + openTag.length
    textarea.selectionEnd = selection.start + openTag.length
  } else {
    // Wrap selected text
    const before = textarea.value.substring(0, selection.start)
    const after = textarea.value.substring(selection.end)
    const wrapped = openTag + selection.text + closeTag
    textarea.value = before + wrapped + after

    // Restore selection to wrapped text
    textarea.selectionStart = selection.start + openTag.length
    textarea.selectionEnd = selection.start + openTag.length + selection.text.length
  }

  // Trigger change event
  textarea.dispatchEvent(new Event('input', { bubbles: true }))
}

/**
 * Check if tag is currently active at cursor position
 */
export function isTagActive(textarea: HTMLTextAreaElement, tag: string): boolean {
  const selection = getTextSelection(textarea)
  const text = textarea.value

  // Look backwards for opening tag
  let depth = 0
  for (let i = selection.start - 1; i >= 0; i--) {
    if (text.substring(i).startsWith(`[/${tag}]`)) {
      depth--
      i -= tag.length + 3
    } else if (text.substring(i).startsWith(`[${tag}`)) {
      if (depth === 0) return true
      depth++
    }
  }

  return false
}
