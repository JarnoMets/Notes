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
  created_at: string
  updated_at: string
}
