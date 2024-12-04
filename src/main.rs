// use std::collections::HashMap;
use std::env;
use gtfs_realtime::FeedMessage;
use gtfs_realtime::FeedEntity;


pub mod staticfeed;
pub mod realtime;
pub mod iohandle;
pub mod search;



#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2);

    let city = &args[1];    

    // let input_links: HashMap<String, String> = staticfeed::static_data(city); 
    // let static_data: HashMap<String, HashMap<String, Vec<String>>> = 
    //     staticfeed::static_data_vector(input_links);

    let buses = realtime::caller(city, "vehicles-bus");
    let bustrips = realtime::caller(city, "trips-bus");
    let buses: FeedMessage = buses.await;
    let bustrips: FeedMessage = bustrips.await;
    let busdata : Vec<FeedEntity> = buses.entity;
    let bustripdata: Vec<FeedEntity> = bustrips.entity;

    // iohandle::format_args(args);
    // println!("{:#?}", &busdata);
    // println!("{:#?}", &bustrips.entity);




    // let realtime_strings: HashMap<&str, String> = realtime::
    // realtime::output_get(city);

    // println!("{:?}", realtime_strings["vehicles-bus"]);
    // for i in realtime_strings {
    //     let formatted = realtime::slicer(i.1);
    // }
    // println!("{}", realtime_strings["vehicles-bus"]);

}
