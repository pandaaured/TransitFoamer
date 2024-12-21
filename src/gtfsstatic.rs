// This module handles the use of data external to the GTFS static definition.
// It is primarily used currently to import the output of my script which
// returns bus stops and the routes which serve them. Data unfortunately in 
// JSON currently. 
// pub mod extdata_gtfs {    
//     use std::collections::HashMap;
//     use std::fs::File;
//     use std::io::Read;

//     #[derive(Debug, serde::Deserialize)]
//     struct Data {
//         stops: HashMap<String, Vec<String>>,
//     }

//     pub fn routes_per_stop(city: &str) -> HashMap<String, Vec<String>> {
//         let file_path = file_path(city);
//         let mut file = File::open(file_path).unwrap();
//         let mut data = String::new();
//         file.read_to_string(&mut data).unwrap();
//         let json: Data = serde_json::from_str(&mut data).expect("Should work");
//         let hash_data: HashMap<String, Vec<String>> = json.stops;

//         hash_data   
//     }

//     fn file_path(city: &str) -> String {
//         let mut string: String = "data/".to_string();
    
//         if city == "satx" { 
//           string.push_str("/via/");
//         } else if city == "pgh" {
//           string.push_str("/prt/");
//         } else if city == "chicago" {
//           string.push_str("/cta/");
//         } else {
//             panic!("Not a valid city!");
//         }

//         string.push_str("routesperstop.json");
    
//         string
//     }
// }

pub mod data {
    use std::collections::HashMap;
    use std::fs;
    use std::slice::Iter;
    use crate::gtfsstatic::{paths, enum_file::{File, Size, is_one, get_first, get_second}};


    pub fn static_data_all(path_data: HashMap<String, String>) -> HashMap<String, HashMap<String, Vec<String>>> {
        let mut static_data: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
        for path in path_data {
            let key = path_io(path.0.clone(), path.1);
            static_data.insert(path.0.clone(), key);
        }
        static_data
    }
    /// Given a valid city and a file name that exists in that city's static definition,
    /// this function returns a HashMap whose keys are the file's associated keys and 
    /// whose values are lines of the file that you can access. 
    pub fn static_data(city: &str, function: String) -> HashMap<String, Vec<String>> {
        let file_path = paths::file_path(city, function.clone());
        let ingest_data = read_to_string(file_path);
        let data = hashmap_populate(ingest_data, function);
        data
    }

    pub fn path_io(function: String, path: String) -> HashMap<String, Vec<String>> {
        let mut map : HashMap<String, Vec<String>> = HashMap::new();
        let data: String = read_to_string(path);
        let mut iterator = data.split('\n');
        let header: Vec<&str> = iterator.next().unwrap().split(',').collect();
        let header_iter = header.iter();
        let index = match function_to_enum(function.clone()) {
            File::Agency => Size::One(find_index("agency_id", &header_iter).unwrap()),
            File::Calendar => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::CalendarDates => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::FareAttributes => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::FareRules => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::FeedInfo => Size::One(find_index("feed_publisher_name", &header_iter).unwrap()),
            File::Frequencies => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::Routes => Size::One(find_index("route_id", &header_iter).unwrap()),
            File::Shapes => Size::One(find_index("shape_id", &header_iter).unwrap()),
            File::StopTimes => Size::Two(find_index("trip_id", &header_iter).unwrap(), 
                                         find_index("stop_sequence", &header_iter).unwrap()),
            File::Stops => Size::One(find_index("stop_id", &header_iter).unwrap()),
            File::Transfers => Size::One(find_index("from_stop_id", &header_iter).unwrap()),
            File::Trips => Size::One(find_index("trip_id", &header_iter).unwrap()),
        };

        if is_one(&index) {
            let key_to_insert = match index {
                Size::One(x) => x,
                Size::Two(x, _) => x,
            };
            for i in iterator {
                let v: Vec<&str> = i.split(',').collect();
                let w : Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
                let key = &w[key_to_insert];
                map.insert(key.clone(), w);
            }
        } else {
            let key_elem_one = get_first(&index);
            let key_elem_two = get_second(&index);
            for i in iterator {
                let v: Vec<&str> = i.split(',').collect();
                let w : Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
                let key_pt_1 = &w[key_elem_one];
                let key_pt_2 = &w[key_elem_two];
                let mut str = String::from("(");
                str.push_str(key_pt_1.to_string().as_str());
                str.push(',');
                str.push_str(key_pt_2.to_string().as_str());
                str.push(')');
                map.insert(str.clone(), w);
            }
        }
        
        map
    }
    
