use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde_json::json;

pub async fn search() -> impl IntoResponse{
    (StatusCode::CREATED, Json(json!({"hello": "world"})))
}