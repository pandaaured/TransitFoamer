// -------- BEGIN MODULE CODE -------- //

/* This module deals with getting usable data out of the GTFS Static files
which each agency uses to define their schedule statically.  */

pub mod data {
    use crate::gtfs::gtfsstatic::{
        utils,
        which::{get_first, get_second, is_one, File, Size},
        data
    };
    use std::{collections::HashMap, collections::HashSet, slice::Iter};

    pub fn static_data(city: &str, function: &str) -> HashMap<String, Vec<String>> {
        let file_path = utils::file_path(city, function);
        let ingest_data = utils::read_to_string(file_path);
        hashmap_populate(ingest_data, function)
    }

    pub fn static_data_unprocessed(city: &str, function: &str) -> String {
        let file_path = utils::file_path(city, function);
        utils::read_to_string(file_path)
    }

    pub fn hashmap_populate(data: String, function: &str) -> HashMap<String, Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut iterator = data.split('\n');
        let header: Vec<&str> = iterator.next().unwrap().split(',').collect();
        let header_iter = header.iter();
        let index = match function_to_enum(function) {
            File::Agency => Size::One(find_index("agency_id", &header_iter).unwrap()),
            File::Calendar => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::CalendarDates => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::FareAttributes => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::FareRules => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::FeedInfo => Size::One(find_index("feed_publisher_name", &header_iter).unwrap()),
            File::Frequencies => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::Routes => Size::One(find_index("route_id", &header_iter).unwrap()),
            File::Shapes => Size::One(find_index("shape_id", &header_iter).unwrap()),
            File::StopTimes => Size::Two(
                find_index("trip_id", &header_iter).unwrap(),
                find_index("stop_sequence", &header_iter).unwrap(),
            ),
            File::Stops => Size::One(find_index("stop_id", &header_iter).unwrap()),
            File::Transfers => Size::One(find_index("from_stop_id", &header_iter).unwrap()),
            File::Trips => Size::One(find_index("trip_id", &header_iter).unwrap()),
        };

        if is_one(&index) {
            let key_to_insert = match index {
                Size::One(x) => x,
                Size::Two(x, _) => x,
            };
            for i in iterator {
                let v: Vec<&str> = i.split(',').collect();
                let w: Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
                let key = &w[key_to_insert];
                map.insert(key.clone(), w);
            }
        } else {
            let key_elem_one = get_first(&index);
            let key_elem_two = get_second(&index);
            for i in iterator {
                let v: Vec<&str> = i.split(',').collect();
                let w: Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
                let key_pt_1 = &w[key_elem_one];
                let key_pt_2 = &w[key_elem_two];
                let mut str = String::from("(");
                str.push_str(key_pt_1.to_string().as_str());
                str.push(',');
                str.push_str(key_pt_2.to_string().as_str());
                str.push(')');
                map.insert(str.clone(), w);
            }
        }
        map
    }

    pub fn find_index(keyword: &str, header: &Iter<'_, &str>) -> Option<usize> {
        header.clone().position(|&x| x == keyword)
    }

    fn function_to_enum(function: &str) -> File {
        let file: File = match function {
            "agency" => File::Agency,
            "calendar" => File::Calendar,
            "calendar_dates" => File::CalendarDates,
            "fare_attributes" => File::FareAttributes,
            "fare_rules" => File::FareRules,
            "feed_info" => File::FeedInfo,
            "frequencies" => File::Frequencies,
            "routes" => File::Routes,
            "shapes" => File::Shapes,
            "stop_times" => File::StopTimes,
            "stops" => File::Stops,
            "transfers" => File::Transfers,
            "trips" => File::Trips,
            _ => panic!(),
        };

        file
    }

    pub fn trips_per_route(city: &str) -> HashMap<String, Vec<String>> {
        let mut trips_per_route: HashMap<String, Vec<String>> = HashMap::new();
        let trips: HashMap<String, Vec<String>> =
            data::static_data(city, "trips");
        for item in trips {
            let route = item.1[1].clone();
            let trip_id = item.0;
            if trips_per_route.contains_key(&route.clone()) {
                let mut vector = trips_per_route[&route.clone()].clone();
                vector.push(trip_id.clone());
                trips_per_route.insert(route.clone(), vector.clone());
            } else {
                trips_per_route.insert(route, vec![trip_id]);
            }
        }
        
        trips_per_route
    }

    pub fn stops_per_trip(city: &str) -> HashMap<String, Vec<String>> {
        let mut stops_per_trip: HashMap<String, Vec<String>> = HashMap::new();
        let stop_times: HashMap<String, Vec<String>> =
        data::static_data(city, "stop_times");
        for item in stop_times {
            let stop_id = item.1[3].clone();
            let trip_id = item.1[0].clone();
            if stops_per_trip.contains_key(&trip_id.clone()) {
                let mut vector = stops_per_trip[&trip_id.clone()].clone();
                vector.push(stop_id.clone());
                stops_per_trip.insert(trip_id.clone(), vector.clone());
            } else {
                stops_per_trip.insert(trip_id, vec![stop_id]);
            }
        }
        stops_per_trip
    }

    fn stops_per_route(city: &str) -> HashMap<String, HashSet<String>> {
        let mut stops_per_route: HashMap<String, HashSet<String>> = HashMap::new();
        let stops_per_trip: HashMap<String, Vec<String>> = stops_per_trip(&city);
        let trips_per_route = trips_per_route(&city);
        for item in trips_per_route {
            let mut stops_set: HashSet<String> = HashSet::new();
            let route = item.0;
            let mut trips = item.1;
            while !trips.is_empty() {
                let elem = trips.pop();
                if elem.is_some() {
                    let elem = elem.unwrap();
                    let stops: &Vec<String> = &stops_per_trip[&elem];
                    while !stops.is_empty() {
                        let mut stops_new = stops.clone();
                        let stop = stops_new.pop();
                        if stop.is_some() {
                            stops_set.insert(stop.clone().unwrap());
                        }
                    }
                }
            }
            stops_per_route.insert(route, stops_set);
        }
        stops_per_route
    }

    }

    




