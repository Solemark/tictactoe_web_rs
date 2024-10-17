mod handlers;
mod routes;

use axum::serve;
use routes::routing;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    serve(
        TcpListener::bind("localhost:8080")
            .await
            .expect("Error in TcpListener"),
        routing().await,
    )
    .await
    .unwrap_or_default();
}
