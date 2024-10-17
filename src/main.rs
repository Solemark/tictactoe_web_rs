mod routes;

#[tokio::main]
async fn main() {
    routes::routing().await;
}
