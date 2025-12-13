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
pub mod static_fetch;
pub mod testing;
pub mod htmlwriter;
pub mod list;

use std::{fs, time::Duration};

use http::StatusCode;

use tower_http::{
    cors::{Any, CorsLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
    services::ServeDir
};

use axum::{response::{Html, Json}, routing::get, Router};

use gtfs_static::Routes;

use tokio::signal;

// Run the main server routine.
#[tokio::main]
async fn main() {
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Creating the axum app.
    let app: Router = Router::new()
        .layer(cors_layer)
        .route("/routes", get(route_list_handler))
        .route("/", get(main_content_handler))
        .nest_service("/static", ServeDir::new("static"))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    let trips = "https://s3.amazonaws.com/kcm-alerts-realtime-prod/tripupdates.pb"
        .to_string();
    let vehicles= "https://s3.amazonaws.com/kcm-alerts-realtime-prod/tripupdates.pb"
        .to_string();
    let alerts = "https://s3.amazonaws.com/kcm-alerts-realtime-prod/tripupdates.pb"
        .to_string();
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

async fn main_content_handler() -> Html<String> {
    let file_path = "static/index.html".to_string();
    let file_as_string =
        fs::read_to_string(file_path).expect("Should have been able to read src/index.html");
    Html(file_as_string)
}

// This function is a handler for the route_list API request.
async fn route_list_handler() -> Json<String> {
    let file_path = "src/static/pittsburgh/prt/".to_string();
    let routes = Routes::new(file_path);
    let json = serde_json::to_string(&routes.unwrap()).unwrap();
    Json(json)
}
