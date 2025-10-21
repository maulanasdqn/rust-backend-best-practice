use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use std::sync::Arc;

use crate::infrastructure::{http::dto::MessageResponse, services::JwtService};

#[derive(Debug)]
pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub email: String,
}

#[derive(Clone)]
pub struct AuthMiddlewareState {
    pub jwt_service: JwtService,
}

impl std::fmt::Debug for AuthMiddlewareState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AuthMiddlewareState")
            .field("jwt_service", &self.jwt_service)
            .finish()
    }
}

pub async fn require_auth(
    State(state): State<Arc<AuthMiddlewareState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<MessageResponse>)> {
    let token = extract_token_from_header(&request)?;

    let claims = state.jwt_service.verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new(format!("Invalid token: {e}"))),
        )
    })?;

    if !claims.is_access_token() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new("Invalid token type")),
        ));
    }

    request.extensions_mut().insert(AuthUser {
        user_id: claims.sub,
        email: claims.email,
    });

    Ok(next.run(request).await)
}

pub async fn optional_auth(
    State(state): State<Arc<AuthMiddlewareState>>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Ok(token) = extract_token_from_header(&request) {
        if let Ok(claims) = state.jwt_service.verify_token(token) {
            if claims.is_access_token() {
                request.extensions_mut().insert(AuthUser {
                    user_id: claims.sub,
                    email: claims.email,
                });
            }
        }
    }

    next.run(request).await
}

fn extract_token_from_header(
    request: &Request,
) -> Result<&str, (StatusCode, Json<MessageResponse>)> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(MessageResponse::new("Missing Authorization header")),
            )
        })?;

    let auth_str = auth_header.to_str().map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new("Invalid Authorization header")),
        )
    })?;

    if !auth_str.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new(
                "Authorization header must start with 'Bearer '",
            )),
        ));
    }

    let token = &auth_str[7..];

    if token.is_empty() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new("Token is empty")),
        ));
    }

    Ok(token)
}

impl axum::extract::FromRequestParts<Arc<AuthMiddlewareState>> for AuthUser {
    type Rejection = (StatusCode, Json<MessageResponse>);

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &Arc<AuthMiddlewareState>,
    ) -> Result<Self, Self::Rejection> {
        parts.extensions.get::<Self>().cloned().ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(MessageResponse::new("Unauthorized")),
            )
        })
    }
}

impl Clone for AuthUser {
    fn clone(&self) -> Self {
        Self {
            user_id: self.user_id,
            email: self.email.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_extract_bearer_token() {}
}
