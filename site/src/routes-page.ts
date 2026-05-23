import type { Route } from './types'

export function renderRouteList(routes: Route[], container: HTMLElement): void {
  container.innerHTML = ''

  const title = document.createElement('h1')
  title.textContent = 'All Routes'
  container.appendChild(title)

  const list = document.createElement('ul')
  list.className = 'route-list'

  routes
    .sort((a, b) => {
      // Sort numerically where possible, fall back to string
      const aNum = parseInt(a.route_short_name ?? a.route_id)
      const bNum = parseInt(b.route_short_name ?? b.route_id)
      if (!isNaN(aNum) && !isNaN(bNum)) return aNum - bNum
      return (a.route_short_name ?? a.route_id).localeCompare(b.route_short_name ?? b.route_id)
    })
    .forEach((route) => {
      const li = document.createElement('li')
      li.className = 'route-item'

      const badge = document.createElement('span')
      badge.className = 'route-badge'
      badge.textContent = route.route_short_name ?? route.route_id
      if (route.route_color) {
        badge.style.background = `#${route.route_color}`
        badge.style.color = `#${route.route_text_color ?? 'ffffff'}`
      }

      const name = document.createElement('span')
      name.className = 'route-name'
      name.textContent = route.route_long_name ?? ''

      const link = document.createElement('a')
      link.href = `/schedule/${route.route_id}`
      link.appendChild(badge)
      link.appendChild(name)
      li.appendChild(link)
      list.appendChild(li)
    })

  container.appendChild(list)
}
