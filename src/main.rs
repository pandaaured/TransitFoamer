use std::fs;
pub mod config;
pub mod gtfs_rt;
pub mod gtfs_static;
pub mod import;
pub mod search;

use gtfs_realtime::FeedMessage;

// -------- BEGIN PROGRAM CODE -------- //

#[tokio::main]
async fn main() {
    let configured: bool = config::is_configured_static();
    if !configured {
        println!("Please complete setup to proceed.");
    } else {
        println!("Success!");
        // let args: Vec<String> = env::args().collect();
        import().await;
    }
}

// -------- TEMPORARILY IN MAIN, WILL BE RELOCATED -------- //

async fn import() {
    let path: String = fs::read_to_string("./config.txt").expect("foo");
    let iter: Vec<&str> = path.split('\n').collect();
    let static_path = iter.get(2)
                                  .unwrap()
                                  .to_string();
    let realtime_path_vehicles = iter.get(3)
                                             .unwrap()
                                             .to_string();
    let realtime_path_trips = iter.get(4)
                                          .unwrap()
                                          .to_string();
    let realtime_path_alerts = iter.get(5)
                                           .unwrap()
                                           .to_string();
    import_static(static_path);
    let vehicles = import_realtime(realtime_path_vehicles).await;
    let trips = import_realtime(realtime_path_trips).await;
    let alerts = import_realtime(realtime_path_alerts).await;
    println!("{:?}", vehicles);
    println!("{:?}", trips);
    println!("{:?}", alerts);
}

fn import_static(file_path: String) {
    let bar = gtfs_static::get_trips(file_path);
    println!("{:#?}", bar);
}

async fn import_realtime(url: String) -> FeedMessage {
    let message = gtfs_rt::requester(&url);
    message.await
}

// -------- END TEMPORARILY IN MAIN -------- //

// -------- END PROGRAM CODE -------- //
