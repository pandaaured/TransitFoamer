use std::{env, fs};
pub mod gtfsrt;
pub mod gtfsstatic;
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

// -- TEMPORARILY IN MAIN, WILL BE RELOCATED -- //

fn import_static() {
    let mut path = fs::read_to_string("./config.txt").expect("foo");
    path.push_str("stop_times.txt");
    gtfsstatic::get_data(path);
}


// -- END TEMPORARILY IN MAIN -- //

// -------- END PROGRAM CODE -------- //
