use axum::http::StatusCode;

pub async fn health_check() -> StatusCode {
    println!("health_check");
    StatusCode::OK
}