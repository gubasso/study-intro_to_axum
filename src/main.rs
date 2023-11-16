mod handlers;

use std::{net::SocketAddr, env, error::Error};

use axum::{
    routing::{get, post},
    Router, Server, Extension,
};
use handlers::{
    get::{path_variables, root, query_params, get_json, get_one_task},
    post::{mirror_body_json, mirror_body_string, validate_data, create_task},
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await
        .expect("failed to connect to DATABASE_URL");
    let cors = CorsLayer::new().allow_origin(Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/get_json", get(get_json))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/validate_data", post(validate_data))
        .route("/tasks", post(create_task))
        .route("/tasks/:id", get(get_one_task))
        .layer(Extension(pool))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
