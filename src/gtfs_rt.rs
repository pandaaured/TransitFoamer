use gtfs_realtime::FeedMessage; // other
use reqwest::Response;

// -------- BEGIN MODULE CODE -------- //

pub async fn requester(url: &str) -> FeedMessage {
    let response: Response = reqwest::get(url).await.unwrap();
    let bytes = response.bytes().await.unwrap();
    let data: Result<gtfs_realtime::FeedMessage, prost::DecodeError> =
        prost::Message::decode(bytes.as_ref());
    data.unwrap()
}
