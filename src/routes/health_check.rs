use axum::http::StatusCode;

pub(crate) async fn health_check() -> StatusCode {
    StatusCode::OK
}
