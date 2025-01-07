pub mod fetch {
    pub mod any_city {
        use crate::{gtfsrt, gtfsstatic, gtfsstatic::data, search::utilities};
        use chrono::Local;
        use gtfs_realtime::{trip_update::StopTimeUpdate, TripUpdate};
        use std::collections::HashMap;

        pub async fn at_stop(stop: &str, city: &str) {
            // ---- STATIC DATA LAYER 1 ---- //
            let static_stops: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "stops");
            let static_trips: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "trips");

            // ---- REALTIME DATA ---- //
            // let buses = gtfsrt::requester(city, "vehicles").await;
            let trips = gtfsrt::requester(city, "trips").await;

            // Let's check if it's a valid stop!
            if !static_stops.contains_key(stop) {
                panic!("Oops! The stop {stop} is not in the static feed definition.");
            }

            let routes_per_stop = data::processing_layer_two::routes_per_stop(city);
            let mut key = String::new();
            key.push_str(stop);

            let stop_name = &static_stops[stop][gtfsstatic::bindings::stops(city, "stop_name")];

            let routes = &routes_per_stop[&key];
            println!("------------ STOP {stop}: {stop_name} ------------");
            // println!("{:?}", routes); // List of routes serviced by the stop.

            for entity in trips.entity {
                let trip_update = entity.trip_update.unwrap();
                if routes.contains(&trip_update.clone().trip.route_id.unwrap()) {
                    let stop_seq = trip_update.clone().stop_time_update;
                    for stop_in_feed in stop_seq {
                        if stop_in_feed.stop_id.unwrap() == stop {
                            let vehicle_id: String =
                                trip_update.clone().vehicle.unwrap().id.unwrap();
                            let route: String = trip_update.clone().trip.route_id.unwrap();
                            let time = stop_in_feed.arrival.unwrap().time.unwrap();
                            let trip_id: String = trip_update.clone().trip.trip_id.unwrap();
                            let formatted: chrono::DateTime<Local> =
                                utilities::time_converter(time);
                            let static_stops_trip_id = &static_trips[&trip_id];
                            let headsign = static_stops_trip_id
                                [gtfsstatic::bindings::trips(city, "trip_headsign")]
                            .clone();
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} arrives at {formatted} \x1b[0m");
                        }
                    }
                }
            }
        }

        pub async fn on_route(number: &str, city: &str) {
            // ---- STATIC DATA LAYER 1 ---- //
            let static_stops: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "stops");
            let static_trips: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "trips");

            // ---- REALTIME DATA ---- //
            // let buses = gtfsrt::requester(city, "vehicles").await;
            let trips = gtfsrt::requester(city, "trips").await;

            for trip in trips.entity {
                let unit: TripUpdate = trip.trip_update.unwrap();
                let route: String = unit.trip.route_id.unwrap();
                if number == route {
                    let vehicle_id: String = unit.vehicle.unwrap().id.unwrap();
                    let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update.clone();
                    if !stop_time_update.is_empty() {
                        let first_stop: StopTimeUpdate = stop_time_update[0].clone();
                        let stop_id: String = first_stop.stop_id.unwrap().clone();
                        let stop_name =
                            &static_stops[&stop_id][gtfsstatic::bindings::stops(city, "stop_name")];
                        let trip_id: String = unit.trip.trip_id.unwrap();
                        let static_stops_trip_id = &static_trips[&trip_id];
                        let headsign = static_stops_trip_id
                            [gtfsstatic::bindings::trips(city, "trip_headsign")]
                        .clone();
                        if first_stop.arrival.is_some() {
                            let arrival = first_stop.arrival.unwrap().time.unwrap();
                            let formatted: chrono::DateTime<Local> =
                                utilities::time_converter(arrival);
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                            println!("    arrives at {stop_name} at {formatted}");
                        }
                        if first_stop.departure.is_some() {
                            let departure = first_stop.departure.unwrap().time.unwrap();
                            let formatted: chrono::DateTime<Local> =
                                utilities::time_converter(departure);
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                            println!("    departs from {stop_name} at {formatted}");
                        }
                    }
                }
            }
        }

        pub async fn in_range(first: &str, last: &str, city: &str) {
            // ---- STATIC DATA LAYER 1 ---- //
            let static_trips: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "trips");
            let static_stops: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "stops");

            // ---- REALTIME DATA ---- //
            // let buses = gtfsrt::requester(city, "vehicles").await;
            let trips = gtfsrt::requester(city, "trips").await;

            for trip in trips.entity {
                let unit: TripUpdate = trip.trip_update.unwrap();
                let vehicle_id: String = unit.vehicle.unwrap().id.unwrap();
                if utilities::within_bounds(vehicle_id.clone(), first, last) {
                    let route: String = unit.trip.route_id.unwrap();
                    let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update;
                    let first_stop: StopTimeUpdate = stop_time_update[0].clone();
                    let stop_id: String = first_stop.stop_id.unwrap().clone();
                    let static_stops_stop_id = &static_stops[&stop_id];
                    let stop_name = static_stops_stop_id
                        [gtfsstatic::bindings::stops(city, "stop_name")]
                    .clone();
                    let trip_id: String = unit.trip.trip_id.unwrap();
                    let static_stops_trip_id = &static_trips[&trip_id];
                    let headsign = static_stops_trip_id
                        [gtfsstatic::bindings::trips(city, "trip_headsign")]
                    .clone();
                    if first_stop.arrival.is_some() {
                        let arrival = first_stop.arrival.unwrap().time.unwrap();
                        let formatted: chrono::DateTime<Local> = utilities::time_converter(arrival);
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                        println!("    arrives at {stop_name} at {formatted}");
                    }
                    if first_stop.departure.is_some() {
                        let departure = first_stop.departure.unwrap().time.unwrap();
                        let formatted: chrono::DateTime<Local> =
                            utilities::time_converter(departure);
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                        println!("    departs from {stop_name} at {formatted}");
                    }
                }
            }
        }
    }

    pub mod san_antonio {
        use crate::{gtfsrt, gtfsstatic, gtfsstatic::data, search::utilities};
        use gtfs_realtime::VehiclePosition;
        use std::collections::HashMap;

        pub async fn on_route_vdata(number: &str, city: &str) {
            // ---- STATIC DATA LAYER 1 ---- //
            let static_trips: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "trips");
            let static_stops: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "stops");

            // ---- REALTIME DATA ---- //
            let buses = gtfsrt::requester(city, "vehicles").await;

            for bus in buses.entity {
                let unit: VehiclePosition = bus.vehicle.unwrap();
                if unit.trip.clone().is_some() {
                    // Checks which type of vehicle
                    let route: String = unit.trip.clone().unwrap().route_id.unwrap();
                    if number == route {
                        let vehicle_id: String = unit.vehicle.unwrap().id.unwrap();
                        let trip_id: String = unit.trip.unwrap().trip_id.unwrap(); // The trip ID.
                        let static_trips_trip_id = &static_trips[&trip_id];
                        let destination = &static_trips_trip_id
                            [gtfsstatic::bindings::trips(city, "trip_headsign")];
                        if unit.stop_id.is_some() {
                            let stop_id = unit.stop_id;
                            let current_stop = stop_id.unwrap().to_string();
                            let static_stops_stop_id = &static_stops[&current_stop];
                            let current_stop_name = &static_stops_stop_id
                                [gtfsstatic::bindings::stops(city, "stop_name")];
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} is in transit to {current_stop_name} \x1b[0m");
                        } else {
                            println!(
                                "\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} \x1b[0m"
                            );
                        }
                    }
                }
            }
        }

        pub async fn in_range_vdata(first: &str, last: &str, city: &str) {
            // ---- STATIC DATA LAYER 1 ---- //
            let static_trips: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "trips");
            let static_stops: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "stops");

            // ---- REALTIME DATA ---- //
            let buses = gtfsrt::requester(city, "vehicles-bus").await;

            for vehicle in buses.entity {
                let unit: VehiclePosition = vehicle.vehicle.unwrap();
                let vehicle_id: String = unit.vehicle.unwrap().id.unwrap();
                if utilities::within_bounds(vehicle_id.clone(), first, last) && unit.trip.is_some()
                {
                    let route: String = unit.trip.clone().unwrap().route_id.unwrap();
                    let trip_id: String = unit.trip.unwrap().trip_id.unwrap();
                    let static_trips_trip_id = &static_trips[&trip_id];
                    let destination =
                        &static_trips_trip_id[gtfsstatic::bindings::trips(city, "trip_headsign")];
                    if unit.stop_id.is_some() {
                        let stop_id = unit.stop_id;
                        let current_stop = stop_id.unwrap().to_string();
                        let static_stops_stop_id = &static_stops[&current_stop];
                        let current_stop_name =
                            &static_stops_stop_id[gtfsstatic::bindings::stops(city, "stop_name")];
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} is in transit to {current_stop_name} \x1b[0m");
                    } else {
                        println!(
                            "\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} \x1b[0m"
                        );
                    }
                }
            }
        }
    }

    pub mod pittsburgh {
        use crate::{gtfsrt, gtfsstatic, gtfsstatic::data, search::utilities};
        use chrono::Local;
        use gtfs_realtime::{trip_update::StopTimeUpdate, TripUpdate};
        use std::collections::HashMap;

        pub async fn at_stop(stop: &str, city: &str) {
            // ---- STATIC DATA LAYER 1 ---- //
            let static_stops: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "stops");
            let static_trips: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "trips");

            // ---- STATIC DATA LAYER 2 ---- //
            let routes_per_stop = data::processing_layer_two::routes_per_stop(city);
            let mut key = String::new();
            key.push_str(stop);

            let stop_name = &static_stops[stop][gtfsstatic::bindings::stops(city, "stop_name")];

            let routes = &routes_per_stop[&key];
            println!("------------ STOP {stop}: {stop_name} ------------");

            // ---- REALTIME DATA ---- //
            // let buses = gtfsrt::requester("/pittsburgh/prt/", "vehicles-bus").await;
            let trips_bus = gtfsrt::requester("/pittsburgh/prt/", "trips-bus").await;
            // let trains = gtfsrt::requester("/pittsburgh/prt/", "vehicles-train").await;
            let trips_train = gtfsrt::requester("/pittsburgh/prt/", "trips-train").await;

            // Let's check if it's a valid stop!
            if !static_stops.contains_key(stop) {
                panic!("Oops! The stop {stop} is not in the static feed definition.");
            }

            // TODO: ADD LOGIC THAT LETS YOU PICK BETWEEN BUS AND TRAIN FEEDS HERE.

            for entity in trips_bus.entity {
                let trip_update = entity.trip_update.unwrap();
                if routes.contains(&trip_update.clone().trip.route_id.unwrap()) {
                    let stop_seq = trip_update.clone().stop_time_update;
                    for stop_in_feed in stop_seq {
                        if stop_in_feed.stop_id.unwrap() == stop {
                            let vehicle_id: String =
                                trip_update.clone().vehicle.unwrap().id.unwrap();
                            let route: String = trip_update.clone().trip.route_id.unwrap();
                            let time = stop_in_feed.arrival.unwrap().time.unwrap();
                            let trip_id: String = trip_update.clone().trip.trip_id.unwrap();
                            let formatted: chrono::DateTime<Local> =
                                utilities::time_converter(time);
                            let static_stops_trip_id = &static_trips[&trip_id];
                            let headsign = static_stops_trip_id
                                [gtfsstatic::bindings::trips(city, "trip_headsign")]
                            .clone();
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} arrives at {formatted} \x1b[0m");
                        }
                    }
                }
            }

            for entity in trips_train.entity {
                let trip_update = entity.trip_update.unwrap();
                if routes.contains(&trip_update.clone().trip.route_id.unwrap()) {
                    let stop_seq = trip_update.clone().stop_time_update;
                    for stop_in_feed in stop_seq {
                        if stop_in_feed.stop_id.unwrap() == stop {
                            let vehicle_id: String =
                                trip_update.clone().vehicle.unwrap().id.unwrap();
                            let route: String = trip_update.clone().trip.route_id.unwrap();
                            let time = stop_in_feed.arrival.unwrap().time.unwrap();
                            let trip_id: String = trip_update.clone().trip.trip_id.unwrap();
                            let formatted: chrono::DateTime<Local> =
                                utilities::time_converter(time);
                            let static_stops_trip_id = &static_trips[&trip_id];
                            let headsign = static_stops_trip_id
                                [gtfsstatic::bindings::trips(city, "trip_headsign")]
                            .clone();
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} arrives at {formatted} \x1b[0m");
                        }
                    }
                }
            }
        }

        pub async fn on_route(number: &str, city: &str) {
            // ---- STATIC DATA LAYER 1 ---- //
            let static_stops: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "stops");
            let static_trips: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "trips");

            // ---- REALTIME DATA ---- //
            // let buses = gtfsrt::requester("/pittsburgh/prt/", "vehicles-bus").await;
            let trips_bus = gtfsrt::requester("/pittsburgh/prt/", "trips-bus").await;
            // let trains = gtfsrt::requester("/pittsburgh/prt/", "vehicles-train").await;
            let trips_train = gtfsrt::requester("/pittsburgh/prt/", "trips-train").await;

            // TODO: ADD LOGIC THAT LETS YOU PICK BETWEEN BUS AND TRAIN FEEDS HERE.

            for trip in trips_bus.entity {
                let unit: TripUpdate = trip.trip_update.unwrap();
                let route: String = unit.trip.route_id.unwrap();
                if number == route {
                    let vehicle_id: String = unit.vehicle.unwrap().id.unwrap();
                    let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update.clone();
                    if !stop_time_update.is_empty() {
                        let first_stop: StopTimeUpdate = stop_time_update[0].clone();
                        let stop_id: String = first_stop.stop_id.unwrap().clone();
                        let stop_name =
                            &static_stops[&stop_id][gtfsstatic::bindings::stops(city, "stop_name")];
                        let trip_id: String = unit.trip.trip_id.unwrap();
                        let static_stops_trip_id = &static_trips[&trip_id];
                        let headsign = static_stops_trip_id
                            [gtfsstatic::bindings::trips(city, "trip_headsign")]
                        .clone();
                        if first_stop.arrival.is_some() {
                            let arrival = first_stop.arrival.unwrap().time.unwrap();
                            let formatted: chrono::DateTime<Local> =
                                utilities::time_converter(arrival);
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                            println!("    arrives at {stop_name} at {formatted}");
                        }
                        if first_stop.departure.is_some() {
                            let departure = first_stop.departure.unwrap().time.unwrap();
                            let formatted: chrono::DateTime<Local> =
                                utilities::time_converter(departure);
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                            println!("    departs from {stop_name} at {formatted}");
                        }
                    }
                }
            }

            for trip in trips_train.entity {
                let unit: TripUpdate = trip.trip_update.unwrap();
                let route: String = unit.trip.route_id.unwrap();
                if number == route {
                    let vehicle_id: String = unit.vehicle.unwrap().id.unwrap();
                    let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update.clone();
                    if !stop_time_update.is_empty() {
                        let first_stop: StopTimeUpdate = stop_time_update[0].clone();
                        let stop_id: String = first_stop.stop_id.unwrap().clone();
                        let stop_name =
                            &static_stops[&stop_id][gtfsstatic::bindings::stops(city, "stop_name")];
                        let trip_id: String = unit.trip.trip_id.unwrap();
                        let static_stops_trip_id = &static_trips[&trip_id];
                        let headsign = static_stops_trip_id
                            [gtfsstatic::bindings::trips(city, "trip_headsign")]
                        .clone();
                        if first_stop.arrival.is_some() {
                            let arrival = first_stop.arrival.unwrap().time.unwrap();
                            let formatted: chrono::DateTime<Local> =
                                utilities::time_converter(arrival);
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                            println!("    arrives at {stop_name} at {formatted}");
                        }
                        if first_stop.departure.is_some() {
                            let departure = first_stop.departure.unwrap().time.unwrap();
                            let formatted: chrono::DateTime<Local> =
                                utilities::time_converter(departure);
                            println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                            println!("    departs from {stop_name} at {formatted}");
                        }
                    }
                }
            }
        }

        pub async fn in_range(first: &str, last: &str, city: &str) {
            // ---- STATIC DATA LAYER 1 ---- //
            let static_stops: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "stops");
            let static_trips: HashMap<String, Vec<String>> =
                data::processing_layer_one::static_data(city, "trips");

            // ---- REALTIME DATA ---- //
            // let buses = gtfsrt::requester("/pittsburgh/prt/", "vehicles-bus").await;
            let trips_bus = gtfsrt::requester("/pittsburgh/prt/", "trips-bus").await;
            // let trains = gtfsrt::requester("/pittsburgh/prt/", "vehicles-train").await;
            let trips_train = gtfsrt::requester("/pittsburgh/prt/", "trips-train").await;

            // TODO: ADD LOGIC THAT LETS YOU PICK BETWEEN BUS AND TRAIN FEEDS HERE.
            for trip in trips_bus.entity {
                let unit: TripUpdate = trip.trip_update.unwrap();
                let vehicle_id: String = unit.vehicle.unwrap().id.unwrap();
                if utilities::within_bounds(vehicle_id.clone(), first, last) {
                    let route: String = unit.trip.route_id.unwrap();
                    let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update;
                    let first_stop: StopTimeUpdate = stop_time_update[0].clone();
                    let stop_id: String = first_stop.stop_id.unwrap().clone();
                    let static_stops_stop_id = &static_stops[&stop_id];
                    let stop_name = static_stops_stop_id
                        [gtfsstatic::bindings::stops(city, "stop_name")]
                    .clone();
                    let trip_id: String = unit.trip.trip_id.unwrap();
                    let static_stops_trip_id = &static_trips[&trip_id];
                    let headsign = static_stops_trip_id
                        [gtfsstatic::bindings::trips(city, "trip_headsign")]
                    .clone();
                    if first_stop.arrival.is_some() {
                        let arrival = first_stop.arrival.unwrap().time.unwrap();
                        let formatted: chrono::DateTime<Local> = utilities::time_converter(arrival);
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                        println!("    arrives at {stop_name} at {formatted}");
                    }
                    if first_stop.departure.is_some() {
                        let departure = first_stop.departure.unwrap().time.unwrap();
                        let formatted: chrono::DateTime<Local> =
                            utilities::time_converter(departure);
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                        println!("    departs from {stop_name} at {formatted}");
                    }
                }
            }

            for trip in trips_train.entity {
                let unit: TripUpdate = trip.trip_update.unwrap();
                let vehicle_id: String = unit.vehicle.unwrap().id.unwrap();
                if utilities::within_bounds(vehicle_id.clone(), first, last) {
                    let route: String = unit.trip.route_id.unwrap();
                    let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update;
                    let first_stop: StopTimeUpdate = stop_time_update[0].clone();
                    let stop_id: String = first_stop.stop_id.unwrap().clone();
                    let static_stops_stop_id = &static_stops[&stop_id];
                    let stop_name = static_stops_stop_id
                        [gtfsstatic::bindings::stops(city, "stop_name")]
                    .clone();
                    let trip_id: String = unit.trip.trip_id.unwrap();
                    let static_stops_trip_id = &static_trips[&trip_id];
                    let headsign = static_stops_trip_id
                        [gtfsstatic::bindings::trips(city, "trip_headsign")]
                    .clone();
                    if first_stop.arrival.is_some() {
                        let arrival = first_stop.arrival.unwrap().time.unwrap();
                        let formatted: chrono::DateTime<Local> = utilities::time_converter(arrival);
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                        println!("    arrives at {stop_name} at {formatted}");
                    }
                    if first_stop.departure.is_some() {
                        let departure = first_stop.departure.unwrap().time.unwrap();
                        let formatted: chrono::DateTime<Local> =
                            utilities::time_converter(departure);
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {headsign} is in transit to {stop_name} \x1b[0m");
                        println!("    departs from {stop_name} at {formatted}");
                    }
                }
            }
        }
    }
}

mod utilities {
    use chrono::{Local, TimeZone};

    fn string_to_int(input: String) -> i64 {
        let conv: i64 = input.parse().expect("Converted to integer");
        conv
    }

    pub fn within_bounds(input: String, left: &str, right: &str) -> bool {
        for int in string_to_int(left.to_string())..string_to_int(right.to_string()) + 1 {
            if int.to_string() == input {
                return true;
            }
        }
        false
    }

    pub fn time_converter(input: i64) -> chrono::DateTime<Local> {
        let date_time: chrono::DateTime<Local> = Local.timestamp_opt(input, 0).unwrap();
        date_time
    }
}
