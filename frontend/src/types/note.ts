// Notes types
export interface NoteFolder {
  id: string
  user_id: string
  parent_id: string | null
  name: string
  position: number
  is_important: boolean
  is_urgent: boolean
  created_at: string
  updated_at: string
}

export interface Note {
  id: string
  user_id: string
  folder_id: string | null
  title: string
  description: string
  content: string
  position: number
  is_important: boolean
  is_urgent: boolean
  created_at: string
  updated_at: string
}

export interface NoteAttachment {
  id: string
  note_id: string
  filename: string
  original_filename: string
  mime_type: string
  size: number
  created_at: string
}

export interface NoteWithAttachments extends Note {
  attachments: NoteAttachment[]
}

export interface NotesTree {
  folders: NoteFolder[]
  notes: Note[]
}

// File tree types for UI
export interface FileTreeItem {
  id: string
  name: string
  type: 'folder' | 'note'
  parentId: string | null
  position: number
  isImportant: boolean
  children?: FileTreeItem[]
  isExpanded?: boolean
}
