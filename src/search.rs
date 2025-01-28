use gtfs_realtime::{FeedEntity, FeedMessage};

// pub async fn at_stop(stop: &str, city: &str, message: FeedMessage) {
// let routes_per_stop = data::routes_per_stop(city);
// let routes_to_check = routes_per_stop.get(stop).unwrap();

// // let trips = gtfsrt::requester(city, "trips").await;

// let entities = message.entity;
// // let entities = trips.entity;
// let result = entities.into_iter().filter(|x| {
//     routes_to_check.contains(
//         x.trip_update
//             .as_ref()
//             .unwrap()
//             .trip
//             .route_id
//             .as_ref()
//             .unwrap(),
//     )
// });
// println!("{:?}", result);
// }

pub fn on_route(number: &str, message: FeedMessage) -> FeedMessage {
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
        } else {
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
        }
    });
    let on_route = result.collect();

    let message_filtered = FeedMessage {
        header: message.header,
        entity: on_route,
    };

    message_filtered
}

pub fn in_range(first: &str, last: &str, message: FeedMessage) -> FeedMessage {
    let entities = message.entity;
    let result = entities.into_iter().filter(|x| {
        if x.vehicle.is_some() { // First check for a VehiclePosition
            utilities::within_bounds(
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
        } else { // Now check for a TripUpdate.
            utilities::within_bounds(
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
        }
    });
    let in_range: Vec<FeedEntity> = result.collect();

    let message_filtered = FeedMessage {
        header: message.header,
        entity: in_range,
    };

    message_filtered
}

mod utilities {
    // use chrono::{Local, TimeZone};

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

    // pub fn time_converter(input: i64) -> chrono::DateTime<Local> {
    //     let date_time: chrono::DateTime<Local> = Local.timestamp_opt(input, 0).unwrap();
    //     date_time
    // }
}

// -------- END MODULE CODE -------- //

#[cfg(test)]
mod test {
    use super::*;
    use crate::search;
    use gtfs_realtime::trip_update::StopTimeUpdate;
    use gtfs_realtime::*;

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
        let on_route_message = search::on_route(route, message.clone());
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
        let on_route_message = search::in_range(low, high, message.clone());
        assert!(message.entity.len() == 2);
        assert!(on_route_message.entity.len() == 1);
    }
}
