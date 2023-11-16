use axum::{Json, Extension};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, query};

#[derive(Deserialize, Debug)]
pub struct NewTask {
    priority: String,
    title: String,
    description: Option<String>,
    is_default: bool,
}

pub async fn create_task(Extension(pool): Extension<PgPool>, Json(new_task): Json<NewTask>) {
    dbg!(&new_task);
    let q = "INSERT INTO tasks (priority, title, description, is_default) VALUES ($1, $2, $3, $4)";
    query(q)
        .bind(&new_task.priority)
        .bind(&new_task.title)
        .bind(&new_task.description)
        .bind(new_task.is_default)
        .execute(&pool)
        .await.expect("problem with insert execution");
}

pub async fn mirror_body_string(body: String) -> String {
    body
}

#[derive(Deserialize, Debug)]
pub struct ValDataLogin {
    username: String,
    password: String,
    detail: Option<String>,
}

pub async fn validate_data(Json(login_data): Json<ValDataLogin>) {
    dbg!(login_data);
}

#[derive(Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize, Debug)]
pub struct MirrorJsonRespose {
    respose: String,
}

pub async fn mirror_body_json(Json(mut body): Json<MirrorJson>) -> Json<MirrorJsonRespose> {
    body.message.push_str(". Now a reponse.");
    Json(MirrorJsonRespose {
        respose: body.message,
    })
}
