use std::fs;
use std::collections::HashMap;
use std::process::Command;


fn main() {
    let data_vec = string_to_vec_of_vec_of_vec();
    // let agency = vec_to_hashmap_ag(&data_vec[0]);
    // let calendar_dates = vec_to_hashmap_cd(&data_vec[1]);
    let routes = vec_to_hashmap_rt(&data_vec[4]);


    // let calendar_dates_vec = &data_vec[1];
    // let calendar_vec = &data_vec[2];
    // let routes_vec = &data_vec[4];
    // let shapes_vec = &data_vec[5];
    // let stop_times_vec = &data_vec[6];
    // let stops_vec = &data_vec[7];
    // let trips_vec = &data_vec[9];
    println!("{:#?}", routes);
}

fn vec_to_hashmap_ag(vec: &Vec<Vec<String>>) -> HashMap<String, Agency> {
    let mut data = HashMap::new();
    for row in vec {
        let ag_struct: Agency = Agency 
            {agency_name: row[1].clone(), 
             agency_url: row[2].clone(), 
             agency_timezone: row[3].clone(), 
             agency_lang: row[4].clone(), 
             agency_phone: row[5].clone(),
            agency_fare_url: row[6].clone()};
        data.insert(row[0].clone(), ag_struct); }   
    data
}

fn vec_to_hashmap_cd(vec: &Vec<Vec<String>>) -> HashMap<String, CalendarDates> {
    let mut data = HashMap::new();
    for row in vec {
        let cd_struct: CalendarDates = CalendarDates 
            {date: row[1].clone(),
             exception_type: row[2].clone()};
        data.insert(row[0].clone(), cd_struct); }   
    data
}

fn vec_to_hashmap_rt(vec: &Vec<Vec<String>>) -> HashMap<String, Routes> {
    let mut data = HashMap::new();
    for row in vec {
        let rt_struct: Routes = Routes 
            {agency_id: row[1].clone(),
             route_short_name: row[2].clone(),
             route_long_name: row[3].clone(),
             route_desc: row[4].clone(),
             route_type: row[5].clone(),
             route_url: row[6].clone(),
             route_color: row[7].clone(),
             route_text_color: row[8].clone()};
        data.insert(row[0].clone(), rt_struct); }   
    data
}

fn string_to_vec_of_vec_of_vec() -> Vec<Vec<Vec<String>>> {
    let mut double_collection: Vec<Vec<Vec<String>>> = Vec::new();
    
    let items: [&str; 10] = ["src/static/pittsburgh/prt/agency.txt",
    "src/static/pittsburgh/prt/calendar_dates.txt",
    "src/static/pittsburgh/prt/calendar.txt",
    "src/static/pittsburgh/prt/feed_info.txt",
    "src/static/pittsburgh/prt/routes.txt",
    "src/static/pittsburgh/prt/shapes.txt",
    "src/static/pittsburgh/prt/stop_times.txt",
    "src/static/pittsburgh/prt/stops.txt",
    "src/static/pittsburgh/prt/transfers.txt",
    "src/static/pittsburgh/prt/trips.txt"];
    
    for path in items {
        let string = 
        fs::read_to_string(path)
            .expect("Should be able to read file.");
        let agency_vec = string_to_vec_of_vec(&string);
        double_collection.push(agency_vec);
    }

    double_collection
}

fn string_to_vec_of_vec(str: &String) -> Vec<Vec<String>> {
    let mut collection: Vec<Vec<String>> = Vec::new();
    let vector: Vec<String> = str.lines().map(String::from).collect();
    for i in vector.iter() {
        let row = string_to_vector(i);
        collection.push(row);
    }
    collection
}

fn string_to_vector(str: &String) -> Vec<String> {
    let split = str.split(',');
    let terms: Vec<String> = split.map(String::from).collect();
    terms
}

#[derive(Debug)]
struct Agency {
    agency_name: String,
    agency_url: String,
    agency_timezone: String,
    agency_lang: String,
    agency_phone: String,
    agency_fare_url: String
}

#[derive(Debug)]
struct CalendarDates {
    date: String,
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

#[derive(Debug)]
struct Routes {
    agency_id: String,
    route_short_name: String,
    route_long_name: String,
    route_desc: String,
    route_type: String,
    route_url: String,
    route_color: String,
    route_text_color: String
}

// struct Trips {
//     route_id: String,
//     service_id: String,
//     trip_id: String,
//     trip_headsign: String,
//     trip_short_name: String,
//     direction_id: String,
//     block_id: String,
//     shape_id: String,
//     wheelchair_accessible: String,
//     bikes_allowed: String
// }