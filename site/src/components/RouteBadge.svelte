<script lang="ts">
  import type { Route } from '../types'
  export let route: Route
</script>

<span
  class="route-badge"
  style={route.route_color
    ? `background: #${route.route_color}; color: #${route.route_text_color ?? 'ffffff'}`
    : ''}
>
  {route.route_short_name ?? route.route_id}
</span>
