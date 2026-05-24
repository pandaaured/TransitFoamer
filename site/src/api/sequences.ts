import type { UniqueSequence } from '../types'

export async function getUniqueSequences(routeId: string): Promise<UniqueSequence[]> {
  try {
    const response = await fetch(`/sequences/${routeId}`)
    if (!response.ok) throw new Error(`Status ${response.status}`)
    return await response.json() as UniqueSequence[]
  } catch {
    console.error(`Failed to fetch sequences for route ${routeId}`)
    return []
  }
}
