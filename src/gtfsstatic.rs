pub mod data {
    use std::{fs, collections::HashMap, slice::Iter};
    use crate::gtfsstatic::enum_file::{File, Size, is_one, get_first, get_second};

    pub fn static_data(city: &str, function: &str) -> HashMap<String, Vec<String>> {
        let file_path = file_path(city, function);
        let ingest_data = read_to_string(file_path);
        hashmap_populate(ingest_data, function)
    }

    fn read_to_string(path: String) -> String {
        let contents : String = 
           fs::read_to_string(path)
              .expect("Should have been able to read the file");
        contents 
    }

    fn hashmap_populate(data: String, function: &str) -> HashMap<String, 
    Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        let mut iterator = data.split('\n');
        let header: Vec<&str> = iterator.next().unwrap().split(',').collect();
        let header_iter = header.iter();
        let index = match function_to_enum(function) {
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

    fn file_path(city: &str, file: &str) -> String {
        let mut string: String = "src/static".to_string();
        string.push_str(city);
        string.push_str(file);
        string.push_str(".txt");

        string
    }
    
    
    fn find_index(keyword: &str, header: &Iter<'_, &str>) -> Option<usize> {
        header.clone().position(|&x| x == keyword)
    }

    fn function_to_enum(function: &str) -> File {
        let file: File = match function {
            "agency" => File::Agency,
            "calendar" => File::Calendar,
            "calendar_dates" => File::CalendarDates,
            "fare_attributes" => File::FareAttributes,
            "fare_rules" => File::FareRules,
            "feed_info" => File::FeedInfo,
            "frequencies" => File::Frequencies,
            "routes" => File::Routes,
            "shapes" => File::Shapes,
            "stop_times" => File::StopTimes,
            "stops" => File::Stops,
            "transfers" => File::Transfers,
            "trips" => File::Trips,
            _ => panic!(),
        };

        file
    }
}

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
