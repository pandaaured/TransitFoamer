<script lang="ts">
  import type { Timetable } from '../types'
  export let timetable: Timetable
</script>

<section class="timetable">
  <h2>
    Service: {timetable.service_id} —
    {timetable.trips.length} trips, {timetable.stops.length} stops
  </h2>
  <p class="service-meta">
    {#if timetable.service_info.days.length > 0}
      {timetable.service_info.days.join(', ')} ·
      {timetable.service_info.start_date} – {timetable.service_info.end_date}
    {:else}
      Exception service · {timetable.service_info.exceptions.length} date(s)
    {/if}
  </p>

  <div class="table-wrapper">
    <table>
      <thead>
        <tr>
          <th>Trip</th>
          {#each timetable.stops as stop}
            <th title={stop}>{stop}</th>
          {/each}
        </tr>
      </thead>
      <tbody>
        {#each timetable.trips as tripId, tripIdx}
          <tr>
            <td>{tripId}</td>
            {#each timetable.stops as _, stopIdx}
              {@const time = timetable.cells[stopIdx][tripIdx]}
              <td class={time ? 'time' : 'no-stop'}>{time ?? '—'}</td>
            {/each}
          </tr>
        {/each}
      </tbody>
    </table>
  </div>
</section>
