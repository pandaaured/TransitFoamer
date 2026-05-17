use std::fs;

use axum::response::Html;

use crate::gtfs_rt::{on_route, url_to_feedmessage};
use crate::structs::Links;

use crate::gtfs_static::{group_stoptimes_by_trip_id, unique_trip_sequences};

use crate::{AppState, State};

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use prost::Message;

//  Handles the main request (returns index.html)
pub async fn main_content_handler() -> Html<String> {
    let file_path = "dist/index.html".to_string();
    let file_as_string =
        fs::read_to_string(file_path).expect("Should have been able to read dist/index.html");
    Html(file_as_string)
}

// Fetches the route list. Returns as HTML ul.
pub async fn route_list_handler(State(state): State<AppState>) -> Html<String> {
    let data = &state.static_info.routes;
    let ids = data
        .iter()
        .map(|x| format!("<li>{}</li>", x.route_id.clone()));
    let items = ids.collect::<Vec<String>>().join("\n");

    Html(format!("<ul>{}</ul>", items))
}

// Fetches the stop times list. Returns as HTML ul.
pub async fn stop_times_list_handler(State(state): State<AppState>) {
    let data = &state.static_info.stop_times;
    let copy = data.clone();
    let grouped = unique_trip_sequences(copy);
    println!("how many vecs are there after filter\n {:?}", grouped);
    println!("{:?}", grouped);
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
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/x-protobuf")],
        encoded,
    )
        .into_response()
}