/* This module deals with presenting information about the system's routes and
services to the user. */

pub mod service {
    use crate::gtfs::gtfsstatic::data;

    pub fn routes(city: &str) {
        let routes = data::static_data(city, "routes");
        let mut route_list: Vec<&String> = routes.keys().collect();
        route_list.sort();

        let number_of_routes = route_list.len();
        println!("This transit network has {} routes", number_of_routes);
        for route in route_list {
            println!(" {} {}", route, routes[route][0])
        }
    }
}

/* This module deals with mapping the keywords used by the GTFS static feed and
ensures that indices return information that the program wants to display */

pub mod bindings {
    pub fn routes(city: &str, token: &str) -> usize {
        match token {
            "route_number" => match city {
                "/chicago/cta/" => 2,
                "/pittsburgh/prt/" => 0,
                "/san_antonio/via/" => 5,
                _ => panic!(),
            },
            "route_name_long" => match city {
                "/chicago/cta/" => 2,
                "/pittsburgh/prt/" => 3,
                "/san_antonio/via/" => 0,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn stops(city: &str, token: &str) -> usize {
        match token {
            "stop_id" => match city {
                "/chicago/cta/" => 0,
                "/pittsburgh/prt/" => 0,
                "/san_antonio/via/" => 10,
                _ => panic!(),
            },
            "stop_name" => match city {
                "/chicago/cta/" => 2,
                "/pittsburgh/prt/" => 2,
                "/san_antonio/via/" => 8,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn trips(city: &str, token: &str) -> usize {
        match token {
            "route_id" => match city {
                "/chicago/cta/" => 0,
                "/pittsburgh/prt/" => 1,
                "/san_antonio/via/" => 2,
                _ => panic!(),
            },
            "trip_id" => match city {
                "/chicago/cta/" => 2,
                "/pittsburgh/prt/" => 0,
                "/san_antonio/via/" => 8,
                _ => panic!(),
            },
            "trip_headsign" => match city {
                "/pittsburgh/prt/" => 3,
                "/san_antonio/via/" => 5,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

pub mod which {
    pub enum File {
        Agency,
        CalendarDates,
        Calendar,
        FareAttributes,
        FareRules,
        FeedInfo,
        Frequencies,
        Routes,
        Shapes,
        StopTimes,
        Stops,
        Transfers,
        Trips,
    }

    pub enum Size {
        One(usize),
        Two(usize, usize),
    }

    pub fn is_one(entry: &Size) -> bool {
        match entry {
            Size::One(_) => true,
            Size::Two(_, _) => false,
        }
    }

    pub fn get_first(entry: &Size) -> usize {
        match entry {
            Size::One(x) => *x,
            Size::Two(x, _) => *x,
        }
    }

    pub fn get_second(entry: &Size) -> usize {
        match entry {
            Size::One(x) => *x,
            Size::Two(_, y) => *y,
        }
    }
}

pub mod utils {
    use crate::gtfs::gtfsstatic::data;
    use std::collections::HashMap;
    use std::fs;

    pub fn static_data(city: &str, function: &str) -> HashMap<String, Vec<String>> {
        let file_path = file_path(city, function);
        let ingest_data = read_to_string(file_path);
        data::hashmap_populate(ingest_data, function)
    }

    pub fn file_path(city: &str, file: &str) -> String {
        let mut string: String = "src/static".to_string();
        string.push_str(city);
        string.push_str(file);
        string.push_str(".txt");

        string
    }

    pub fn read_to_string(path: String) -> String {
        let contents: String =
            fs::read_to_string(path).expect("Should have been able to read the file");
        contents
    }
}

