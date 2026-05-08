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
            vehicles: "https://truetime.portauthority.org/gtfsrt-bus/vehicles" .to_string(),
            trips: "https://truetime.portauthority.org/gtfsrt-bus/trips".to_string(),
            alerts: "hhttps://truetime.portauthority.org/gtfsrt-bus/alerts".to_string()
        }
    }
}
