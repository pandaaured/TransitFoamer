// use std::process::Command;  // Use this if we use the python script.

pub mod typeconv;
pub mod classes;

fn main() {
    // Importing a python process to call and decode GTFS-RT protobuf files.
    // let python3_child = {Command::new("python3")
    //     .arg("fetcher.py")
    //     .output()
    //     .expect("failed to execute process")};
    // We make a vector with all our data.
    let data_vec = typeconv::string_to_vec_of_vec_of_vec();
    // Aliasing the elements of this vector.
    let agency_vec = &data_vec[0];
    let calendar_dates_vec = &data_vec[1];
    let calendar_vec = &data_vec[2];
    let routes_vec = &data_vec[4];
    let stop_times_vec = &data_vec[6];
    let stops_vec = &data_vec[7];
    let trips_vec = &data_vec[9];
    // We make Hash Maps with the data we want.
    let agency = 
        typeconv::vec_to_hashmap_agency(agency_vec);
    let calendar_dates = 
        typeconv::vec_to_hashmap_cal_dates(calendar_dates_vec);
    let calendar = 
        typeconv::vec_to_hashmap_cal(calendar_vec);
    let routes = 
        typeconv::vec_to_hashmap_routes(routes_vec);
    let stop_times = 
        typeconv::vec_to_hashmap_stop_times(stop_times_vec);
    let stops = 
        typeconv::vec_to_hashmap_stop_times(stops_vec);
    let trips = 
        typeconv::vec_to_hashmap_stop_times(trips_vec);



}
