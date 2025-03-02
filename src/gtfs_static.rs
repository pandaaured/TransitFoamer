use std::io::Error;
use std::{collections::HashMap, fs};

// -------- BEGIN MODULE CODE -------- //

/* This module deals with getting usable data out of the GTFS Static files
which each agency uses to define their schedule statically.  */

fn get_agency(file_path: String) -> Vec<Agency> {
    let mut agency: Vec<Agency> = Vec::new(); // Initializes the mutable data.

    let mut path: String = file_path.clone(); // Getting the file path and
    path.push_str("agency.txt");
    let read: Result<String, Error> = fs::read_to_string(path);

    let read = match read {
        Ok(string) => string,
        Err(_) => panic!("Sorry, that wasn't a valid path."),
    };

    let data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
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
        let id_index: Option<usize> = indices.get("id").unwrap().clone();
        let name_index: usize = indices.get("name").unwrap().unwrap();
        let url_index: usize = indices.get("url").unwrap().unwrap();
        let timezone_index: usize = indices.get("timezone").unwrap().unwrap();
        let lang_index: Option<usize> = indices.get("lang").unwrap().clone();
        let phone_index: Option<usize> = indices.get("phone").unwrap().clone();
        let fare_url_index: Option<usize> = indices.get("fare_url").unwrap().clone();
        let email_index: Option<usize> = indices.get("email").unwrap().clone();

        agency.push(Agency {
            id: match id_index {
                Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                None => None,
            },
            name: vec.iter().nth(name_index).unwrap().to_owned(),
            url: vec.iter().nth(url_index).unwrap().to_owned(),
            timezone: vec.iter().nth(timezone_index).unwrap().to_owned(),
            lang: match lang_index {
                Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                None => None,
            },
            phone: match phone_index {
                Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                None => None,
            },
            fare_url: match fare_url_index {
                Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                None => None,
            },
            email: match email_index {
                Some(index) => Some(vec.iter().nth(index).unwrap().to_owned()),
                None => None,
            },
        })
    }
    agency
}

fn get_calendardates(file_path: String) -> Vec<CalendarDates> {
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
    indices.insert(
        "date",
        header.iter().position(|x| *x == "date".to_string()),
    );
    indices.insert(
        "exception_type",
        header.iter().position(|x| *x == "exception_type".to_string() || *x == "exception_type\r".to_string()),
    );

    for line in data {
        let vec: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
        let service_id_index: usize = indices.get("service_id").unwrap().unwrap();
        let date_index: usize = indices.get("date").unwrap().unwrap();
        let exception_type_index: usize = indices.get("exception_type").unwrap().unwrap();

        calendar_dates.push(CalendarDates {
            service_id: vec.iter().nth(service_id_index).unwrap().to_owned(),
            date: vec.iter().nth(date_index).unwrap().to_owned(),
            exception_type: string_to_exceptiontype(
                vec.iter().nth(exception_type_index).unwrap().to_owned(),
            ),
        })
    }
    calendar_dates
}

fn get_calendar(file_path: String) -> Vec<Calendar> {
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

    println!("{:?}", data);
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
        header.iter().position(|x| *x == "end_date".to_string() || *x == "end_date\r".to_string()),
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
            monday: string_to_availableforall(vec.iter().nth(monday_index).unwrap().to_owned()),
            tuesday: string_to_availableforall(vec.iter().nth(tuesday_index).unwrap().to_owned()),
            wednesday: string_to_availableforall(
                vec.iter().nth(wednesday_index).unwrap().to_owned(),
            ),
            thursday: string_to_availableforall(vec.iter().nth(thursday_index).unwrap().to_owned()),
            friday: string_to_availableforall(vec.iter().nth(friday_index).unwrap().to_owned()),
            saturday: string_to_availableforall(vec.iter().nth(saturday_index).unwrap().to_owned()),
            sunday: string_to_availableforall(vec.iter().nth(sunday_index).unwrap().to_owned()),
            start_date: vec.iter().nth(start_date_index).unwrap().to_owned(),
            end_date: vec.iter().nth(end_date_index).unwrap().to_owned(),
        })
    }
    calendar
}

