use std::env;

pub mod config;
pub mod gtfs_rt;
pub mod gtfs_static;
pub mod import;
pub mod search;

// -------- BEGIN PROGRAM CODE -------- //

#[tokio::main]
async fn main() {
    let configured: bool = config::is_configured_static();
    if !configured {
        println!("Please complete setup to proceed.");
    } else {
        println!("Success!");
        let args: Vec<String> = env::args().collect();
        import::import().await;
        import::handle_arguments(args);
    }
}

// -------- END PROGRAM CODE -------- //
