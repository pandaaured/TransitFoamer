

pub mod fetch {
    use gtfs_realtime::FeedEntity;
    // use gtfs_realtime::VehiclePosition;
    // use gtfs_realtime::VehicleDescriptor;    
    use gtfs_realtime::TripUpdate;
    // use gtfs_realtime::TripDescriptor;
    use gtfs_realtime::trip_update::StopTimeUpdate;
    use std::collections::HashMap;

    use crate::search::deoption;
    use crate::search::fields;
    use crate::search::utilities;

    pub fn result (vehicles: Vec<FeedEntity>, trips: Vec<FeedEntity>,
            data: HashMap<String, HashMap<String, Vec<String>>>, 
            function: &str, number: &str, number2: &str,
            stops: HashMap<String, Vec<String>> ) {
        if function == "route" {
            on_route(vehicles, trips, number, data);
        } else if function == "stop" {
            at_stop(vehicles, trips, number, data, stops);
        } else if function == "range" {
            in_range(vehicles, trips, number, number2, data);
        }
    }

    fn on_route(_vehicles: Vec<FeedEntity>, trips: Vec<FeedEntity>, number: &str,
                data: HashMap<String, HashMap<String, Vec<String>>>) {
        for trip in trips {
            // Primary element in this function, all others derive from it.
            let unit: TripUpdate = deoption::trip_update(trip.trip_update);
            // Secondary elements.
            let route: String = fields::get_route_id(unit.trip.clone());
            if number == route {
                let vehicle_id: String = fields::get_vehicle_id(deoption::vehicle_descriptor(unit.vehicle));
                let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update;
                if !stop_time_update.is_empty() {
                    let first_stop = stop_time_update[0].clone();
                    let stop_id = deoption::stop_id(first_stop.stop_id);
                    let stop_name = data["stops"][&stop_id][2].clone();
                    let trip_id: String = fields::get_trip_id(unit.trip);
                    let headsign = data["trips"].get(&trip_id).clone();
                    let headsign_vec: Vec<String> = match headsign {
                        None => vec![],
                        Some(value) => {value.to_vec()},
                    };
                    if headsign_vec.is_empty() {
                    println!("\x1b[0;31m{route}\x1b[0m {vehicle_id}");
                        println!("    arrives at {stop_name}");
                    } else {
                        let destination: String = headsign_vec[3].clone();
                        println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[0m {destination} is in transit to {stop_name}");
                        println!("    arrives at {stop_name} at foo.bar");

                    }


                }
            }
        }
    }

    fn in_range(vehicles: Vec<FeedEntity>, trips: Vec<FeedEntity>, first: &str,
                last: &str, data: HashMap<String, HashMap<String, Vec<String>>>) {
        for trip in trips {
            // Primary element in this function, all others derive from it.
            let unit: TripUpdate = deoption::trip_update(trip.trip_update);
            // Secondary elements.
            let vehicle_id: String = fields::get_vehicle_id(deoption::vehicle_descriptor(unit.vehicle));
            if utilities::within_bounds(vehicle_id.clone(), first, last) {
                let route: String = fields::get_route_id(unit.trip.clone());
                let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update;
                let first_stop = stop_time_update[0].clone();
                let stop_id = deoption::stop_id(first_stop.stop_id);
                let stop_name = data["stops"][&stop_id][2].clone();
                let trip_id: String = fields::get_trip_id(unit.trip);
                let headsign = data["trips"].get(&trip_id).clone();
                let headsign_vec: Vec<String> = match headsign {
                    None => vec![],
                    Some(value) => {value.to_vec()},
                };
                if headsign_vec.is_empty() {
                println!("\x1b[0;31m{route}\x1b[0m {vehicle_id}");
                    println!("    arrives at {stop_name}");
                } else {
                    let destination: String = headsign_vec[3].clone();
                    println!("\x1B[41m {route} \x1b[43m {vehicle_id} \x1b[0m {destination} is in transit to {stop_name}");
                    println!("    arrives at {stop_name} at foo.bar");

                }
            }
        }
    } 

    fn at_stop(vehicles: Vec<FeedEntity>, trips: Vec<FeedEntity>, stop: &str,
         data: HashMap<String, HashMap<String, Vec<String>>>, stops: HashMap<String, Vec<String>>) {

        }

}


mod fields {
    use gtfs_realtime::VehicleDescriptor;
    use gtfs_realtime::TripDescriptor;
    use crate::search::deoption;

