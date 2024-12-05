use axum::Router;

mod day_minusone;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().merge(day_minusone::router());

    Ok(router.into())
}
