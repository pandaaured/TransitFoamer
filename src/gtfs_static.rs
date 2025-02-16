use std::{collections::HashMap, fs};

// -------- BEGIN MODULE CODE -------- //

/* This module deals with getting usable data out of the GTFS Static files
which each agency uses to define their schedule statically.  */

fn get_data(file_path: String, function: &str) -> GtfsStaticData {
    let mut static_data = GtfsStaticData::new();
    let mut path: String = file_path.clone();
    path.push_str(function);

    let mut data_hash: HashMap<Key, Vec<String>> = HashMap::new();
    println!("{}", file_path);
    let read: String = fs::read_to_string(path)
                       .expect("Should be valid.");
    let data: Vec<String> = read.split('\n').map(|s|s.to_string()).collect();
    let header: Vec<String> = read.clone().split('\n').map(|s|s.to_string()).collect();
    let header = header.iter().nth(0).unwrap().split(',').map(|s|s.to_string()).collect();

    static_data.header = header;

    
    let key = get_key(function); // Assuming we're using 
                                                    // a valid file name here
    let key = key.unwrap();
   

    if key.is_one() {
        let key = key.unwrap_one();
        let index = static_data.index(key);    // Guaranteed safe.
        for line in data {
            let iter: Vec<String> = line.split(',').map(|s|s.to_string()).collect();
            let key = iter.iter().nth(index).unwrap().to_string();
            data_hash.insert(Key::One(key), iter);
        }
    } else {
        let key = key.unwrap_two();
        let index_one_value = key.0;
        let index_two_value = key.1;
        let index_one = static_data.index(index_one_value);
        let index_two = static_data.index(index_two_value);
        for line in data {
            let iter: Vec<String> = line.split(',').map(|s|s.to_string()).collect();
            let key_one = iter.iter().nth(index_one).unwrap().to_string();
            let key_two = iter.iter().nth(index_two).unwrap().to_string();
            data_hash.insert(Key::Two((key_one, key_two)), iter);
        }
    }

    static_data.data = data_hash;
    static_data
}

pub fn get_agency(file_path: String) -> GtfsStaticData {
    get_data(file_path, "agency.txt")
}

pub fn get_calendar(file_path: String) -> GtfsStaticData {
    get_data(file_path, "calendar.txt")
}

pub fn get_calendar_dates(file_path: String) -> GtfsStaticData {
    get_data(file_path, "calendar_dates.txt")
}

pub fn get_routes(file_path: String) -> GtfsStaticData {
    get_data(file_path, "routes.txt")
}

pub fn get_stops(file_path: String) -> GtfsStaticData {
    get_data(file_path, "stops.txt")
}

pub fn get_stop_times(file_path: String) -> GtfsStaticData {
    get_data(file_path, "stop_times.txt")
}

pub fn get_trips(file_path: String) -> GtfsStaticData {
    get_data(file_path, "trips.txt")
}

// use std::collections::{HashMap, HashSet};
// use crate::gtfs_static;

// pub fn trips_per_route(file_path: String, function: &str) -> HashMap<String, Vec<String>> {
//     let mut trips_per_route: HashMap<String, Vec<String>> = HashMap::new();
//     let mut path = file_path.to_string();
//     path.push_str(function);


//     let trips = gtfs_static::get_data(file_path, function);


//     for item in trips.data.keys() {
//         let route = trips.data.get(k)
//         let route = item.1.route_id;
//         let trip_id = item.0;
//         if trips_per_route.contains_key(&route) {
//             let mut vector = trips_per_route.remove(&route).unwrap();
//             vector.push(trip_id.clone());
//             trips_per_route.insert(route.clone(), vector.clone());
//         } else {
//             trips_per_route.insert(route, vec![trip_id]);
//         }
//     }

//     trips_per_route

//     pub fn stops_per_trip(city: &str) -> HashMap<String, Vec<String>> {
//         let mut stops_per_trip: HashMap<String, Vec<String>> = HashMap::new();

//         let mut file_path = String::from("src/static");
//         file_path.push_str(city);
//         let gtfs = Gtfs::new(file_path.as_str()).expect("Should have been able to read file.");
//         let trips = gtfs.trips;

//         for item in trips {
//             let stop_times = item.1.stop_times;
//             let trip_id = item.0;
//             if stops_per_trip.contains_key(&trip_id.clone()) {
//                 for stop_time in stop_times {
//                     let stop_id = &stop_time.stop.id;
//                     let mut vector = stops_per_trip[&trip_id.clone()].clone();
//                     vector.push(stop_id.clone());
//                     stops_per_trip.insert(trip_id.clone(), vector.clone());
//                 }
//             } else {
//                 for stop_time in stop_times {
//                     let stop_id = &stop_time.stop.id;
//                     stops_per_trip.insert(trip_id.clone(), vec![stop_id.clone()]);
//                 }
//             }
//         }

