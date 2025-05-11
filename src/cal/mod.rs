use axum::Router;

mod day00;
mod day02;
mod day05;

pub(crate) fn router() -> Router {
    Router::new()
        .nest("/-1", day00::router())
        .nest("/2", day02::router())
        .nest("/5", day05::router())
}
