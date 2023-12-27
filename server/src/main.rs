use axum::{response::Json, routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use glob::glob;
use serde::Serialize;

#[derive(Serialize)]
struct Video {
    path: String
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_stream_to_file=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/", get(show_form));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Handler that returns HTML for a multipart form.
async fn show_form() -> Json<Vec<Video>> {
    Json(
        match glob("**/*.mp4") {
            Ok(s) => s.map(|r| match r {
                Ok(p) => p.to_string_lossy().to_string(),
                Err(_) => String::new(),
            }).map(|v| Video{path: v}).collect(),
            Err(_) => vec![Video{path: String::new()}],
        }
    )
}