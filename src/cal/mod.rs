use axum::Router;

mod day00;

pub(crate) fn router() -> Router {
    Router::new().nest("/-1", day00::router())
}
