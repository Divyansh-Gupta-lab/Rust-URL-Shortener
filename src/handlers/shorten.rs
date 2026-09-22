use axum::{
    Json,
    extract::{Path, State},
    response::Redirect,
};

use crate::{
    dto::{AppState, ShortenUrl}, error::AppError, extractors::AuthUser, services::{redirect_url, shorten_url},
};

pub async fn create_short_url(
    AuthUser(_): AuthUser,
    State(state): State<AppState>,
    Json(body): Json<ShortenUrl>,
) -> Result<Json<ShortenUrl>, AppError> {
    let response: ShortenUrl = shorten_url(state.pool, body).await?;
    Ok(Json(response))
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<Redirect, AppError> {
    let uri: String = redirect_url(state.pool, code).await?;
    return Ok(Redirect::temporary(&uri));
}
