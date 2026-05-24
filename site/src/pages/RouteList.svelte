<script lang="ts">
  import { onMount } from 'svelte'
  import type { Route } from '../types'
  import { getRoutes } from '../api/routes'
  import RouteBadge from '../components/RouteBadge.svelte'

  let routes: Route[] = []
  let search = ''

  $: filtered = routes
    .filter(r =>
      (r.route_short_name ?? r.route_id).toLowerCase().includes(search.toLowerCase()) ||
      (r.route_long_name ?? '').toLowerCase().includes(search.toLowerCase())
    )
    .sort((a, b) => {
      const aNum = parseInt(a.route_short_name ?? a.route_id)
      const bNum = parseInt(b.route_short_name ?? b.route_id)
      if (!isNaN(aNum) && !isNaN(bNum)) return aNum - bNum
      return (a.route_short_name ?? a.route_id).localeCompare(b.route_short_name ?? b.route_id)
    })

  onMount(async () => {
    routes = await getRoutes()
  })
</script>

<div style="padding: 16px">
  <h1>All Routes</h1>
  <input
    type="text"
    placeholder="Search routes..."
    bind:value={search}
    style="margin: 12px 0; padding: 8px 12px; font-size: 14px; border: 1px solid #ddd; border-radius: 6px; width: 240px;"
  />
</div>

<div class="route-grid">
  {#each filtered as route (route.route_id)}
    <a class="route-card" href="#/schedule/{route.route_id}">
      <RouteBadge {route} />
      <span class="route-name">{route.route_long_name ?? ''}</span>
    </a>
  {/each}
</div>
