pub mod list {
    use crate::gtfs::gtfsstatic::data;
    use std::collections::{HashMap, HashSet};

    pub fn trips_per_route(city: &str) -> HashMap<String, Vec<String>> {
        let mut trips_per_route: HashMap<String, Vec<String>> = HashMap::new();
        let trips: HashMap<String, Vec<String>> = data::static_data(city, "trips");
        for item in trips {
            let route = item.1[1].clone();
            let trip_id = item.0;
            if trips_per_route.contains_key(&route.clone()) {
                let mut vector = trips_per_route[&route.clone()].clone();
                vector.push(trip_id.clone());
                trips_per_route.insert(route.clone(), vector.clone());
            } else {
                trips_per_route.insert(route, vec![trip_id]);
            }
        }

        trips_per_route
    }

    pub fn stops_per_trip(city: &str) -> HashMap<String, Vec<String>> {
        let mut stops_per_trip: HashMap<String, Vec<String>> = HashMap::new();
        let stop_times: HashMap<String, Vec<String>> = data::static_data(city, "stop_times");
        for item in stop_times {
            let stop_id = item.1[3].clone();
            let trip_id = item.1[0].clone();
            if stops_per_trip.contains_key(&trip_id.clone()) {
                let mut vector = stops_per_trip[&trip_id.clone()].clone();
                vector.push(stop_id.clone());
                stops_per_trip.insert(trip_id.clone(), vector.clone());
            } else {
                stops_per_trip.insert(trip_id, vec![stop_id]);
            }
        }
        stops_per_trip
    }

    pub fn stops_per_route(city: &str) -> HashMap<String, HashSet<String>> {
        let mut stops_per_route: HashMap<String, HashSet<String>> = HashMap::new();

        let static_stop_times: HashMap<String, Vec<String>> = data::static_data(city, "stop_times");
        let static_trips: HashMap<String, Vec<String>> = data::static_data(city, "trips");

        let keys = static_stop_times.keys();

        for key in keys {
            let line = &static_stop_times[key];
            let trip_id = &line[0];
            let stop_id = &line[3];
            let route = static_trips[trip_id][1].clone();
            if !stops_per_route.contains_key(stop_id) {
                let mut set = HashSet::new();
                set.insert(route);
                stops_per_route.insert(stop_id.to_string(), set);
            } else {
                let mut set = stops_per_route[stop_id].clone();
                set.insert(route);
                stops_per_route.insert(stop_id.to_string(), set);
            }
        }

        stops_per_route
    }
}
