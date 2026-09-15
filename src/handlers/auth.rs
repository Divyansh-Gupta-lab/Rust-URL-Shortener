use axum::{Json, extract::State};

use crate::{
    dto::{AppState, Login, Register},
    error::AppError,
    models::User,
    services::user,
};

pub async fn register(
    State(state): State<AppState>,
    Json(body): Json<Register>,
) -> Result<Json<User>, AppError> {
    let response: User = user::register(state.pool, body).await?;
    Ok(Json(response))
}

pub async fn register_admin(
    State(state): State<AppState>,
    Json(body): Json<Register>,
) -> Result<Json<User>, AppError> {
    let response: User = user::register_admin(state.pool, body).await?;
    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(body): Json<Login>,
) -> Result<Json<String>, AppError> {
    let response: String = user::login(state.pool, state.config, body).await?;
    Ok(Json(response))
}
