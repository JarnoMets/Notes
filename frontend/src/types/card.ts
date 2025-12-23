// Card types
export interface Card {
  id: string
  list_id: string
  title: string
  description?: string
  position: number
  due_date?: string
  labels: string[]
  archived: boolean
  status?: string
  created_at: string
  updated_at: string
}

export interface CardAttachment {
  id: string
  card_id: string
  filename: string
  original_filename: string
  mime_type: string
  size: number
  created_at: string
}

export interface CardWithAttachments extends Card {
  attachments: CardAttachment[]
}
