use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{error::AppError, service::auth::decode_access_token, AppState};

/// Extractor that validates the Bearer JWT and yields the authenticated user ID.
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: Uuid,
}

#[axum::async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok());

        let token = auth_header
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or_else(|| AppError::Unauthorized.into_response())?;

        let claims = decode_access_token(token, &state.cfg.jwt_secret)
            .map_err(|e| e.into_response())?;

        Ok(AuthUser {
            user_id: claims.sub,
        })
    }
}
