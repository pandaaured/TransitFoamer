//! GTFS_RT
//!
//! A library for fetching and analyzing GTFS Realtime data from a transit feed.
//! Many of the functions contained within this module can be run with only
//! GTFS Realtime data URLs.

use gtfs_realtime::{FeedEntity, FeedMessage};
use prost::DecodeError;
use reqwest::Response;

/// Returns a Result type containing either an error or a valid FeedMessage decoded
/// from the inputted URL.
pub async fn protobuf_request_from_url(url: &str) -> Result<FeedMessage, DecodeError> {
    let response: Response = reqwest::get(url).await.unwrap(); // Fix this function so all errors are handled properly.
    let bytes = response.bytes().await.unwrap(); // Also here.
    let data: Result<gtfs_realtime::FeedMessage, prost::DecodeError> =
        prost::Message::decode(bytes.as_ref());
    data
}

// Describe function when implemented.
pub fn protobuf_request_from_path(path: &str) -> () {
    // TODO: Implement this!
    // TODO: change return type to match protobuf_request_from_url!
}

/// Returns a serialized JSON string from the FeedMessage struct. 
pub fn gtfs_to_json(message: FeedMessage) -> Result<String, serde_json::Error> {
    serde_json::to_string(&message)
}

/// Returns a FeedMessage which filters all entities in the FeedMessage which do
/// not contain VehiclePosition.
fn has_vehicleposition(message: FeedMessage) -> FeedMessage {
    let filtered = message
        .entity
        .into_iter()
        .filter(|x| x.vehicle.is_some());

    FeedMessage {
        header: message.header,
        entity: filtered.collect(),
    }
}

/// Returns a FeedMessage which filters all entities in the FeedMessage which do
/// not contain TripUpdate.
fn has_tripupdate(message: FeedMessage) -> FeedMessage {
    let filtered = message
        .entity
        .into_iter()
        .filter(|x| x.trip_update.is_some());
    FeedMessage {
        header: message.header,
        entity: filtered.collect(),
    }
}

/// Returns a FeedMessage which filters all entities in the FeedMessage which do
/// not contain Alerts.
fn has_alerts(message: FeedMessage) -> FeedMessage {
    let filtered = message.entity.into_iter().filter(|x| x.alert.is_some());
    FeedMessage {
        header: message.header,
        entity: filtered.collect(),
    }
}

/// Returns a FeedMessage which filters all entities in the FeedMessage which do
/// not contain stop_ids.
fn has_stop_id(message: FeedMessage) -> FeedMessage {
    let filtered = has_tripupdate(message.clone()); // Filter anything without a TripUpdate so we don't unwrap None!

    FeedMessage {
        header: filtered.header,
        entity: filtered
            .entity
            .into_iter()
            .filter(|x| {
                x.trip_update
                    .clone() // Fix performance later.
                    .unwrap() // Guaranteed safe by passing through has_tripupdate().
                    .stop_time_update
                    .into_iter()
                    .all(|x| x.stop_id.is_some())
            })
            .collect(),
    }
}

/// Given a FeedMessage and a first and last vehicle_id, returns a FeedMessage
/// containing only FeedEntities running within these vehicle_ids.
/// (TODO: Have input list be generated or be an explicit collection.)
pub fn filter_for_in_range(first: &str, last: &str, message: FeedMessage) -> FeedMessage {
    let entities = message.entity;
    let result = entities.into_iter().filter(|x| {
        if x.vehicle.is_some() {
            // First check for a VehiclePosition
            within_bounds(
                x.vehicle
                    .as_ref()
                    .unwrap()
                    .vehicle
                    .as_ref()
                    .unwrap()
                    .id
                    .as_ref()
                    .unwrap(),
                first,
                last,
            )
        } else if x.trip_update.is_some() {
            // Now check for a TripUpdate.
            within_bounds(
                x.trip_update
                    .as_ref()
                    .unwrap()
                    .vehicle
                    .as_ref()
                    .unwrap()
                    .id
                    .as_ref()
                    .unwrap(),
                first,
                last,
            )
        } else {
            true // If neither is_some, then return empty.
        }
    });
    let in_range: Vec<FeedEntity> = result.collect();

    FeedMessage {
        header: message.header,
        entity: in_range,
    }
}

/// Given a FeedMessage and a route_id, returns a FeedMessage
/// containing only FeedEntities running on that route.
/// (TODO: Have input list be generated or be an explicit collection.)
pub fn filter_for_on_route(number: &str, message: FeedMessage) -> FeedMessage {
    let entities = message.entity;
    let result = entities.into_iter().filter(|x| {
        if x.trip_update.is_some() {
            // Check if the entity has TripUpdate.
            *x.trip_update
                .as_ref()
                .unwrap()
                .trip
                .route_id
                .as_ref()
                .unwrap()
                == *number.to_owned()
        } else if x.vehicle.is_some() {
            // Now check for a VehiclePosition, as message isn't TripUpdate
            *x.vehicle
                .as_ref()
                .unwrap()
                .trip
                .as_ref()
                .unwrap()
                .route_id
                .as_ref()
                .unwrap()
                == *number.to_owned()
        } else {
            true // Just return the whole list if neither is contained.
        }
    });
    let on_route = result.collect();

    FeedMessage {
        header: message.header,
        entity: on_route,
    }
}

