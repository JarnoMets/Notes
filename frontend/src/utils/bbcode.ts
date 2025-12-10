/**
 * BBCode Parser and Renderer for Notes, Boards, and Attachments
 * Handles parsing [note]id[/note], [board]id[/board], and [attachment]id[/attachment] tags
 */

interface ParsedBBCode {
  type: 'text' | 'note-link' | 'board-link' | 'attachment-link'
  content: string
  id?: string // Referenced note/board/attachment ID
}

/**
 * Parse BBCode in content and return structured data
 */
export function parseBBCode(content: string): ParsedBBCode[] {
  const result: ParsedBBCode[] = []
  const regex = /\[(?:note|board|attachment)\]([^\[]+?)\[\/(?:note|board|attachment)\]/g
  let lastIndex = 0
  let match

  while ((match = regex.exec(content)) !== null) {
    // Add text before the match
    if (match.index > lastIndex) {
      result.push({
        type: 'text',
        content: content.substring(lastIndex, match.index),
      })
    }

    // Determine the type
    const tag = match[0].startsWith('[note]') ? 'note-link' : match[0].startsWith('[board]') ? 'board-link' : 'attachment-link'
    const id = match[1]

    result.push({
      type: tag as 'note-link' | 'board-link' | 'attachment-link',
      content: id,
      id: id,
    })

    lastIndex = regex.lastIndex
  }

  // Add remaining text
  if (lastIndex < content.length) {
    result.push({
      type: 'text',
      content: content.substring(lastIndex),
    })
  }

  return result.length > 0 ? result : [{ type: 'text', content }]
}

/**
 * Find and extract all BBCode references from content
 */
export function extractBBCodeReferences(content: string): Array<{
  id: string
  type: 'note' | 'board' | 'attachment'
  display: string
}> {
  const references: Array<{ id: string; type: 'note' | 'board' | 'attachment'; display: string }> = []
  const noteRegex = /\[note\]([^\[]+?)\[\/note\]/g
  const boardRegex = /\[board\]([^\[]+?)\[\/board\]/g
  const attachmentRegex = /\[attachment\]([^\[]+?)\[\/attachment\]/g

  let match
  while ((match = noteRegex.exec(content)) !== null) {
    references.push({
      id: match[1],
      type: 'note',
      display: `[${match[1]}]`,
    })
  }

  while ((match = boardRegex.exec(content)) !== null) {
    references.push({
      id: match[1],
      type: 'board',
      display: `[${match[1]}]`,
    })
  }

  while ((match = attachmentRegex.exec(content)) !== null) {
    references.push({
      id: match[1],
      type: 'attachment',
      display: `[${match[1]}]`,
    })
  }

  return references
}

/**
 * Insert a note, board, or attachment reference using BBCode format
 */
export function insertBBCodeReference(id: string, type: 'note' | 'board' | 'attachment'): string {
  const tag = type === 'note' ? 'note' : type === 'board' ? 'board' : 'attachment'
  return `[${tag}]${id}[/${tag}]`
}

/**
 * Convert HTML internal links to BBCode format
 */
export function htmlLinksTobbcode(html: string): string {
  // Convert note:// links
  html = html.replace(
    /<a[^>]*href=["']notes:\/\/([^"']+)["'][^>]*>([^<]*)<\/a>/g,
    '[note]$1[/note]'
  )

  // Convert board:// links
  html = html.replace(
    /<a[^>]*href=["']boards:\/\/([^"']+)["'][^>]*>([^<]*)<\/a>/g,
    '[board]$1[/board]'
  )

  // Convert attachment:// links
  html = html.replace(
    /<a[^>]*href=["']attachments:\/\/([^"']+)["'][^>]*>([^<]*)<\/a>/g,
    '[attachment]$1[/attachment]'
  )

  return html
}

/**
 * Convert BBCode to HTML links (for backward compatibility if needed)
 */
export function bbcodeToHtmlLinks(content: string): string {
  // Convert [note]id[/note] to HTML
  content = content.replace(
    /\[note\]([^\[]+?)\[\/note\]/g,
    '<a href="notes://$1" class="internal-link note-link">[$1]</a>'
  )

  // Convert [board]id[/board] to HTML
  content = content.replace(
    /\[board\]([^\[]+?)\[\/board\]/g,
    '<a href="boards://$1" class="internal-link board-link">[$1]</a>'
  )

  // Convert [attachment]id[/attachment] to HTML
  content = content.replace(
    /\[attachment\]([^\[]+?)\[\/attachment\]/g,
    '<a href="attachments://$1" class="internal-link attachment-link">[$1]</a>'
  )

  return content
}

/**
 * Remove all BBCode references to a specific attachment from content
 */
export function removeAttachmentReferences(content: string, attachmentId: string): string {
  // Remove [attachment]attachmentId[/attachment] tags
  const regex = new RegExp(`\\[attachment\\]${escapeRegex(attachmentId)}\\[\\/attachment\\]`, 'g')
  return content.replace(regex, '')
}

/**
 * Escape special regex characters in a string
 */
function escapeRegex(str: string): string {
  return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}
