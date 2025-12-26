import type { Card } from './card'
import type { BoardLabel } from './label'
import type { AutomationRule } from './automation'

// Board folder types
export interface BoardFolder {
  id: string
  user_id: string
  name: string
  parent_id: string | null
  position: number
  is_important: boolean
  is_urgent: boolean
  created_at: string
  updated_at: string
}

// Board types
export interface Board {
  id: string
  user_id: string
  name: string
  description?: string
  color?: string
  folder_id?: string | null
  position?: number
  is_important: boolean
  is_urgent: boolean
  created_at: string
  updated_at: string
}

export interface List {
  id: string
  board_id: string
  name: string
  position: number
  archived: boolean
  created_at: string
  updated_at: string
}

export interface ListWithCards {
  list: List
  cards: Card[]
}

export interface BoardWithLists {
  board: Board
  lists: ListWithCards[]
  labels: BoardLabel[]
  automations: AutomationRule[]
}

export interface BoardsTree {
  folders: BoardFolder[]
  boards: Board[]
}
