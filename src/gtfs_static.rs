// -------- BEGIN MODULE CODE -------- //

//! GTFS_STATIC
//!
//! A library for fetching and analyzing GTFS Static data from a transit feed.

pub mod get {
    pub mod vecs {
        use crate::gtfs_static::structs::{
            Agency, Calendar, CalendarDates, Routes, Shapes, 
            StopTimes, Stops, Trips,
        };
        use std::io::Error;

        pub fn agency(file_path: String) -> Result<Vec<Agency>, Error> {
            let mut agency: Vec<Agency> = Vec::new(); // Initializes the mutable data.
            let mut path: String = file_path.clone(); // Getting the file path and
            path.push_str("agency.txt");
            let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
            for result in rdr.deserialize() {
                let record: Agency = result?;
                agency.push(record);
            }

            Ok(agency)
        }

        pub fn calendardates(file_path: String) -> Result<Vec<CalendarDates>, Error> {
            let mut calendardates: Vec<CalendarDates> = Vec::new(); // Initializes the mutable data.
            let mut path: String = file_path.clone(); // Getting the file path and
            path.push_str("calendar_dates.txt");
            let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
            for result in rdr.deserialize() {
                let record: CalendarDates = result?;
                calendardates.push(record);
            }

            Ok(calendardates)
        }

        pub fn calendar(file_path: String) -> Result<Vec<Calendar>, Error> {
            let mut calendar: Vec<Calendar> = Vec::new(); // Initializes the mutable data.
            let mut path: String = file_path.clone(); // Getting the file path and
            path.push_str("calendar.txt");
            let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
            for result in rdr.deserialize() {
                let record: Calendar = result?;
                calendar.push(record);
            }

            Ok(calendar)
        }

        pub fn routes(file_path: String) -> Result<Vec<Routes>, Error> {
            let mut routes: Vec<Routes> = Vec::new(); // Initializes the mutable data.
            let mut path: String = file_path.clone(); // Getting the file path and
            path.push_str("routes.txt");
            let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
            for result in rdr.deserialize() {
                let record: Routes = result?;
                routes.push(record);
            }

            Ok(routes)
        }

        pub fn shapes(file_path: String) -> Result<Vec<Shapes>, Error> {
            let mut shapes: Vec<Shapes> = Vec::new(); // Initializes the mutable data.
            let mut path: String = file_path.clone(); // Getting the file path and
            path.push_str("shapes.txt");
            let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
            for result in rdr.deserialize() {
                let record: Shapes = result?;
                shapes.push(record);
            }

            Ok(shapes)
        }

        pub fn stops(file_path: String) -> Result<Vec<Stops>, Error> {
            let mut stops: Vec<Stops> = Vec::new(); // Initializes the mutable data.
            let mut path: String = file_path.clone(); // Getting the file path and
            path.push_str("stops.txt");
            let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
            for result in rdr.deserialize() {
                let record: Stops = result?;
                stops.push(record);
            }

            Ok(stops)
        }

        pub fn stoptimes(file_path: String) -> Result<Vec<StopTimes>, Error> {
            let mut stoptimes: Vec<StopTimes> = Vec::new(); // Initializes the mutable data.
            let mut path: String = file_path.clone(); // Getting the file path and
            path.push_str("stop_times.txt");
            let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
            for result in rdr.deserialize() {
                let record: StopTimes = result?;
                stoptimes.push(record);
            }

            Ok(stoptimes)
        }

        pub fn trips(file_path: String) -> Result<Vec<Trips>, Error> {
            let mut trips: Vec<Trips> = Vec::new(); // Initializes the mutable data.
            let mut path: String = file_path.clone(); // Getting the file path and
            path.push_str("trips.txt");
            let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
            for result in rdr.deserialize() {
                let record: Trips = result?;
                trips.push(record);
            }

            Ok(trips)
        }

        #[cfg(test)]
        mod test {
            use super::*;
    
            #[test]
            fn agency_prints() {
                let path = "src/static/pittsburgh/prt/";
                let agency = agency(path.to_string());
                println!("{:?}", agency);
            }
    
            #[test]
            fn calendar_prints() {
                let path = "src/static/pittsburgh/prt/";
                let calendar = calendar(path.to_string());
                println!("{:?}", calendar);
            }
    
            #[test]
            fn calendardates_prints() {
                let path = "src/static/pittsburgh/prt/";
                let calendardates = calendardates(path.to_string());
                println!("{:?}", calendardates);
            }
    
