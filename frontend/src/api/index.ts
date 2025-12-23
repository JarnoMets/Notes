// Re-export all API modules
import api from './client'

export { foldersApi, notesApi } from './notes'
export { boardsApi, boardFoldersApi } from './boards'
export { listsApi } from './lists'
export { cardsApi } from './cards'
export { labelsApi } from './labels'
export { automationsApi } from './automations'
export { settingsApi } from './settings'
export { remindersApi } from './reminders'
export { graphsApi, graphFoldersApi, graphNodesApi, graphEdgesApi } from './graphs'

export default api