    fn read_to_string(path: String) -> String {
        // println!("{}", path);
        let contents : String = 
           fs::read_to_string(path)
              .expect("Should have been able to read the file");
        contents 
    }

    fn hashmap_populate(data: String, function: String) -> HashMap<String, 
    Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut iterator = data.split('\n');
        let header: Vec<&str> = iterator.next().unwrap().split(',').collect();
        let header_iter = header.iter();
        let index = match function_to_enum(function.clone()) {
            File::Agency => Size::One(find_index("agency_id", &header_iter).unwrap()),
            File::Calendar => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::CalendarDates => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::FareAttributes => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::FareRules => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::FeedInfo => Size::One(find_index("feed_publisher_name", &header_iter).unwrap()),
            File::Frequencies => Size::One(find_index("service_id", &header_iter).unwrap()),
            File::Routes => Size::One(find_index("route_id", &header_iter).unwrap()),
            File::Shapes => Size::One(find_index("shape_id", &header_iter).unwrap()),
            File::StopTimes => Size::Two(find_index("trip_id", &header_iter).unwrap(), 
                                         find_index("stop_sequence", &header_iter).unwrap()),
            File::Stops => Size::One(find_index("stop_id", &header_iter).unwrap()),
            File::Transfers => Size::One(find_index("from_stop_id", &header_iter).unwrap()),
            File::Trips => Size::One(find_index("trip_id", &header_iter).unwrap()),
        };

        if is_one(&index) {
            let key_to_insert = match index {
                Size::One(x) => x,
                Size::Two(x, _) => x,
            };
            for i in iterator {
                let v: Vec<&str> = i.split(',').collect();
                let w : Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
                let key = &w[key_to_insert];
                map.insert(key.clone(), w);
            }
        } else {
            let key_elem_one = get_first(&index);
            let key_elem_two = get_second(&index);
            for i in iterator {
                let v: Vec<&str> = i.split(',').collect();
                let w : Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
                let key_pt_1 = &w[key_elem_one];
                let key_pt_2 = &w[key_elem_two];
                let mut str = String::from("(");
                str.push_str(key_pt_1.to_string().as_str());
                str.push(',');
                str.push_str(key_pt_2.to_string().as_str());
                str.push(')');
                map.insert(str.clone(), w);
            }
        }
        map
    }

    fn function_to_enum(function: String) -> crate::gtfsstatic::enum_file::File {
        use crate::gtfsstatic::enum_file::File;        
        if function == "agency" {
            return File::Agency;        // Use pattern matching in a rewrite.
        } else if function == "calendar" { 
            return File::Calendar;        // Use pattern matching in a rewrite.
        } else if function == "calendar_dates" {
            return File::CalendarDates;        // Use pattern matching in a rewrite.
        } else if function == "fare_attributes" { 
            return File::FareAttributes;        // Use pattern matching in a rewrite.
        } else if function == "fare_rules" {
            return File::FareRules;
        } else if function == "feed_info" {            
            return File::FeedInfo;
        } else if function == "frequencies" {
            return File::Frequencies;
        } else if function == "routes" {
            return File::Routes;
        } else if function == "shapes" {
            return File::Shapes;
        } else if function == "stop_times" {
            return File::StopTimes;
        } else if function == "stops" {
            return File::Stops;
        } else if function == "transfers" {
            return File::Transfers;
        } else if function == "trips" {
            return File::Trips;
        } else {
            panic!();
        }
    }

    fn find_index(keyword: &str, header: &Iter<'_, &str>) -> Option<usize> {
        let value = header.clone().position(|&x| x == keyword);
        value
    }
}

