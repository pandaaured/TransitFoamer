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
    use crate::gtfsstatic::{bindings, data};

    pub fn routes(city: &str) {
        let routes = data::static_data(city, "routes");

        let mut route_list: Vec<&String> = routes.keys().collect();
        route_list.sort();

        let number_of_routes = route_list.len();
        println!("This transit network has {} routes", number_of_routes);
        for route in route_list {
            if city == "/seattle/king_county/" {
                let number = &routes[route][bindings::routes(city, "route_short_name")];
                let text = &routes[route][bindings::routes(city, "route_desc")];
                println!("{} {}", number, text);
            } else {
                let number = &routes[route][bindings::routes(city, "route_id")];
                let text = &routes[route][bindings::routes(city, "route_long_name")];
                println!("{} {}", number, text);
            }
        }
    }
}

/* This module deals with mapping the routewords used by the GTFS static feed and
ensures that indices return information that the program wants to display */

/* REMARK: This is done manually because there are annoying discrepancies
between agencies and the functions that each keyword fulfills. Requires that
cities are placed in specific directories. */

pub mod bindings {
    pub fn agency(city: &str, token: &str) -> usize {
        match city {
            "/pittsburgh/prt/" => match token {
                "agency_id" => 0,
                "agency_name" => 1,
                "agency_url" => 2,
                "agency_timezone" => 3,
                "agency_lang" => 4,
                "agency_phone" => 5,
                "agency_fare_url" => 6,
                _ => panic!(),
            },
            "/san_antonio/via/" => match token {
                "agency_id" => 3,
                "agency_name" => 4,
                "agency_url" => 2,
                "agency_timezone" => 5,
                "agency_lang" => 6,
                "agency_phone" => 0,
                "agency_fare_url" => 1,
                _ => panic!(),
            },
            "/seattle/king_county/" => match token {
                "agency_id" => 0,
                "agency_name" => 1,
                "agency_url" => 2,
                "agency_timezone" => 3,
                "agency_lang" => 4,
                "agency_phone" => 5,
                "agency_fare_url" => 6,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn calendar_dates(city: &str, token: &str) -> usize {
        match city {
            "/pittsburgh/prt/" => match token {
                "service_id" => 0,
                "date" => 1,
                "exception_type" => 2,
                _ => panic!(),
            },
            "/san_antonio/via/" => match token {
                "service_id" => 0,
                "date" => 1,
                "exception_type" => 2,
                _ => panic!(),
            },
            "/seattle/king_county/" => match token {
                "service_id" => 0,
                "date" => 1,
                "exception_type" => 2,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn calendar(city: &str, token: &str) -> usize {
        match city {
            "/pittsburgh/prt/" => match token {
                "service_id" => 0,
                "start_date" => 8,
                "end_date" => 9,
                "monday" => 1,
                "tuesday" => 2,
                "wednesday" => 3,
                "thursday" => 4,
                "friday" => 5,
                "saturday" => 6,
                "sunday" => 7,
                _ => panic!(),
            },
            "/san_antonio/via/" => match token {
                "service_id" => 0,
                "start_date" => 1,
                "end_date" => 2,
                "monday" => 3,
                "tuesday" => 4,
                "wednesday" => 5,
                "thursday" => 6,
                "friday" => 7,
                "saturday" => 8,
                "sunday" => 9,
                _ => panic!(),
            },
            "/seattle/king_county/" => match token {
                "service_id" => 0,
                "start_date" => 8,
                "end_date" => 9,
                "monday" => 1,
                "tuesday" => 2,
                "wednesday" => 3,
                "thursday" => 4,
                "friday" => 5,
                "saturday" => 6,
                "sunday" => 7,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn routes(city: &str, token: &str) -> usize {
        match city {
            "/pittsburgh/prt/" => match token {
                "route_id" => 0,
                "agency_id" => 1,
                "route_short_name" => 2,
                "route_long_name" => 3,
                "route_desc" => 4,
                "route_type" => 5,
                "route_url" => 6,
                "route_color" => 7,
                "route_text_color" => 8,
                _ => panic!(),
            },
            "/san_antonio/via/" => match token {
                "route_id" => 5,
                "agency_id" => 4,
                "route_short_name" => 8,
                "route_long_name" => 0,
                "route_desc" => 7,
                "route_type" => 1,
                "route_url" => 6,
                "route_color" => 3,
                "route_text_color" => 2,
                _ => panic!(),
            },
            "/seattle/king_county/" => match token {
                "route_id" => 0,
                "agency_id" => 1,
                "route_short_name" => 2,
                "route_long_name" => 3,
                "route_desc" => 4,
                "route_type" => 5,
                "route_url" => 6,
                "route_color" => 7,
                "route_text_color" => 8,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn stop_times(city: &str, token: &str) -> usize {
        match city {
            "/pittsburgh/prt/" => match token {
                "trip_id" => 0,
                "arrival_time" => 1,
                "departure_time" => 2,
                "stop_id" => 3,
                "stop_sequence" => 4,
                "stop_headsign" => 5,
                "pickup_type" => 6,
                "drop_off_type" => 7,
                "shape_dist_traveled" => 8,
                "timepoint" => 9,
                _ => panic!(),
            },
            "/san_antonio/via/" => match token {
                "trip_id" => 0,
                "arrival_time" => 1,
                "departure_time" => 2,
                "stop_id" => 3,
                "stop_sequence" => 4,
                "stop_headsign" => 5,
                "pickup_type" => 6,
                "drop_off_type" => 7,
                "shape_dist_traveled" => 8,
                "timepoint" => 9,
                _ => panic!(),
            },
            "/seattle/king_county/" => match token {
                "trip_id" => 0,
                "arrival_time" => 1,
                "departure_time" => 2,
                "stop_id" => 3,
                "stop_sequence" => 4,
                "stop_headsign" => 5,
                "pickup_type" => 6,
                "drop_off_type" => 7,
                "shape_dist_traveled" => 8,
                "timepoint" => 9,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn stops(city: &str, token: &str) -> usize {
        match city {
            "/pittsburgh/prt/" => match token {
                "stop_id" => 0,
                "stop_code" => 1,
                "stop_name" => 2,
                "stop_desc" => 3,
                "stop_lat" => 4,
                "stop_lon" => 5,
                "zone_id" => 6,
                "stop_url" => 7,
                "location_type" => 8,
                "parent_station" => 9,
                "stop_timezone" => 10,
                "wheelchair_boarding" => 11,
                _ => panic!(),
            },
            "/san_antonio/via/" => match token {
                "stop_id" => 10,
                "stop_code" => 2,
                "stop_name" => 8,
                "stop_desc" => 7,
                "stop_lat" => 0,
                "stop_lon" => 3,
                "zone_id" => 11,
                "stop_url" => 5,
                "location_type" => 9,
                "parent_station" => 6,
                "stop_timezone" => 4,
                "wheelchair_boarding" => 1,
                _ => panic!(),
            },
            "/seattle/king_county/" => match token {
                "stop_id" => 0,
                "stop_code" => 1,
                "stop_name" => 2,
                "stop_desc" => 3,
                "stop_lat" => 4,
                "stop_lon" => 5,
                "zone_id" => 6,
                "stop_url" => 7,
                "location_type" => 8,
                "parent_station" => 9,
                "stop_timezone" => 10,
                "wheelchair_boarding" => 11,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }

    pub fn trips(city: &str, token: &str) -> usize {
        match city {
            "/pittsburgh/prt/" => match token {
                "trip_id" => 0,
                "route_id" => 1,
                "service_id" => 2,
                "trip_headsign" => 3,
                "trip_short_name" => 4,
                "direction_id" => 5,
                "block_id" => 6,
                "shape_id" => 7,
                "wheelchair_accessible" => 8,
                "bikes_allowed" => 9,
                _ => panic!(),
            },
            "/san_antonio/via/" => match token {
                "trip_id" => 8,
                "route_id" => 2,
                "service_id" => 7,
                "trip_headsign" => 5,
                "trip_short_name" => 9,
                "direction_id" => 4,
                "block_id" => 0,
                "shape_id" => 6,
                "wheelchair_accessible" => 3,
                "bikes_allowed" => 1,
                _ => panic!(),
            },
            "/seattle/king_county/" => match token {
                "trip_id" => 2,
                "route_id" => 0,
                "service_id" => 1,
                "trip_headsign" => 3,
                "trip_short_name" => 4,
                "direction_id" => 5,
                "block_id" => 6,
                "shape_id" => 7,
                "wheelchair_accessible" => 10,
                "bikes_allowed" => 11,
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
