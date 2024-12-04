// use std::collections::HashMap;
use std::env;
use gtfs_realtime::FeedMessage;
use gtfs_realtime::FeedEntity;
use gtfs_realtime::VehiclePosition;
use std::collections::HashMap;

pub mod staticfeed;
pub mod realtime;
pub mod iohandle;
pub mod search;



#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);
 

    let index: i32 = iohandle::state(args.clone());
    let city = &args[1];    


    if index == 0 { // Grabbing GTFS-RT data from Rust and Static.
        let buses = realtime::requester(&city, "vehicles-bus");
        let bustrips = realtime::requester(&city, "trips-bus");
        let buses: FeedMessage = buses.await;
        let bustrips: FeedMessage = bustrips.await;
        let busdata : Vec<FeedEntity> = buses.entity;
        let bustripdata: Vec<FeedEntity> = bustrips.entity;
        let bus: &Option<VehiclePosition> = &busdata[0].vehicle;
        let input_links: HashMap<String, String> = staticfeed::static_data(&city); 
        let static_data: HashMap<String, HashMap<String, Vec<String>>> = 
            staticfeed::static_data_vector(input_links);
    } else if index == 1 { // Grabbing GTFS-static only.
        let input_links: HashMap<String, String> = staticfeed::static_data(&city); 
        let static_data: HashMap<String, HashMap<String, Vec<String>>> = 
            staticfeed::static_data_vector(input_links);
    } else if index == 2 { // Grabbing GTFS-RT from Python.
        realtime::caller(&city);

  
    }







    


   




    // println!("{:?}", realtime_strings["vehicles-bus"]);
    // for i in realtime_strings {
    //     let formatted = realtime::slicer(i.1);
    // }
    // println!("{}", realtime_strings["vehicles-bus"]);

}


