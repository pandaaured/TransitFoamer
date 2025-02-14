use std::{env, fs};
pub mod gtfs_realtime;
pub mod gtfs_static;
pub mod import;
pub mod search;
pub mod config;

// -------- BEGIN PROGRAM CODE -------- //

#[tokio::main]
async fn main() {
    let configured: bool = config::is_configured();
    if !configured { println!("Please complete setup to proceed."); }
    else { println!("Success!"); 
    let args: Vec<String> = env::args().collect();
    import_static(); }

}

// -------- TEMPORARILY IN MAIN, WILL BE RELOCATED -------- //

fn import_static() {
    let path = fs::read_to_string("./config.txt").expect("foo");
    let bar = gtfs_static::get_data(path, "trips.txt");
    println!("{:#?}", bar);
}


// -------- END TEMPORARILY IN MAIN -------- //

// -------- END PROGRAM CODE -------- //
