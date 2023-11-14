mod handlers;

use std::net::SocketAddr;

use axum::{
    routing::{get, post},
    Router, Server,
};
use handlers::{
    get::{path_variables, root, query_params, get_json},
    post::{mirror_body_json, mirror_body_string, validate_data},
};
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/get_json", get(get_json))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/validate_data", post(validate_data))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