pub mod paths {
    use std::collections::HashMap;
    pub fn static_data(city: &str) -> HashMap<String, String> {

        let files : Vec<&str> = city_files(&city);
        println!("Data for \x1b[0;31m{}\x1b[0m has been fetched.", city);
        let length = &files.len();
    
        let mut file_paths : HashMap<String, String> = HashMap::new();
    
        for file in files {
            file_paths.insert(file.to_string(), file_path(city, file.to_string()));
        }
        println!("All files to import have been imported.");

        assert!(file_paths.len() == *length);
        file_paths
    }
    
    pub fn file_path(city: &str, file: String) -> String {
        let mut string: String = "src/static".to_string();
        string.push_str(city);

    
        if file == "agency" {
            string.push_str("agency.txt");
        } else if file == "calendar" {
            string.push_str("calendar.txt");
        } else if file == "calendar_dates" {
            string.push_str("calendar_dates.txt");
        } else if file == "fare_attributes" {
            string.push_str("fare_attributes.txt");
        } else if file == "fare_rules" {
            string.push_str("fare_rules.txt");
        } else if file == "feed_info" {
            string.push_str("feed_info.txt");
        } else if file == "frequencies" {
            string.push_str("frequencies.txt");
        } else if file == "routes" {
            string.push_str("routes.txt");
        } else if file == "shapes" {
            string.push_str("shapes.txt");
        } else if file == "stops" {
            string.push_str("stops.txt");
        } else if file == "stop_times" {
            string.push_str("stop_times.txt");  
        } else if file == "timepoints" {
            string.push_str("timepoints.txt");  
        } else if file == "transfers" {
            string.push_str("transfers.txt");
        } else if file == "trips" {
            string.push_str("trips.txt");
        } else if file == "direction_names_exceptions" {
            string.push_str("direction_names_exceptions.txt");  
        } else if file == "stop_order_exceptions" {
            string.push_str("stop_order_exceptions.txt");  
        } else if file == "modifications" {
            string.push_str("modifications.txt");  

        } else {
            panic!("Not a valid city!");
        }
    
        string
    }
    
