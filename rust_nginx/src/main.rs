use axum::{Router, routing::get};
use std::env;

#[tokio::main]
async fn main() {
    let replica = env::var("APP_NAME").unwrap_or_else(|_| "Unknown".to_string());

    println!("This is my server running on {}", replica);

    let app = Router::new().route(
        "/",
        get({
            let replica_for_handler = replica.clone();
            move || hello(replica_for_handler.clone())
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello(replica: String) -> String {
    format!("Hello, World! Served by {}", replica)
}
