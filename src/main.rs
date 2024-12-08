mod cal;

use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

async fn hello_bird() -> &'static str {
    "Hello, bird!"
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    Ok(Router::new()
        .route("/", get(hello_bird))
        .nest("/", cal::router())
        .layer(TraceLayer::new_for_http())
        .into())
}
