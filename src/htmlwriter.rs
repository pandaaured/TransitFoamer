use gtfs_realtime::{FeedMessage, FeedEntity};

use crate::gtfs_rt::on_route;

// This function takes a feedmessage and returns an HTML formatted string
// with all of the vehicles on a given route.
// Pre: requires the passed data to be a TripUpdate feed entry.
fn vehicles_on_route_to_html(vehicles: FeedMessage, route: &str) {
    let mut string: String = "".to_string();
    let filtered_msg: FeedMessage = on_route(route, vehicles);
    let vec: Vec<FeedEntity> = filtered_msg.entity;

}
