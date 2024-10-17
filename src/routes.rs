use crate::handlers::{index_handler, script_handler, style_handler};
use axum::{routing::get, Router};

pub async fn routing() -> Router {
    Router::new()
        .route("/", get(index_handler))
        .route("/style", get(style_handler))
        .route("/script", get(script_handler))
}
