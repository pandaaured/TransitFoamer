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
pub mod schedule;
pub mod structs;

use std::sync::Arc;
use std::time::Duration;

use http::StatusCode;

use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

use axum::{extract::State, routing::get, Router};

use structs::{AppState, Links, StaticInfo};

use tokio::signal;

// Parse command line arguments.
// TODO: Make this formatted more cleanly in the future.
#[tokio::main]
async fn main() {
    println!("Main called.");
    server_routine().await;
}

async fn server_routine() {
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    println!("Loading static data...");
    let static_info = tokio::task::spawn_blocking(|| {
        Arc::new(StaticInfo::new())
    }).await.unwrap();
    println!("Static data loaded! Starting server...");

    let state = AppState {
        static_info,
        links: Arc::new(Links::new()),
    };
    let app: Router = Router::new()
        .layer(cors_layer)
        .route("/rtlist", get(handlers::route_list_handler))
        .route("/routes/{route_id}", get(handlers::route_handler))
        .route("/schedule/{route_id}", get(handlers::schedule_handler))
        .with_state(state)
        .nest_service("/dist", ServeDir::new("dist")
            .append_index_html_on_directories(true))
        .nest_service("/assets", ServeDir::new("dist/assets"))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ));
    println!("Router initialized with appropriate endpoints exposed.");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("TCP Listener bound to the address 127.0.0.1:8080.");

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
    println!("Axum app served.");
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
