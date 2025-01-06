use crate::{gtfs, search};
use gtfs_realtime::{FeedEntity, FeedMessage};
use std::fs;
use gtfs::gtfsstatic;

// -------- BEGIN MODULE CODE -------- //

pub async fn data(args: Vec<String>) {
    let import: String =
        fs::read_to_string("src/static/index.txt").expect("Should have been able to read file");
    handle_args(import, args).await;
}

async fn handle_args(import: String, args: Vec<String>) {
    println!("{:?}", args);
    if args.len() == 1 {      // If you use just cargo run, then it displays the list of cities.
        has_length_one(import);
    } else if args.len() == 3 {
        has_length_three(import, args).await;
    } else if args.len() == 4 {
        has_length_four(import, args).await;
    } else {
        panic! {"Argument number {} is invalid. Sorry!", args.len()};
    }
}

fn has_length_one(import: String) {
    let import_lines = import.lines();
    for line in import_lines {
        let values: Vec<&str> = line.split(',').collect();
        let index = values[0];
        let city = values[2];
        println!("[{}] is {}.", index, city);
    }
}

async fn has_length_three(import: String, args: Vec<String>) {
    let import_lines = import.lines();
    for line in import_lines {
        let values: Vec<&str> = line.split(',').collect();
        if values[0] == args[1] {
            let city_path: &str = values[1];
            let function = &args[2];
            let city_call = match city_path {
                "/san_antonio/via/" => import_data_sa(function, args.clone()).await,
                _ => import_data_general(function, args.clone(), city_path).await,
            };
            println!("{:#?}", crate::script::list::stops_per_route(city_path));
        }
    }
}

async fn has_length_four(import: String, args: Vec<String>) {
    let import_lines = import.lines();
    for line in import_lines {
        let values: Vec<&str> = line.split(',').collect();
        if values[0] == args[1] {
            let city_path: &str = values[1];
            let function = &args[2];
            let city_call = match city_path {
                "/san_antonio/via/" => import_data_sa(function, args.clone()).await,
                _ => import_data_general(function, args.clone(), city_path).await,
            };

        }
    }
}

async fn import_data_general(function: &String, args: Vec<String>, city_path: &str) -> String {
    if function == "range" {
        let buses = gtfs::gtfsrt::requester(city_path, "vehicles-bus").await;
        let busdata: Vec<FeedEntity> = buses.entity;
        let trips = gtfs::gtfsrt::requester(city_path, "trips-bus").await;
        let tripdata: Vec<FeedEntity> = trips.entity;
        let items: Vec<&str> = args[3].split(',').collect();
        for range in items {
            let pair: Vec<&str> = range.split('-').collect();
            let first = pair[0];
            let last = pair[1];
            search::fetch::in_range(busdata.clone(), tripdata.clone(), first, last, city_path);
        }
    } else if function == "route" {
        let buses = gtfs::gtfsrt::requester(city_path, "vehicles-bus").await;
        let busdata: Vec<FeedEntity> = buses.entity;
        let trips = gtfs::gtfsrt::requester(city_path, "trips-bus").await;
        let tripdata: Vec<FeedEntity> = trips.entity;
        let routes: Vec<&str> = args[3].split(',').collect();
        for route in routes {
            search::fetch::on_route(busdata.clone(), tripdata.clone(), route, city_path);
        }
    } else if function == "routes" {
        gtfs::gtfsstatic::service::routes(city_path);
    } else {
        panic!("Not a valid function, sorry!");
    }

    "done!".to_string()
}

async fn import_data_sa(function: &String, args: Vec<String>) -> String {
    let buses = gtfs::gtfsrt::requester("/san_antonio/via/", "vehicles-bus");
    let buses: FeedMessage = buses.await;
    let busdata: Vec<FeedEntity> = buses.entity;
    if function == "range" {
        let items: Vec<&str> = args[3].split(',').collect();
        for range in items {
            let pair: Vec<&str> = range.split('-').collect();
            let first = pair[0];
            let last = pair[1];
            search::fetch::in_range_vdata(busdata.clone(), first, last, "/san_antonio/via/");
        }
    } else if function == "route" {
        let routes: Vec<&str> = args[3].split(',').collect();
        for route in routes {
            search::fetch::on_route_vdata(busdata.clone(), route, "/san_antonio/via/");
        }
    } else if function == "routes" {
        gtfs::gtfsstatic::service::routes("/san_antonio/via/");
    } else {
        panic!("Not a valid function, sorry!");
    }

    "done!".to_string()
}
