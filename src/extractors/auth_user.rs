use axum::{
    extract::FromRequestParts,
    http::{HeaderValue, request::Parts},
};
use jsonwebtoken::{Algorithm, DecodingKey, TokenData, Validation, decode};

use crate::{
    dto::{AppState, Claim},
    error::AppError,
};

pub struct AuthUser(pub Claim);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token: &HeaderValue = parts
            .headers
            .get("Authorization")
            .ok_or(AppError::JwtError)?;
        let decoding_key: &DecodingKey =
            &DecodingKey::from_secret(&state.config.jwt_secret.as_bytes());
        let validation: &Validation = &Validation::new(Algorithm::HS256);

        let token_data: TokenData<Claim> = decode::<Claim>(token, decoding_key, validation)
            .map_err(|e| {
                tracing::error!("JWT verification failed: {:?}", e);
                AppError::JwtError
            })?;

        Ok(Self(token_data.claims))
    }
}
