fn what() {
    let mut reader = BytesReader::from_bytes(&bytes);
    let foobar = FeedMessage::from_reader(&mut reader, &bytes).expect("Cannot read FooBar");
    
    
    // let mut reader = BytesReader::from_bytes(&bytes);
    // let feed_message = FeedMessage::from_reader(&mut reader, &bytes).expect("Cannot read FeedMessage");
    
}


