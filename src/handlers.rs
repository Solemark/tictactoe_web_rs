use axum::{body::Body, http::StatusCode, response::Response};
use std::fs::read_to_string;

pub async fn index_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(
            read_to_string("static/index.html").unwrap_or_else(|e| format!("{}", e)),
        ))
        .unwrap_or_default()
}

pub async fn style_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(Body::from(
            read_to_string("static/style.css").unwrap_or_else(|e| format!("{}", e)),
        ))
        .unwrap_or_default()
}

pub async fn script_handler() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/javascript")
        .body(Body::from(
            read_to_string("static/script.js").unwrap_or_else(|err| format!("{}", err)),
        ))
        .unwrap_or_default()
}
