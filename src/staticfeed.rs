// This module handles the use of data external to the GTFS static definition.
// It is primarily used currently to import the output of my script which
// returns bus stops and the routes which serve them. Data unfortunately in 
// JSON currently. 
pub mod extdata_gtfs {    
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Read;

    #[derive(Debug, serde::Deserialize)]
    struct Data {
        stops: HashMap<String, Vec<String>>,
    }

    pub fn routes_per_stop(city: &str) -> HashMap<String, Vec<String>> {
        let file_path = file_path(city);
        let mut file = File::open(file_path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        let json: Data = serde_json::from_str(&mut data).expect("Should work");
        let hash_data: HashMap<String, Vec<String>> = json.stops;

        hash_data   
    }

    fn file_path(city: &str) -> String {
        let mut string: String = "data/".to_string();
    
        if city == "satx" { 
          string.push_str("/via/");
        } else if city == "pgh" {
          string.push_str("/prt/");
        } else if city == "chicago" {
          string.push_str("/cta/");
        } else {
            panic!("Not a valid city!");
        }

        string.push_str("routesperstop.json");
    
        string
    }
}

pub mod dict_gtfs {
    use std::collections::HashMap;
    use std::fs;

    pub fn static_data_vector(path_data: HashMap<String, String>) -> HashMap<String, HashMap<String, Vec<String>>> {
        let mut static_data: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
        for path in path_data {
            let foo = path_io(path.0.clone(), path.1);
            static_data.insert(path.0.clone(), foo);
        }
        static_data
    }
    
    fn path_io(function: String, path: String) -> HashMap<String, Vec<String>> {
        let mut map : HashMap<String, Vec<String>> = HashMap::new();
        let data: String = path_io_helper(path);
        let iterator = data.split('\n');
        for i in iterator {
            let v: Vec<&str> = i.split(',').collect();
            let w : Vec<String> = v.into_iter().map(|x| x.to_string()).collect();
            let mut key = String::new();
            assert!(!w.is_empty());
            if function == "agency" {       // Use pattern matching in a rewrite.
                key = w[0].clone();
            } else if function == "calendar" {
                key = w[0].clone();
            } else if function == "calendar_dates" {
                key = w[1].clone();
            } else if function == "fare_attributes" {
                key = w[0].clone();
            } else if function == "fare_rules" {
                key = w[0].clone();
            } else if function == "feed_info" {
                key = w[0].clone();
            } else if function == "frequencies" {
                key = w[0].clone();
            } else if function == "routes" {
                key = w[0].clone();
            } else if function == "shapes" {
                key = w[0].clone();
            } else if function == "stop_times" {
                key = w[0].clone();
            } else if function == "stops" {
                key = w[10].clone();
            } else if function == "transfers" {
                key = w[0].clone();
            } else if function == "trips" {
                key = w[8].clone();
            }
            map.insert(key.to_string(), w);
        }
        map
    }
    
    fn path_io_helper(path: String) -> String {
        println!("{}", path);
        let contents : String = 
           fs::read_to_string(path)
              .expect("Should have been able to read the file");
        contents 
    }
}

pub mod path_gtfs {
    use std::collections::HashMap;
    pub fn static_data(city: &str) -> HashMap<String, String> {

        let files : Vec<&str> = city_files(&city);
        println!("Data for \x1b[0;31m{}\x1b[0m has been fetched.", city);
        let length = &files.len();
    
        let mut file_paths : HashMap<String, String> = HashMap::new();
    
        for file in files {
            file_paths.insert(file.to_string(), file_path(city, file));
        }
        println!("All files to import have been imported.");
    
        assert!(file_paths.len() == *length);
        file_paths
    }
    
    fn file_path(city: &str, file: &str) -> String {
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
    
    fn city_files(city: &str) -> Vec<&str> {
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

// #[cfg(test)]
// mod tests {
//     use std::fs;
//     use std::collections::HashMap;
//     use super::*;

// }