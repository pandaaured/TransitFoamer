pub mod fetch {
    use gtfs_realtime::FeedEntity;
    use gtfs_realtime::VehiclePosition;
    use gtfs_realtime::{TripUpdate, trip_update::StopTimeUpdate};
    use std::collections::HashMap;
    use chrono::Local;

    use crate::search::utilities;

    pub fn on_route(_vehicles: Vec<FeedEntity>, trips: Vec<FeedEntity>, number: &str,
                data: HashMap<String, HashMap<String, Vec<String>>>) {
        for trip in trips {
            // Primary element.
            let unit: TripUpdate = trip.trip_update.unwrap();
            // Secondary element.
            let route: String = unit.trip.route_id.unwrap();
            if number == route {
                // Remaining elements. Not computed unless condition is met.
                let vehicle_id: String = unit.vehicle.unwrap()
                                             .id.unwrap();
                let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update.clone();
                if !stop_time_update.is_empty() {
                    let first_stop: StopTimeUpdate = stop_time_update[0].clone();
                    let stop_id: String = first_stop.stop_id
                                                    .unwrap().clone();
                    let stop_name = data["stops"][&stop_id][2].clone();
                    let trip_id: String = unit.trip.trip_id.unwrap();
                    let headsign = data["trips"].get(&trip_id)
                                                              .unwrap();
                    if first_stop.arrival != None {
                        let arrival = first_stop.arrival.unwrap()
                                                     .time.unwrap();
                        let formatted: chrono::DateTime<Local> = utilities::time_converter(arrival);
                        let destination: String = headsign[3].clone();
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} is in transit to {stop_name} \x1b[0m");
                        println!("    arrives at {stop_name} at {formatted}");
                        }
                    if first_stop.departure != None {
                        let departure = first_stop.departure.unwrap()
                                                       .time.unwrap();
                        let formatted: chrono::DateTime<Local> = utilities::time_converter(departure);
                        let destination: String = headsign[3].clone();
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} is in transit to {stop_name} \x1b[0m");
                        println!("    departs from {stop_name} at {formatted}");
                    }
            }
                
        }
        }
    }   

    pub fn in_range(_vehicles: Vec<FeedEntity>, trips: Vec<FeedEntity>, first: &str,
                last: &str, data: HashMap<String, HashMap<String, Vec<String>>>) {
        for trip in trips {
            // Primary element in this function, all others derive from it.
            let unit: TripUpdate = trip.trip_update.unwrap();
            // Secondary elements.
            let vehicle_id: String = unit.vehicle.unwrap()
                                         .id.unwrap();            
            // Remaining elements. Not computed unless condition is met.
            if utilities::within_bounds(vehicle_id.clone(), first, last) {
                let route: String = unit.trip.route_id.unwrap();
                let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update;
                    let first_stop: StopTimeUpdate = stop_time_update[0].clone();
                    let stop_id: String = first_stop.stop_id
                                                    .unwrap()
                                                    .clone();
                    let stop_name = data["stops"][&stop_id][2].clone();
                    let trip_id: String = unit.trip.trip_id.unwrap();
                    let headsign = data["trips"].get(&trip_id)
                                                              .unwrap();
                if first_stop.arrival != None {
                    let arrival = first_stop.arrival.unwrap()
                                                     .time.unwrap();
                        let formatted: chrono::DateTime<Local> = utilities::time_converter(arrival);
                        let destination: String = headsign[3].clone();
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} is in transit to {stop_name} \x1b[0m");
                        println!("    arrives at {stop_name} at {formatted}");
                }
                if first_stop.departure != None {
                    let departure = first_stop.departure.unwrap()
                                                   .time.unwrap();
                    let formatted: chrono::DateTime<Local> = utilities::time_converter(departure);
                    let destination: String = headsign[3].clone();
                    println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} is in transit to {stop_name} \x1b[0m");
                    println!("    departs from {stop_name} at {formatted}");
                }
            }
        }
    } 


    pub fn on_route_vdata(vehicles: Vec<FeedEntity>, number: &str, 
                          data: HashMap<String, HashMap<String, Vec<String>>>) {
        for vehicle in vehicles {
            // Primary element.
            let unit: VehiclePosition = vehicle.vehicle.unwrap();
            // Secondary element.
            if unit.trip.clone() != None {
                let route: String = unit.trip.clone()
                                    .unwrap()
                                    .route_id
                                    .unwrap();
                if number == route {
                    // Remaining elements. Not computed unless condition is met.
                    let vehicle_id: String = unit.vehicle
                                                    .unwrap()
                                                    .id
                                                    .unwrap();  
                    let trip_id: String = unit.trip
                                                .unwrap()
                                                .trip_id
                                                .unwrap();
                    let destination: String = data["trips"]
                                            .get(&trip_id).unwrap()[5]
                                            .clone();

                    if unit.stop_id != None {
                        let current_stop = unit.stop_id.unwrap().to_string();
                        let current_stop_name = &data["stops"]
                                                        .get(&current_stop)
                                                        .unwrap()[8];
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} is in transit to {current_stop_name} \x1b[0m");

                    } else {
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} \x1b[0m");
                    }
                }
            }
        }
    }
}
    
mod utilities {
    use chrono::{TimeZone, Local};

    fn string_to_int(input: String) -> i64 {
        let conv: i64 = input.parse().expect("Converted to integer");
        conv
    }

    pub fn within_bounds(input: String, left: &str, right: &str) -> bool {
        if string_to_int(left.to_string()) <= string_to_int(input.clone()) 
        && string_to_int(input) <= string_to_int(right.to_string()) {
            return true;
        }
        false
    }

    pub fn time_converter(input: i64) -> chrono::DateTime<Local> {
        let date_time: chrono::DateTime<Local> = Local.timestamp_opt(input, 0).unwrap();
        date_time
    }
}
