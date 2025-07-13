/// This file consists of test "print" functions that output the result of
/// computations to the terminal. This is purely intended as a debug/test of
/// the functionality of the files and does not represent the finished product
/// of this crate.
use gtfs_realtime::{FeedEntity, FeedMessage};

/// Takes a FeedMessage which has been filtered to only display vehicles within
/// a certain range and prints their information.
fn print_in_range(message: FeedMessage, low: i32, high: i32) {
    let entity = message.entity;
    let mut has_route: Vec<FeedEntity> = Vec::new();
    let mut has_no_route: Vec<FeedEntity> = Vec::new();

    for item in entity {
        if item.trip_update.is_some() {
            has_route.push(item);
        } else {
            has_no_route.push(item);
        }
    }
}
