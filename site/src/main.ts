import './styles.css'
import { transit_realtime } from 'gtfs-realtime-bindings'

async function getRouteInfo(routeId: string) {
  try {
    const response = await fetch(`/routes/${routeId}`)    
    console.log('Status:', response.status)  // add this
    if (!response.ok) {
      throw new Error(`Error: status ${response.status}`)
    }

    const buffer = await response.arrayBuffer()
    const feed = transit_realtime.FeedMessage.decode(new Uint8Array(buffer))
    return feed
  } catch {
    console.error('Failed to fetch vehicle positions')
    return null
  }
}

// Example usage
const feed = await getRouteInfo('61C')
console.log(feed)
