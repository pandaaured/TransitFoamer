import type { Schedule, Timetable } from './types'

function renderTimetable(timetable: Timetable): HTMLElement {
  const section = document.createElement('section')
  section.className = 'timetable'

  const heading = document.createElement('h2')
  heading.textContent = `Service: ${timetable.service_id} — ${timetable.trips.length} trips, ${timetable.stops.length} stops`
  section.appendChild(heading)

  const info = timetable.service_info
  const meta = document.createElement('p')
  meta.className = 'service-meta'
  if (info.days.length > 0) {
    meta.textContent = `${info.days.join(', ')} · ${info.start_date} – ${info.end_date}`
  } else {
    meta.textContent = `Exception service · ${info.exceptions.length} date(s)`
  }
  section.appendChild(meta)

  const wrapper = document.createElement('div')
  wrapper.className = 'table-wrapper'
  const table = document.createElement('table')

  const thead = document.createElement('thead')
  const headerRow = document.createElement('tr')
  const corner = document.createElement('th')
  corner.textContent = 'Trip'
  headerRow.appendChild(corner)
  timetable.stops.forEach((stopName) => {
    const th = document.createElement('th')
    th.textContent = stopName
    th.title = stopName
    headerRow.appendChild(th)
  })
  thead.appendChild(headerRow)
  table.appendChild(thead)

  const tbody = document.createElement('tbody')
  timetable.trips.forEach((tripId, tripIdx) => {
    const row = document.createElement('tr')
    const tripCell = document.createElement('td')
    tripCell.className = 'trip-id'
    tripCell.textContent = tripId
    row.appendChild(tripCell)
    timetable.stops.forEach((_, stopIdx) => {
      const time = timetable.cells[stopIdx][tripIdx]
      const td = document.createElement('td')
      td.className = time ? 'time' : 'no-stop'
      td.textContent = time ?? '—'
      row.appendChild(td)
    })
    tbody.appendChild(row)
  })
  table.appendChild(tbody)

  wrapper.appendChild(table)
  section.appendChild(wrapper)
  return section
}

export function renderSchedule(schedule: Schedule, container: HTMLElement): void {
  container.innerHTML = ''
  const title = document.createElement('h1')
  title.textContent = `Route ${schedule.route_id} Schedule`
  container.appendChild(title)

  if (schedule.timetables.length === 0) {
    const msg = document.createElement('p')
    msg.textContent = 'No timetables found for this route.'
    container.appendChild(msg)
    return
  }

  schedule.timetables.forEach((timetable) => {
    container.appendChild(renderTimetable(timetable))
  })
}
