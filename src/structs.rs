use std::sync::Arc;
use serde::Serialize;
use crate::gtfs_static::{Agency, Calendar, CalendarDates, Routes, StopTimes, Stops, Trips};

// A struct which contains the GTFS-RT URLs for a given feed.
#[derive(Debug)]
pub struct Links {
    pub vehicles: String,
    pub trips: String,
    pub alerts: String,
}

impl Links {
    // Initializes a struct with the hard-coded URLs. Currently set for PRT.
    pub fn new() -> Links {
        Links {
            vehicles: "https://truetime.portauthority.org/gtfsrt-bus/vehicles".to_string(),
            trips: "https://truetime.portauthority.org/gtfsrt-bus/trips".to_string(),
            alerts: "hhttps://truetime.portauthority.org/gtfsrt-bus/alerts".to_string(),
        }
    }
}

// A struct which contains the required GTFS Static feed items.
#[derive(Debug)]
pub struct StaticInfo {
    pub agency: Vec<Agency>,
    pub routes: Vec<Routes>,
    pub stops: Vec<Stops>,
    pub stop_times: Vec<StopTimes>,
    pub trips: Vec<Trips>,
    pub calendar: Vec<Calendar>,
    pub calendar_dates: Vec<CalendarDates>,
}

impl StaticInfo {
    // Initializes a struct with the data parsed from the PRT data in my repository.
    pub fn new() -> StaticInfo {
        let path: String = "gtfs_static/prt/".to_string();
        StaticInfo {
            agency: Agency::new_vec(&path).unwrap(),
            routes: Routes::new_vec(&path).unwrap(),
            stops: Stops::new_vec(&path).unwrap(),
            stop_times: StopTimes::new_vec(&path).unwrap(),
            trips: Trips::new_vec(&path).unwrap(),
            calendar: Calendar::new_vec(&path).unwrap(),
            calendar_dates: CalendarDates::new_vec(&path).unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub static_info: Arc<StaticInfo>,
    pub links: Arc<Links>,
}

/// One timetable represents a unique (service_id, stop_pattern) group.
/// Trips with identical stop sequences share the same timetable.
#[derive(Debug, Serialize)]
pub struct Timetable {
    pub service_id: String,
    pub service_info: ServiceInfo,
    pub stop_pattern: Vec<String>,
    pub stops: Vec<String>,
    pub trips: Vec<String>,
    pub cells: Vec<Vec<Option<String>>>,
}

/// A full schedule for one route, containing one timetable per unique trip pattern.
#[derive(Debug, Serialize)]
pub struct Schedule {
    pub route_id: String,
    pub timetables: Vec<Timetable>,
}

#[derive(Debug, Serialize)]
pub struct ServiceInfo {
    pub service_id: String,
    pub days: Vec<String>,        // e.g. ["Monday", "Tuesday", "Wednesday"]
    pub start_date: String,
    pub end_date: String,
    pub exceptions: Vec<ServiceException>,
}

#[derive(Debug, Serialize)]
pub struct ServiceException {
    pub date: String,
    pub added: bool, // true = added, false = removed
}
