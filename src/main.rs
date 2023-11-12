mod handlers;

use std::net::SocketAddr;

use axum::{routing::{get, post}, Router, Server};
use handlers::{get::root, post::{mirror_body_string, mirror_body_json}};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json));

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
