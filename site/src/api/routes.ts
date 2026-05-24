import type { Route } from '../types'

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
