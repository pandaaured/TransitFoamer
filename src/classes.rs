#[derive(Debug)]
pub struct Agency {
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: String,
    pub agency_phone: String,
    pub agency_fare_url: String
}

#[derive(Debug)]
pub struct CalendarDates {
    pub date: String,
    pub exception_type: String
}

#[derive(Debug)]
pub struct Calendar {
    pub monday: String,
    pub tuesday: String,
    pub wednesday: String,
    pub thursday: String,
    pub friday: String,
    pub saturday: String,
    pub sunday: String,
    pub start_date: String,
    pub end_date: String
}

#[derive(Debug)]
pub struct Routes {
    pub agency_id: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_desc: String,
    pub route_type: String,
    pub route_url: String,
    pub route_color: String,
    pub route_text_color: String
}

#[derive(Debug)]
pub struct StopTimes {
    pub arrival_time: String,
    pub departure_time: String,
    pub stop_id: String,
    pub stop_sequence: String,
    pub stop_headsign: String,
    pub pickup_type: String,
    pub drop_off_type: String,
    pub shape_dist_traveled: String,
    pub timepoint: String
}

#[derive(Debug)]
pub struct Stops {
    pub stop_code: String,
    pub stop_name: String,
    pub stop_desc: String,
    pub stop_lat: String,
    pub stop_lon: String,
    pub zone_id: String,
    pub stop_url: String, 
    pub location_type: String, 
    pub parent_station: String,
    pub stop_timezone: String,
    pub wheelchair_boarding: String
}

#[derive(Debug)]
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