/**
 * Composable for file utilities
 */

/**
 * Check if a MIME type represents an image
 */
export function isImageMimeType(mimeType: string): boolean {
  return mimeType.startsWith('image/')
}

/**
 * Get an icon name based on MIME type
 */
export function getFileIcon(mimeType: string): string {
  if (mimeType.startsWith('image/')) return 'image'
  if (mimeType.startsWith('video/')) return 'video'
  if (mimeType.startsWith('audio/')) return 'music'
  if (mimeType.includes('pdf')) return 'file-text'
  if (mimeType.includes('word') || mimeType.includes('document')) return 'file-text'
  if (mimeType.includes('sheet') || mimeType.includes('excel')) return 'table'
  if (mimeType.includes('zip') || mimeType.includes('archive')) return 'archive'
  return 'paperclip'
}