            #[test]
            fn routes_prints() {
                let path = "src/static/pittsburgh/prt/";
                let routes = routes(path.to_string());
                println!("{:?}", routes);
            }

            #[test]
            fn shapes_prints() {
                let path = "src/static/pittsburgh/prt/";
                let shapes = shapes(path.to_string());
                println!("{:?}", shapes);
            }
    
            #[test]
            fn stops_prints() {
                let path = "src/static/pittsburgh/prt/";
                let stops = stops(path.to_string());
                println!("{:?}", stops);
            }
    
            #[test]
            fn stoptimes_prints() {
                let path = "src/static/pittsburgh/prt/";
                let stoptimes = stoptimes(path.to_string());
                println!("{:?}", stoptimes);
            }
    
            #[test]
            fn trips_prints() {
                let path = "src/static/pittsburgh/prt/";
                let trips = trips(path.to_string());
                println!("{:?}", trips);
            }
        }
    }

    pub mod hash {
        use crate::gtfs_static::structs::{
            Agency, Calendar, CalendarDates, Routes, Shapes, StopTimes, Stops, Trips,
        };
        use std::collections::HashMap;

        pub fn agency(agency: Vec<Agency>) -> HashMap<String, Agency> {
            let mut map: HashMap<String, Agency> = HashMap::new();
            for entry in agency {
                let key: &String = entry.agency_id.as_ref().unwrap();
                map.insert(key.to_string(), entry);
            }

            map
        }

        pub fn calendardates(
            calendardates: Vec<CalendarDates>,
        ) -> HashMap<(String, String), CalendarDates> {
            let mut map: HashMap<(String, String), CalendarDates> = HashMap::new();
            for entry in calendardates {
                let key_one: &String = &entry.service_id;
                let key_two: &String = &entry.date;
                map.insert((key_one.to_string(), key_two.to_string()), entry);
            }

            map
        }

        pub fn calendar(calendar: Vec<Calendar>) -> HashMap<String, Calendar> {
            let mut map: HashMap<String, Calendar> = HashMap::new();
            for entry in calendar {
                let key: &String = &entry.service_id;
                map.insert(key.to_string(), entry);
            }

            map
        }

        pub fn routes(routes: Vec<Routes>) -> HashMap<String, Routes> {
            let mut map: HashMap<String, Routes> = HashMap::new();
            for entry in routes {
                let key: &String = &entry.route_id;
                map.insert(key.to_string(), entry);
            }

            map
        }

        pub fn shapes(shapes: Vec<Shapes>) -> HashMap<(String, String), Shapes> {
            let mut map: HashMap<(String, String), Shapes> = HashMap::new();
            for entry in shapes {
                let key_one: &String = &entry.shape_id;
                let key_two: &u32 = &entry.shape_pt_sequence;
                map.insert((key_one.to_string(), key_two.to_string()), entry);
            }

            map
        }

        pub fn stops(stops: Vec<Stops>) -> HashMap<String, Stops> {
            let mut map: HashMap<String, Stops> = HashMap::new();
            for entry in stops {
                let key: &String = &entry.stop_id;
                map.insert(key.to_string(), entry);
            }

            map
        }

        pub fn stoptimes(stoptimes: Vec<StopTimes>) -> HashMap<(String, String), StopTimes> {
            let mut map: HashMap<(String, String), StopTimes> = HashMap::new();
            for entry in stoptimes {
                let key_one: &String = &entry.trip_id;
                let key_two: &String = &entry.stop_sequence;
                map.insert((key_one.to_string(), key_two.to_string()), entry);
            }

            map
        }

        pub fn trips(trips: Vec<Trips>) -> HashMap<String, Trips> {
            let mut map: HashMap<String, Trips> = HashMap::new();
            for entry in trips {
                let key: &String = &entry.trip_id;
                map.insert(key.to_string(), entry);
            }

            map
        }
    }

    pub mod data {
        use std::collections::{HashMap, HashSet};
        use crate::gtfs_static::structs::{StopTimes, Trips};

