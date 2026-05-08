use std::fs;

use axum::response::Html;

use crate::gtfs_rt::{on_route, url_to_feedmessage};
use crate::structs::Links;

use axum::{
    extract::Path,
    response::{IntoResponse, Response},
    http::{header, StatusCode},
};
use prost::Message;

//  Handles the main request (returns index.html)
pub async fn main_content_handler() -> Html<String> {
    let file_path = "dist/index.html".to_string();
    let file_as_string =
        fs::read_to_string(file_path).expect("Should have been able to read dist/index.html");
    Html(file_as_string)
}

// Filters a GTFS-RT feed by route. 
pub async fn route_handler(Path(route_id): Path<String>) -> Response {
    let links = Links::new();

    // Decodes protobuf into FeedMessage
    let feed = match url_to_feedmessage(links.trips).await {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to fetch feed: {e}");
            return StatusCode::BAD_GATEWAY.into_response();
        }
    };    

    // Filters FeedMessage
    let filtered = on_route(route_id, feed);

    // Encodes FeedMessage into Protobuf
    let encoded = filtered.encode_to_vec();
    (StatusCode::OK, [(header::CONTENT_TYPE, "application/x-protobuf")], encoded).into_response()
}
