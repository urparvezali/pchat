mod db;
mod handlers;
mod types;

use axum::{
    routing::{get, post},
    serve, Router,
};

use db::init;
use handlers::{auth, root};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let tcp = TcpListener::bind("localhost:8000").await.unwrap();
    let client = init().await.unwrap();
    let app = Router::new()
        .route("/", get(root))
        .route("auth", post(auth))
        .with_state(client);
    serve(tcp, app.into_make_service()).await.unwrap();
}
