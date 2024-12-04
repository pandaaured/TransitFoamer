use std::collections::HashMap;
// use std::collections::HashSet;
// use prost::Message;
use reqwest::Response;
use gtfs_realtime::FeedMessage;

// use std::process::Command;  // Use this if we use the python script.
// use std::fs;



// fn functions(city: &str) -> HashSet<&str> {
//     let mut ret = HashSet::new();

//     if city == "sa" {
//         ret.insert("vehicles-bus");
//         ret.insert("trips-bus");
//     } else if city == "pgh" {
//         ret.insert("vehicles-bus");
//         ret.insert("trips-bus");
//         ret.insert("vehicles-train");
//         ret.insert("trips-train");
//     }

//     ret
// }








pub async fn caller(city: &str, function: &str) -> FeedMessage {
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

    if city == "pgh" {
        urls.insert("vehicles-bus", "https://truetime.portauthority.org/gtfsrt-bus/vehicles");
        urls.insert("trips-bus", "https://truetime.portauthority.org/gtfsrt-bus/trips");
        urls.insert("vehicles-train", "https://truetime.portauthority.org/gtfsrt-train/vehicles");
        urls.insert("trips-train", "https://truetime.portauthority.org/gtfsrt-train/trips");
    } else if city == "sa" {
        urls.insert("vehicles-bus", "https://truetime.portauthority.org/gtfsrt-bus/vehicles");
        urls.insert("trips-bus", "https://truetime.portauthority.org/gtfsrt-bus/vehicles");
    }
    
    urls
}


// Functions assume we are using the Python GTFS thingy.

// fn data_to_import(city: &str) -> Vec<(&str, &str)> {
//     let mut files: Vec<(&str, &str)> = vec![];
//     if city == "sa" {
//         files.push(("vehicles-bus", "data/via/vehicles-bus.txt"));
//         files.push(("trips-bus", "data/via/trips-bus.txt"));
//     } else if city == "pgh" {
//         files.push(("vehicles-bus", "data/prt/vehicles-bus.txt"));
//         files.push(("vehicles-train", "data/prt/vehicles-train.txt"));
//         files.push(("trips-bus", "data/prt/trips-bus.txt"));
//         files.push(("trips-train", "data/prt/trips-train.txt"));
//     }
    
//     files
// }

// pub fn caller(city: &str) {
//     let file_name = file_name(city);
//     let python3_child = {Command::new("python3")
//         .arg(file_name)
//         .output()
//         .expect("failed to execute process")};

//     println!("{:#?}", python3_child);
// }

// fn file_name(city: &str) -> String {
//     let mut file = "fetchers/".to_string();
//     file.push_str(city);
//     file.push_str(".py");

//     file
// }


