use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Message {
    message: String
}

#[tracing::instrument]
pub(crate) async fn health_check() -> Json<Message> {
    Json(Message {
        message: "Hello from backend!".into()
    })
}
