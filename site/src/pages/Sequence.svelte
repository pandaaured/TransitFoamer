<script lang="ts">
  import { onMount } from 'svelte'
  import type { UniqueSequence } from '../types'
  import { getUniqueSequences } from '../api/sequences'

  export let routeId: string

  let sequences: UniqueSequence[] = []
  let loading = true

  onMount(async () => {
    sequences = await getUniqueSequences(routeId)
    loading = false
  })
</script>

<div style="padding: 0 16px">
  <a href="#/schedule/{routeId}" class="back-link">← Back to Schedule</a>
</div>

<h1 style="padding: 24px 16px 8px">Route {routeId} Stop Patterns</h1>

{#if loading}
  <p style="padding: 16px">Loading diagram...</p>
{:else if sequences.length === 0}
  <p style="padding: 16px">No sequences found for this route.</p>
{:else}
  <div class="sequence-wrapper">
    {#each sequences as seq, colIdx}
      <div class="sequence-column">
        <div class="sequence-heading">Pattern {colIdx + 1}</div>
        <div class="sequence-line">
          {#each seq.stops as [_stopId, stopName, _seqNum]}
            <div class="sequence-stop">
              <div class="sequence-dot"></div>
              <span class="sequence-label">{stopName}</span>
            </div>
          {/each}
        </div>
      </div>
    {/each}
  </div>
{/if}
