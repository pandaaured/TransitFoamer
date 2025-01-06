// -------- BEGIN MODULE CODE -------- //

/* This module deals with getting usable data out of the GTFS Static files
which each agency uses to define their schedule statically.  */

pub mod data {
    use crate::gtfsstatic::{
        utils,
        which::{get_first, get_second, is_one, File, Size},
    };
    use std::{collections::HashMap, slice::Iter};

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
}

/* This module deals with presenting information about the system's routes and
services to the user. */

pub mod service {
    use crate::gtfsstatic::data;

    pub fn routes(city: &str) {
        let routes = data::static_data(city, "routes");

        let mut route_list: Vec<&String> = routes.keys().collect();
        route_list.sort();

        let number_of_routes = route_list.len();
        println!("This transit network has {} routes", number_of_routes);
        for route in route_list {
            if city == "/seattle/king_county/" {
                let number = &routes[route][2];
                let text = &routes[route][4];
                println!("{} {}", number, text);
            } else if city == "/pittsburgh/prt/" {
                let number = &routes[route][0];
                let text = &routes[route][3];
                println!("{} {}", number, text);
            } else if city == "/san_antonio/via/" {
                let number = &routes[route][5];
                let text = &routes[route][0];
                println!("{} {}", number, text);
            }
        }
    }
}

/* This module deals with mapping the routewords used by the GTFS static feed and
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
    use crate::gtfsstatic::data;
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
