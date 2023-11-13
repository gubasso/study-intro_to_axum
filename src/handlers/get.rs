use axum::{extract::{Path, Query}, Json};

use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct JsonResponse {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<JsonResponse> {
    let resp = JsonResponse {
        message: "Hello".to_string(),
        count: 32,
        username: "Josh".to_string(),
    };
    Json(resp)
}


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
