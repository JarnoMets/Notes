import api from './client'

export interface IcsCalendar {
  id: string
  name: string
  url: string
  color: string
  enabled: boolean
}

export interface SettingsResponse {
  theme: string
  week_starts_on_monday: boolean
  ics_calendars: IcsCalendar[]
}

export interface UpdateSettingsRequest {
  theme?: string
  week_starts_on_monday?: boolean
  ics_calendars?: IcsCalendar[]
}

export const settingsApi = {
  get: () => api.get<SettingsResponse>('/settings'),
  update: (data: UpdateSettingsRequest) => api.put<SettingsResponse>('/settings', data)
}
