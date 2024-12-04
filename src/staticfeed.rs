use std::fs;
use std::collections::HashMap;


pub fn static_data_vector(path_data: HashMap<String, String>) -> HashMap<String, HashMap<String, Vec<String>>> {
    let mut static_data: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    for path in path_data {
        let foo = path_io(path.0.clone(), path.1);
        static_data.insert(path.0.clone(), foo);
    }
    static_data
}

fn path_io(function: String, path: String) -> HashMap<String, Vec<String>> {
    let mut map : HashMap<String, Vec<String>> = HashMap::new();
    let data: String = path_io_helper(path);
    let iterator = data.split('\n');
    for i in iterator {
        let v: Vec<&str> = i.split(',').collect();
        let w : Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
        let mut key = String::new();
        assert!(!w.is_empty());
        if function == "agency" {       // Use pattern matching in a rewrite.
            key = w[0].clone();
        } else if function == "calendar" {
            key = w[0].clone();
        } else if function == "calendar_dates" {
            key = w[1].clone();
        } else if function == "fare_attributes" {
            key = w[0].clone();
        } else if function == "fare_rules" {
            key = w[0].clone();
        } else if function == "feed_info" {
            key = w[0].clone();
        } else if function == "frequencies" {
            key = w[0].clone();
        } else if function == "routes" {
            key = w[0].clone();
        } else if function == "shapes" {
            key = w[0].clone();
        } else if function == "stop_times" {
            key = w[0].clone();
        } else if function == "stops" {
            key = w[0].clone();
        } else if function == "transfers" {
            key = w[0].clone();
        } else if function == "trips" {
            key = w[0].clone();
        }
        map.insert(key.to_string(), w);
    }
    map
}

fn path_io_helper(path: String) -> String {
    println!("{}", path);
    let contents : String = 
       fs::read_to_string(path)
          .expect("Should have been able to read the file");
    contents 
}

// These functions are used/exhausted by `let path_data...` (line 14 main.rs)

pub fn static_data(city: &str) -> HashMap<String, String> {

    let files : Vec<&str> = city_files(&city);
    println!("Data for \x1b[0;31m{}\x1b[0m has been fetched.", city);
    let length = &files.len();

    let mut file_paths : HashMap<String, String> = HashMap::new();

    for file in files {
        file_paths.insert(file.to_string(), file_path(city, file));
    }
    println!("All files to import have been imported.");

    assert!(file_paths.len() == *length);
    file_paths
}

fn file_path(city: &str, file: &str) -> String {
    let mut string: String = "src/static".to_string();

    if city == "satx" { 
      string.push_str("/san_antonio/via/");
    } else if city == "pgh" {
      string.push_str("/pittsburgh/prt/");
    } else if city == "chicago" {
      string.push_str("/chicago/cta/");
    } else {
        panic!("Not a valid city!");
    }

    if file == "agency" {
        string.push_str("agency.txt");
    } else if file == "calendar" {
        string.push_str("calendar.txt");
    } else if file == "calendar_dates" {
        string.push_str("calendar_dates.txt");
    } else if file == "fare_attributes" {
        string.push_str("fare_attributes.txt");
    } else if file == "fare_rules" {
        string.push_str("fare_rules.txt");
    } else if file == "feed_info" {
        string.push_str("feed_info.txt");
    } else if file == "frequencies" {
        string.push_str("frequencies.txt");
    } else if file == "routes" {
        string.push_str("routes.txt");
    } else if file == "shapes" {
        string.push_str("shapes.txt");
    } else if file == "stops" {
        string.push_str("stops.txt");
    } else if file == "stop_times" {
        string.push_str("stop_times.txt");  
    } else if file == "transfers" {
        string.push_str("transfers.txt");
    } else if file == "trips" {
        string.push_str("trips.txt");
    } else {
        panic!("Not a valid city!");
    }

    string
}

fn city_files(city: &str) -> Vec<&str> {
    println!("Checking city input: you inputted \x1b[0;31m{}\x1b[0m", city);
    let mut files: Vec<&str> = vec![];
    if city == "satx" {
        files.push("agency");
        files.push("calendar");
        files.push("calendar_dates");
        files.push("feed_info");
        files.push("routes");
        files.push("shapes");
        files.push("stop_times");
        files.push("transfers");
        files.push("trips");
    } else if city == "pgh" {
        files.push("agency");
        files.push("calendar");
        files.push("calendar_dates");
        files.push("fare_attributes");
        files.push("fare_rules");
        files.push("feed_info");
        files.push("frequencies");
        files.push("routes");
        files.push("shapes");
        files.push("stop_times");
        files.push("stops");
        files.push("transfers");
        files.push("trips");
    } else {
        panic!("Not a valid city!");
    }

    files
}

// println!("{}", collection[0]);

// let header = &collection[0];
// let iter : Vec<&str> = header.split(',').collect();
// assert!(!iter.is_empty());
    
// let len = &iter.len();
// println!("{}", len);
        
// let reference = vec!["route_id", "service_id", "trip_id", 
//     "trip_headsign", "trip_short_name", "direction_id", "block_id",
//     "shape_id", "wheelchair_accessible", "bikes_allowed"];