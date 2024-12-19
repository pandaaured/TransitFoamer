pub mod fetch {
    use gtfs_realtime::FeedEntity;
    use gtfs_realtime::VehiclePosition;
    use gtfs_realtime::{TripUpdate, trip_update::StopTimeUpdate};
    use std::collections::HashMap;
    use chrono::Local;
    use crate::search::utilities;
    use crate::gtfsstatic;

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

    pub fn on_route_vdata(vehicles: Vec<FeedEntity>, number: &str, city: &str) {
        for vehicle in vehicles {
            // Primary element.
            let unit: VehiclePosition = vehicle.vehicle.unwrap();
            // Two types of vehicles:
            // (a) vehicle running a route
            // (b) vehicle not running a route. 
            if unit.trip.clone() != None {      // Checks which type of vehicle
                let route: String = unit.trip
                                        .clone()
                                        .unwrap()
                                        .route_id 
                                        // Mandatory field of TripDescriptor. 
                                        .unwrap();
                if number == route {
                    // Remaining elements. Not computed unless condition is met.
                    let vehicle_id: String = unit.vehicle
                                                    .unwrap()
                                                    .id // The bus number. 
                                                    .unwrap();  
                    let trip_id: String = unit.trip
                                                .unwrap()
                                                .trip_id
                                                .unwrap(); // The trip ID. 
                    // Here we get specifically the "trips" section of GTFS
                    let path_to_trips = gtfsstatic::paths::file_path(city, "trips");
                    let static_trips: HashMap<String, Vec<String>> = gtfsstatic::dict_gtfs::path_io("trips".to_string(), path_to_trips);

                    // let static_trips_header = &static_trips["trip_id"];
                    // let index = static_trips_header.iter()
                    //                                       .position(|x| x == "trip_headsign")
                    //                                       .unwrap();
                    // // Appropriate piece of information for destinations. 
                    let static_trips_trip_id = &static_trips[&trip_id];
                    let destination = &static_trips_trip_id[5];   
                    if unit.stop_id != None {
                        let stop_id = unit.stop_id;
                        let current_stop = stop_id.unwrap().to_string();
                        let path_to_stops = gtfsstatic::paths::file_path(city, "stops");
                        let static_stops = gtfsstatic::dict_gtfs::path_io("stops".to_string(), path_to_stops);
                        let static_stops_stop_id = &static_stops[&current_stop];
                        // let index = static_stops_stop_id.iter()
                        //                                                  .position(|x| x == "stop_name")
                        //                                                  .unwrap();
                        let current_stop_name = &static_stops_stop_id[8];
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} is in transit to {current_stop_name} \x1b[0m");

                    } else {
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[44m {destination} \x1b[0m");
                    }
                }
            }
        }
    }
    
    pub fn in_range_vdata(vehicles: Vec<FeedEntity>, first: &str,
                    last: &str, city: &str) {
        for vehicle in vehicles {
            // Primary element in this function, all others derive from it.
            let unit: VehiclePosition = vehicle.vehicle.unwrap();
            // Secondary elements.
            let vehicle_id: String = unit.vehicle.unwrap()
                                        .id.unwrap();            
            // Remaining elements. Not computed unless condition is met.
            if utilities::within_bounds(vehicle_id.clone(), first, last) {
                if unit.trip != None {
                    let route: String = unit.trip
                                        .clone()
                                        .unwrap()
                                        .route_id
                                        .unwrap();                    
                    let trip_id: String = unit.trip
                                          .unwrap()
                                          .trip_id
                                          .unwrap();
                    let path_to_trips = gtfsstatic::paths::file_path(city, "trips");
                    let static_trips: HashMap<String, Vec<String>> = gtfsstatic::dict_gtfs::path_io("trips".to_string(), path_to_trips);
                    let static_trips_trip_id = &static_trips[&trip_id];
                    let destination = &static_trips_trip_id[5];   
                    if unit.stop_id != None {
                        let stop_id = unit.stop_id;
                        let current_stop = stop_id.unwrap().to_string();
                        let path_to_stops = gtfsstatic::paths::file_path(city, "stops");
                        let static_stops = gtfsstatic::dict_gtfs::path_io("stops".to_string(), path_to_stops);
                        let static_stops_stop_id = &static_stops[&current_stop];
                        // let index = static_stops_stop_id.iter()
                        //                                                  .position(|x| x == "stop_name")
                        //                                                  .unwrap();
                        let current_stop_name = &static_stops_stop_id[8];
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
        for int in string_to_int(left.to_string())..string_to_int(right.to_string())+1 {
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
