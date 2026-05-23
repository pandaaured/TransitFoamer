import './styles.css'
import { getSchedule, getRoutes } from './api'
import { renderSchedule } from './render'
import { renderRouteList } from './routes-page'

const container = document.getElementById('app')!

async function navigate() {
  const hash = window.location.hash

  if (hash.startsWith('#/schedule/')) {
    const routeId = hash.replace('#/schedule/', '')
    container.innerHTML = '<p>Loading schedule...</p>'
    const schedule = await getSchedule(routeId)
    if (schedule) {
      renderSchedule(schedule, container)
    } else {
      container.innerHTML = '<p>Failed to load schedule.</p>'
    }
  } else {
    // Default: route list
    container.innerHTML = '<p>Loading routes...</p>'
    const routes = await getRoutes()
    renderRouteList(routes, container)
  }
}

window.addEventListener('hashchange', navigate)
navigate()
