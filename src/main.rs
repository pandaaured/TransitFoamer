//! # TransitFoamer
//!
//! TransitFoamer is a collection of GTFS Static and GTFS Realtime utilities
//! which can be used to find various types of information from both GTFS
//! Static and GTFS Realtime information.
//!
//! The GTFS (General Transit Feed Specification) site containing the documentation
//! for the standard can be found at the following location:
//! `https://gtfs.org`
//!
//!
pub mod gtfs_rt;
pub mod gtfs_static;
pub mod handlers;
pub mod structs;
pub mod testing;

use std::{fs, time::Duration};

use http::StatusCode;

use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use axum::{
    extract::Query,
    response::{Html, Json},
    routing::get,
    Router,
};

use tokio::signal;
use gtfs_rt::{on_route, url_to_feedmessage};
use gtfs_static::{Agency, Routes, StopTimes, Stops, Trips};
use structs::Links;

// Run the main server routine.
#[tokio::main]
async fn main() {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.trips).await.unwrap();
    let static_data = StaticInfo::new();

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Creating the axum app. We add routers for our various API functions described below.
    // 1) / returns the HTML index.
    // 2) /route/{route} is the `route` call. Displays information about a specific route.
    //    Uses GTFS-RT!
    // 3) /routes is the `routes` call, returns a list of routes and associated data.
    let app: Router = Router::new()
        .layer(cors_layer)
        .route("/", get(handlers::main_content_handler))
        .route("/routes", get(handlers::route_list_handler))
        .route("/fleet", get(handlers::fleet_handler)) 
        .nest_service("/static", ServeDir::new("static"))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

// Deliver the main file to the client. (index.html)

// This function is a handler for the route API request. Uses GTFS-RT.
/* async fn route_gtfsrt_handler(Query(routes) : Query<RouteQuery> ) -> Json<String> {
    let links = Links::new();
    let trip_update = url_to_feedmessage(links.vehicles).await.unwrap();
    let route = routes.route;
    let route_entries = on_route(route, trip_update);
    let json = serde_json::to_string(&route_entries).unwrap();
    Json(json)
} */

/* async fn fleet_analyzer() -> Json<String> {
    let file_path = "src/static/seattle/kc/".to_string();
} */

struct StaticInfo {
    agency: Vec<Agency>,
    routes: Vec<Routes>,
    stops: Vec<Stops>,
    stop_times: Vec<StopTimes>,
    trips: Vec<Trips>,
}

impl StaticInfo {
    fn new() -> StaticInfo {
        let path: String = "src/static/seattle/kc/".to_string();
        StaticInfo {
            agency: Agency::new_vec(&path).unwrap(),
            routes: Routes::new_vec(&path).unwrap(),
            stops: Stops::new_vec(&path).unwrap(),
            stop_times: StopTimes::new_vec(&path).unwrap(),
            trips: Trips::new_vec(&path).unwrap(),
        }
    }
}

struct RouteQuery {
    route: String,
}
