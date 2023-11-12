mod handlers;

use std::net::SocketAddr;

use axum::{routing::get, Router, Server};
use handlers::get::root;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127,0,0,1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
