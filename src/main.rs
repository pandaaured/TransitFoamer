use std::any::type_name_of_val;
use std::env;
use std::str::Lines;
use std::iter::Iterator;
use std::collections::HashSet;
use std::collections::HashMap;
use reqwest;
use std::fs::File;
use std::fs;
use serde_json::{Result, Value};
use serde_json::json;

fn main() {
    let args: Vec<String> = env::args().collect();
    // request_maker(); // For making HTTP(S) requests.
    static_feed_json("seattle", "king_county");
}

fn static_feed_json(city: &str, agency: &str) {
    let file = fs::read_to_string("data/static_feeds.json").unwrap();
    let json = json!(file);
}

// fn request_maker() {
//     let url =  String::from("https://truetime.portauthority.org/gtfsrt-bus/vehicles");
//     let body = reqwest::get(url);
// }


fn gtfs_static_file_fetcher(city_name: &str, file_name: &str) -> String {
    // Constructing the file name for each file.
    let mut file_path = "static/".to_string();
    file_path.push_str(city_name);
    file_path.push('/');
    file_path.push_str(file_name);
    println!("In file {}", file_path);
    let file_contents = fs::read_to_string(file_path).unwrap();
    file_contents
}


fn gtfs_static_string_processor(string: &str) {
    let lines_from_string = string.lines();





}



// fn line_splitter(line: &str, file_name: &str) {
//     let term = line.split(",");
//     let variable: Vec<&str> = term.collect();
//
//
//     // let mut agency = HashMap::new();
//     // let mut calendar_dates = HashMap::new();
//     // let mut calendar = HashMap::new();
//     // let mut routes = HashMap::new();
//     // let mut stops = HashMap::new();
//     // let mut trips = HashMap::new();
// }

//
//     if file_name == "agency" {
//         let term = agency_assignments(variable);
//         let name = term.name.to_string();
//         agency.insert(name, term);
//     } else if file_name == "calendar_dates" {
//         let term = calendar_dates_assignments(variable);
//     } else if file_name == "calendar" {
//         let term = calendar_assignments(variable);
//     } else if file_name == "routes" {
//         let term = routes_assignments(variable);
//         let route_id = term.route_id.to_string();
//         routes.insert(route_id, term);
//     } else if file_name == "stops" {
//         let term = stops_assignments(variable);
//     } else if file_name == "trips" {
//         let term = trips_assignments(variable);
//     }
//
// }
// //
// // fn agency_assignments(variable: Vec<&str>) -> Agency {
// //     // Populating agency fields with information.
// //     let agency_id = variable[0].to_string();
// //     let agency_name = variable[1].to_string();
// //     let agency_url = variable[2].to_string();
// //     let agency_timezone = variable[3].to_string();
// //     let agency_lang = variable[4].to_string();
// //     let agency_phone = variable[5].to_string();
// //     let agency_fare_url = variable[6].to_string();
// //
// //
// //
// //
// // fn calendar_dates_assignments(variable: Vec<&str>) -> CalendarDates {
// //     // Populating calendar_dates fields with information.
// //     let service_id = variable[0].to_string();
// //     let date = variable[1].to_string();
// //     let exception_type = variable[2].to_string();
// //
// //     let calendar_dates_info = CalendarDates {
// //         service_id: service_id,
// //         date: date,
// //         exception_type: exception_type,
// //     };
// //     calendar_dates_info
// // }
// //
// // fn calendar_assignments(variable: Vec<&str>) -> Calendar {
// //     let service_id = variable[0].to_string();
// //     let monday = variable[1].to_string();
// //     let tuesday = variable[2].to_string();
// //     let wednesday = variable[3].to_string();
// //     let thursday = variable[4].to_string();
// //     let friday = variable[5].to_string();
// //     let saturday = variable[6].to_string();
// //     let sunday = variable[7].to_string();
// //     let start_date = variable[8].to_string();
// //     let end_date = variable[9].to_string();
// //
// //     let calendar_info = Calendar {
// //         service_id: service_id,
// //         monday: monday,
// //         tuesday: tuesday,
// //         wednesday: wednesday,
// //         thursday: thursday,
// //         friday: friday,
// //         saturday: saturday,
// //         sunday: sunday,
// //         start_date: start_date,
// //         end_date: end_date,
// //     };
// //     calendar_info
// // }
// //
// // fn routes_assignments(variable: Vec<&str>) -> Routes {
// //     let route_id = variable[0].to_string();
// //     let agency_id = variable[1].to_string();
// //     let route_short_name = variable[2].to_string();
// //     let route_long_name = variable[3].to_string();
// //     let route_desc = variable[4].to_string();
// //     let route_type = variable[5].to_string();
// //     let route_url = variable[6].to_string();
// //     let route_color = variable[7].to_string();
// //     let route_text_color = variable[8].to_string();
// //
// //     let routes_info = Routes {
// //         route_id: route_id,
// //         agency_id: agency_id,
// //         route_short_name: route_short_name,
// //         route_long_name: route_long_name,
// //         route_desc: route_desc,
// //         route_type: route_type,
// //         route_url: route_url,
// //         route_color: route_color,
// //         route_text_color: route_text_color
//     };
//     routes_info
// }
//
// fn stops_assignments(variable: Vec<&str>) -> Stops {
//     let stop_id = variable[0].to_string();
//     let stop_code = variable[1].to_string();
//     let stop_name = variable[2].to_string();
//     let stop_desc = variable[3].to_string();
//     let stop_lat = variable[4].to_string();
//     let stop_lon = variable[5].to_string();
//     let zone_id = variable[6].to_string();
//     let stop_url = variable[7].to_string();
//     let location_type = variable[8].to_string();
//     let parent_station = variable[9].to_string();
//     let stop_timezone = variable[10].to_string();
//     let wheelchair_boarding = variable[11].to_string();
//
//     let stops_info = Stops {
//         stop_id: stop_id,
//         stop_code: stop_code,
//         stop_name: stop_name,
//         stop_desc: stop_desc,
//         stop_lat: stop_lat,
//         stop_lon: stop_lon,
//         zone_id: zone_id,
//         stop_url: stop_url,
//         location_type: location_type,
//         parent_station: parent_station,
//         stop_timezone: stop_timezone,
//         wheelchair_boarding: wheelchair_boarding,
//     };
//     stops_info
// }
//
// fn trips_assignments(variable: Vec<&str>) -> Trips {
//     let trip_id = variable[0].to_string();
//     let route_id = variable[1].to_string();
//     let service_id = variable[2].to_string();
//     let trip_headsign = variable[3].to_string();
//     let trip_short_name = variable[4].to_string();
//     let direction_id = variable[5].to_string();
//     let block_id = variable[6].to_string();
//     let shape_id = variable[7].to_string();
//     let wheelchair_accessible = variable[8].to_string();
//     let bikes_allowed = variable[9].to_string();
//
//     let trips_info = Trips {
//         trip_id: trip_id,
//         route_id: route_id,
//         service_id: service_id,
//         trip_headsign: trip_headsign,
//         trip_short_name: trip_short_name,
//         direction_id: direction_id,
//         block_id: block_id,
//         shape_id: shape_id,
//         wheelchair_accessible: wheelchair_accessible,
//         bikes_allowed: bikes_allowed
//     };
//     trips_info
// }