/// Given a FeedMessage and a stop_id, returns a FeedMessage
/// containing only FeedEntities which have this stop_id in their StopTimeUpdate
/// sequence.
/// (TODO: Have input list be generated or be an explicit collection.)
pub fn vehicles_approaching_stop(message: FeedMessage, stop_id: String) -> FeedMessage {
    let filtered = has_stop_id(message);

    FeedMessage {
        header: filtered.header,
        entity: filtered
            .entity
            .into_iter()
            .filter(|x| {
                x.trip_update
                    .clone() // Fix performance eventually.
                    .unwrap() // Guaranteed safety by has_stop_id().
                    .stop_time_update
                    .into_iter()
                    .find(|x| x.clone().stop_id.unwrap() == stop_id) // Guaranteed safety by has_stop_id().
                    .is_some()
            })
            .collect(),
    }
}

fn string_to_int(input: String) -> i64 {
    let conv: i64 = input.parse().expect("Converted to integer");
    conv
}

pub fn within_bounds(input: &String, left: &str, right: &str) -> bool {
    for int in string_to_int(left.to_string())..string_to_int(right.to_string()) + 1 {
        if int.to_string() == *input {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;
    use gtfs_realtime::trip_update::StopTimeUpdate;
    use gtfs_realtime::*;

    #[tokio::test]
    async fn prt_vehicles_test() {
        let x = protobuf_request_from_url("https://truetime.portauthority.org/gtfsrt-bus/vehicles").await;
        let r = filter_for_in_range("6801", "6840", x.unwrap());
        println!("{:#?}", r);
    }

    #[tokio::test]
    async fn prt_vehicles_test_two() {
        let x = protobuf_request_from_url("https://truetime.portauthority.org/gtfsrt-bus/vehicles").await;
        let r = filter_for_in_range("7000", "7106", x.unwrap());
        println!("{:#?}", r);
    }

    #[tokio::test]
    async fn route() {
        let x = protobuf_request_from_url("https://truetime.portauthority.org/gtfsrt-bus/vehicles").await;
        let r = filter_for_on_route("74", x.unwrap());
        println!("{:#?}", r);
    }

    #[tokio::test]
    async fn prt_trips_test() {
        let x = protobuf_request_from_url("https://truetime.portauthority.org/gtfsrt-bus/trips").await;
        let r = filter_for_in_range("6701", "6740", x.unwrap());
        println!("{:#?}", r);
    }

    #[tokio::test]
    async fn prt_alerts_test() {
        let x = protobuf_request_from_url("https://truetime.portauthority.org/gtfsrt-bus/alerts").await;
        println!("{:#?}", x);
    }

    #[tokio::test]
    async fn vehicles_approaching_route_test() {
        let x = protobuf_request_from_url("https://truetime.portauthority.org/gtfsrt-bus/trips").await;
        let r = vehicles_approaching_stop(x.unwrap(), "10920".to_string());
        println!("{:#?}", r);
    }

    #[test]
    fn on_route_filter_test() {
        // This test has two FeedEntity objects, one matches our condition.
        // Goal: Want to show that on_route removes the object not matching our
        // constraints and that the result only returns objects which meet
        // our conditions.

        // Entity one of our test FeedMessage.
        let trip_descriptor_one = TripDescriptor {
            trip_id: None,
            route_id: Some("61C".to_string()),
            direction_id: Some(1),
            start_time: None,
            start_date: None,
            schedule_relationship: None,
            modified_trip: None,
        };

        let stop_time_update_one = StopTimeUpdate {
            stop_sequence: None,
            stop_id: None,
            arrival: None,
            departure: None,
            departure_occupancy_status: None,
            schedule_relationship: None,
            stop_time_properties: None,
        };

        let mut vec_stop_time_update_one: Vec<StopTimeUpdate> = Vec::new();
        vec_stop_time_update_one.push(stop_time_update_one);

        let trip_update_one = TripUpdate {
            trip: trip_descriptor_one,
            vehicle: None,
            stop_time_update: vec_stop_time_update_one,
            timestamp: None,
            delay: None,
            trip_properties: None,
        };

        let entity_one = FeedEntity {
            id: "10000000".to_string(),
            is_deleted: None,
            trip_update: Some(trip_update_one),
            vehicle: None,
            alert: None,
            shape: None,
            stop: None,
            trip_modifications: None,
        };

        // Entity two of our test FeedMessage. Has a different route.
        let trip_descriptor_two = TripDescriptor {
            trip_id: None,
            route_id: Some("61D".to_string()),
            direction_id: Some(1),
            start_time: None,
            start_date: None,
            schedule_relationship: None,
            modified_trip: None,
        };

        let stop_time_update_two = StopTimeUpdate {
            stop_sequence: None,
            stop_id: None,
            arrival: None,
            departure: None,
            departure_occupancy_status: None,
            schedule_relationship: None,
            stop_time_properties: None,
        };

        let mut vec_stop_time_update_two: Vec<StopTimeUpdate> = Vec::new();
        vec_stop_time_update_two.push(stop_time_update_two);

        let trip_update_two = TripUpdate {
            trip: trip_descriptor_two,
            vehicle: None,
            stop_time_update: vec_stop_time_update_two,
            timestamp: None,
            delay: None,
            trip_properties: None,
        };

        let entity_two = FeedEntity {
            id: "10000001".to_string(),
            is_deleted: None,
            trip_update: Some(trip_update_two),
            vehicle: None,
            alert: None,
            shape: None,
            stop: None,
            trip_modifications: None,
        };

        let mut vec_entity: Vec<FeedEntity> = Vec::new();
        vec_entity.push(entity_one);
        vec_entity.push(entity_two);

        let header = FeedHeader {
            gtfs_realtime_version: "2.0".to_string(),
            incrementality: None,
            timestamp: None,
        };

        let message = FeedMessage {
            header: header,
            entity: vec_entity,
        };

        let route = "61C";
        let on_route_message = filter_for_on_route(route, message.clone());
        assert!(message.entity.len() == 2);
        assert!(on_route_message.entity.len() == 1);

        for i in on_route_message.entity {
            assert!(i.trip_update.unwrap().trip.route_id.unwrap() == route);
        }

        // This test takes a FeedMessage and tests whether filtering it will
        // remove an element with a route that is not the input.
    }

    #[test]
    fn in_range_filter_test() {
        // This test has two FeedEntity objects, one matches our condition.
        // Goal: Want to show that on_route removes the object not matching our
        // constraints and that the result only returns objects which meet
        // our conditions.
        // Entity one of our test FeedMessage.
        let trip_descriptor_one = TripDescriptor {
            trip_id: None,
            route_id: Some("61C".to_string()),
            direction_id: Some(1),
            start_time: None,
            start_date: None,
            schedule_relationship: None,
            modified_trip: None,
        };

        let vehicle_descriptor_one = VehicleDescriptor {
            id: Some("3401".to_string()),
            label: None,
            license_plate: None,
            wheelchair_accessible: None,
        };

        let stop_time_update_one = StopTimeUpdate {
            stop_sequence: None,
            stop_id: None,
            arrival: None,
            departure: None,
            departure_occupancy_status: None,
            schedule_relationship: None,
            stop_time_properties: None,
        };

        let mut vec_stop_time_update_one: Vec<StopTimeUpdate> = Vec::new();
        vec_stop_time_update_one.push(stop_time_update_one);

        let trip_update_one = TripUpdate {
            trip: trip_descriptor_one,
            vehicle: Some(vehicle_descriptor_one),
            stop_time_update: vec_stop_time_update_one,
            timestamp: None,
            delay: None,
            trip_properties: None,
        };

        let entity_one = FeedEntity {
            id: "10000000".to_string(),
            is_deleted: None,
            trip_update: Some(trip_update_one),
            vehicle: None,
            alert: None,
            shape: None,
            stop: None,
            trip_modifications: None,
        };

        // Entity two of our test FeedMessage. Has a different route.
        let trip_descriptor_two = TripDescriptor {
            trip_id: None,
            route_id: Some("61D".to_string()),
            direction_id: Some(1),
            start_time: None,
            start_date: None,
            schedule_relationship: None,
            modified_trip: None,
        };

        let vehicle_descriptor_two = VehicleDescriptor {
            id: Some("3501".to_string()),
            label: None,
            license_plate: None,
            wheelchair_accessible: None,
        };

        let stop_time_update_two = StopTimeUpdate {
            stop_sequence: None,
            stop_id: None,
            arrival: None,
            departure: None,
            departure_occupancy_status: None,
            schedule_relationship: None,
            stop_time_properties: None,
        };

        let mut vec_stop_time_update_two: Vec<StopTimeUpdate> = Vec::new();
        vec_stop_time_update_two.push(stop_time_update_two);

        let trip_update_two = TripUpdate {
            trip: trip_descriptor_two,
            vehicle: Some(vehicle_descriptor_two),
            stop_time_update: vec_stop_time_update_two,
            timestamp: None,
            delay: None,
            trip_properties: None,
        };

        let entity_two = FeedEntity {
            id: "10000001".to_string(),
            is_deleted: None,
            trip_update: Some(trip_update_two),
            vehicle: None,
            alert: None,
            shape: None,
            stop: None,
            trip_modifications: None,
        };

        let mut vec_entity: Vec<FeedEntity> = Vec::new();
        vec_entity.push(entity_one);
        vec_entity.push(entity_two);

        let header = FeedHeader {
            gtfs_realtime_version: "2.0".to_string(),
            incrementality: None,
            timestamp: None,
        };

        let message = FeedMessage {
            header: header,
            entity: vec_entity,
        };

        let low = "3400";
        let high = "3425";
        let on_route_message = filter_for_in_range(low, high, message.clone());
        assert!(message.entity.len() == 2);
        assert!(on_route_message.entity.len() == 1);
    }
}
