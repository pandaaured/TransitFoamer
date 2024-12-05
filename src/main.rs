use std::env;
use gtfs_realtime::FeedMessage;
use gtfs_realtime::FeedEntity;
use std::collections::HashMap;
// use gtfs_realtime::VehiclePosition;

pub mod staticfeed;
pub mod realtime;
pub mod search;



#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);
    let city = &args[1];    
    // let function = &args[2];
    // let numbers = &args[3];
    // let number2 = &args[4];

    let routes_per_stop = staticfeed::extdata_gtfs::routes_per_stop(city);
    // let input_links: HashMap<String, String> = staticfeed::path_gtfs::static_data(&city); 
    // let static_data: HashMap<String, HashMap<String, Vec<String>>> = 
    //     staticfeed::dict_gtfs::static_data_vector(input_links);
    // for (k, v) in static_data {
    //     println!("{}", k);
    // }

    // let buses = realtime::requester(&city, "vehicles-bus");
    // let trips = realtime::requester(&city, "trips-bus");
    // let buses: FeedMessage = buses.await;
    // let trips: FeedMessage = trips.await;
    // let busdata : Vec<FeedEntity> = buses.entity;
    // let tripdata: Vec<FeedEntity> = trips.entity;

    // search::fetch::result(busdata, tripdata, static_data, function, numbers, number2);
}


