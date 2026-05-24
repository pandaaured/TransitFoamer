<script lang="ts">
  import { onMount } from 'svelte'
  import type { Schedule } from '../types'
  import { getSchedule } from '../api/schedule'
  import Timetable from './Timetable.svelte'

  export let routeId: string

  let schedule: Schedule | null = null
  let loading = true

  onMount(async () => {
    schedule = await getSchedule(routeId)
    loading = false
  })
</script>

<div style="padding: 0 16px">
  <a href="#/routes" class="back-link">← All Routes</a>
  <a href="#/diagram/{routeId}" class="back-link" style="margin-left: 16px">View Stop Diagram →</a>
</div>

{#if loading}
  <p style="padding: 16px">Loading schedule...</p>
{:else if schedule}
  <h1 style="padding: 24px 16px 8px">Route {schedule.route_id} Schedule</h1>
  {#each schedule.timetables as timetable}
    <Timetable {timetable} />
  {/each}
{:else}
  <p style="padding: 16px">Failed to load schedule.</p>
{/if}
