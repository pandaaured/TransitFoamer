pub mod fetch {
    pub mod any_city {
        use crate::{gtfsrt, gtfsstatic, gtfsstatic::data, search::utilities};
        use chrono::Local;
        use gtfs_realtime::{trip_update::StopTimeUpdate, TripUpdate, FeedEntity};
        use std::collections::HashMap;
        use gtfs_structures::Gtfs;

        pub async fn at_stop(stop: &str, city: &str) {
            let mut file_path = String::from("src/static");
            file_path.push_str(city);
            let gtfs = Gtfs::new(file_path.as_str()).expect("Should have been able to read file.");

            let routes_per_stop = data::processing_layer_two::routes_per_stop(city);
            let routes_to_check = routes_per_stop.get(stop).unwrap();

            let trips = gtfsrt::requester(city, "trips").await;
            let entities = trips.entity;
            let result = entities.into_iter().filter(|x| routes_to_check.contains(x.trip_update.as_ref().unwrap().trip.route_id.as_ref().unwrap()));
            println!("{:?}", result);

            
        }

        pub async fn on_route(number: &str, city: &str) -> Vec<FeedEntity> {
            let trips = gtfsrt::requester(city, "trips").await;
            let entities = trips.entity;
            let result = entities.into_iter().filter(|x| *x.trip_update.as_ref().unwrap().trip.route_id.as_ref().unwrap() == *number.to_owned());
            let on_route = result.collect();
            on_route
        }

        pub async fn in_range(first: &str, last: &str, city: &str) -> Vec<FeedEntity> {
            let trips = gtfsrt::requester(city, "trips").await;
            let entities = trips.entity;
            let result = entities.into_iter().filter(|x| utilities::within_bounds(x.trip_update.as_ref().unwrap().vehicle.as_ref().unwrap().id.as_ref().unwrap(), first, last));
            let on_route: Vec<FeedEntity> = result.collect();

            on_route
        }
    }
}

mod utilities {
    use chrono::{Local, TimeZone};

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

    pub fn time_converter(input: i64) -> chrono::DateTime<Local> {
        let date_time: chrono::DateTime<Local> = Local.timestamp_opt(input, 0).unwrap();
        date_time
    }
}

// -------- END MODULE CODE -------- //