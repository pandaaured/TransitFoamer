// -------- BEGIN MODULE CODE -------- //

//! GTFS_STATIC
//!
//! A library for fetching and analyzing GTFS Static data from a transit feed.

use std::collections::{HashMap, HashSet};
use std::io::{Error, Write};

use itertools::Itertools;

// Takes in trips_per_route data and stops_per_trip data and returns a HashMap
// containing stop_ids as keys and a vector of the route_ids for values.
pub fn routes_per_stop(
    trips_per_route: HashMap<String, Vec<String>>,
    stops_per_trip: HashMap<String, Vec<String>>,
) -> HashMap<String, HashSet<String>> {
    let mut routes_per_stop: HashMap<String, HashSet<String>> = HashMap::new();

    for route in trips_per_route {
        let trip_vec = route.clone().1;
        for trip in trip_vec {
            let stops = stops_per_trip.get(&trip).unwrap();
            for stop in stops {
                if !routes_per_stop.contains_key(stop) {
                    let mut set = HashSet::new();
                    set.insert(route.clone().0);
                    routes_per_stop.insert(stop.to_string(), set);
                } else {
                    let mut set = routes_per_stop.get(stop).unwrap().to_owned();
                    set.insert(route.clone().0);
                    routes_per_stop.insert(stop.to_string(), set);
                }
            }
        }
    }

    routes_per_stop
}

pub fn unique_trip_sequences(data: Vec<StopTimes>) -> Vec<Vec<(Option<String>, String)>> {
    let grouped = group_stoptimes_by_trip_id(data);
    let mapped = map_to_vec(grouped);
    mapped
}

pub fn group_stoptimes_by_trip_id(data: Vec<StopTimes>) -> Vec<Vec<StopTimes>> {
    let data_new: Vec<Vec<StopTimes>> = data
        .into_iter()
        .into_group_map_by(|s| s.trip_id.clone())
        .into_values()
        .collect();
    data_new
}

pub fn map_to_vec(data: Vec<Vec<StopTimes>>) -> Vec<Vec<(Option<String>, String)>> {
    let new: Vec<Vec<(Option<String>, String)>> = data
        .iter()
        .map(|f| {
            f.iter()
                .map(|g| (g.stop_id.clone(), g.stop_sequence.clone()))
                .collect()
        })
        .collect();

    let new_prime: Vec<Vec<(Option<String>, String)>> = new.into_iter().unique().collect();

    new_prime
}

pub fn count_trips_per_route(
    trips_per_route: HashMap<String, Vec<String>>,
) -> HashMap<String, usize> {
    let mut count_trips_per_route: HashMap<String, usize> = HashMap::new();
    for key in trips_per_route.keys() {
        count_trips_per_route.insert(key.clone(), trips_per_route.get(key).unwrap().len());
    }

    count_trips_per_route
}

pub fn count_trips_per_service_id(
    trips_per_service_id: HashMap<String, Vec<String>>,
) -> HashMap<String, usize> {
    let mut count_trips_per_service_id: HashMap<String, usize> = HashMap::new();
    for key in trips_per_service_id.keys() {
        count_trips_per_service_id
            .insert(key.clone(), trips_per_service_id.get(key).unwrap().len());
    }

    count_trips_per_service_id
}

#[derive(Debug, serde::Deserialize)]
pub struct Agency {
    pub agency_id: Option<String>,
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: Option<String>,
    pub agency_phone: Option<String>,
    pub agency_fare_url: Option<String>,
    pub agency_email: Option<String>,
}

