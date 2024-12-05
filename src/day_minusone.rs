use axum::{
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};

async fn hello_bird() -> &'static str {
    "Hello, bird!"
}

async fn seek() -> impl IntoResponse {
    (
        StatusCode::FOUND,
        Redirect::to("https://www.youtube.com/watch?v=9Gc4QTqslN4"),
    )
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(hello_bird))
        .route("/-1/seek", get(seek))
}
