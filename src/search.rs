use gtfs_realtime::FeedEntity;
use gtfs_realtime::VehiclePosition;
use gtfs_realtime::VehicleDescriptor;
use gtfs_realtime::TripUpdate;
use gtfs_realtime::TripDescriptor;
use gtfs_realtime::trip_update::StopTimeUpdate;

use std::collections::HashMap;

pub fn string_to_int(input: &str) -> i32 {
    let conv: i32 = input.parse().expect("Converted to integer");
    conv
}



pub fn result (vehicles: Vec<FeedEntity>, trips: Vec<FeedEntity>,
               data: HashMap<String, HashMap<String, Vec<String>>>, 
               function: &str, number: &str) {
    if function == "route" {
        on_route(vehicles, trips, number, data);
    } else if function == "stop" {
        // on_stop(vehicles, trips, number);
    }
}

fn get_stop_name(stop: String, data: HashMap<String, HashMap<String, Vec<String>>>) -> String {
    let stop_name = data["stops"][&stop][2].clone();
    stop_name
}

fn on_route(vehicles: Vec<FeedEntity>, trips: Vec<FeedEntity>, number: &str,
            data: HashMap<String, HashMap<String, Vec<String>>>) {
    for trip in trips {
        // Primary element in this function, all others derive from it.
        let unit: TripUpdate = trip_update(trip.trip_update);
        // Secondary elements.
        let route: String = get_route_id(unit.trip.clone());
        if number == route {
            let vehicle_id: String = get_vehicle_id(vehicle_descriptor(unit.vehicle));
            let stop_time_update: Vec<StopTimeUpdate> = unit.stop_time_update;
            if !stop_time_update.is_empty() {
                let first_stop = stop_time_update[0].clone();
                let stop_id = stop_id(first_stop.stop_id);
                let stop_name = data["stops"][&stop_id][2].clone();
                let trip_id: String = get_trip_id(unit.trip);
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
                    println!("\x1b[0;31m{route}\x1b[0m {vehicle_id} is in transit to {destination}");
                    println!("    arrives at {stop_name}");
                }


            }
        }
    }
}



// Extracting substructures from GTFS-RT structs:
// (1) VehiclePosition.
// Note: This will be re-done with generics and impl. at a later date. 

/* Requires: For our purposes, that we know the option evaluates to some. */ 
/* Ensures: The result is of type VehiclePosition. */
fn vehicle_position (entity: Option<VehiclePosition>) -> VehiclePosition {
    let position : VehiclePosition = match entity {
        None => panic!("This wasn't a vehicle position."),
        Some(position) => {position}
    };

    position
}

/* Requires: For our purposes, that we know the option evaluates to some. */ 
/* Ensures: The result is of type VehicleDescriptor. */
fn vehicle_descriptor (entity: Option<VehicleDescriptor>) -> VehicleDescriptor {
    let descriptor : VehicleDescriptor = match entity {
        None => panic!("This wasn't a vehicle descriptor."),
        Some(descriptor) => {descriptor}
    };

    descriptor
}

/* Requires: For our purposes, that we know the option evaluates to some. */ 
/* Ensures: The result is of type String. */
fn id (entity: Option<String>) -> String {
    let id : String = match entity {
        None => panic!("This wasn't an id."),
        Some(id) => {id}
    };

    id
}


// (2) TripUpdate.
/* Requires: For our purposes, that we know the option evaluates to some. */ 
/* Ensures: The result is of type TripUpdate. */
fn trip_update (entity: Option<TripUpdate>) -> TripUpdate {
    let trip_update : TripUpdate = match entity {
        None => panic!("This wasn't a trip update."),
        Some(trip_update) => {trip_update}
    };

    trip_update
}

fn trip_descriptor (entity: Option<TripDescriptor>) -> TripDescriptor {
    let descriptor : TripDescriptor = match entity {
        None => panic!("This wasn't a trip descriptor."),
        Some(descriptor) => {descriptor}
    };

    descriptor
}

fn stop_sequence (entity: Option<u32>) -> u32 {
    let stop_sequence : u32 = match entity {
        None => panic!("This wasn't a stop sequence."),
        Some(stop_sequence) => {stop_sequence}
    };

    stop_sequence
}

fn stop_id (entity: Option<String>) -> String {
    let stop_id : String = match entity {
        None => panic!("This wasn't a stop sequence."),
        Some(stop_id) => {stop_id}
    };

    stop_id
}

fn arrival () {

}

fn departure () {

}

fn trip_id (entity: Option<String>) -> String {
    let trip_id : String = match entity {
        None => panic!("This wasn't a trip_id."),
        Some(trip_id) => {trip_id}
    };

    trip_id
}

fn route_id (entity: Option<String>) -> String {
    let route_id : String = match entity {
        None => panic!("This wasn't a route_id."),
        Some(route_id) => {route_id}
    };

    route_id
}

// fn direction_id (entity: Option<String>) -> String {
//     let direction_id : String = match entity {
//         None => panic!("This wasn't an id."),
//         Some(direction_id) => {direction_id}
//     };
//     direction_id
// }

// Getting the data from GTFS-RT structs:  

// New arbitrary version.
fn get_vehicle_id (entity: VehicleDescriptor) -> String {
    // println!("Calling get_vehicle_id");
    let id: String = id(entity.id);
    id
}

fn get_trip_id (entity: TripDescriptor) -> String {
    // println!("Calling get_trip_id");
    let trip_id: String = trip_id(entity.trip_id);
    trip_id
}

fn get_route_id (entity: TripDescriptor) -> String {
    // println!("Calling get_route_id");
    let route_id: String = route_id(entity.route_id);
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