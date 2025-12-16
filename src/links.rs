pub struct Links {
    pub vehicles: String,
    pub trips: String,
    pub alerts: String
}

impl Links {
    pub fn new() -> Links {
        Links {
            vehicles: "https://s3.amazonaws.com/kcm-alerts-realtime-prod/vehiclepositions.pb".to_string(),
            trips: "https://s3.amazonaws.com/kcm-alerts-realtime-prod/tripupdates.pb".to_string(),
            alerts: "https://s3.amazonaws.com/kcm-alerts-realtime-prod/alerts.pb".to_string()
        }
    }
}
