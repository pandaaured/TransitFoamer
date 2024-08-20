extern crate quick_protobuf;

mod transit_realtime; // (see 1.)

use transit_realtime::FeedMessage;
use quick_protobuf::{MessageRead, BytesReader};
use quick_protobuf::reader::Reader;


fn main() {
    let bytes: Vec<u8>;
    bytes = vec![];


    let mut reader = BytesReader::from_bytes(&bytes);
    let foobar = FeedMessage::from_reader(&mut reader, &bytes).expect("Cannot read FooBar");

    let reader = 

    // let mut reader = BytesReader::from_bytes(&bytes);
    // let feed_message = FeedMessage::from_reader(&mut reader, &bytes).expect("Cannot read FeedMessage");




}