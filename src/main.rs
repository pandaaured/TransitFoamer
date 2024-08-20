use std::fs;

fn main() {
    let base_directory: String = "src/static/".to_string();
    let subdirectory_1 = "pittsburgh/";
    let subdirectory_2 = "prt/";
    let file_chosen = "agency.txt"; // Function is currently fixed. Wish to change this later.
    let request = base_directory + subdirectory_1 + subdirectory_2 + file_chosen;
    let string = fs::read_to_string(request)
        .expect("Should be able to read file.");
    let header_index = header_end_index(&string);

    let header_string = &string[..header_index].to_string();
    string_to_vector(header_string);

    

    let vector: Vec<String> = string.lines().map(String::from).collect();
}

fn header_end_index(str: &String) -> usize {
    let index: Option<usize> = str.find("\n");
    let mut value: usize = 0;
    if index != None {
        value = index.unwrap();
    }
    value
}

fn string_to_vector(str: &String) {
    let split = str.split(',');
    let terms: Vec<String> = split.map(String::from).collect();
    println!("{}", terms[0]);
}

#[derive(Debug)]
struct Agency {
    agency_id: i32,
    agency_name: String,
    agency_url: String,
    agency_timezone: String,
    agency_lang: String,
    agency_phone: String,
    agency_fare_url: String
}

struct CalendarDates {
    service_id: String,
    date: i32,
    exception_type: String
}

struct Calendar {
    service_id: String,
    monday: bool,
    tuesday: bool,
    wednesday: bool,
    thursday: bool,
    friday: bool,
    saturday: bool,
    sunday: bool,
    start_date: i32,
    end_date: i32
}

struct Routes {
    route_id: String,
    agency_id: String,
    route_short_name: String,
    route_long_name: String,
    route_desc: String,
    route_type: String,
    route_url: String,
    route_color: String,
    route_text_color: String
}

struct Trips {
    route_id: String,
    service_id: String,
    trip_id: String,
    trip_headsign: String,
    trip_short_name: String,
    direction_id: String,
    block_id: String,
    shape_id: String,
    wheelchair_accessible: String,
    bikes_allowed: String
}