/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare global {
  interface Window {
    __openNote?: (noteId: string) => void
    __openBoard?: (boardId: string) => void
  }
}