// pub struct Agency {
//     id: String,
//     name: String,
//     url: String,
//     timezone: String,
//     lang: String,
//     phone: String,
//     fare_url: String
// }
//
// pub struct CalendarDates {
//     service_id: String,
//     date: String,
//     exception_type: String
// }
//
// pub struct Calendar {
//     service_id: String,
//     monday: String,
//     tuesday: String,
//     wednesday: String,
//     thursday: String,
//     friday: String,
//     saturday: String,
//     sunday: String,
//     start_date: String,
//     end_date: String
// }
//
// pub struct FareAttributes {
//     fare_id: String,
//     price: String,
//     currency_type: String,
//     payment_method: String,
//     transfers: String,
//     transfer_duration: String
// }
//
// pub struct FareRules {
//
// }
//
// pub struct Frequencies {
//
// }
//
// pub struct Routes {
//     route_id: String,
//     agency_id: String,
//     route_short_name: String,
//     route_long_name: String,
//     route_desc: String,
//     route_type: String,
//     route_url: String,
//     route_color: String,
//     route_text_color: String
// }
//
// pub struct Shapes {
//
// }
//
// pub struct StopTimes {
//
// }
//
// pub struct Stops {
//     stop_id: String,
//     stop_code: String,
//     stop_name: String,
//     stop_desc: String,
//     stop_lat: String,
//     stop_lon: String,
//     zone_id: String,
//     stop_url: String,
//     location_type: String,
//     parent_station: String,
//     stop_timezone: String,
//     wheelchair_boarding: String
// }
//
// pub struct Transfers {
//     from_stop_id: String,
//     to_stop_id: String,
//     transfer_type: String,
//     min_transfer_time: String
// }
//
// pub struct Trips {
//     trip_id: String,
//     route_id: String,
//     service_id: String,
//     trip_headsign: String,
//     trip_short_name: String,
//     direction_id: String,
//     block_id: String,
//     shape_id: String,
//     wheelchair_accessible: String,
//     bikes_allowed: String
// }
//

