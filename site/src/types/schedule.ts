export interface ServiceException {
  date: string
  added: boolean
}

export interface ServiceInfo {
  service_id: string
  days: string[]
  start_date: string
  end_date: string
  exceptions: ServiceException[]
}

export interface Timetable {
  service_id: string
  service_info: ServiceInfo
  stop_pattern: string[]
  stops: string[]
  trips: string[]
  cells: (string | null)[][]
}

export interface Schedule {
  route_id: string
  timetables: Timetable[]
}
