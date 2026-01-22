// ==================================================
// Minimal Rust API Example (Axum) – JSON API Style
// ==================================================
// This example satisfies the "Rust Application Requirements"
// documented in the README, with **standard JSON responses**:
//
// - Reads configuration from environment variables
// - Stdout-first logging
// - Graceful shutdown handling
// - Health endpoint
// - Minimal HTTP endpoints:
//     GET /      -> JSON Hello World
//     GET /api   -> JSON Hello API
//     GET /health -> JSON health status
//
// This code is intentionally simple, explicit, and production-safe.

use axum::{
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::{
    env,
    net::SocketAddr,
    time::Duration,
};
use tokio::{net::TcpListener, signal};
use tracing::{info, warn};

// --------------------------------------------------
// Response Models
// --------------------------------------------------

#[derive(Serialize)]
struct ApiResponse<T> {
    status: &'static str,
    message: &'static str,
    data: T,
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

// --------------------------------------------------
// Application entrypoint
// --------------------------------------------------

#[tokio::main]
async fn main() {
    // --------------------------------------------------
    // Init logging FIRST (critical for Docker)
    // --------------------------------------------------
    tracing_subscriber::fmt::init();

    info!("Booting application");

    // --------------------------------------------------
    // Load environment variables (fail-fast but logged)
    // --------------------------------------------------

    let database_url = match env::var("DATABASE_URL") {
        Ok(v) => v,
        Err(_) => {
            warn!("DATABASE_URL is not set");
            std::process::exit(1);
        }
    };

    info!("DATABASE_URL loaded (value hidden)");

    let app_port: u16 = env::var("APP_PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .unwrap_or(8080);

    let shutdown_timeout: u64 = env::var("GRACEFUL_SHUTDOWN_TIMEOUT")
        .unwrap_or_else(|_| "10".into())
        .parse()
        .unwrap_or(10);

    // --------------------------------------------------
    // Build router
    // --------------------------------------------------

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/api", get(api_handler))
        .route("/health", get(health_handler));

    // --------------------------------------------------
    // Start HTTP server
    // --------------------------------------------------

    let addr = SocketAddr::from(([0, 0, 0, 0], app_port));
    info!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    if let Err(err) = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(shutdown_timeout))
        .await
    {
        warn!("Server terminated: {}", err);
    }

    info!("Server exited cleanly");
}

// --------------------------------------------------
// HTTP Handlers
// --------------------------------------------------

async fn root_handler() -> impl IntoResponse {
    Json(ApiResponse {
        status: "success",
        message: "Hello World",
        data: (),
    })
}

async fn api_handler() -> impl IntoResponse {
    Json(ApiResponse {
        status: "success",
        message: "Hello API",
        data: (),
    })
}

/// Healthcheck endpoint
/// Must be FAST and ALWAYS return 200
async fn health_handler() -> impl IntoResponse {
    Json(HealthResponse { status: "ok" })
}

// --------------------------------------------------
// Graceful shutdown
// --------------------------------------------------

async fn shutdown_signal(timeout_secs: u64) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm =
            signal(SignalKind::terminate()).expect("Failed to install SIGTERM handler");
        sigterm.recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown signal received, draining for {}s", timeout_secs);
    tokio::time::sleep(Duration::from_secs(timeout_secs)).await;
}
