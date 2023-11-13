use axum::{extract::{Path, Query}, Json};

use serde::{Serialize, Deserialize};

pub async fn root() -> &'static str {
    "Hello, World!!!!"
}

pub async fn path_variables(Path(id): Path<i32>) -> String {
    id.to_string()
}

#[derive(Serialize, Deserialize)]
pub struct QueryParams {
    id: i32,
    name: String,
    message: String,
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}