impl Agency {
    /// Checks given root file path for the appropriate file.
    /// Returns a Result type containing either an error or a Vector with elements
    /// being Agency structs, each corresponding to a nonempty line of the
    /// data contained in the GTFS static definition.
    pub fn new_vec(file_path: &String) -> Result<Vec<Agency>, Error> {
        let mut agency: Vec<Agency> = Vec::new(); // Initializes the mutable data.
        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("agency.txt");
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        for result in rdr.deserialize() {
            let record: Agency = result?;
            agency.push(record);
        }

        Ok(agency)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Attributions {
    pub attribution_id: Option<String>,
    pub agency_id: Option<String>,
    pub route_id: Option<String>,
    pub trip_id: Option<String>,
    pub organization_name: String,
    pub is_producer: Option<String>,
    pub is_operator: Option<String>,
    pub is_authority: Option<String>,
    pub attribution_url: Option<String>,
    pub attribution_email: Option<String>,
    pub attribution_phone: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Calendar {
    pub service_id: String,
    pub monday: String,
    pub tuesday: String,
    pub wednesday: String,
    pub thursday: String,
    pub friday: String,
    pub saturday: String,
    pub sunday: String,
    pub start_date: String,
    pub end_date: String,
}

impl Calendar {
    /// Checks given root file path for the appropriate file.
    /// Returns a Result type containing either an error or a Vector with elements
    /// being Calendar structs, each corresponding to a nonempty line of the
    /// data contained in the GTFS static definition.
    pub fn new_vec(file_path: &String) -> Result<Vec<Calendar>, Error> {
        let mut calendar: Vec<Calendar> = Vec::new(); // Initializes the mutable data.
        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("calendar.txt");
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        for result in rdr.deserialize() {
            let record: Calendar = result?;
            calendar.push(record);
        }

        Ok(calendar)
    }

    fn service_ids(calendar_data: Vec<Calendar>) -> HashSet<String> {
        let mut service_ids: HashSet<String> = HashSet::new();

        for item in calendar_data {
            service_ids.insert(item.service_id);
        }

        service_ids
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CalendarDates {
    pub service_id: String,
    pub date: String,
    pub exception_type: String,
}

impl CalendarDates {
    /// Checks given root file path for the appropriate file.
    /// Returns a Result type containing either an error or a Vector with elements
    /// being CalendarDates structs, each corresponding to a nonempty line of the
    /// data contained in the GTFS static definition.
    fn new_vec(file_path: &String) -> Result<Vec<CalendarDates>, Error> {
        let mut calendardates: Vec<CalendarDates> = Vec::new(); // Initializes the mutable data.
        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("calendar_dates.txt");
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        for result in rdr.deserialize() {
            let record: CalendarDates = result?;
            calendardates.push(record);
        }

        Ok(calendardates)
    }
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Routes {
    pub route_id: String,
    pub agency_id: Option<String>,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub route_desc: Option<String>,
    pub route_type: String,
    pub route_url: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
    pub route_sort_order: Option<String>, // Change to u32 eventually.
    pub continuous_pickup: Option<String>,
    pub continuous_dropoff: Option<String>,
    pub network_id: Option<String>,
}

impl Routes {
    /// Checks given root file path for the appropriate file.
    /// Returns a Result type containing either an error or a Vector with elements
    /// being Routes structs, each corresponding to a nonempty line of the
    /// data contained in the GTFS static definition.
    pub fn new_vec(file_path: &String) -> Result<Vec<Routes>, Error> {
        let mut routes: Vec<Routes> = Vec::new(); // Initializes the mutable data.
        let mut path: String = file_path.to_string().clone(); // Getting the file path and
        println!("{:?}", path);
        path.push_str("routes.txt");
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        for result in rdr.deserialize() {
            let record: Routes = result?;
            routes.push(record);
        }

        Ok(routes)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Shapes {
    pub id: String,
    pub pt_lat: String,
    pub pt_lon: String,
    pub pt_sequence: String,
    pub dist_traveled: Option<f32>,
}

impl Shapes {
    /// Checks given root file path for the appropriate file.
    /// Returns a Result type containing either an error or a Vector with elements
    /// being Shapes structs, each corresponding to a nonempty line of the
    /// data contained in the GTFS static definition.
    pub fn new_vec(file_path: &String) -> Result<Vec<Shapes>, Error> {
        let mut shapes: Vec<Shapes> = Vec::new(); // Initializes the mutable data.
        let mut path: String = file_path.to_string().clone(); // Getting the file path and
        path.push_str("shapes.txt");
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        for result in rdr.deserialize() {
            let record: Shapes = result?;
            shapes.push(record);
        }

        Ok(shapes)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Stops {
    pub stop_id: String,
    pub stop_code: Option<String>,
    pub stop_name: Option<String>,
    pub tts_stop_name: Option<String>,
    pub stop_desc: Option<String>,
    pub stop_lat: Option<String>,
    pub stop_lon: Option<String>,
}

impl Stops {
    /// Checks given root file path for the appropriate file.
    /// Returns a Result type containing either an error or a Vector with elements
    /// being Stops structs, each corresponding to a nonempty line of the
    /// data contained in the GTFS static definition.
    pub fn new_vec(file_path: &String) -> Result<Vec<Stops>, Error> {
        let mut stops: Vec<Stops> = Vec::new(); // Initializes the mutable data.
        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("stops.txt");
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        for result in rdr.deserialize() {
            let record: Stops = result?;
            stops.push(record);
        }

        Ok(stops)
    }

    pub fn new_hash(stops: Vec<Stops>) -> HashMap<String, Stops> {
        let mut map: HashMap<String, Stops> = HashMap::new();
        for entry in stops {
            let key: &String = &entry.stop_id;
            map.insert(key.to_string(), entry);
        }

        map
    }
}

#[derive(Debug, serde::Deserialize, Clone)]
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
    pub pickup_type: Option<String>,
    pub dropoff_type: Option<String>,
    pub continuous_pickup: Option<String>,
    pub continuous_dropoff: Option<String>,
    pub shape_dist_traveled: Option<String>,
    pub timepoint: Option<String>,
    pub pickup_booking_rule_id: Option<String>,
    pub drop_off_booking_rule_id: Option<String>,
}

impl StopTimes {
    /// Checks given root file path for the appropriate file.
    /// Returns a Result type containing either an error or a Vector with elements
    /// being StopTimes structs, each corresponding to a nonempty line of the
    /// data contained in the GTFS static definition.
    pub fn new_vec(file_path: &String) -> Result<Vec<StopTimes>, Error> {
        let mut stoptimes: Vec<StopTimes> = Vec::new(); // Initializes the mutable data.
        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("stop_times.txt");
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        for result in rdr.deserialize() {
            let record: StopTimes = result?;
            stoptimes.push(record);
        }

        Ok(stoptimes)
    }

    pub fn _new_hash(stoptimes: Vec<StopTimes>) -> HashMap<(String, String), StopTimes> {
        let mut map: HashMap<(String, String), StopTimes> = HashMap::new();
        for entry in stoptimes {
            let key_one: &String = &entry.trip_id;
            let key_two: &String = &entry.stop_sequence;
            map.insert((key_one.to_string(), key_two.to_string()), entry);
        }

        map
    }

    // Takes a vector of StopTimes data and returns a HashMap containing trip_ids
    // as keys and a vector of the stops for each trip_id for values.
    pub fn stops_per_trip(stop_times_data: Vec<StopTimes>) -> HashMap<String, Vec<String>> {
        let mut stops_per_trip: HashMap<String, Vec<String>> = HashMap::new();

        for item in stop_times_data {
            let trip_id = item.trip_id;
            if stops_per_trip.contains_key(&trip_id.clone()) {
                let stop_id = item.stop_id.unwrap();
                let mut vector = stops_per_trip[&trip_id.clone()].clone();
                vector.push(stop_id.clone());
                stops_per_trip.insert(trip_id.clone(), vector.clone());
            } else {
                let stop_id = item.stop_id.unwrap();
                stops_per_trip.insert(trip_id.clone(), vec![stop_id.clone()]);
            }
        }

        stops_per_trip
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Timeframes {
    pub timeframe_group_id: String,
    pub start_time: String,
    pub end_time: String,
    pub service_id: String,
}

impl Timeframes {
    /// Checks given root file path for the appropriate file.
    /// Returns a Result type containing either an error or a Vector with elements
    /// being Timeframes structs, each corresponding to a nonempty line of the
    /// data contained in the GTFS static definition.
    fn _new_vec(file_path: &String) -> Result<Vec<Timeframes>, Error> {
        let mut timeframes: Vec<Timeframes> = Vec::new(); // Initializes the mutable data.
        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("timeframes.txt");
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        for result in rdr.deserialize() {
            let record: Timeframes = result?;
            timeframes.push(record);
        }

        Ok(timeframes)
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Trips {
    pub route_id: String,
    pub service_id: String,
    pub trip_id: String,
    pub trip_headsign: Option<String>,
    pub trip_short_name: String,
    pub direction_id: Option<String>,
    pub block_id: Option<String>,
    pub shape_id: Option<String>,
    pub wheelchair_accessible: Option<String>,
    pub bikes_allowed: Option<String>,
}

impl Trips {
    /// Checks given root file path for the appropriate file.
    /// Returns a Result type containing either an error or a Vector with elements
    /// being Trips structs, each corresponding to a nonempty line of the
    /// data contained in the GTFS static definition.
    pub fn new_vec(file_path: &String) -> Result<Vec<Trips>, Error> {
        let mut trips: Vec<Trips> = Vec::new(); // Initializes the mutable data.
        let mut path: String = file_path.clone(); // Getting the file path and
        path.push_str("trips.txt");
        let mut rdr = csv::ReaderBuilder::new().from_path(path)?;
        for result in rdr.deserialize() {
            let record: Trips = result?;
            trips.push(record);
        }

        Ok(trips)
    }

    pub fn new_hash(trip_data: Vec<Trips>) -> HashMap<String, Trips> {
        let mut map: HashMap<String, Trips> = HashMap::new();
        for entry in trip_data {
            let key: &String = &entry.trip_id;
            map.insert(key.to_string(), entry);
        }

        map
    }

    // Takes a vector of Trips data and returns a HashMap containing route_ids
    // as keys and a vector of the corresponding trip_ids as values.
    pub fn trips_per_route(trip_data: Vec<Trips>) -> HashMap<String, Vec<String>> {
        let mut trips_per_route: HashMap<String, Vec<String>> = HashMap::new();
        for item in trip_data {
            let route = item.route_id;
            let trip = item.trip_id;
            if trips_per_route.contains_key(&route) {
                let mut vector = trips_per_route.remove(&route).unwrap();
                vector.push(trip.clone());
                trips_per_route.insert(route.clone(), vector.clone());
            } else {
                trips_per_route.insert(route, vec![trip]);
            }
        }

        trips_per_route
    }

    // Takes a vector of Trips data and returns a HashMap containing service_ids
    // as keys and a vector of the corresponding trip_ids as values.
    pub fn trips_per_service_id(trip_data: Vec<Trips>) -> HashMap<String, Vec<String>> {
        let mut trips_per_service_id: HashMap<String, Vec<String>> = HashMap::new();
        for item in trip_data {
            let service_id = item.service_id;
            let trip = item.trip_id;
            if trips_per_service_id.contains_key(&service_id) {
                let mut vector = trips_per_service_id.remove(&service_id).unwrap();
                vector.push(trip.clone());
                trips_per_service_id.insert(service_id.clone(), vector.clone());
            } else {
                trips_per_service_id.insert(service_id, vec![trip]);
            }
        }

        trips_per_service_id
    }
}

#[cfg(test)]
mod test {
    use std::fs::File;

    use super::*;

    #[test]
    fn agency_prints() {
        let path = "src/static/pittsburgh/prt/";
        let agency = Agency::new_vec(&path.to_string()).unwrap();
        println!("{:?}", agency);
    }

    #[test]
    fn calendar_prints() {
        let path = "src/static/pittsburgh/prt/";
        let calendar = Calendar::new_vec(&path.to_string()).unwrap();
        println!("{:?}", calendar);
    }

    #[test]
    fn calendardates_prints() {
        let path = "src/static/pittsburgh/prt/";
        let calendardates = CalendarDates::new_vec(&path.to_string()).unwrap();
        println!("{:?}", calendardates);
    }

    #[test]
    fn routes_prints() {
        let path = "src/static/pittsburgh/prt/";
        let routes = Routes::new_vec(&path.to_string()).unwrap();
        println!("{:?}", routes);
    }

    #[test]
    fn stops_prints() {
        let path = "src/static/pittsburgh/prt/";
        let stops = Stops::new_vec(&path.to_string()).unwrap();
        println!("{:?}", stops);
    }

    #[test]
    fn stoptimes_prints() {
        let path = "GTFS/";
        let stoptimes = StopTimes::new_vec(&path.to_string()).unwrap();
        println!("{:?}", stoptimes);
    }

    #[test]
    fn trips_prints() {
        let path = "GTFS/";
        let trips = Trips::new_vec(&path.to_string()).unwrap();
        println!("{:?}", trips);
    }

    #[test]
    fn service_ids_test() {
        let path = "GTFS/";
        let sid = Calendar::service_ids(Calendar::new_vec(&path.to_string()).unwrap());
        println!("{:?}", sid);
    }

    #[test]
    fn trips_per_service_id_test() {
        let path = "GTFS/";
        let tpsid = Trips::trips_per_service_id(Trips::new_vec(&path.to_string()).unwrap());
        println!("{:?}", tpsid);
    }

    #[test]
    fn routes_per_stop_test() {
        let path = "GTFS/";
        let tpr = Trips::trips_per_route(Trips::new_vec(&path.to_string()).unwrap());
        let spt = StopTimes::stops_per_trip(StopTimes::new_vec(&path.to_string()).unwrap());
        let rps = routes_per_stop(tpr, spt);
        println!("{:?}", rps);
    }

    #[test]
    fn count_trips_per_route_test() {
        let path = "GTFS/";
        let tpr = Trips::trips_per_route(Trips::new_vec(&path.to_string()).unwrap());
        let count = count_trips_per_route(tpr);
        println!("{:?}", count);
    }

    #[test]
    fn count_trips_per_service_id_test() {
        let path = "GTFS/";
        let tpr = Trips::trips_per_service_id(Trips::new_vec(&path.to_string()).unwrap());
        let count = count_trips_per_service_id(tpr);
        println!("{:?}", count);
    }

    #[test]
    fn proper_count_test() {
        let path = "GTFS/";
        let stop_times = StopTimes::new_vec(&path.to_string()).unwrap();
        let info = unique_trip_sequences(stop_times);
        let mut file = File::create("proper_count.txt").unwrap(); // unsafe.
        for group in &info {
            writeln!(file, "{:#?}", group).unwrap();
        }
    }

    /*
     * Given a StopTimes vector, takes the initial entry's trip_id and then produces
     * written output for the sequence of stops associated with that trip_id. The
     * output is sorted by stop_sequence.
     *
     * TODO: Make this output CSV with more metadata to facilitate better debugging.
     */
    #[test]
    fn check_one_trip_id() {
        let path = "gtfs_static/prt/";
        let stop_times = StopTimes::new_vec(&path.to_string()).unwrap();
        let trip_id_use = stop_times.get(0).unwrap().trip_id.clone();
        let stop_times_filtered: Vec<StopTimes> = stop_times
            .into_iter()
            .filter(|x| x.trip_id == trip_id_use)
            .collect();
        let mut reduced: Vec<(Option<String>, String)> = stop_times_filtered
            .into_iter()
            .map(|x| (x.stop_id, x.stop_sequence))
            .collect();
        reduced.sort_by(|x, y| {
            x.1.parse::<u32>()
                .unwrap()
                .cmp(y.1.parse::<u32>().as_ref().unwrap())
        });
        let mut file = File::create("test/check_one_trip.txt").unwrap(); // unsafe.
        writeln!(file, "trip_id = {}\n", trip_id_use).unwrap();
        for group in reduced {
            writeln!(file, "{:#?}", group).unwrap();
        }
    }

    /*
     * Given a StopTimes vector, takes the initial entry's trip_id and then produces a list of
     * different sequences of stops associated with that trip_id's route_id. The output is sorted
     * by stop_sequence. 
     *
     * TODO: Make this output CSV with more metadata to facilitate better debugging.
     */
    #[test]
    fn check_one_route_id() {
        let path = "gtfs_static/prt/";
        let stop_times = StopTimes::new_vec(&path.to_string()).unwrap();
        let trips = Trips::new_vec(&path.to_string()).unwrap();
        let trip_id_use = stop_times.get(0).unwrap().trip_id.clone();
        let route_id = trips
            .iter()
            .find(|x| x.trip_id == trip_id_use)
            .unwrap()
            .route_id
            .clone();
        let trip_id_hash = Trips::trips_per_route(trips);
        let trip_ids = trip_id_hash.get(&route_id).unwrap();
        let stop_times_filtered: Vec<StopTimes> = stop_times // Only the data on a
            // certain route is
            // included now.
            .into_iter()
            .filter(|x| trip_ids.contains(&x.trip_id))
            .collect();
        let grouped = group_stoptimes_by_trip_id(stop_times_filtered);
        let reduced: Vec<Vec<(Option<String>, String)>> = grouped
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(|y| (y.stop_id, y.stop_sequence))
                    .collect()
            })
            .collect();
        let unique: Vec<Vec<(Option<String>, String)>> = reduced.into_iter().unique().collect();

        let unique: Vec<Vec<(Option<String>, String)>> = unique
            .into_iter()
            .map(|mut x| {
                x.sort_by(|a, b| {
                    let a = a.1.parse::<u32>().unwrap();
                    let b = b.1.parse::<u32>().unwrap();
                    a.cmp(&b)
                });
                x
            })
            .collect();

        let mut file = File::create("test/check_one_route.txt").unwrap(); // unsafe.
        writeln!(file, "route_id = {}", route_id).unwrap();
        writeln!(file, "number of unique sequences = {}", unique.len()).unwrap();
        for group in &unique {
            writeln!(file, "{:#?}", group).unwrap();
        }
    }
}