    pub fn city_files(city: &str) -> Vec<&str> {
        println!("Checking city input: you inputted \x1b[0;31m{}\x1b[0m", city);
        let mut files: Vec<&str> = vec![];
        if city == "/san_antonio/via/" {
            files.push("agency");
            files.push("calendar");
            files.push("calendar_dates");
            files.push("feed_info");
            files.push("routes");
            files.push("shapes");
            files.push("stops");
            files.push("stop_times");
            files.push("transfers");
            files.push("trips");
        } else if city == "/pittsburgh/prt/" {
            files.push("agency");
            files.push("calendar");
            files.push("calendar_dates");
            files.push("fare_attributes");
            files.push("fare_rules");
            files.push("feed_info");
            files.push("frequencies");
            files.push("routes");
            files.push("shapes");
            files.push("stop_times");
            files.push("stops");
            files.push("transfers");
            files.push("trips");
        } else if city == "/seattle/king_county/" {
            files.push("agency");
            files.push("calendar");
            files.push("calendar_dates");
            files.push("fare_attributes");
            files.push("fare_rules");
            files.push("feed_info");
            files.push("routes");
            files.push("shapes");
            files.push("stop_times");
            files.push("stops");
            files.push("transfers");
            files.push("trips");            
        } else if city == "/chicago/cta/" {
            files.push("agency");
            files.push("calendar");
            files.push("calendar_dates");
            files.push("frequencies");
            files.push("routes");
            files.push("shapes");
            files.push("stops");
            files.push("transfers");
            files.push("trips");      
        } else if city == "/san_francisco/muni/" {
            files.push("agency");
            files.push("calendar");
            files.push("calendar_dates");
            files.push("directions");
            files.push("fare_attributes");
            files.push("fare_rules");
            files.push("routes");
            files.push("shapes");
            files.push("stop_times");
            files.push("stops");
            files.push("timepoints");
            files.push("trips"); 
        } else if city == "/seattle/community/" {
            files.push("agency");
            files.push("calendar");
            files.push("calendar_dates");
            files.push("direction_names_exceptions");
            files.push("feed_info");
            files.push("routes");
            files.push("shapes");
            files.push("stop_order_exceptions");
            files.push("stops");
            files.push("transfers");
            files.push("trips");             
        } else if city == "/seattle/everett/" {
            files.push("agency");
            files.push("calendar");
            files.push("calendar_dates");
            files.push("feed_info");
            files.push("routes");
            files.push("shapes");
            files.push("stop_times");
            files.push("stops");
            files.push("trips");              
        } else if city == "/seattle/ferries/" {
            files.push("agency");
            files.push("calendar");
            files.push("fare_attributes");
            files.push("fare_rules");
            files.push("feed_info");
            files.push("routes");
            files.push("shapes");
            files.push("stop_times");
            files.push("stops");
            files.push("trips");      
        } else if city == "/seattle/intercity/" {
            files.push("agency");
            files.push("calendar_dates");
            files.push("fare_attributes");
            files.push("feed_info");
            files.push("routes");
            files.push("shapes");
            files.push("stop_times");
            files.push("stops");
            files.push("transfers");               
            files.push("trips");               
        } else if city == "/seattle/monorail/" {
            files.push("agency");
            files.push("calendar_dates");
            files.push("calendar");
            files.push("fare_attributes");
            files.push("feed_info");
            files.push("routes");
            files.push("shapes");
            files.push("stop_times");
            files.push("stops");
            files.push("transfers");               
            files.push("trips");           
        } else if city == "/seattle/pierce/" {
            files.push("calendar_dates");
            files.push("calendar");
            files.push("fare_attributes");
            files.push("fare_rules");
            files.push("feed_info");
            files.push("routes");
            files.push("shapes");
            files.push("stop_times");
            files.push("stops");              
            files.push("trips");                
        } else if city == "/seattle/sound_transit/" {
            files.push("agency");
            files.push("alternate_stop_names_exceptions");
            files.push("calendar");
            files.push("calendar_dates");
            files.push("direction_names_exceptions");
            files.push("facilities_properties_definitions");
            files.push("facilities_properties");
            files.push("facilities");
            files.push("fare_attributes");
            files.push("fare_rules");
            files.push("feed_info");
            files.push("modifications");
            files.push("routes");
            files.push("shapes");
            files.push("stop_times");
            files.push("stops");
            files.push("trips");    
        } else {
            panic!("Not a valid city!");
        }
    
        files
    }    
}

// This is used for pattern matching the file so that a long chain of if-elses 
// is not necessary. 
pub mod enum_file {
    pub enum File {
        Agency,
        CalendarDates,
        Calendar,
        FareAttributes,
        FareRules,
        FeedInfo,
        Frequencies,
        Routes,
        Shapes,
        StopTimes,
        Stops,
        Transfers,
        Trips
    }

    pub enum Size {
        One(usize),
        Two(usize, usize)
    }

    pub fn is_one(entry : &Size) -> bool {
        match entry {
            Size::One(_) => true,
            Size::Two(_, _) => false,
        }
    }

    pub fn get_first(entry: &Size) -> usize {
        match entry {
            Size::One(x) => *x,
            Size::Two(x, _) => *x,
        }
    }

    pub fn get_second(entry: &Size) -> usize {
        match entry {
            Size::One(x) => *x,
            Size::Two(_, y) => *y,
        }
    }
}
