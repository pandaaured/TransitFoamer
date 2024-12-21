use gtfs_realtime::FeedMessage;
use reqwest::Response;
use std::collections::HashMap;

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

    data.unwrap()
}

fn urls(city: &str) -> HashMap<&str, &str> {
    let mut urls: HashMap<&str, &str> = HashMap::new();
    if city == "/pittsburgh/prt/" {
        urls.insert(
            "vehicles-bus",
            "https://truetime.portauthority.org/gtfsrt-bus/vehicles",
        );
        urls.insert(
            "trips-bus",
            "https://truetime.portauthority.org/gtfsrt-bus/trips",
        );
        urls.insert(
            "vehicles-train",
            "https://truetime.portauthority.org/gtfsrt-train/vehicles",
        );
        urls.insert(
            "trips-train",
            "https://truetime.portauthority.org/gtfsrt-train/trips",
        );
    } else if city == "/san_antonio/via/" {
        urls.insert(
            "vehicles-bus",
            "http://gtfs.viainfo.net/vehicle/vehiclepositions.pb",
        );
        urls.insert(
            "trips-bus",
            "https://gtfs.viainfo.net/tripupdate/tripupdates.pb",
        );
    } else {
        panic!("Wrong city");
    }

    urls
}
