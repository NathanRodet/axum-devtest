use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageJson {
    message: String,
}

#[derive(Serialize)]
pub struct MessageJsonResponse {
    message: String,
    message_from_server: String,
}

pub async fn post_json(Json(body): Json<MessageJson>) -> Json<MessageJsonResponse> {
    Json(MessageJsonResponse {
        message: body.message,
        message_from_server: "Hello from Axum".to_owned(),
    })
}