import type { Schedule } from '../types'

export async function getSchedule(routeId: string): Promise<Schedule | null> {
  try {
    const response = await fetch(`/schedule/${routeId}`)
    if (!response.ok) throw new Error(`Status ${response.status}`)
    return await response.json() as Schedule
  } catch {
    console.error(`Failed to fetch schedule for route ${routeId}`)
    return null
  }
}
