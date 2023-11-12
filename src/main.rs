mod handlers;

use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router, Server,
};
use handlers::{
    get::{path_variables, root},
    post::{mirror_body_json, mirror_body_string},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/path_variables/:id", get(path_variables))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
