// use std::collections::HashMap;
use std::env;
use gtfs_realtime::FeedMessage;
use gtfs_realtime::FeedEntity;
use gtfs_realtime::VehiclePosition;
use std::collections::HashMap;

pub mod staticfeed;
pub mod realtime;
pub mod search;



#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args.len());
    assert!(args.len() >= 2);
    let city = &args[1];    


    let buses = realtime::requester(&city, "vehicles-bus");
    let trips = realtime::requester(&city, "trips-bus");
    let buses: FeedMessage = buses.await;
    let trips: FeedMessage = trips.await;
    let busdata : Vec<FeedEntity> = buses.entity;
    let tripdata: Vec<FeedEntity> = trips.entity;

    for bus in busdata {
        println!("{:?}", bus);
    }

    for trip in tripdata {
        println!("{:?}", trip);
    }

    let input_links: HashMap<String, String> = staticfeed::static_data(&city); 
    let static_data: HashMap<String, HashMap<String, Vec<String>>> = 
        staticfeed::static_data_vector(input_links);
    let input_links: HashMap<String, String> = staticfeed::static_data(&city); 
    let static_data: HashMap<String, HashMap<String, Vec<String>>> = 
        staticfeed::static_data_vector(input_links);
    








    


   




    // println!("{:?}", realtime_strings["vehicles-bus"]);
    // for i in realtime_strings {
    //     let formatted = realtime::slicer(i.1);
    // }
    // println!("{}", realtime_strings["vehicles-bus"]);

}


