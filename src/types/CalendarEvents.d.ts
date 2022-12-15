export type CalendarEvents = CalendarEvent[]

export interface CalendarEvent {
  alarms: any[]
  events: Event[]
  free_busys: any[]
  journals: any[]
  properties: GlobalProperty[]
  timezones: Timezone[]
  todos: any[]
}

export interface Event {
  alarms: Alarm[]
  properties: EventProperty[]
}

export interface Alarm {
  properties: AlarmProperty[]
}

export interface AlarmProperty {
  name: string
  params: any
  value: string
}

export interface EventProperty {
  name: string
  params?: [string, string[]][]
  value: string
}

export interface GlobalProperty {
  name: string
  params: any
  value: string
}

export interface Timezone {
  properties: TimezoneProperty[]
  transitions: Transition[]
}

export interface TimezoneProperty {
  name: string
  params: any
  value: string
}

export interface Transition {
  properties: TransitionProperty[]
}

export interface TransitionProperty {
  name: string
  params: any
  value: string
}
