use std::env;
// use std::collections::HashMap;
use std::fs;
use gtfs_realtime::FeedMessage;
use gtfs_realtime::FeedEntity;
// use gtfs_realtime::VehiclePosition;


pub mod gtfsstatic;
pub mod gtfsrt;
pub mod search;
pub mod feedmessage;
pub mod scripts;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let import: String = fs::read_to_string("src/static/index.txt")
                            .expect("Should have been able to read file");
    data(args, import).await
}

async fn data(args: Vec<String>, import: String) {
    let import_lines = import.lines();
    if args.len() == 1 {
        for line in import_lines {
            let values: Vec<&str> = line.split(',').collect();
            let index = values[0];
            let path = values[1];
            if index.len() == 1 {
                println!("[0{}] has path {}", index, path);
            } else {
                println!("[{}] has path {}", index, path);
            }
        }
    } else {
        for line in import_lines {
            let values: Vec<&str> = line.split(',').collect();
            if values[0] == args[1] {
                let city_path: &str = values[1];
                let function = &args[2];
                // let input_links: HashMap<String, String> = gtfsstatic::paths::static_data(&city_path); 
                // let routes_per_stop = staticfeed::extdata_gtfs::routes_per_stop(&city_path);

                // let static_data: HashMap<String, HashMap<String, Vec<String>>> = 
                //     gtfsstatic::dict_gtfs::static_data_vector(input_links);

                let buses = gtfsrt::requester(&city_path, "vehicles-bus");
                let buses: FeedMessage = buses.await;
                let busdata : Vec<FeedEntity> = buses.entity;
                // let trips = realtime::requester(&city_path, "trips-bus");
                // let trips: FeedMessage = trips.await;
                // let tripdata: Vec<FeedEntity> = trips.entity; 

                if function == "range" {
                    let items: Vec<&str> = args[3].split(',')
                                                  .collect();
                                                          
                    for range in items {
                        let pair: Vec<&str> = range.split('-').collect();
                        let first = pair[0];
                        let last = pair[1];
                        search::fetch::in_range_vdata(busdata.clone(), first, last, city_path);
                    }

                } else if function == "route" {
                    let routes: Vec<&str> = args[3].split(',')
                                                  .collect();
                    for route in routes {
                        search::fetch::on_route_vdata(busdata.clone(), route, city_path);

                    }
                }       
            }
        }
    }
}
