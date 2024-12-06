use axum::{extract::Query, routing::get, Router};
use serde::Deserialize;
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Deserialize)]
struct EeQuery {
    from: Ipv4Addr,
    key: Ipv4Addr,
}

async fn egregious_encryption(Query(q): Query<EeQuery>) -> String {
    let octets = q
        .from
        .octets()
        .into_iter()
        .zip(q.key.octets().into_iter())
        .map(|(o1, o2)| o1.overflowing_add(o2).0)
        .collect::<Vec<u8>>();
    let octets: [u8; 4] = octets.try_into().unwrap();
    Ipv4Addr::from(octets).to_string()
}

#[derive(Deserialize)]
struct ReverseEeQuery {
    from: Ipv4Addr,
    to: Ipv4Addr,
}

async fn egregious_decryption(Query(q): Query<ReverseEeQuery>) -> String {
    let octets =
        q.to.octets()
            .into_iter()
            .zip(q.from.octets().into_iter())
            .map(|(o1, o2)| o1.overflowing_sub(o2).0)
            .collect::<Vec<u8>>();
    let octets: [u8; 4] = octets.try_into().unwrap();
    Ipv4Addr::from(octets).to_string()
}

#[derive(Deserialize)]
struct EeQueryV6 {
    from: Ipv6Addr,
    key: Ipv6Addr,
}

async fn egregious_encryption_v6(Query(q): Query<EeQueryV6>) -> String {
    let from = q.from.to_bits();
    let key = q.key.to_bits();

    Ipv6Addr::from_bits(from ^ key).to_string()
}

#[derive(Deserialize)]
struct ReverseEeQueryV6 {
    from: Ipv6Addr,
    to: Ipv6Addr,
}

async fn egregious_decryption_v6(Query(q): Query<ReverseEeQueryV6>) -> String {
    egregious_encryption_v6(Query(EeQueryV6 {
        from: q.from,
        key: q.to,
    }))
    .await
}

pub fn router() -> Router {
    Router::new().nest(
        "/2",
        Router::new()
            .route("/dest", get(egregious_encryption))
            .route("/key", get(egregious_decryption))
            .route("/v6/dest", get(egregious_encryption_v6))
            .route("/v6/key", get(egregious_decryption_v6)),
    )
}
