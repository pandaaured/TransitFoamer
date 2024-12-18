use std::collections::HashMap;
use reqwest::Response;
use gtfs_realtime::FeedMessage;

// use std::process::Command;  // Use this if we use the python script.
// use std::fs;

// -------- BEGIN PROGRAM CODE -------- //

pub async fn requester(city: &str, function: &str) -> FeedMessage {
    let urls = urls(city);
    let url = urls[function];
    let response: Response = reqwest::get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    let data: Result<gtfs_realtime::FeedMessage, prost::DecodeError> = 
        prost::Message::decode(bytes.as_ref());
    let data = data.unwrap();
    
    data
}

fn urls(city: &str) -> HashMap<&str, &str> {
    let mut urls: HashMap<&str, &str> = HashMap::new();
    if city == "/pittsburgh/prt/" {
        urls.insert("vehicles-bus", "https://truetime.portauthority.org/gtfsrt-bus/vehicles");
        urls.insert("trips-bus", "https://truetime.portauthority.org/gtfsrt-bus/trips");
        urls.insert("vehicles-train", "https://truetime.portauthority.org/gtfsrt-train/vehicles");
        urls.insert("trips-train", "https://truetime.portauthority.org/gtfsrt-train/trips");
    } else if city == "/san_antonio/via/" {
        urls.insert("vehicles-bus", "http://gtfs.viainfo.net/vehicle/vehiclepositions.pb");
        urls.insert("trips-bus", "https://gtfs.viainfo.net/tripupdate/tripupdates.pb");
    } else {
        panic!("Wrong city");
    }
    
    urls
}

// Functions assume we are using the Python GTFS thingy.

// fn data_grab(city: &str, function: &str) -> String {
//     let path = data_to_import(city);
//     let contents : String = 
//     fs::read_to_string(path[function])
//        .expect("Should have been able to read the file");

//     contents
// }

// fn data_to_import(city: &str) -> HashMap<&str, &str> {
//     let mut files: HashMap<&str, &str> = HashMap::new();
//     if city == "satx" {
//         files.insert("vehicles-bus", "data/via/vehicles-bus.txt");
//         files.insert("trips-bus", "data/via/trips-bus.txt");
//     } else if city == "pgh" {
//         files.insert("vehicles-bus", "data/prt/vehicles-bus.txt");
//         files.insert("vehicles-train", "data/prt/vehicles-train.txt");
//         files.insert("trips-bus", "data/prt/trips-bus.txt");
//         files.insert("trips-train", "data/prt/trips-train.txt");
//     }
//     files
// }

// pub fn caller(city: &str) {
//     let file_name = file_name(city);
//     let python3_child = {Command::new("python3")
//         .arg(file_name)
//         .output()
//         .expect("failed to execute process")};
//  }

// fn file_name(city: &str) -> String {
//     let mut file = "fetchers/".to_string();
//     file.push_str(city);
//     file.push_str(".py");

//     file
// }


