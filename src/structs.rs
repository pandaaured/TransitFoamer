use std::sync::Arc;

use crate::gtfs_static::{Agency, Routes, StopTimes, Stops, Trips};
#[derive(Debug)]
pub struct Links {
    pub vehicles: String,
    pub trips: String,
    pub alerts: String,
}

// A struct which contains the GTFS-RT URLs for a given feed.
impl Links {
    // Initializes a struct with the hard-coded URLs.
    pub fn new() -> Links {
        Links {
            vehicles: "https://truetime.portauthority.org/gtfsrt-bus/vehicles".to_string(),
            trips: "https://truetime.portauthority.org/gtfsrt-bus/trips".to_string(),
            alerts: "hhttps://truetime.portauthority.org/gtfsrt-bus/alerts".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct StaticInfo {
    pub agency: Vec<Agency>,
    pub routes: Vec<Routes>,
    pub stops: Vec<Stops>,
    pub stop_times: Vec<StopTimes>,
    pub trips: Vec<Trips>,
}

impl StaticInfo {
    pub fn new() -> StaticInfo {
        let path: String = "gtfs_static/prt/".to_string();
        StaticInfo {
            agency: Agency::new_vec(&path).unwrap(),
            routes: Routes::new_vec(&path).unwrap(),
            stops: Stops::new_vec(&path).unwrap(),
            stop_times: StopTimes::new_vec(&path).unwrap(),
            trips: Trips::new_vec(&path).unwrap(),
        }
    }
}

struct RouteQuery {
    route: String,
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub static_info: Arc<StaticInfo>,
    pub links: Arc<Links>,
}
