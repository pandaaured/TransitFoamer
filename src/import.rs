use crate::{search, gtfsstatic};
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

fn has_length_one(import_lines: Lines<'_>) {
    // This is associated with no function (default)
    for line in import_lines {
        let values: Vec<&str> = line.split(',').collect();
        let index = values[0]; //
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
            handle_things(function, args.clone(), city_path).await;
        }
    }
}

async fn has_length_four(import_lines: Lines<'_>, args: Vec<String>) {
    for line in import_lines {
        let values: Vec<&str> = line.split(',').collect();
        if values[0] == args[1] {
            let city_path: &str = values[1];
            let function = &args[2];
            handle_things(function, args.clone(), city_path).await;
        }
    }
}

async fn handle_things(function: &str, args: Vec<String>, city_path: &str) {
    if function == "range" {
        handle_range(args, city_path).await;
    } else if function == "route" {
        handle_route(args, city_path).await;
    } else if function == "stop" {
        handle_stop(args, city_path).await;
    } else if function == "routes" {
        handle_routes(city_path).await;
    }
}

async fn handle_range(args: Vec<String>, city_path: &str) {
    let items: Vec<&str> = args[3].split(',').collect();
    if city_path == "/pittsburgh/prt/" {
        for range in items {
            let pair: Vec<&str> = range.split('-').collect();
            let first = pair[0];
            let last = pair[1];
            search::fetch::pittsburgh::in_range(first, last, city_path).await;
        }
    } else if city_path == "/san_antonio/via/" {
        for range in items {
            let pair: Vec<&str> = range.split('-').collect();
            let first = pair[0];
            let last = pair[1];
            search::fetch::san_antonio::in_range_vdata(first, last, city_path).await;
        }
    } else {
        for range in items {
            let pair: Vec<&str> = range.split('-').collect();
            let first = pair[0];
            let last = pair[1];
            search::fetch::any_city::in_range(first, last, city_path).await;
        }
    }
}

async fn handle_route(args: Vec<String>, city_path: &str) {
    let routes: Vec<&str> = args[3].split(',').collect();
    if city_path == "/pittsburgh/prt/" {
        for route in routes {
            search::fetch::pittsburgh::on_route(route, city_path).await;
        }
    } else if city_path == "/san_antonio/via/" {
        for route in routes {
            search::fetch::san_antonio::on_route_vdata(route, city_path).await;
        }
    } else {
        for route in routes {
            search::fetch::any_city::on_route(route, city_path).await;
        }
    }
}

async fn handle_stop(args: Vec<String>, city_path: &str) {
    let stops: Vec<&str> = args[3].split(',').collect();

    if city_path == "/pittsburgh/prt/" {
        for stop in stops {
            search::fetch::pittsburgh::at_stop(stop, city_path).await;
        }
    } else if city_path == "/san_antonio/via/" {
        // for stop in stops {
        //     search::fetch::san_antonio::at_stop(stop, "/san_antonio/via/");
        // }
    } else {
        for stop in stops {
            search::fetch::any_city::at_stop(stop, city_path).await;
        }
    }
}

async fn handle_routes(city_path: &str) {
    gtfsstatic::service::routes(city_path);
}
