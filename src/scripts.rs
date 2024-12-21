// pub mod scripts {
//     use crate::gtfsstatic::data;
//     use std::collections::HashSet;
//     use std::collections::HashMap;

//     pub fn get_data_for_analysis(city: &str) -> HashMap<String, HashSet<&String>> {
//         let static_trips = data::static_data(city, "trips".to_string());
//         let mut trips_per_route: HashMap<String, HashSet<&String>> = HashMap::new();
//         for trip in static_trips.clone().keys() {
//             let route = static_trips[trip][2].clone();
//             if !trips_per_route.contains_key(&route) {
//                 let mut set: HashSet<&String> = HashSet::new();
//                 set.insert(trip);
//                 trips_per_route.insert(route, set);
//             } else {
//                 let mut set = trips_per_route[&route].clone();
//                 set.insert(trip);
//                 trips_per_route.insert(route, set);
//             }
//         }
    
//         trips_per_route
//     }
// }