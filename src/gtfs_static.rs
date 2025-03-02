use std::{collections::HashMap, fs};

// -------- BEGIN MODULE CODE -------- //

/* This module deals with getting usable data out of the GTFS Static files
which each agency uses to define their schedule statically.  */

fn get_data(file_path: String, function: &str) -> GtfsStaticData {
    let mut static_data = GtfsStaticData::new();
    let mut path: String = file_path.clone();
    path.push_str(function);
    let mut data_hash: HashMap<Key, Vec<String>> = HashMap::new();
    let read: String = fs::read_to_string(path)
        .expect("Should be valid.");
    let data: Vec<String> = read.split('\n')
                                .map(|s| s.to_string())
                                .collect();
    let header: Vec<String> = read.clone()
                                  .split('\n')
                                  .map(|s| s.to_string())
                                  .collect();
    let header = header
        .iter()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|s| s.to_string())
        .collect();

    static_data.header = header;

    let key = get_key(function); // Assuming we're using
                                 // a valid file name here
    let key = key.unwrap();

    if key.is_one() {
        let key = key.unwrap_one();
        let index = static_data.index(key); // Guaranteed safe.
        for line in data {
            let iter: Vec<String> = line.split(',')
                                        .map(|s| s.to_string())
                                        .collect();
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
            let iter: Vec<String> = line.split(',')
                                        .map(|s| s.to_string())
                                        .collect();
            let key_one = iter.iter()
                                      .nth(index_one)
                                      .unwrap()
                                      .to_string();
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

pub fn get_key(file: &str) -> Option<Key> {
    let key = match file {
        "agency.txt" => Some(Key::One("agency_id".to_string())),
        "stops.txt" => Some(Key::One("stop_id".to_string())),
        "routes.txt" => Some(Key::One("route_id".to_string())),
        "trips.txt" => Some(Key::One("trip_id".to_string())),
        "stop_times.txt" => Some(Key::Two((
            "trip_id".to_string(),
            "stop_sequence".to_string(),
        ))),
        "calendar.txt" => Some(Key::One("service_id".to_string())),
        "calendar_dates.txt" => Some(Key::Two(("service_id".to_string(), "date".to_string()))),
        "fare_attributes.txt" => Some(Key::One("fare_id".to_string())),
        "shapes.txt" => Some(Key::Two((
            "shape_id".to_string(),
            "shape_pt_sequence".to_string(),
        ))),
        "frequencies.txt" => Some(Key::Two(("trip_id".to_string(), "start_time".to_string()))),
        _ => None,
    };

    key
}

// -------- END MODULE DATA -------- //

// -------- BEGIN MODULE STRUCTS -------- //

#[derive(Debug)]
pub struct GtfsStaticData {
    header: Vec<String>,
    data: HashMap<Key, Vec<String>>,
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
        let index = header.iter().position(|x| *x == key).unwrap();

        index
    }

    // fn iter(&self) {
    //     let iter_keys: Vec<&Key> = self.data.keys().collect();
    // }

    // fn get_data_row(&self, key: Key) -> Vec<String> {
    //     let row: Vec<String> = self.data.get(&key).unwrap().to_owned();
    //     row
    // }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Key {
    One(String),
    Two((String, String)),
}

impl Key {
    fn is_one(&self) -> bool {
        if let Key::One(_) = self {
            return true;
        }
        false
    }

    fn unwrap_one(&self) -> String {
        match self {
            Key::One(i) => i.to_string(),
            _ => panic!("Must be Key::One to unwrap with this method."),
        }
    }

    fn unwrap_two(&self) -> (String, String) {
        match self {
            Key::Two((i, j)) => (i.to_string(), j.to_string()),
            _ => panic!("Must be Key::Two to unwrap with this method."),
        }
    }
}
// -- GTFS Static structs which specify all data in the feeds. -- //
struct Agency {
    agency_id: Option<String>,
    agency_name: String,
    agency_url: String,
    agency_timezone: String,
    agency_lang: Option<String>,
    agency_phone: Option<String>,
    agency_fare_url: Option<String>,
    agency_email: Option<String>,
}

struct Calendar {
    service_id: String,
    monday: AvailableForall,
    tuesday:AvailableForall,
    wednesday: AvailableForall,
    thursday: AvailableForall,
    friday: AvailableForall,
    saturday: AvailableForall,
    sunday: AvailableForall,
    start_date: String,
    end_date: String,
}

struct CalendarDates {
    service_id: String,
    date: String,
    exception_type: ExceptionType,
}

struct Shapes {
    shape_id: String,
    shape_pt_lat: String,
    shape_pt_lon: String,
    shape_pt_sequence: String,
    shape_dist_traveled: Option<f32>,
}

struct Stops {
    stop_id: String,
    stop_code: Option<String>,
    stop_name: Option<String>,
    tts_stop_name: Option<String>,
    stop_desc: Option<String>,
    stop_lat: Option<String>,
    stop_lon: Option<String>,
}

struct Routes {
    route_id: String,
    agency_id: Option<String>,
    route_short_name: Option<String>,
    route_long_name: Option<String>,
    route_desc: Option<String>,
    route_type: String,
    route_url: Option<String>,
    route_color: Option<String>,
    route_text_color: Option<String>,
    route_sort_order: Option<u32>,
}

struct Trips {
    route_id: String,
    service_id: String,
    trip_id: String,
    trip_headsign: Option<String>,
    trip_short_name: String,
    direction_id: Direction,
    block_id: Option<String>,
    shape_id: Option<String>,
    wheelchair_accessible: Wheelchair,
    bikes_allowed: Bikes
}


enum AvailableForall {
    Zero, // Not available for all.
    One   // Available for all.
}

enum ExceptionType {
    One, // Service added for the specified date.
    Two  // Service removed for the specified date.
}

enum Direction {
    Zero,
    One
}

enum Wheelchair {
    Zero, // No accessibility information for the trip.
    One, // Vehicle cal accommodate at least one rider in a wheelchair.
    Two, // No riders in wheelchairs can be accommodated on this trip.
}

enum Bikes {
    Zero, // No vehicle information for this trip.
    One, // Vehicles on this trip can accommodate at least one bicycle.
    Two // No bicycles are allowed on this trip.
}