//         stops_per_trip
//     }

//     pub fn routes_per_stop(city: &str) -> HashMap<String, HashSet<String>> {
//         let mut routes_per_stop: HashMap<String, HashSet<String>> = HashMap::new();

//         let trips_per_route = trips_per_route(city);
//         let stops_per_trip = stops_per_trip(city);

//         for route in trips_per_route {
//             let trip_vec = route.clone().1;
//             for trip in trip_vec {
//                 let stops = stops_per_trip.get(&trip).unwrap();
//                 for stop in stops {
//                     if !routes_per_stop.contains_key(stop) {
//                         let mut set = HashSet::new();
//                         set.insert(route.clone().0);
//                         routes_per_stop.insert(stop.to_string(), set);
//                     } else {
//                         let mut set = routes_per_stop.get(stop).unwrap().to_owned();
//                         set.insert(route.clone().0);
//                         routes_per_stop.insert(stop.to_string(), set);
//                     }
//                 }
//             }
//         }

//         println!("{:?}", routes_per_stop);

//         routes_per_stop
//     }

//     // let mut file_path = String::from("src/static");
//     // file_path.push_str(city);
//     // let gtfs = Gtfs::new(file_path.as_str()).expect("Should have been able to read file.");
//     // let trips = gtfs.trips;

//     // for i in trips {
//     //     let stop_times = i.1.stop_times;
//     //     for j in stop_times {
//     //         j.stop.id
//     //     }
//     // }

//     // let keys = static_stop_times.keys();

//     // for key in keys {
//     //     let line = &static_stop_times[key];
//     //     let trip_id = &line[0];
//     //     let stop_id = &line[3];
//     //     let route = static_trips[trip_id][1].clone();
//     //     if !stops_per_route.contains_key(stop_id) {
//     //         let mut set = HashSet::new();
//     //         set.insert(route);
//     //         stops_per_route.insert(stop_id.to_string(), set);
//     //     } else {
//     //         let mut set = stops_per_route[stop_id].clone();
//     //         set.insert(route);
//     //         stops_per_route.insert(stop_id.to_string(), set);
//     //     }
//     // }
//     // stops_per_route
// }


// -------- BEGIN MODULE DATA -------- //

pub fn get_key(file: &str) -> Option<Key> {
    let key = match file {
        "agency.txt" => Some(Key::One("agency_id".to_string())),
        "stops.txt" => Some(Key::One("stop_id".to_string())),
        "routes.txt" => Some(Key::One("route_id".to_string())),
        "trips.txt" => Some(Key::One("trip_id".to_string())),
        "stop_times.txt" => Some(Key::Two(("trip_id".to_string(), "stop_sequence".to_string()))),
        "calendar.txt" => Some(Key::One("service_id".to_string())),
        "calendar_dates.txt" => Some(Key::Two(("service_id".to_string(), "date".to_string()))),
        "fare_attributes.txt" => Some(Key::One("fare_id".to_string())),
        "shapes.txt" => Some(Key::Two(("shape_id".to_string(), "shape_pt_sequence".to_string()))),
        "frequencies.txt" => Some(Key::Two(("trip_id".to_string(), "start_time".to_string()))),
        _ => None
    };

    key
}

// -------- END MODULE DATA -------- //

// -------- BEGIN MODULE STRUCTS -------- //

#[derive(Debug)]
pub struct GtfsStaticData {
    header: Vec<String>,
    data: HashMap<Key, Vec<String>>
}

impl GtfsStaticData {
    fn new() -> Self {
        GtfsStaticData {
            header: Vec::new(),
            data: HashMap::new(),
        }
    }
    
    fn index(&self, key: String) -> usize {
        let header = &self.header;
        println!("{:?}", header);
        let index = header.iter()
                                 .position(|x| *x == key)
                                 .unwrap();

        index
    }

    fn iter(&self) {
        let iter_keys: Vec<&Key> = self.data.keys().collect();

    }

    fn get_data_row(&self, key: Key) -> Vec<String> {
        let row: Vec<String> = self.data.get(&key).unwrap().to_owned();
        row
    }

}


#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Key {
    One(String),
    Two((String, String))
}

impl Key {
    fn is_one(&self) -> bool {
        if let Key::One(_) = self {
            return true
        }
        false
    }

    fn unwrap_one(&self) -> String {
        match self {
            Key::One(i) => i.to_string(),
            _ => panic!("Must be Key::One to unwrap with this method.")
        }
    }

    fn unwrap_two(&self) -> (String, String) {
        match self {
            Key::Two((i, j)) => (i.to_string(), j.to_string()),
            _ => panic!("Must be Key::Two to unwrap with this method.")
        }
    }

}

// impl Copy for Key { }

// impl Clone for Key {
//     fn clone(&self) -> Key {
//         *self
//     }
// }