    pub fn get_vehicle_id (entity: VehicleDescriptor) -> String {
        // println!("Calling get_vehicle_id");
        let id: String = deoption::id(entity.id);
        id
    }
    
    pub fn get_trip_id (entity: TripDescriptor) -> String {
        // println!("Calling get_trip_id");
        let trip_id: String = deoption::trip_id(entity.trip_id);
        trip_id
    }
    
    pub fn get_route_id (entity: TripDescriptor) -> String {
        // println!("Calling get_route_id");
        let route_id: String = deoption::route_id(entity.route_id);
        route_id
    }
    
    // fn get_direction_id (entity: VehiclePosition) -> String {
    //     // println!("Calling get_direction_id");
    
    //     let trip: TripDescriptor = trip_descriptor(entity.trip);
    //     let direction_id: String = direction_id(trip.trip_id);
    
    //     direction_id
    // }
    
    // fn get_stop_id (entity: StopTimeUpdate) -> String {
    //     let stop
    // }

}

mod deoption {
    use gtfs_realtime::VehicleDescriptor;
    use gtfs_realtime::TripDescriptor;
    use gtfs_realtime::TripUpdate;


    // pub fn vehicle_position (entity: Option<VehiclePosition>) -> VehiclePosition {
    //     let position : VehiclePosition = match entity {
    //         None => panic!("This wasn't a vehicle position."),
    //         Some(position) => {position}
    //     };

    //     position
    // }

    pub fn trip_id (entity: Option<String>) -> String {
        let trip_id : String = match entity {
            None => panic!("This wasn't a trip_id."),
            Some(trip_id) => {trip_id}
        };
    
        trip_id
    }
    
    pub fn route_id (entity: Option<String>) -> String {
        let route_id : String = match entity {
            None => panic!("This wasn't a route_id."),
            Some(route_id) => {route_id}
        };
    
        route_id
    }
    
    // pub fn direction_id (entity: Option<String>) -> String {
    //     let direction_id : String = match entity {
    //         None => panic!("This wasn't an id."),
    //         Some(direction_id) => {direction_id}
    //     };
    //     direction_id
    // }

    pub fn id (entity: Option<String>) -> String {
        let id : String = match entity {
            None => panic!("This wasn't an id."),
            Some(id) => {id}
        };
    
        id
    }

    pub fn stop_id (entity: Option<String>) -> String {
        let stop_id : String = match entity {
            None => panic!("This wasn't a stop sequence."),
            Some(stop_id) => {stop_id}
        };
    
        stop_id
    }

    pub fn trip_descriptor (entity: Option<TripDescriptor>) -> TripDescriptor {
        let descriptor : TripDescriptor = match entity {
            None => panic!("This wasn't a trip descriptor."),
            Some(descriptor) => {descriptor}
        };
    
        descriptor
    }
        
    pub fn stop_sequence (entity: Option<u32>) -> u32 {
        let stop_sequence : u32 = match entity {
            None => panic!("This wasn't a stop sequence."),
            Some(stop_sequence) => {stop_sequence}
        };
    
        stop_sequence
    }

    pub fn vehicle_descriptor (entity: Option<VehicleDescriptor>) -> VehicleDescriptor {
        let descriptor : VehicleDescriptor = match entity {
            None => panic!("This wasn't a vehicle descriptor."),
            Some(descriptor) => {descriptor}
        };
    
        descriptor
    }

    pub fn trip_update (entity: Option<TripUpdate>) -> TripUpdate {
        let trip_update : TripUpdate = match entity {
            None => panic!("This wasn't a trip update."),
            Some(trip_update) => {trip_update}
        };
    
        trip_update
    }
}

mod utilities {
    fn string_to_int(input: String) -> i32 {
        let conv: i32 = input.parse().expect("Converted to integer");
        conv
    }

    pub fn within_bounds(input: String, left: &str, right: &str) -> bool {
        if string_to_int(left.to_string()) <= string_to_int(input.clone()) 
        && string_to_int(input) <= string_to_int(right.to_string()) {
            return true;
        }
        false
    }
}
