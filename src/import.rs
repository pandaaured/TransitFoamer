use crate::{gtfsrt, gtfsstatic, search};
use gtfs_realtime::FeedEntity;
use std::{fs, str::Lines};

// -------- BEGIN MODULE CODE -------- //

pub async fn data(args: Vec<String>) {
    let import: String =
        fs::read_to_string("src/static/index.txt").expect("Should have been able to read file");
    handle_args(import, args).await;
}

async fn handle_args(import: String, args: Vec<String>) {
    // println!("{:?}", args);
    let import_lines = import.lines(); // Convert import and pass it as an argument.

    if args.len() == 1 {
        // If you use just cargo run, then it displays the list of cities.
        has_length_one(import_lines);
    } else if args.len() == 2 {
        has_length_two(import_lines, args);
    } else if args.len() == 3 {
        has_length_three(import_lines, args).await;
    } else if args.len() == 4 {
        has_length_four(import_lines, args).await;
    } else {
        panic! {"Argument number {} is invalid. Sorry!", args.len()};
    }
}

fn has_length_one(import_lines: Lines<'_>) { // This is associated with no function (default)
    for line in import_lines {
        let values: Vec<&str> = line.split(',').collect();
        let index = values[0];  // 
        let city = values[2];
        println!("[{}] is {}.", index, city);
    }
}

fn has_length_two(_import: Lines<'_>, _args: Vec<String>) {
    // This function will do something with just the city input in the future.
}

async fn has_length_three(import_lines: Lines<'_>, args: Vec<String>) {
    for line in import_lines {
        let values: Vec<&str> = line.split(',').collect();
        if values[0] == args[1] {
            let city_path: &str = values[1];
            let function = &args[2];
            if city_path == "/san_antonio/via/" {
                import_data_sa(function, args.clone()).await;
            } else {
                import_data_general(function, args.clone(), city_path).await;
            }
            gtfsstatic::data::processing_layer_two::routes_per_stop(city_path);
        }
    }
}

async fn has_length_four(import_lines: Lines<'_>, args: Vec<String>) {
    for line in import_lines {
        let values: Vec<&str> = line.split(',').collect();
        if values[0] == args[1] {
            let city_path: &str = values[1];
            let function = &args[2];
            handle_things(function, args.clone(), city_path);
            if city_path == "/san_antonio/via/" {
                import_data_sa(function, args.clone()).await;
            } else {
                import_data_general(function, args.clone(), city_path).await;
            }
        }
    }
}


fn handle_things(function: &str, args: Vec<String>, city_path: &str) {
    if function == "range" {
        handle_range(args, city_path);
    } else if function == "route" {
        handle_route(args, city_path);
    } else if function == "stop" {
        handle_stop(args, city_path);
    } else if function == "routes" {
        handle_routes(args, city_path);
    }
}

async fn handle_range(args: Vec<String>, city_path: &str) {
    let items: Vec<&str> = args[3].split(',').collect();
    if city_path == "/pittsburgh/prt/" {
        for range in items {
            let pair: Vec<&str> = range.split('-').collect();
            let first = pair[0];
            let last = pair[1];
            search::fetch::pittsburgh::in_range(first, last, "/pittsburgh/prt/");
        }
    } else if city_path == "/san_antonio/via/" {
        for range in items {
            let pair: Vec<&str> = range.split('-').collect();
            let first = pair[0];
            let last = pair[1];
            search::fetch::san_antonio::in_range_vdata(first, last, "/san_antonio/via/");
        }
    }
        
}

fn handle_route(args: Vec<String>, city_path: &str) {

}

fn handle_stop(args: Vec<String>, city_path: &str) {

}

fn handle_routes(args: Vec<String>, city_path: &str) {

}

async fn import_data_general(function: &String, args: Vec<String>, city_path: &str) -> String {
    if function == "range" {
        let items: Vec<&str> = args[3].split(',').collect();
        for range in items {
            let pair: Vec<&str> = range.split('-').collect();
            let first = pair[0];
            let last = pair[1];
            search::fetch::in_range(busdata.clone(), tripdata.clone(), first, last, city_path);
        }
    } else if function == "route" {
        let buses = gtfsrt::requester(city_path, "vehicles-bus").await;
        let busdata: Vec<FeedEntity> = buses.entity;
        let trips = gtfsrt::requester(city_path, "trips-bus").await;
        let tripdata: Vec<FeedEntity> = trips.entity;
        let routes: Vec<&str> = args[3].split(',').collect();
        for route in routes {
            search::fetch::on_route(busdata.clone(), tripdata.clone(), route, city_path);
        }
    } else if function == "stop" {
        let buses = gtfsrt::requester(city_path, "vehicles-bus").await;
        let busdata: Vec<FeedEntity> = buses.entity;
        let trips = gtfsrt::requester(city_path, "trips-bus").await;
        let tripdata: Vec<FeedEntity> = trips.entity;
        let stops: Vec<&str> = args[3].split(',').collect();
        for stop in stops {
            search::fetch::at_stop(busdata.clone(), tripdata.clone(), stop, city_path);
        }
    } else if function == "routes" {
        gtfsstatic::service::routes(city_path);
    } else {
        panic!("Not a valid function, sorry!");
    }

    "done!".to_string()
}

async fn import_data_sa(function: &String, args: Vec<String>) -> String {
    if function == "range" {
        let buses = gtfsrt::requester("/san_antonio/via/", "vehicles-bus").await;
        let busdata: Vec<FeedEntity> = buses.entity;
        
    } else if function == "route" {
        let buses = gtfsrt::requester("/san_antonio/via/", "vehicles-bus").await;
        let busdata: Vec<FeedEntity> = buses.entity;
        let routes: Vec<&str> = args[3].split(',').collect();
        for route in routes {
            search::fetch::san_antonio::on_route_vdata(busdata.clone(), route, "/san_antonio/via/");
        }
    } else if function == "routes" {
        gtfsstatic::service::routes("/san_antonio/via/");
    } else {
        panic!("Not a valid function, sorry!");
    }

    "done!".to_string()
}
