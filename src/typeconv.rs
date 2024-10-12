use std::collections::HashMap;
use std::fs;

use crate::classes;

pub fn vec_to_hashmap_agency(vec: &Vec<Vec<String>>) -> HashMap<String, classes::Agency> {
    let mut data = HashMap::new();
    for row in vec {
        let ag_struct: classes::Agency = classes::Agency 
            {agency_name: row[1].clone(), 
             agency_url: row[2].clone(), 
             agency_timezone: row[3].clone(), 
             agency_lang: row[4].clone(), 
             agency_phone: row[5].clone(),
            agency_fare_url: row[6].clone()};
        data.insert(row[0].clone(), ag_struct); }   
    data
}

pub fn vec_to_hashmap_cal(vec: &Vec<Vec<String>>) -> HashMap<String, classes::Calendar> {
    let mut data = HashMap::new();
    for row in vec {
        let rt_struct: classes::Calendar = classes::Calendar 
            {monday: row[1].clone(),
             tuesday: row[2].clone(),
             wednesday: row[3].clone(),
             thursday: row[4].clone(),
             friday: row[5].clone(),
             saturday: row[6].clone(),
             sunday: row[7].clone(),
             start_date: row[8].clone(),
             end_date: row[9].clone()};
        data.insert(row[0].clone(), rt_struct); }   
    data
}

pub fn vec_to_hashmap_cal_dates(vec: &Vec<Vec<String>>) -> HashMap<String, classes::CalendarDates> {
    let mut data = HashMap::new();
    for row in vec {
        let cd_struct: classes::CalendarDates = classes::CalendarDates 
            {date: row[1].clone(),
             exception_type: row[2].clone()};
        data.insert(row[0].clone(), cd_struct); }   
    data
}

pub fn vec_to_hashmap_routes(vec: &Vec<Vec<String>>) -> HashMap<String, classes::Routes> {
    let mut data = HashMap::new();
    for row in vec {
        let rt_struct: classes::Routes = classes::Routes 
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

pub fn vec_to_hashmap_stop_times(vec: &Vec<Vec<String>>) -> HashMap<String, classes::StopTimes> {
    let mut data = HashMap::new();
    for row in vec {
        let rt_struct: classes::StopTimes = classes::StopTimes 
            {arrival_time: row[1].clone(),
             departure_time: row[2].clone(),
             stop_id: row[3].clone(),
             stop_sequence: row[4].clone(),
             stop_headsign: row[5].clone(),
             pickup_type: row[6].clone(),
             drop_off_type: row[7].clone(),
             shape_dist_traveled: row[8].clone(),
             timepoint: row[9].clone()};
        data.insert(row[0].clone(), rt_struct); }   
    data
}

pub fn vec_to_hashmap_stops(vec: &Vec<Vec<String>>) -> HashMap<String, classes::Stops> {
    let mut data = HashMap::new();
    for row in vec {
        let rt_struct: classes::Stops = classes::Stops 
            {stop_code: row[1].clone(),
             stop_name: row[2].clone(),
             stop_desc: row[3].clone(),
             stop_lat: row[4].clone(),
             stop_lon: row[5].clone(),
             zone_id: row[6].clone(),
             stop_url: row[7].clone(),
             location_type: row[8].clone(),
             parent_station: row[9].clone(),
             stop_timezone: row[10].clone(),
             wheelchair_boarding: row[11].clone()};
        data.insert(row[0].clone(), rt_struct); }   
    data
}

pub fn vec_to_hashmap_trips(vec: &Vec<Vec<String>>) -> HashMap<String, classes::Trips> {
    let mut data = HashMap::new();
    for row in vec {
        let rt_struct: classes::Trips = classes::Trips
            {route_id: row[1].clone(),
             service_id: row[2].clone(),
             trip_id: row[3].clone(),
             trip_headsign: row[4].clone(),
             trip_short_name: row[5].clone(),
             direction_id: row[6].clone(),
             block_id: row[7].clone(),
             shape_id: row[8].clone(),
             wheelchair_accessible: row[9].clone(),
             bikes_allowed: row[10].clone()};
        data.insert(row[0].clone(), rt_struct); }   
    data
}

pub struct Trips {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: String,
    pub trip_short_name: String,
    pub direction_id: String,
    pub block_id: String,
    pub shape_id: String,
    pub wheelchair_accessible: String,
    pub bikes_allowed: String
}


pub fn string_to_vec_of_vec_of_vec() -> Vec<Vec<Vec<String>>> {
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
        let vec = string_to_vec_of_vec(&string);
        double_collection.push(vec);
    }

    double_collection
}

pub fn string_to_vec_of_vec(str: &String) -> Vec<Vec<String>> {
    let mut collection: Vec<Vec<String>> = Vec::new();
    let vector: Vec<String> = str.lines().map(String::from).collect();
    for i in vector.iter() {
        let row = string_to_vector(i);
        collection.push(row);
    }
    collection
}

pub fn string_to_vector(str: &String) -> Vec<String> {
    let split = str.split(',');
    let terms: Vec<String> = split.map(String::from).collect();
    terms
}