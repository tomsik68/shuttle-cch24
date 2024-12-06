use axum::Router;

mod day_minusone;
mod day_two;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .merge(day_minusone::router())
        .merge(day_two::router());

    Ok(router.into())
}
