// -------- BEGIN MODULE CODE -------- //

/* This module deals with getting usable data out of the GTFS Static files
which each agency uses to define their schedule statically.  */

pub mod get {
    use crate::gtfs_static::{{Agency, Calendar, CalendarDates, Routes, StopTimes, Stops, Trips},
    strconv};
    
    use std::{
        io::Error,
        {collections::HashMap, fs},
    };

    pub fn agency(file_path: String) -> Vec<Agency> {
        let mut agency: Vec<Agency> = Vec::new(); // Initializes the mutable data.

        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("agency.txt");
        let read: Result<String, Error> = fs::read_to_string(path);

        let read = match read {
            Ok(string) => string,
            Err(_) => panic!("Sorry, that wasn't a valid path."),
        };

        let mut data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
        data.remove(0);

        let header: Vec<String> = read.clone().split('\n').map(|s| s.to_string()).collect();
        let header: Vec<String> = header
            .iter()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let mut indices: HashMap<&str, Option<usize>> = HashMap::new();
        indices.insert(
            "id",
            header.iter().position(|x| *x == "agency_id".to_string()),
        );
        indices.insert(
            "name",
            header.iter().position(|x| *x == "agency_name".to_string()),
        );
        indices.insert(
            "url",
            header.iter().position(|x| *x == "agency_url".to_string()),
        );
        indices.insert(
            "timezone",
            header
                .iter()
                .position(|x| *x == "agency_timezone".to_string()),
        );
        indices.insert(
            "lang",
            header.iter().position(|x| *x == "agency_lang".to_string()),
        );
        indices.insert(
            "phone",
            header.iter().position(|x| *x == "agency_phone".to_string()),
        );
        indices.insert(
            "fare_url",
            header
                .iter()
                .position(|x| *x == "agency_fare_url".to_string()),
        );
        indices.insert(
            "email",
            header.iter().position(|x| *x == "agency_email".to_string()),
        );

        for line in data {
            let vec: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            let a: Option<usize> = indices.get("id").unwrap().clone();
            let b: usize = indices.get("name").unwrap().unwrap();
            let c: usize = indices.get("url").unwrap().unwrap();
            let d: usize = indices.get("timezone").unwrap().unwrap();
            let e: Option<usize> = indices.get("lang").unwrap().clone();
            let f: Option<usize> = indices.get("phone").unwrap().clone();
            let g: Option<usize> = indices.get("fare_url").unwrap().clone();
            let h: Option<usize> = indices.get("email").unwrap().clone();

            agency.push(Agency {
                id: match a {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                name: vec.iter().nth(b).unwrap().to_owned(),
                url: vec.iter().nth(c).unwrap().to_owned(),
                timezone: vec.iter().nth(d).unwrap().to_owned(),
                lang: match e {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                phone: match f {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                fare_url: match g {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                email: match h {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
            })
        }
        agency
    }

    pub fn calendardates(file_path: String) -> Vec<CalendarDates> {
        let mut calendar_dates: Vec<CalendarDates> = Vec::new(); // Initializes the mutable data.

        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("calendar_dates.txt");
        let read: Result<String, Error> = fs::read_to_string(path);

        let read = match read {
            Ok(string) => string,
            Err(_) => panic!("Sorry, that wasn't a valid path."),
        };

        let mut data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
        data.remove(0);

        let header: Vec<String> = read.clone().split('\n').map(|s| s.to_string()).collect();
        let header: Vec<String> = header
            .iter()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        println!("{:?}", header);

        let mut indices: HashMap<&str, Option<usize>> = HashMap::new();
        indices.insert(
            "service_id",
            header.iter().position(|x| *x == "service_id".to_string()),
        );
        indices.insert("date", header.iter().position(|x| *x == "date".to_string()));
        indices.insert(
            "exception_type",
            header.iter().position(|x| {
                *x == "exception_type".to_string() || *x == "exception_type\r".to_string()
            }),
        );

        for line in data {
            let vec: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            let service_id_index: usize = indices.get("service_id").unwrap().unwrap();
            let date_index: usize = indices.get("date").unwrap().unwrap();
            let exception_type_index: usize = indices.get("exception_type").unwrap().unwrap();

            calendar_dates.push(CalendarDates {
                service_id: vec.iter().nth(service_id_index).unwrap().to_owned(),
                date: vec.iter().nth(date_index).unwrap().to_owned(),
                exception_type: strconv::exceptiontype(
                    vec.iter().nth(exception_type_index).unwrap().to_owned(),
                ),
            })
        }
        calendar_dates
    }

    pub fn calendar(file_path: String) -> Vec<Calendar> {
        let mut calendar: Vec<Calendar> = Vec::new(); // Initializes the mutable data.

        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("calendar.txt");
        let read: Result<String, Error> = fs::read_to_string(path);

        let read = match read {
            Ok(string) => string,
            Err(_) => panic!("Sorry, that wasn't a valid path."),
        };

        let mut data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
        data.remove(0);

        let header: Vec<String> = read.clone().split('\n').map(|s| s.to_string()).collect();
        let header: Vec<String> = header
            .iter()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        println!("{:?}", header);

        let mut indices: HashMap<&str, Option<usize>> = HashMap::new();
        indices.insert(
            "service_id",
            header.iter().position(|x| *x == "service_id".to_string()),
        );
        indices.insert(
            "monday",
            header.iter().position(|x| *x == "monday".to_string()),
        );
        indices.insert(
            "tuesday",
            header.iter().position(|x| *x == "tuesday".to_string()),
        );
        indices.insert(
            "wednesday",
            header.iter().position(|x| *x == "wednesday".to_string()),
        );
        indices.insert(
            "thursday",
            header.iter().position(|x| *x == "thursday".to_string()),
        );
        indices.insert(
            "friday",
            header.iter().position(|x| *x == "friday".to_string()),
        );
        indices.insert(
            "saturday",
            header.iter().position(|x| *x == "saturday".to_string()),
        );
        indices.insert(
            "sunday",
            header.iter().position(|x| *x == "sunday".to_string()),
        );
        indices.insert(
            "start_date",
            header.iter().position(|x| *x == "start_date".to_string()),
        );
        indices.insert(
            "end_date",
            header
                .iter()
                .position(|x| *x == "end_date".to_string() || *x == "end_date\r".to_string()),
        );
        println!("indices: {:?}", indices);

        let service_id_index: usize = indices.get("service_id").unwrap().unwrap();
        let monday_index: usize = indices.get("monday").unwrap().unwrap();
        let tuesday_index: usize = indices.get("tuesday").unwrap().unwrap();
        let wednesday_index: usize = indices.get("wednesday").unwrap().unwrap();
        let thursday_index: usize = indices.get("thursday").unwrap().unwrap();
        let friday_index: usize = indices.get("friday").unwrap().unwrap();
        let saturday_index: usize = indices.get("saturday").unwrap().unwrap();
        let sunday_index: usize = indices.get("sunday").unwrap().unwrap();
        let start_date_index: usize = indices.get("start_date").unwrap().unwrap();
        let end_date_index: usize = indices.get("end_date").unwrap().unwrap();

        for line in data {
            let vec: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

            calendar.push(Calendar {
                service_id: vec.iter().nth(service_id_index).unwrap().to_owned(),
                monday: strconv::availableforall(vec.iter().nth(monday_index).unwrap().to_owned()),
                tuesday: strconv::availableforall(
                    vec.iter().nth(tuesday_index).unwrap().to_owned(),
                ),
                wednesday: strconv::availableforall(
                    vec.iter().nth(wednesday_index).unwrap().to_owned(),
                ),
                thursday: strconv::availableforall(
                    vec.iter().nth(thursday_index).unwrap().to_owned(),
                ),
                friday: strconv::availableforall(vec.iter().nth(friday_index).unwrap().to_owned()),
                saturday: strconv::availableforall(
                    vec.iter().nth(saturday_index).unwrap().to_owned(),
                ),
                sunday: strconv::availableforall(vec.iter().nth(sunday_index).unwrap().to_owned()),
                start_date: vec.iter().nth(start_date_index).unwrap().to_owned(),
                end_date: vec.iter().nth(end_date_index).unwrap().to_owned(),
            })
        }
        calendar
    }

    pub fn routes(file_path: String) -> Vec<Routes> {
        let mut routes: Vec<Routes> = Vec::new(); // Initializes the mutable data.

        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("routes.txt");
        let read: Result<String, Error> = fs::read_to_string(path);

        let read = match read {
            Ok(string) => string,
            Err(_) => panic!("Sorry, that wasn't a valid path."),
        };

        let mut data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
        data.remove(0);

        let header: Vec<String> = read.clone().split('\n').map(|s| s.to_string()).collect();
        let header: Vec<String> = header
            .iter()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let mut indices: HashMap<&str, Option<usize>> = HashMap::new();
        indices.insert(
            "route_id",
            header.iter().position(|x| *x == "route_id".to_string()),
        );
        indices.insert(
            "agency_id",
            header.iter().position(|x| *x == "agency_id".to_string()),
        );
        indices.insert(
            "short_name",
            header
                .iter()
                .position(|x| *x == "route_short_name".to_string()),
        );
        indices.insert(
            "long_name",
            header
                .iter()
                .position(|x| *x == "route_long_name".to_string()),
        );
        indices.insert(
            "route_desc",
            header.iter().position(|x| *x == "route_desc".to_string()),
        );
        indices.insert(
            "route_type",
            header.iter().position(|x| *x == "route_type".to_string()),
        );
        indices.insert(
            "route_url",
            header.iter().position(|x| *x == "route_url".to_string()),
        );
        indices.insert(
            "route_color",
            header.iter().position(|x| *x == "route_color".to_string()),
        );
        indices.insert(
            "route_text_color",
            header
                .iter()
                .position(|x| *x == "route_text_color".to_string()),
        );
        indices.insert(
            "route_sort_order",
            header
                .iter()
                .position(|x| *x == "route_sort_order".to_string()),
        );
        indices.insert(
            "continuous_pickup",
            header
                .iter()
                .position(|x| *x == "continuous_pickup".to_string()),
        );
        indices.insert(
            "continuous_drop_off",
            header
                .iter()
                .position(|x| *x == "continuous_drop_off".to_string()),
        );
        indices.insert(
            "network_id",
            header.iter().position(|x| *x == "network_id".to_string()),
        );

        let route_id_index: usize = indices.get("route_id").unwrap().unwrap();
        let agency_id_index: Option<usize> = indices.get("agency_id").unwrap().to_owned();
        let short_name_index: Option<usize> = indices.get("short_name").unwrap().to_owned();
        let long_name_index: Option<usize> = indices.get("long_name").unwrap().to_owned();
        let desc_index: Option<usize> = indices.get("route_desc").unwrap().to_owned();
        let type_index: usize = indices.get("route_type").unwrap().unwrap();
        let url_index: Option<usize> = indices.get("route_url").unwrap().to_owned();
        let color_index: Option<usize> = indices.get("route_color").unwrap().to_owned();
        let text_color_index: Option<usize> = indices.get("route_text_color").unwrap().to_owned();
        let sort_order_index: Option<usize> = indices.get("route_sort_order").unwrap().to_owned();
        let continuous_pickup_index: Option<usize> =
            indices.get("continuous_pickup").unwrap().to_owned();
        let continuous_dropoff_index: Option<usize> =
            indices.get("continuous_drop_off").unwrap().to_owned();
        let network_id_index: Option<usize> = indices.get("network_id").unwrap().to_owned();

        for line in data {
            let vec: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            routes.push(Routes {
                route_id: vec.iter().nth(route_id_index).unwrap().to_owned(),
                agency_id: match agency_id_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                short_name: match short_name_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                long_name: match long_name_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                desc: match desc_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                route_type: vec.iter().nth(type_index).unwrap().to_owned(),
                url: match url_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                color: match color_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                text_color: match text_color_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                sort_order: match sort_order_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                continuous_dropoff: match continuous_dropoff_index {
                    Some(index) => Some(strconv::continuousdropoff(
                        vec.iter().nth(index).unwrap().to_owned(),
                    )),
                    None => None,
                },
                continuous_pickup: match continuous_pickup_index {
                    Some(index) => Some(strconv::continuouspickup(
                        vec.iter().nth(index).unwrap().to_owned(),
                    )),
                    None => None,
                },
                network_id: match network_id_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
            })
        }
        routes
    }

    pub fn stops(file_path: String) -> Vec<Stops> {
        let mut stops: Vec<Stops> = Vec::new(); // Initializes the mutable data.

        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("stops.txt");
        let read: Result<String, Error> = fs::read_to_string(path);

        let read = match read {
            Ok(string) => string,
            Err(_) => panic!("Sorry, that wasn't a valid path."),
        };

        let mut data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
        data.remove(0);

        let header: Vec<String> = read.clone().split('\n').map(|s| s.to_string()).collect();
        let header: Vec<String> = header
            .iter()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let mut indices: HashMap<&str, Option<usize>> = HashMap::new();

        indices.insert(
            "id",
            header.iter().position(|x| *x == "stop_id".to_string()),
        );
        indices.insert(
            "code",
            header.iter().position(|x| *x == "stop_code".to_string()),
        );
        indices.insert(
            "name",
            header.iter().position(|x| *x == "stop_name".to_string()),
        );
        indices.insert(
            "tts_name",
            header
                .iter()
                .position(|x| *x == "stop_tts_name".to_string()),
        );
        indices.insert(
            "desc",
            header.iter().position(|x| *x == "stop_desc".to_string()),
        );
        indices.insert(
            "lat",
            header.iter().position(|x| *x == "stop_lat".to_string()),
        );
        indices.insert(
            "lon",
            header.iter().position(|x| *x == "stop_lon".to_string()),
        );

        println!("{:?}", indices);

        let id_index: usize = indices.get("id").unwrap().unwrap();
        let code_index: Option<usize> = indices.get("code").unwrap().to_owned();
        let name_index: Option<usize> = indices.get("name").unwrap().to_owned();
        let tts_name_index: Option<usize> = indices.get("tts_name").unwrap().to_owned();
        let desc_index: Option<usize> = indices.get("desc").unwrap().to_owned();
        let lat_index: Option<usize> = indices.get("lat").unwrap().to_owned();
        let lon_index: Option<usize> = indices.get("lon").unwrap().to_owned();

        for line in data {
            let vec: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            stops.push(Stops {
                id: vec.iter().nth(id_index).unwrap().to_owned(),
                code: match code_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                name: match name_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                tts_name: match tts_name_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                desc: match desc_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                lat: match lat_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                lon: match lon_index {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
            })
        }

        stops
    }

    pub fn stoptimes(file_path: String) -> Vec<StopTimes> {
        let mut stoptimes: Vec<StopTimes> = Vec::new(); // Initializes the mutable data.

        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("stop_times.txt");
        let read: Result<String, Error> = fs::read_to_string(path);

        let read = match read {
            Ok(string) => string,
            Err(_) => panic!("Sorry, that wasn't a valid path."),
        };

        let mut data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
        data.remove(0);

        let header: Vec<String> = read.clone().split('\n').map(|s| s.to_string()).collect();
        let header: Vec<String> = header
            .iter()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let mut indices: HashMap<&str, Option<usize>> = HashMap::new();

        indices.insert(
            "trip_id",
            header.iter().position(|x| *x == "trip_id".to_string()),
        );
        indices.insert(
            "arrival_time",
            header.iter().position(|x| *x == "arrival_time".to_string()),
        );
        indices.insert(
            "departure_time",
            header
                .iter()
                .position(|x| *x == "departure_time".to_string()),
        );
        indices.insert(
            "stop_id",
            header.iter().position(|x| *x == "stop_id".to_string()),
        );
        indices.insert(
            "location_group_id",
            header
                .iter()
                .position(|x| *x == "location_group_id".to_string()),
        );
        indices.insert(
            "location_id",
            header.iter().position(|x| *x == "location_id".to_string()),
        );
        indices.insert(
            "stop_sequence",
            header
                .iter()
                .position(|x| *x == "stop_sequence".to_string()),
        );
        indices.insert(
            "stop_headsign",
            header
                .iter()
                .position(|x| *x == "stop_headsign".to_string()),
        );
        indices.insert(
            "start_pickup_drop_off_window",
            header
                .iter()
                .position(|x| *x == "start_pickup_drop_off_window".to_string()),
        );
        indices.insert(
            "end_pickup_drop_off_window",
            header
                .iter()
                .position(|x| *x == "end_pickup_drop_off_window".to_string()),
        );
        indices.insert(
            "pickup_type",
            header.iter().position(|x| *x == "pickup_type".to_string()),
        );
        indices.insert(
            "dropoff_type",
            header.iter().position(|x| *x == "dropoff_type".to_string()),
        );
        indices.insert(
            "continuous_pickup",
            header
                .iter()
                .position(|x| *x == "continuous_pickup".to_string()),
        );
        indices.insert(
            "continuous_dropoff",
            header
                .iter()
                .position(|x| *x == "continuous_dropoff".to_string()),
        );
        indices.insert(
            "shape_dist_traveled",
            header
                .iter()
                .position(|x| *x == "shape_dist_traveled".to_string()),
        );
        indices.insert(
            "timepoint",
            header.iter().position(|x| *x == "timepoint".to_string()),
        );
        indices.insert(
            "pickup_booking_rule_id",
            header
                .iter()
                .position(|x| *x == "pickup_booking_rule_id".to_string()),
        );
        indices.insert(
            "drop_off_booking_rule_id",
            header
                .iter()
                .position(|x| *x == "drop_off_booking_rule_id".to_string()),
        );

        let a: usize = indices.get("trip_id").unwrap().unwrap();
        let b: Option<usize> = indices.get("arrival_time").unwrap().to_owned();
        let c: Option<usize> = indices.get("departure_time").unwrap().to_owned();
        let d: Option<usize> = indices.get("stop_id").unwrap().to_owned();
        let e: Option<usize> = indices.get("location_group_id").unwrap().to_owned();
        let f: Option<usize> = indices.get("location_id").unwrap().to_owned();
        let g: usize = indices.get("stop_sequence").unwrap().unwrap();
        let h: Option<usize> = indices.get("stop_headsign").unwrap().to_owned();
        let i: Option<usize> = indices
            .get("start_pickup_drop_off_window")
            .unwrap()
            .to_owned();
        let j: Option<usize> = indices
            .get("end_pickup_drop_off_window")
            .unwrap()
            .to_owned();
        let k: Option<usize> = indices.get("pickup_type").unwrap().to_owned();
        let l: Option<usize> = indices.get("dropoff_type").unwrap().to_owned();
        let m: Option<usize> = indices.get("continuous_pickup").unwrap().to_owned();
        let n: Option<usize> = indices.get("continuous_dropoff").unwrap().to_owned();
        let o: Option<usize> = indices.get("shape_dist_traveled").unwrap().to_owned();
        let p: Option<usize> = indices.get("timepoint").unwrap().to_owned();
        let q: Option<usize> = indices.get("pickup_booking_rule_id").unwrap().to_owned();
        let r: Option<usize> = indices.get("drop_off_booking_rule_id").unwrap().to_owned();

        for line in data {
            let vec: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            stoptimes.push(StopTimes {
                trip_id: vec.iter().nth(a).unwrap().to_owned(),
                arrival_time: match b {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                departure_time: match c {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                stop_id: match d {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                location_group_id: match e {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                location_id: match f {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                stop_sequence: vec.iter().nth(g).unwrap().to_owned(),
                stop_headsign: match h {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                start_pickup_drop_off_window: match i {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                end_pickup_drop_off_window: match j {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                pickup_type: match k {
                    Some(index) => Some(strconv::pickuptype(
                        vec.iter().nth(index).unwrap().to_owned(),
                    )),
                    None => None,
                },
                dropoff_type: match l {
                    Some(index) => Some(strconv::dropofftype(
                        vec.iter().nth(index).unwrap().to_owned(),
                    )),
                    None => None,
                },
                continuous_pickup: match m {
                    Some(index) => Some(strconv::continuouspickup(
                        vec.iter().nth(index).unwrap().to_owned(),
                    )),
                    None => None,
                },
                continuous_dropoff: match n {
                    Some(index) => Some(strconv::continuousdropoff(
                        vec.iter().nth(index).unwrap().to_owned(),
                    )),
                    None => None,
                },
                shape_dist_traveled: match o {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                timepoint: match p {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                pickup_booking_rule_id: match q {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                drop_off_booking_rule_id: match r {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
            })
        }

        stoptimes
    }

    pub fn trips(file_path: String) -> Vec<Trips> {
        let mut trips: Vec<Trips> = Vec::new(); // Initializes the mutable data.

        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("trips.txt");
        let read: Result<String, Error> = fs::read_to_string(path);

        let read = match read {
            Ok(string) => string,
            Err(_) => panic!("Sorry, that wasn't a valid path."),
        };

        let mut data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
        data.remove(0);

        let header: Vec<String> = read.clone().split('\n').map(|s| s.to_string()).collect();
        let header: Vec<String> = header
            .iter()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();

        let mut indices: HashMap<&str, Option<usize>> = HashMap::new();

        indices.insert(
            "route_id",
            header.iter().position(|x| *x == "route_id".to_string()),
        );
        indices.insert(
            "service_id",
            header.iter().position(|x| *x == "service_id".to_string()),
        );
        indices.insert(
            "trip_id",
            header.iter().position(|x| *x == "trip_id".to_string()),
        );
        indices.insert(
            "headsign",
            header
                .iter()
                .position(|x| *x == "trip_headsign".to_string()),
        );
        indices.insert(
            "short_name",
            header
                .iter()
                .position(|x| *x == "trip_short_name".to_string()),
        );
        indices.insert(
            "direction_id",
            header.iter().position(|x| *x == "direction_id".to_string()),
        );
        indices.insert(
            "block_id",
            header.iter().position(|x| *x == "block_id".to_string()),
        );
        indices.insert(
            "shape_id",
            header.iter().position(|x| *x == "shape_id".to_string()),
        );
        indices.insert(
            "wheelchair_accessible",
            header
                .iter()
                .position(|x| *x == "wheelchair_accessible".to_string()),
        );
        indices.insert(
            "bikes_allowed",
            header
                .iter()
                .position(|x| *x == "bikes_allowed".to_string()),
        );

        let a: usize = indices.get("route_id").unwrap().unwrap();
        let b: usize = indices.get("service_id").unwrap().unwrap();
        let c: usize = indices.get("trip_id").unwrap().unwrap();
        let d: Option<usize> = indices.get("headsign").unwrap().to_owned();
        let e: usize = indices.get("short_name").unwrap().unwrap();
        let f: Option<usize> = indices.get("direction_id").unwrap().to_owned();
        let g: Option<usize> = indices.get("block_id").unwrap().to_owned();
        let h: Option<usize> = indices.get("shape_id").unwrap().to_owned();
        let i: Option<usize> = indices.get("wheelchair_accessible").unwrap().to_owned();
        let j: Option<usize> = indices.get("bikes_allowed").unwrap().to_owned();

        for line in data {
            let vec: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
            trips.push(Trips {
                route_id: vec.iter().nth(a).unwrap().to_owned(),
                service_id: vec.iter().nth(b).unwrap().to_owned(),
                trip_id: vec.iter().nth(c).unwrap().to_owned(),
                headsign: match d {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                short_name: vec.iter().nth(e).unwrap().to_owned(),
                direction_id: match f {
                    Some(index) => Some(strconv::direction(
                        vec.iter().nth(index).unwrap().to_owned(),
                    )),
                    None => None,
                },
                block_id: match g {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                shape_id: match h {
                    Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                    None => None,
                },
                wheelchair_accessible: match i {
                    Some(index) => Some(strconv::wheelchair(
                        vec.iter().nth(index).unwrap().to_owned(),
                    )),
                    None => None,
                },
                bikes_allowed: match j {
                    Some(index) => Some(strconv::bikes(vec.iter().nth(index).unwrap().to_owned())),
                    None => None,
                },
            })
        }

        trips
    }
}

pub mod strconv {
    use crate::gtfs_static::{
        AvailableForall, Bikes, ContinuousDropoff, ContinuousPickup, Direction, DropoffType,
        ExceptionType, PickupType, Wheelchair,
    };

    pub fn availableforall(string: String) -> AvailableForall {
        if string == "1".to_string() {
            return AvailableForall::One;
        }
        AvailableForall::Zero
    }

    pub fn exceptiontype(string: String) -> ExceptionType {
        if string == "1".to_string() || string == "1\r".to_string() {
            return ExceptionType::One;
        } else if string == "2".to_string() || string == "2\r".to_string() {
            return ExceptionType::Two;
        }
        panic!("This data was unimplemented.");
    }

    pub fn pickuptype(string: String) -> PickupType {
        if string == "1" {
            return PickupType::One;
        } else if string == "2" {
            return PickupType::Two;
        } else if string == "3" {
            return PickupType::Three;
        }
        PickupType::Zero
    }

    pub fn dropofftype(string: String) -> DropoffType {
        if string == "1" {
            return DropoffType::Zero;
        } else if string == "2" {
            return DropoffType::Two;
        } else if string == "3" {
            return DropoffType::Three;
        }
        DropoffType::Zero
    }

    pub fn continuouspickup(string: String) -> ContinuousPickup {
        if string == "0" {
            return ContinuousPickup::Zero;
        } else if string == "2" {
            return ContinuousPickup::Two;
        } else if string == "3" {
            return ContinuousPickup::Three;
        }
        ContinuousPickup::One
    }

    pub fn continuousdropoff(string: String) -> ContinuousDropoff {
        if string == "0" {
            return ContinuousDropoff::Zero;
        } else if string == "2" {
            return ContinuousDropoff::Two;
        } else if string == "3" {
            return ContinuousDropoff::Three;
        }
        ContinuousDropoff::One
    }

    pub fn direction(string: String) -> Direction {
        if string == "1" {
            return Direction::One;
        }
        Direction::Zero
    }

    pub fn wheelchair(string: String) -> Wheelchair {
        if string == "1" {
            return Wheelchair::One;
        } else if string == "2" {
            return Wheelchair::Two;
        }
        Wheelchair::Zero
    }

    pub fn bikes(string: String) -> Bikes {
        if string == "1" {
            return Bikes::One;
        } else if string == "2" {
            return Bikes::Two;
        }
        Bikes::Zero
    }
}

// -------- END MODULE DATA -------- //

// -------- BEGIN MODULE STRUCTS -------- //

// -- GTFS Static structs which specify all data in the feeds. -- //

#[derive(Debug)]
pub struct Agency {
    pub id: Option<String>,
    pub name: String,
    pub url: String,
    pub timezone: String,
    pub lang: Option<String>,
    pub phone: Option<String>,
    pub fare_url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug)]
pub struct Calendar {
    pub service_id: String,
    pub monday: AvailableForall,
    pub tuesday: AvailableForall,
    pub wednesday: AvailableForall,
    pub thursday: AvailableForall,
    pub friday: AvailableForall,
    pub saturday: AvailableForall,
    pub sunday: AvailableForall,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug)]
pub struct CalendarDates {
    pub service_id: String,
    pub date: String,
    pub exception_type: ExceptionType,
}

#[derive(Debug)]
pub struct Routes {
    pub route_id: String,
    pub agency_id: Option<String>,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub desc: Option<String>,
    pub route_type: String,
    pub url: Option<String>,
    pub color: Option<String>,
    pub text_color: Option<String>,
    pub sort_order: Option<String>, // Change to u32 eventually.
    pub continuous_pickup: Option<ContinuousPickup>,
    pub continuous_dropoff: Option<ContinuousDropoff>,
    pub network_id: Option<String>,
}

#[derive(Debug)]
pub struct Shapes {
    pub id: String,
    pub pt_lat: String,
    pub pt_lon: String,
    pub pt_sequence: String,
    pub dist_traveled: Option<f32>,
}

#[derive(Debug)]
pub struct Stops {
    pub id: String,
    pub code: Option<String>,
    pub name: Option<String>,
    pub tts_name: Option<String>,
    pub desc: Option<String>,
    pub lat: Option<String>,
    pub lon: Option<String>,
}

#[derive(Debug)]
pub struct StopTimes {
    pub trip_id: String,
    pub arrival_time: Option<String>,
    pub departure_time: Option<String>,
    pub stop_id: Option<String>,
    pub location_group_id: Option<String>,
    pub location_id: Option<String>,
    pub stop_sequence: String,
    pub stop_headsign: Option<String>,
    pub start_pickup_drop_off_window: Option<String>,
    pub end_pickup_drop_off_window: Option<String>,
    pub pickup_type: Option<PickupType>,
    pub dropoff_type: Option<DropoffType>,
    pub continuous_pickup: Option<ContinuousPickup>,
    pub continuous_dropoff: Option<ContinuousDropoff>,
    pub shape_dist_traveled: Option<String>,
    pub timepoint: Option<String>,
    pub pickup_booking_rule_id: Option<String>,
    pub drop_off_booking_rule_id: Option<String>,
}

#[derive(Debug)]
pub struct Trips {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub headsign: Option<String>,
    pub short_name: String,
    pub direction_id: Option<Direction>,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
    pub wheelchair_accessible: Option<Wheelchair>,
    pub bikes_allowed: Option<Bikes>,
}

#[derive(Debug)]
pub enum AvailableForall {
    Zero, // Not available for all.
    One,  // Available for all.
}

#[derive(Debug)]
pub enum ExceptionType {
    One, // Service added for the specified date.
    Two, // Service removed for the specified date.
}

#[derive(Debug)]
pub enum Direction {
    Zero,
    One,
}

#[derive(Debug)]
pub enum Wheelchair {
    Zero, // No accessibility information for the trip.
    One,  // Vehicle cal accommodate at least one rider in a wheelchair.
    Two,  // No riders in wheelchairs can be accommodated on this trip.
}

#[derive(Debug)]
pub enum Bikes {
    Zero, // No vehicle information for this trip.
    One,  // Vehicles on this trip can accommodate at least one bicycle.
    Two,  // No bicycles are allowed on this trip.
}

#[derive(Debug)]
pub enum PickupType {
    Zero,  // Regularly scheduled pickup (empty parses as this.)
    One,   // No pickup available
    Two,   // Must phone agency to arrange pickup.
    Three, // Must coordinate with driver to arrange pickup.
}

#[derive(Debug)]
pub enum DropoffType {
    Zero,  // Regularly scheduled dropoff (empty parses as this.)
    One,   // No dropoff available
    Two,   // Must phone agency to arrange dropoff.
    Three, // Must coordinate with driver to arrange dropoff.
}

#[derive(Debug)]
pub enum ContinuousPickup {
    Zero,  // Continuous stopping pickup.
    One,   // No continuous stopping pickup (empty parses as this.)
    Two,   // Must phone agency to arrange  continuous stopping pickup.
    Three, // Must coordinate with driver to arrange continuous stopping pickup.
}

#[derive(Debug)]
pub enum ContinuousDropoff {
    Zero,  // Continuous stoppong dropoff.
    One,   // No continuous stopping dropoff (empty parses as this.)
    Two,   // Must phone agency to arrange  continuous stopping dropoff.
    Three, // Must coordinate with driver to arrange continuous stopping dropoff.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn agency_prints() {
        let path = "src/static/pittsburgh/prt/";
        let agency = get::agency(path.to_string());
        println!("{:?}", agency);
    }

    #[test]
    fn calendar_prints() {
        let path = "src/static/pittsburgh/prt/";
        let calendar = get::calendar(path.to_string());
        println!("{:?}", calendar);
    }

    #[test]
    fn calendardates_prints() {
        let path = "src/static/pittsburgh/prt/";
        let calendardates = get::calendardates(path.to_string());
        println!("{:?}", calendardates);
    }

    #[test]
    fn routes_prints() {
        let path = "src/static/pittsburgh/prt/";
        let routes = get::routes(path.to_string());
        println!("{:?}", routes);
    }

    #[test]
    fn stops_prints() {
        let path = "src/static/pittsburgh/prt/";
        let stops = get::stops(path.to_string());
        println!("{:?}", stops);
    }

    #[test]
    fn stoptimes_prints() {
        let path = "src/static/pittsburgh/prt/";
        let stoptimes = get::stoptimes(path.to_string());
        println!("{:?}", stoptimes);
    }

    #[test]
    fn trips_prints() {
        let path = "src/static/pittsburgh/prt/";
        let trips = get::trips(path.to_string());
        println!("{:?}", trips);
    }
}

// -------- END TESTING CODE -------- //
