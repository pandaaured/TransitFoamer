<script lang="ts">
  import { onMount } from 'svelte'
  import Header from './components/Header.svelte'
  import Home from './pages/Home.svelte'
  import RouteList from './pages/RouteList.svelte'
  import Schedule from './pages/Schedule.svelte'
  import Sequence from './pages/Sequence.svelte'

  let hash = window.location.hash

  onMount(() => {
    window.addEventListener('hashchange', () => {
      hash = window.location.hash
    })
  })

  $: routeId = hash.startsWith('#/schedule/')
    ? hash.replace('#/schedule/', '')
    : hash.startsWith('#/diagram/')
    ? hash.replace('#/diagram/', '')
    : null
</script>

<Header />

{#if hash.startsWith('#/schedule/') && routeId}
  <Schedule {routeId} />
{:else if hash.startsWith('#/diagram/') && routeId}
  <Sequence {routeId} />
{:else if hash === '#/routes'}
  <RouteList />
{:else}
  <Home />
{/if}
