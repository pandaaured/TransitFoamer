use std::env;
pub mod gtfs;
pub mod import;
pub mod search;

// -------- BEGIN PROGRAM CODE -------- //

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    import::data(args).await;
}
