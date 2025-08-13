//! Testing
//!
//! A library with testing infrastructure for the GTFS_RT library. Contains
//! multiple functions that deal with converting between protocol buffers, 
//! serialized structs, JSON, and files. I enumerate a list of conversions 
//! below:
//! 1. File path -> _:
//!   a. file path -> bytes: path_to_bytes
//!   b. file path -> string: path_to_string
//! 2. Bytes -> _:
//!   a. bytes -> GTFS: bytes_to_gtfs
//! 3. GTFS -> _:
//!   a. GTFS -> JSON: gtfs_to_json
//! 4. JSON -> _:
//!   a. JSON -> file path: json_to_path
//! 
//! These steps are done incrementally so that chained computations can be done
//! in a more simplistic, "compositional" way that avoids creating too many 
//! monolithic functions and retains flexibility for future test-case writing.

use gtfs_realtime::{FeedMessage};
use prost::DecodeError;
use std::fs;

/// Returns either an error or a protobuf file opened as bytes
pub fn path_to_bytes(path: &str) -> Result<Vec<u8>, std::io::Error> {
    let bytes: Result<Vec<u8>, std::io::Error> = fs::read(path);
    bytes 
}

/// Returns either an error (decoding) or a decoded FeedMessage from bytes.
pub fn bytes_to_gtfs(bytes: Vec<u8>) -> Result<FeedMessage, DecodeError> {
    let data: Result<gtfs_realtime::FeedMessage, prost::DecodeError> = 
        prost::Message::decode(bytes.as_ref());

    data
}

/// Returns a serialized JSON string from the FeedMessage struct.
pub fn gtfs_to_json(message: FeedMessage) -> Result<String, serde_json::Error> {
    serde_json::to_string(&message)
}

pub fn json_to_path(string: String, path: String) -> std::io::Result<()> {
    fs::write(path, string)?;
    Ok(())
}

pub fn path_to_string(path: String) -> Result<String, std::io::Error> {
    let json_string: Result<String, std::io::Error> = fs::read_to_string(path);
    json_string
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn serde_json_prt_bus_trips() {
        let x =
            crate::gtfs_rt::url_to_feedmessage("https://truetime.portauthority.org/gtfsrt-bus/trips").await;
        let y = gtfs_to_json(x.unwrap()).unwrap();
        println!("{:#?}", y);
    }

    #[test]
    fn read_and_parse_tripupdate() {
        let path: &str = "test/tripupdates.pb";
        let bytes = path_to_bytes(path);
        let gtfs = bytes_to_gtfs(bytes.unwrap());
        println!("{:#?}", gtfs);
    }

    #[test]
    fn read_and_parse_vehicleposition() {
        let path: &str = "test/vehiclepositions.pb";
        let bytes = path_to_bytes(path);
        let gtfs = bytes_to_gtfs(bytes.unwrap());
        println!("{:#?}", gtfs);
    }

    #[test]
    fn read_and_parse_alerts() {
        let path: &str = "test/alerts.pb";
        let bytes = path_to_bytes(path);
        let gtfs = bytes_to_gtfs(bytes.unwrap());
        println!("{:#?}", gtfs);
    }

    #[test]
    fn write_json_for_tripupdate() {
        let path: &str = "test/tripupdates.pb";
        let bytes = path_to_bytes(path);
        let gtfs = bytes_to_gtfs(bytes.unwrap());
        let json = gtfs_to_json(gtfs.unwrap());
        let _ = json_to_path(json.unwrap(), "test/tripupdates.json".to_string());
    }

    #[test]
    fn write_json_for_vehicleposition() {
        let path: &str = "test/vehiclepositions.pb";
        let bytes = path_to_bytes(path);
        let gtfs = bytes_to_gtfs(bytes.unwrap());
        let json = gtfs_to_json(gtfs.unwrap());
        let _ = json_to_path(json.unwrap(), "test/vehiclepositions.json".to_string());
    }

    #[test]
    fn write_json_for_alerts() {
        let path: &str = "test/alerts.pb";
        let bytes = path_to_bytes(path);
        let gtfs = bytes_to_gtfs(bytes.unwrap());
        let json = gtfs_to_json(gtfs.unwrap());
        let _ = json_to_path(json.unwrap(), "test/alerts.json".to_string());
    }
}