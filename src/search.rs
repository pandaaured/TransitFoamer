use gtfs_realtime::FeedEntity;
use gtfs_realtime::VehiclePosition;
use gtfs_realtime::VehicleDescriptor;
use gtfs_realtime::TripUpdate;
use gtfs_realtime::TripDescriptor;

pub fn string_to_int(input: &str) -> i32 {
    let conv: i32 = input.parse().expect("Converted to integer");
    conv
}

pub fn result (vehicles: Vec<FeedEntity>, trips: Vec<FeedEntity>, 
               function: &str, number: &str) {
    if function == "route" {
        on_route(vehicles, trips, number);
    } else if function == "stop" {
        // on_stop(vehicles, trips, number);
    }
}

fn on_route(vehicles: Vec<FeedEntity>, _trips: Vec<FeedEntity>, number: &str) {
    for vehicle in vehicles {
        let vehicle_new: VehiclePosition = vehicle_position(vehicle.vehicle);
        if vehicle_new.trip != None {
            let route: String = get_route_id(vehicle_new.clone());
            if number == route {
                let vehicle_id: String = get_vehicle_id(vehicle_new.clone());
                let trip_id: String = get_trip_id(vehicle_new.clone());
                println!("{vehicle_id}, {route}");
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
fn get_vehicle_id (entity: VehiclePosition) -> String {
    // println!("Calling get_vehicle_id");
    let vehicle: VehicleDescriptor = vehicle_descriptor(entity.vehicle);
    let id: String = id(vehicle.id);

    id
}

fn get_trip_id (entity: VehiclePosition) -> String {
    println!("Calling get_trip_id");
    let trip: TripDescriptor = trip_descriptor(entity.trip);
    let trip_id: String = trip_id(trip.trip_id);

    trip_id
}

fn get_route_id (entity: VehiclePosition) -> String {
    // println!("Calling get_route_id");

    let trip: TripDescriptor = trip_descriptor(entity.trip);
    let route_id: String = route_id(trip.route_id);

    route_id
}

// fn get_direction_id (entity: VehiclePosition) -> String {
//     // println!("Calling get_direction_id");

//     let trip: TripDescriptor = trip_descriptor(entity.trip);
//     let direction_id: String = direction_id(trip.trip_id);

//     direction_id
// }
