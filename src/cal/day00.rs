use axum::{
    http::{header, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};

pub(crate) fn router() -> Router {
    Router::new().route("/seek", get(seek))
}

async fn seek() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )],
    )
}
