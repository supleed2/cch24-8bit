use axum::{
    body::{to_bytes, Body},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use cargo_manifest::Manifest;
use http_body_util::BodyExt as _;
use itertools::Itertools as _;
use serde::Deserialize;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/manifest", post(manifest))
        .layer(axum::middleware::from_fn(print_request_response))
}

#[derive(Debug, Deserialize)]
struct Metadata {
    orders: Vec<Option<Order>>,
}

#[derive(Debug, Deserialize)]
struct Order {
    item: String,
    quantity: usize,
}

//https://docs.rs/cch24-validator/23.0.0/src/cch24_validator/lib.rs.html#346

async fn manifest(body: Body) -> Result<Response, Response> {
    let bytes = to_bytes(body, usize::MAX)
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid manifest".to_string()).into_response())?;
    println!("[Parsed bytes]");
    let manifest = Manifest::<Metadata>::from_slice_with_metadata(&bytes).map_err(|e| {
        println!("{e:?}");
        StatusCode::NO_CONTENT.into_response()
    })?;
    println!("Manifest: {manifest:?}");
    let package = manifest
        .package
        .ok_or(StatusCode::NO_CONTENT.into_response())?;
    let metadata = package
        .metadata
        .ok_or(StatusCode::NO_CONTENT.into_response())?;
    println!("Metadata: {metadata:?}");
    let manifest = metadata
        .orders
        .into_iter()
        .flatten()
        .map(|order| format!("{}: {}", order.item, order.quantity))
        .join("\n");
    println!("{manifest}");
    Ok(manifest.into_response())
}

async fn print_request_response(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let (parts, body) = req.into_parts();
    println!();
    let bytes = buffer_and_print("request", body).await?;
    let req = axum::extract::Request::from_parts(parts, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = buffer_and_print("response", body).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn buffer_and_print<B>(
    direction: &str,
    body: B,
) -> Result<axum::body::Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = axum::body::Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"),
            ));
        }
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        println!("{direction} body = {body:?}");
    }

    Ok(bytes)
}
