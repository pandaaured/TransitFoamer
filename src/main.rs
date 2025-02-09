use std::env;
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
    let args: Vec<String> = env::args().collect(); }

    // import::data(args).await;
}

// -------- END PROGRAM CODE -------- //
