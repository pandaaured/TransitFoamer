use gtfs_realtime::FeedMessage; // other
use reqwest::Response;
use std::collections::HashMap; // std imports

// -------- BEGIN MODULE CODE -------- //

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
            "alerts-bus",
            "https://truetime.portauthority.org/gtfsrt-bus/alerts",
        );
        urls.insert(
            "vehicles-train",
            "https://truetime.portauthority.org/gtfsrt-train/vehicles",
        );
        urls.insert(
            "trips-train",
            "https://truetime.portauthority.org/gtfsrt-train/trips",
        );
        urls.insert(
            "alerts-train",
            "https://truetime.portauthority.org/gtfsrt-train/alerts",
        );
    } else if city == "/san_antonio/via/" {
        urls.insert(
            "vehicles",
            "http://gtfs.viainfo.net/vehicle/vehiclepositions.pb",
        );
        urls.insert(
            "trips",
            "https://gtfs.viainfo.net/tripupdate/tripupdates.pb",
        );
        urls.insert("alerts", "https://gtfs.viainfo.net/alerts/alerts.pb");
    } else if city == "/seattle/king_county/" {
        urls.insert(
            "trips",
            "https://s3.amazonaws.com/kcm-alerts-realtime-prod/tripupdates.pb",
        );
        urls.insert(
            "vehicles",
            "https://s3.amazonaws.com/kcm-alerts-realtime-prod/vehiclepositions.pb",
        );
        urls.insert(
            "alerts",
            "https://s3.amazonaws.com/kcm-alerts-realtime-prod/alerts.pb",
        );
    } else {
        panic!("Wrong city");
    }
    urls
}

// -------- END MODULE CODE -------- //
