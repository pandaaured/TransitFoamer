use axum::Json;

use crate::gtfs_rt::{on_route, url_to_feedmessage};
use crate::schedule::build_schedule;
use crate::structs::{AppState, Links, Schedule};

use crate::gtfs_static::{Routes};

use crate::State;

use axum::{
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use prost::Message;

// Fetches the route list. Returns as JSON.
pub async fn route_list_handler(State(state): State<AppState>) -> Json<Vec<Routes>> {
    println!("API request made to /rtlist -- list of routes.");
    Json(state.static_info.routes.clone())
}

// Returns a JSON schedule for a given route_id, grouped by (service_id, stop_pattern).
// Trips with identical stop sequences are collapsed into the same timetable.
pub async fn schedule_handler(
    Path(route_id): Path<String>,
    State(state): State<AppState>,
) -> Json<Schedule> {
    println!("API request made to /schedule/{route_id} -- a schedule for this route.");
    Json(build_schedule(
        &route_id,
        &state.static_info.trips,
        &state.static_info.stop_times,
        &state.static_info.stops,
        &state.static_info.calendar,
        &state.static_info.calendar_dates,
    ))
}

// Filters a GTFS-RT feed by route_id.
pub async fn route_handler(Path(route_id): Path<String>) -> Response {
    println!("API request made to /routes/{route_id} -- realtime information for this route.");
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
