use axum::Json;
use serde::{Serialize, Deserialize};

pub async fn mirror_body_string(body: String) -> String {
    body
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
    Json(MirrorJsonRespose { respose: body.message })
}
