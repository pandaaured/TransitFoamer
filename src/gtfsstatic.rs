use std::{collections::HashMap, fs};

// -------- BEGIN MODULE CODE -------- //

/* This module deals with getting usable data out of the GTFS Static files
which each agency uses to define their schedule statically.  */

pub fn get_data(file_path: String) {
    let mut static_data = GtfsStaticData::new();

    let read= fs::read_to_string(file_path.clone())
                       .expect("Should be valid.");
    let mut data: Vec<&str> = read.split('\n')
                                  .collect();
    let header: Vec<&str> = data.clone().remove(0)
                                           .split(',')
                                           .collect();
    static_data.header = header;

    
    if file_path.ends_with("agency.txt") {
        let index = static_data.index("agency_id"); // Guaranteed safe.
        let &key = data.iter()
                                     .nth(index)
                                     .unwrap();
        

    } else if file_path.ends_with("stops.txt") {
        let index = static_data.index("stop_id");
    }


    

    println!("{:?}", data);
}

pub fn data_header(file_path: String) {

}

// pub mod data {
//     use std::collections::{HashMap, HashSet};

//     pub fn trips_per_route(city: &str) -> HashMap<String, Vec<String>> {
//         let mut trips_per_route: HashMap<String, Vec<String>> = HashMap::new();

//         let mut file_path = String::from("src/static");
//         file_path.push_str(city);
//         let gtfs = Gtfs::new(file_path.as_str()).expect("Should have been able to read file.");
//         let trips = gtfs.trips;

//         for item in trips {
//             let route = item.1.route_id;
//             let trip_id = item.0;
//             if trips_per_route.contains_key(&route) {
//                 let mut vector = trips_per_route.remove(&route).unwrap();
//                 vector.push(trip_id.clone());
//                 trips_per_route.insert(route.clone(), vector.clone());
//             } else {
//                 trips_per_route.insert(route, vec![trip_id]);
//             }
//         }

//         trips_per_route
//     }

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

/* This module deals with presenting information about the system's routes and
services to the user. */

pub mod service {
    // use gtfs_structures::Gtfs;

    // pub fn routes(city: &str) {
    //     let mut file_path = String::from("src/static");
    //     file_path.push_str(city);
    //     let gtfs = Gtfs::new(file_path.as_str()).expect("Should have been able to read file.");
    //     let routes = gtfs.routes;

    //     let mut route_list: Vec<&String> = routes.keys().collect();
    //     route_list.sort();

    //     let number_of_routes = route_list.len();
    //     println!("This transit network has {} routes", number_of_routes);
    //     for route in route_list {
    //         // if city == "/seattle/king_county/" {
    //         //     let number = &routes[route][bindings::routes(city, "route_short_name")];
    //         //     let text = &routes[route][bindings::routes(city, "route_desc")];
    //         //     println!("{} {}", number, text);
    //         // } else {
    //         //     let number = &routes[route][bindings::routes(city, "route_id")];
    //         //     let text = &routes[route][bindings::routes(city, "route_long_name")];
    //         //     println!("{} {}", number, text);
    //         // }
    //     }
    // }
}

pub mod utils {
    // use crate::gtfsstatic::data;
    // use std::collections::HashMap;
    // use std::fs;

    // pub fn static_data(city: &str, function: &str) -> HashMap<String, Vec<String>> {
    //     let file_path = file_path(city, function);
    //     let ingest_data = read_to_string(file_path);
    //     data::processing_layer_one::hashmap_populate(ingest_data, function)
    // }

    // pub fn file_path(city: &str, file: &str) -> String {
    //     let mut string: String = "src/static".to_string();
    //     string.push_str(city);
    //     string.push_str(file);
    //     string.push_str(".txt");

    //     string
    // }

    // pub fn read_to_string(path: String) -> String {
    //     let contents: String =
    //         fs::read_to_string(path).expect("Should have been able to read the file");
    //     contents
    // }
}

pub struct GtfsStaticData<'a> {
    header: Vec<&'a str>,
    data: HashMap<String, Vec<String>>
}

impl<'a> GtfsStaticData<'a> {
    fn get_field(&self, key: &str, tag: &str) -> &String {
        let header = &self.header;
        let index = header.iter()
                                   .position(|&x| x == key)
                                   .unwrap(); // This is guaranteed safe.
        let &field = &self.data
                                                  .get(key)
                                                  .unwrap() // Not guaranteed safe.
                                                  .iter()
                                                  .nth(index)
                                                  .unwrap();

        field
    }

    fn index(&self, key: &str) -> usize {
        let header = &self.header;
        let index = header.iter()
                                 .position(|&x| x == key)
                                 .unwrap();

        index
    }

    fn index_data(&self) -> HashMap<&str, i8> {
        let mut data = HashMap::new();
        let header = &self.header;
        let mut count: i8 = 0;
        for item in header {
            data.insert(*item, count);
            count += 1;
        }

        data
    }
    fn new() -> Self {
        GtfsStaticData {
            header: Vec::new(),
            data: HashMap::new(),
        }
    }


}


// -------- END MODULE CODE -------- //

// -------- BEGIN MODULE TESTS -------- //
