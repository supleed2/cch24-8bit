use std::{
    iter::zip,
    net::{Ipv4Addr, Ipv6Addr},
};

use axum::{extract::Query, response::IntoResponse, routing::get, Router};
use serde::Deserialize;

pub(crate) fn router() -> Router {
    Router::new()
        .route("/dest", get(dest))
        .route("/key", get(key))
        .route("/v6/dest", get(v6dest))
        .route("/v6/key", get(v6key))
}

fn process<U: Sized, const N: usize>(mut l: [U; N], r: [U; N], f: impl Fn(&U, U) -> U) -> [U; N] {
    for (l, r) in zip(&mut l, r) {
        *l = f(l, r);
    }
    l
}

#[derive(Deserialize)]
struct Dest {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

async fn dest(Query(Dest { from, key }): Query<Dest>) -> impl IntoResponse {
    let to: Ipv4Addr = process(from.octets(), key.octets(), |l, r| l.wrapping_add(r)).into();
    to.to_string()
}

#[derive(Deserialize)]
struct Key {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

async fn key(Query(Key { from, to }): Query<Key>) -> impl IntoResponse {
    let key: Ipv4Addr = process(to.octets(), from.octets(), |l, r| l.wrapping_sub(r)).into();
    key.to_string()
}

#[derive(Deserialize)]
struct V6Dest {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

async fn v6dest(Query(V6Dest { from, key }): Query<V6Dest>) -> impl IntoResponse {
    let to: Ipv6Addr = process(from.segments(), key.segments(), |l, r| l ^ r).into();
    to.to_string()
}

#[derive(Deserialize)]
struct V6Key {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

async fn v6key(Query(V6Key { from, to }): Query<V6Key>) -> impl IntoResponse {
    let key: Ipv6Addr = process(to.segments(), from.segments(), |l, r| l ^ r).into();
    key.to_string()
}