        pub fn trips_per_route(trip_data: Vec<Trips>) -> HashMap<String, Vec<String>> {
            let mut trips_per_route: HashMap<String, Vec<String>> = HashMap::new();
            for item in trip_data {
                let route = item.route_id;
                let trip = item.trip_id;
                if trips_per_route.contains_key(&route) {
                    let mut vector = trips_per_route.remove(&route).unwrap();
                    vector.push(trip.clone());
                    trips_per_route.insert(route.clone(), vector.clone());
                } else {
                    trips_per_route.insert(route, vec![trip]);
                }
            }

            trips_per_route
        }

        // Needs more proper testing.
        pub fn stops_per_trip(stop_times_data: Vec<StopTimes>) -> HashMap<String, Vec<String>> {
            let mut stops_per_trip: HashMap<String, Vec<String>> = HashMap::new();

            for item in stop_times_data {
                let trip_id = item.trip_id;
                if stops_per_trip.contains_key(&trip_id.clone()) {
                    let stop_id = item.stop_id.unwrap();
                    let mut vector = stops_per_trip[&trip_id.clone()].clone();
                    vector.push(stop_id.clone());
                    stops_per_trip.insert(trip_id.clone(), vector.clone());
                } else {
                    let stop_id = item.stop_id.unwrap();
                    stops_per_trip.insert(trip_id.clone(), vec![stop_id.clone()]);
                }
            }

            stops_per_trip
        }

        // Needs more proper testing.
        pub fn routes_per_stop(
            trips_per_route: HashMap<String, Vec<String>>,
            stops_per_trip: HashMap<String, Vec<String>>,
        ) -> HashMap<String, HashSet<String>> {
            let mut routes_per_stop: HashMap<String, HashSet<String>> = HashMap::new();

            for route in trips_per_route {
                let trip_vec = route.clone().1;
                for trip in trip_vec {
                    let stops = stops_per_trip.get(&trip).unwrap();
                    for stop in stops {
                        if !routes_per_stop.contains_key(stop) {
                            let mut set = HashSet::new();
                            set.insert(route.clone().0);
                            routes_per_stop.insert(stop.to_string(), set);
                        } else {
                            let mut set = routes_per_stop.get(stop).unwrap().to_owned();
                            set.insert(route.clone().0);
                            routes_per_stop.insert(stop.to_string(), set);
                        }
                    }
                }
            }

            routes_per_stop
        }

        #[cfg(test)]
        mod test {
            use super::*;
            use crate::gtfs_static::get::vecs;

            #[test]
            fn routes_per_stop_test() {
                let path = "src/static/pittsburgh/prt/";
                let tpr = trips_per_route(vecs::trips(path.to_string()).unwrap());
                let spt = stops_per_trip(vecs::stoptimes(path.to_string()).unwrap());
                let rps = routes_per_stop(tpr, spt);
                println!("{:?}", rps);
            }
            
        }
    }
}

mod structs {
    // use std::time::{SystemTime, UNIX_EPOCH};
    // use chrono::{DateTime, Datelike};
    
    #[derive(Debug, serde::Deserialize)]
    pub struct Agency {
        pub agency_id: Option<String>,
        pub agency_name: String,
        pub agency_url: String,
        pub agency_timezone: String,
        pub agency_lang: Option<String>,
        pub agency_phone: Option<String>,
        pub agency_fare_url: Option<String>,
        pub agency_email: Option<String>,
    }
    
    #[derive(Debug, serde::Deserialize)]
    pub struct Calendar {
        pub service_id: String,
        pub monday: String,
        pub tuesday: String,
        pub wednesday: String,
        pub thursday: String,
        pub friday: String,
        pub saturday: String,
        pub sunday: String,
        pub start_date: String,
        pub end_date: String,
    }
    
    // impl Calendar {
        // fn is_valid_date(&mut self) -> bool {
        //     let time = match SystemTime::now().duration_since(UNIX_EPOCH) {
        //         Ok(n) => n.as_secs(),
        //         Err(_) => panic!("SystemTime before UNIX Epoch!")
        //     };
    
        //     let date = DateTime::from_timestamp(time.try_into().unwrap(), 0).unwrap();
        //     let year = date.year();
        //     let month = date.month();
        //     let day = date.day();
    
        //     let start_year = &self.start_date[0..4].parse::<i32>().unwrap();
        //     let start_month = &self.start_date[4..6].parse::<u32>().unwrap();
        //     let start_day = &self.start_date[6..8].parse::<u32>().unwrap();
        //     let end_year = &self.end_date[0..4].parse::<i32>().unwrap();
        //     let end_month = &self.end_date[4..6].parse::<u32>().unwrap();
        //     let end_day = &self.end_date[6..8].parse::<u32>().unwrap();
    
