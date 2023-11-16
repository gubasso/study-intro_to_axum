use axum::{extract::{Path, Query}, Json, Extension, http::StatusCode};

use serde::{Serialize, Deserialize};
use sqlx::{PgPool, prelude::FromRow};

#[derive(Serialize, FromRow)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
}

pub async fn get_one_task(Extension(pool): Extension<PgPool>, Path(id): Path<i32>) -> Result<Json<ResponseTask>, StatusCode> {
    let q = r"
        SELECT id, title, priority, description
        FROM tasks
        WHERE id = $1
    ";
    let task: Option<ResponseTask> = sqlx::query_as(q)
        .bind(id)
        .fetch_optional(&pool)
        .await
        .expect("failed task");

    if let Some(t) = task {
        Ok(Json(t))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

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
