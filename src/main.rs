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
pub mod html;
pub mod links;
pub mod list;
pub mod testing;

use std::{fs, time::Duration};

use http::StatusCode;

use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
    services::ServeDir
};

use axum::{response::{Html, Json}, routing::get, Router,
           extract::Query};

use tokio::signal;

use links::Links;
use list::Fleet;

use gtfs_static::{Agency, Routes, Stops, StopTimes, Trips};
use gtfs_rt::{url_to_feedmessage, on_route};

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
        .route("/fleetnfi_de_60_lf_2008", get(handlers::nfi_de60lf_2008_handler))
        .route("/fleetnfi_de_60_lf_2009", get(handlers::nfi_de60lf_2009_handler))
        .route("/fleetnfi_de_60_lfa_2009", get(handlers::nfi_de60lfa_2009_handler))
        .route("/fleetnfi_de_60_lfr_2010", get(handlers::nfi_de60lfr_2010_handler))
        .route("/fleetorion_vii_2010", get(handlers::orion_2010_handler))
        .route("/fleetorion_vii_2011", get(handlers::orion_2011_handler))
        .route("/fleetnfi_de_60_lfr_2011", get(handlers::nfi_de60lfr_2011_handler))
        .route("/fleetnfi_de_60_lfr_2012", get(handlers::nfi_de60lfr_2012_handler))
        .route("/fleetnfi_de_60_lfr_2013", get(handlers::nfi_de60lfr_2013_handler))
        .route("/fleetnfi_xde_35_2014", get(handlers::nfi_xde35_2014_handler))
        .route("/fleetnfi_xde_40_2014", get(handlers::nfi_xde40_2014_handler))
        .route("/fleetnfi_xde_60_2014", get(handlers::nfi_xde60_2014_handler))
        .route("/fleetnfi_xt_40_2014", get(handlers::nfi_xt40_2014_handler))
        .route("/fleetnfi_xde_60_2015", get(handlers::nfi_xde60_2015_handler))
        .route("/fleetnfi_xt_60_2015", get(handlers::nfi_xt60_2015_handler))
        .route("/fleetnfi_xde_60_2016", get(handlers::nfi_xde60_2016_handler))
        .route("/fleetgillig_hev_40_2017", get(handlers::gillig_hev40_2017_handler))
        .route("/fleetnfi_xde_40_2017", get(handlers::nfi_xde40_2017_handler))
        .route("/fleetnfi_xde_60_2017", get(handlers::nfi_xde60_2017_handler))
        .route("/fleetnfi_xde_60_2018", get(handlers::nfi_xde60_2018_handler))
        .route("/fleetgillig_hev_40_2018", get(handlers::gillig_hev40_2018_handler))
        .route("/fleetnfi_xde_60_2019", get(handlers::nfi_xde60_2019_handler))
        .route("/fleetgillig_hev_40_2019", get(handlers::gillig_hev40_2019_handler))
        .route("/fleetnfi_xe_40_2021", get(handlers::nfi_xe40_2021_handler))
        .route("/fleetnfi_xe_60_2021", get(handlers::nfi_xe60_2021_handler))
        .route("/fleetnfi_xde_60_2023", get(handlers::nfi_xde60_2023_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT,
                Duration::from_secs(10)),
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

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
    route: String
}