        //     if year < *start_year {
        //         return false;
        //     } else {
        //         if month < *start_month {
        //             return false;
        //         } else {
        //             if day < *start_day {
        //                 return false;
        //             }
        //         }
        //     }
    
        //     if year > *end_year {
        //         return false;
        //     } else {
        //         if month > *end_month {
        //             return false;
        //         } else {
        //             if day > *end_day {
        //                 return false;
        //             }
        //         }
        //     }
    
        //     true
        // }
    //     fn year(&self) -> i32 {
    //         self.start_date[0..4].parse::<i32>().unwrap()
    //     }

    //     fn month(&self) -> u32 {
    //         self.start_date[4..6].parse::<u32>().unwrap()
    //     }

    //     fn day(&self) -> u32 {
    //         self.start_date[6..8].parse::<u32>().unwrap()
    //     }

    // }

    // impl PartialEq for Calendar {
    //     fn eq(&self, other: &Self) -> bool {
    //         self.year() == other.year() &&
    //         self.month() == other.month() &&
    //         self.day() == other.day()
    //     }
    // }
    
    // #[cfg(test)]
    // mod test {
    //     use super::*;
    //     use crate::gtfs_static::{structs::Calendar,
    //         get::vecs};
    
    //     #[test]
    //     fn check_date_validity() {
    //         let path = "src/static/pittsburgh/prt/".to_string();
    //         let date = vecs::calendar(path).unwrap();
    //         for mut item in date {
    //             // println!("{}", item.is_valid_date());
    //         }
    //     }
    // }
    
    #[derive(Debug, serde::Deserialize)]
    pub struct CalendarDates {
        pub service_id: String,
        pub date: String,
        pub exception_type: String,
    }
    
    #[derive(Debug, serde::Deserialize)]
    pub struct Routes {
        pub route_id: String,
        pub agency_id: Option<String>,
        pub route_short_name: Option<String>,
        pub route_long_name: Option<String>,
        pub route_desc: Option<String>,
        pub route_type: String,
        pub route_url: Option<String>,
        pub route_color: Option<String>,
        pub route_text_color: Option<String>,
        pub route_sort_order: Option<String>, // Change to u32 eventually.
        pub continuous_pickup: Option<String>,
        pub continuous_dropoff: Option<String>,
        pub network_id: Option<String>,
    }
    
    #[derive(Debug, serde::Deserialize)]
    pub struct Shapes {
        pub shape_id: String,
        pub shape_pt_lat: f32,
        pub shape_pt_lon: f32,
        pub shape_pt_sequence: u32,
        pub shape_dist_traveled: Option<f32>,
    }
    
    #[derive(Debug, serde::Deserialize)]
    pub struct Stops {
        pub stop_id: String,
        pub stop_code: Option<String>,
        pub stop_name: Option<String>,
        pub tts_stop_name: Option<String>,
        pub stop_desc: Option<String>,
        pub stop_lat: Option<String>,
        pub stop_lon: Option<String>,
    }
    
    #[derive(Debug, serde::Deserialize)]
    pub struct StopTimes {
        pub trip_id: String,
        pub arrival_time: Option<String>,
        pub departure_time: Option<String>,
        pub stop_id: Option<String>,
        pub location_group_id: Option<String>,
        pub location_id: Option<String>,
        pub stop_sequence: String,
        pub stop_headsign: Option<String>,
        pub start_pickup_drop_off_window: Option<String>,
        pub end_pickup_drop_off_window: Option<String>,
        pub pickup_type: Option<String>,
        pub dropoff_type: Option<String>,
        pub continuous_pickup: Option<String>,
        pub continuous_dropoff: Option<String>,
        pub shape_dist_traveled: Option<String>,
        pub timepoint: Option<String>,
        pub pickup_booking_rule_id: Option<String>,
        pub drop_off_booking_rule_id: Option<String>,
    }
    
    #[derive(Debug, serde::Deserialize)]
    pub struct Trips {
        pub route_id: String,
        pub service_id: String,
        pub trip_id: String,
        pub trip_headsign: Option<String>,
        pub trip_short_name: String,
        pub direction_id: Option<String>,
        pub block_id: Option<String>,
        pub shape_id: Option<String>,
        pub wheelchair_accessible: Option<String>,
        pub bikes_allowed: Option<String>,
    }
}

