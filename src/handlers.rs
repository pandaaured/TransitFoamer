use std::fs;

use axum::response::Html;
use axum::Json;

use crate::gtfs_rt::{on_route, url_to_feedmessage};
use crate::schedule::build_schedule;
use crate::structs::{AppState, Links, Schedule};

use crate::gtfs_static::{Routes};
use crate::gtfs_static::{group_stoptimes_by_trip_id, unique_trip_sequences};

use crate::State;

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use prost::Message;

// Fetches the route list. Returns as HTML ul.
pub async fn route_list_handler(State(state): State<AppState>) -> Json<Vec<Routes>> {
    Json(state.static_info.routes.clone())
}

// Fetches the stop times list. Returns as HTML ul.
pub async fn stop_times_list_handler(State(state): State<AppState>) {
    let data = &state.static_info.stop_times;
    let copy = data.clone();
    let grouped = unique_trip_sequences(copy);
}

// Returns a JSON schedule for a given route_id, grouped by (service_id, stop_pattern).
// Trips with identical stop sequences are collapsed into the same timetable.
pub async fn schedule_handler(
    Path(route_id): Path<String>,
    State(state): State<AppState>,
) -> Json<Schedule> {
    Json(build_schedule(
        &route_id,
        &state.static_info.trips,
        &state.static_info.stop_times,
        &state.static_info.stops,
        &state.static_info.calendar,
        &state.static_info.calendar_dates,
    ))
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
