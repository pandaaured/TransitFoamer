// use std::collections::HashMap;

// // use gtfs_realtime::TripUpdate;
// // use gtfs_realtime::VehiclePosition;
// // use gtfs_realtime::TripDescriptor;
// use gtfs_realtime::TripUpdate;
// // 

// pub fn dict_convert(buses: Vec<FeedEntity>) -> HashMap<String, HashMap<String, String>> {
//     let mut bus_map: HashMap<String, HashMap<String, String>> = HashMap::new();
//     for bus in buses {
//         let id: String = bus.vehicle.unwrap()
//                             .vehicle.unwrap()
//                             .id.unwrap();
//         bus_map.insert(id, HashMap::new());
//         if bus.vehicle.unwrap().trip.unwrap().trip_id != None {
//             bus_map[&id].insert("trip_id".to_string(), bus.trip_update
//                                                                .unwrap()
//                                                                .trip
//                                                                .trip_id
//                                                                .unwrap());
//         }
//         if bus.vehicle.unwrap().trip.unwrap().route_id != None {
//             bus_map[&id].insert("route_id".to_string(), bus.trip_update);
//         }
//         if bus.vehicle.unwrap().trip.unwrap().direction_id != None {
//             bus_map[&id].insert("direction_id".to_string(), direction_id);
//         }
//     }

//     bus_map
// }

// pub fn bus_dict_add (mut dict: HashMap<String, HashMap<String, Vec<StopTimeUpdate>>>, trips: Vec<FeedEntity>) -> HashMap<String, HashMap<String, String>> {
//     for trip in trips {
//         let id: String = trip.trip_update.unwrap()
//                              .vehicle.unwrap()
//                              .id.unwrap();
//         if !trip.trip_update.unwrap().stop_time_update.is_empty() {
//             dict[&id].insert("stop_time_update".to_string(), stop_time_update);
//         }
//     }

//     dict
// }
