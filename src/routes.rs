use axum::{body::Body, http::StatusCode, response::Response, routing::get, Router};
use std::fs::read_to_string;

pub async fn routing() {
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/style", get(style_handler))
        .route("/script", get(script_handler));

    let listener = tokio::net::TcpListener::bind("localhost:8080")
        .await
        .expect("Error in TcpListener");
    axum::serve(listener, app).await.unwrap_or_default();
}

async fn index_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(
            read_to_string("static/index.html").unwrap_or_else(|e| format!("{}", e)),
        ))
        .unwrap_or_default()
}

async fn style_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(Body::from(
            read_to_string("static/style.css").unwrap_or_else(|e| format!("{}", e)),
        ))
        .unwrap_or_default()
}

async fn script_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(Body::from(
            read_to_string("static/script.js").unwrap_or_else(|err| format!("{}", err)),
        ))
        .unwrap_or_default()
}
