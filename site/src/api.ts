import type { Schedule  } from './types'
import type { Route } from './types'

/*
Returns a promise to the GTFS Static data associated with routes.txt.
 */
export async function getRoutes(retries = 5, delayMs = 1000): Promise<Route[]> {
  for (let i = 0; i < retries; i++) {
    try {
      const response = await fetch('/rtlist')
      if (!response.ok) throw new Error(`Status ${response.status}`)
      return await response.json() as Route[]
    } catch {
      if (i < retries - 1) {
        await new Promise(res => setTimeout(res, delayMs))
      }
    }
  }
  console.error('Failed to fetch routes after retries')
  return []
}

/*
Returns a promise to the schedule table associated with GTFS Static data
associated with stop_times.txt for a given route.
*/
export async function getSchedule(routeId: string): Promise<Schedule | null> {
  try {
    const response = await fetch(`/schedule/${routeId}`)
    if (!response.ok) throw new Error(`Error: status ${response.status}`)
    return await response.json() as Schedule
  } catch {
    console.error(`Failed to fetch schedule for route ${routeId}`)
    return null
  }
}