fn get_routes(file_path: String) -> Vec<Routes> {
    let mut routes: Vec<Routes> = Vec::new(); // Initializes the mutable data.

    let mut path: String = file_path.clone(); // Getting the file path and
    path.push_str("routes.txt");
    let read: Result<String, Error> = fs::read_to_string(path);

    let read = match read {
        Ok(string) => string,
        Err(_) => panic!("Sorry, that wasn't a valid path."),
    };

    let data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
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
                Some(index) => Some(string_to_dropofftype(
                    vec.iter().nth(index).unwrap().to_owned(),
                )),
                None => None,
            },
            continuous_pickup: match continuous_pickup_index {
                Some(index) => Some(string_to_pickuptype(
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

fn get_stops(file_path: String) -> Vec<Stops> {
    let mut stops: Vec<Stops> = Vec::new(); // Initializes the mutable data.

    let mut path: String = file_path.clone(); // Getting the file path and
    path.push_str("routes.txt");
    let read: Result<String, Error> = fs::read_to_string(path);

    let read = match read {
        Ok(string) => string,
        Err(_) => panic!("Sorry, that wasn't a valid path."),
    };

    let data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
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
        header
            .iter()
            .position(|x| *x == "stop_name".to_string()),
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
    
    let id_index: usize = indices.get("id").unwrap().unwrap();
    let code_index: Option<usize> = indices.get("code").unwrap().to_owned();
    let name_index: Option<usize> = indices.get("name").unwrap().to_owned();
    let tts_name_index: Option<usize> = indices.get("tts_name").unwrap().to_owned();
    let desc_index: Option<usize> = indices.get("desc").unwrap().to_owned();
    let lat_index: Option<usize> = indices.get("lat").unwrap().to_owned();
    let lon_index:Option<usize> = indices.get("lon").unwrap().to_owned();

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

fn get_stoptimes(file_path: String) -> Vec<StopTimes> {
    let mut stoptimes: Vec<StopTimes> = Vec::new(); // Initializes the mutable data.

    let mut path: String = file_path.clone(); // Getting the file path and
    path.push_str("routes.txt");
    let read: Result<String, Error> = fs::read_to_string(path);

    let read = match read {
        Ok(string) => string,
        Err(_) => panic!("Sorry, that wasn't a valid path."),
    };

    let data: Vec<String> = read.split('\n').map(|s| s.to_string()).collect();
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
        header.iter().position(|x| *x == "departure_time".to_string()),
    );
    indices.insert(
        "stop_id",
        header.iter().position(|x| *x == "stop_id".to_string()),
    );
    indices.insert(
        "location_group_id",
        header.iter().position(|x| *x == "location_group_id".to_string()),
    );
    indices.insert(
        "location_id",
        header.iter().position(|x| *x == "location_id".to_string()),
    );
    indices.insert(
        "stop_sequence",
        header.iter().position(|x| *x == "stop_sequence".to_string()),
    );
    indices.insert(
        "stop_headsign",
        header.iter().position(|x| *x == "stop_headsign".to_string()),
    );
    indices.insert(
        "start_pickup_drop_off_window",
        header.iter().position(|x| *x == "start_pickup_drop_off_window".to_string()),
    );
    indices.insert(
        "end_pickup_drop_off_window",
        header.iter().position(|x| *x == "end_pickup_drop_off_window".to_string()),
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
        header.iter().position(|x| *x == "continuous_pickup".to_string()),
    );
    indices.insert(
        "continuous_dropoff",
        header.iter().position(|x| *x == "continuous_dropoff".to_string()),
    );
    indices.insert(
        "shape_dist_traveled",
        header.iter().position(|x| *x == "shape_dist_traveled".to_string()),
    );
    indices.insert(
        "timepoint",
        header.iter().position(|x| *x == "timepoint".to_string()),
    );
    indices.insert(
        "pickup_booking_rule_id",
        header.iter().position(|x| *x == "pickup_booking_rule_id".to_string()),
    );
    indices.insert(
        "drop_off_booking_rule_id",
        header.iter().position(|x| *x == "drop_off_booking_rule_id".to_string()),
    );

    let trip_id_index: usize = indices.get("trip_id").unwrap().unwrap();
    let arrival_time_index: Option<usize> = indices.get("arrival_time").unwrap().to_owned();
    let departure_time_index: Option<usize> = indices.get("departure_time").unwrap().to_owned();
    let stop_id_index: Option<usize> = indices.get("stop_id").unwrap().to_owned();
    let location_group_id_index: Option<usize> = indices.get("location_group_id").unwrap().to_owned();
    let location_id_index: Option<usize> = indices.get("location_id").unwrap().to_owned();
    let stop_sequence_index :Option<usize> = indices.get("stop_sequence").unwrap().to_owned();

    stoptimes
}

fn string_to_availableforall(string: String) -> AvailableForall {
    if string == "1".to_string() {
        return AvailableForall::One;
    }
    AvailableForall::Zero
}

fn string_to_exceptiontype(string: String) -> ExceptionType {
    if string == "1".to_string() || string == "1\r".to_string() {
        return ExceptionType::One;
    } else if string == "2".to_string() || string == "2\r".to_string() {
        return ExceptionType::Two;
    }
    panic!("This data was unimplemented.");
}

fn string_to_pickuptype(string: String) -> PickupType {
    if string == "0" {
        return PickupType::Zero;
    } else if string == "2" {
        return PickupType::Two;
    } else if string == "3" {
        return PickupType::Three;
    }
    PickupType::One
}

fn string_to_dropofftype(string: String) -> DropoffType {
    if string == "0" {
        return DropoffType::Zero;
    } else if string == "2" {
        return DropoffType::Two;
    } else if string == "3" {
        return DropoffType::Three;
    }
    DropoffType::One
}

// -------- END MODULE DATA -------- //

// -------- BEGIN MODULE STRUCTS -------- //

// -- GTFS Static structs which specify all data in the feeds. -- //

enum File {
    Agency,
    Calendar,
    CalendarDates,
    Shapes,
    Stops,
    StopTimes,
    Routes,
    Trips,
}

enum Exception {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

#[derive(Debug)]
struct Agency {
    id: Option<String>,
    name: String,
    url: String,
    timezone: String,
    lang: Option<String>,
    phone: Option<String>,
    fare_url: Option<String>,
    email: Option<String>,
}
#[derive(Debug)]
struct Calendar {
    service_id: String,
    monday: AvailableForall,
    tuesday: AvailableForall,
    wednesday: AvailableForall,
    thursday: AvailableForall,
    friday: AvailableForall,
    saturday: AvailableForall,
    sunday: AvailableForall,
    start_date: String,
    end_date: String,
}

#[derive(Debug)]
struct CalendarDates {
    service_id: String,
    date: String,
    exception_type: ExceptionType,
}

struct Routes {
    route_id: String,
    agency_id: Option<String>,
    short_name: Option<String>,
    long_name: Option<String>,
    desc: Option<String>,
    route_type: String,
    url: Option<String>,
    color: Option<String>,
    text_color: Option<String>,
    sort_order: Option<String>, // Change to u32 eventually.
    continuous_pickup: Option<PickupType>,
    continuous_dropoff: Option<DropoffType>,
    network_id: Option<String>,
}

struct Shapes {
    id: String,
    pt_lat: String,
    pt_lon: String,
    pt_sequence: String,
    dist_traveled: Option<f32>,
}

struct Stops {
    id: String,
    code: Option<String>,
    name: Option<String>,
    tts_name: Option<String>,
    desc: Option<String>,
    lat: Option<String>,
    lon: Option<String>,
}

struct StopTimes {
    trip_id: String,
    arrival_time: Option<String>,
    departure_time: Option<String>,
    stop_id: Option<String>,
    location_group_id: Option<String>,
    location_id: Option<String>,
    stop_sequence: u32,
    stop_headsign: Option<String>,
    start_pickup_drop_off_window: Option<String>,
    end_pickup_drop_off_window: Option<String>,
    pickup_type: Option<PickupType>,
    dropoff_type: Option<DropoffType>,
    continuous_pickup: Option<PickupType>,
    continuous_dropoff: Option<DropoffType>,
    shape_dist_traveled: Option<String>,
    timepoint: Option<Timepoint>,
    pickup_booking_rule_id: Option<String>,
    drop_off_booking_rule_id: Option<String>,
}

struct Trips {
    route_id: String,
    service_id: String,
    trip_id: String,
    headsign: Option<String>,
    short_name: String,
    direction_id: Direction,
    block_id: Option<String>,
    shape_id: Option<String>,
    wheelchair_accessible: Wheelchair,
    bikes_allowed: Bikes,
}

#[derive(Debug)]
enum AvailableForall {
    Zero, // Not available for all.
    One,  // Available for all.
}

#[derive(Debug)]
enum ExceptionType {
    One, // Service added for the specified date.
    Two, // Service removed for the specified date.
}

enum Direction {
    Zero,
    One,
}

enum Wheelchair {
    Zero, // No accessibility information for the trip.
    One,  // Vehicle cal accommodate at least one rider in a wheelchair.
    Two,  // No riders in wheelchairs can be accommodated on this trip.
}

enum Bikes {
    Zero, // No vehicle information for this trip.
    One,  // Vehicles on this trip can accommodate at least one bicycle.
    Two,  // No bicycles are allowed on this trip.
}

enum PickupType {
    Zero,  // Regularly scheduled pickup (empty parses as this.)
    One,   // No pickup available
    Two,   // Must phone agency to arrange pickup.
    Three, // Must coordinate with driver to arrange pickup.
}

enum DropoffType {
    Zero,  // Regularly scheduled dropoff (empty parses as this.)
    One,   // No dropoff available
    Two,   // Must phone agency to arrange dropoff.
    Three, // Must coordinate with driver to arrange dropoff.
}

enum ContinuousPickup {
    Zero,  // Continuous stopping pickup.
    One,   // No continuous stopping pickup (empty parses as this.)
    Two,   // Must phone agency to arrange  continuous stopping pickup.
    Three, // Must coordinate with driver to arrange continuous stopping pickup.
}

enum ContinuousDropoff {
    Zero,  // Continuous stoppong dropoff.
    One,   // No continuous stopping dropoff (empty parses as this.)
    Two,   // Must phone agency to arrange  continuous stopping dropoff.
    Three, // Must coordinate with driver to arrange continuous stopping dropoff.
}

enum Timepoint {
    Zero, // Times are considered approximate.
    One,  // Times are considered exact.
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn agency_prints() {
        let path = "src/static/pittsburgh/prt/";
        let agency = get_agency(path.to_string());
        println!("{:?}", agency);
    }

    #[test]
    fn calendar_prints() {
        let path = "src/static/pittsburgh/prt/";
        let calendar = get_calendar(path.to_string());
        println!("{:?}", calendar);
    }

    #[test]
    fn calendardates_prints() {
        let path = "src/static/pittsburgh/prt/";
        let calendardates = get_calendardates(path.to_string());
        println!("{:?}", calendardates);
    }

}

// -------- END TESTING CODE -------- //
