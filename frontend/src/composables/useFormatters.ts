/**
 * Composable for common formatting utilities
 */

/**
 * Format a date string to a human-readable format
 */
export function formatDate(dateStr: string, options?: Intl.DateTimeFormatOptions): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('en-US', options || { 
    month: 'short', 
    day: 'numeric',
    year: 'numeric'
  })
}

/**
 * Format a date for display in cards (short format)
 */
export function formatShortDate(dateStr: string): string {
  const date = new Date(dateStr)
  return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' })
}

/**
 * Check if a date is in the past
 */
export function isOverdue(dateStr: string): boolean {
  return new Date(dateStr) < new Date()
}

/**
 * Format file size in bytes to human-readable format
 */
export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return bytes + ' B'
  if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB'
  if (bytes < 1024 * 1024 * 1024) return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
  return (bytes / (1024 * 1024 * 1024)).toFixed(1) + ' GB'
}

/**
 * Truncate text with ellipsis
 */
export function truncateText(text: string, maxLength: number): string {
  if (!text) return ''
  return text.length > maxLength ? text.substring(0, maxLength) + '...' : text
}

/**
 * Get preview text from content
 */
export function getPreview(text: string, maxLength = 80): string {
  return truncateText(text, maxLength)
}

/**
 * Get description preview (longer format)
 */
export function getDescriptionPreview(description: string): string {
  if (!description) return 'No description'
  return truncateText(description, 150)
}
