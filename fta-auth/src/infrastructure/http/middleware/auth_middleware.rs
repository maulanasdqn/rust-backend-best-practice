use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use std::sync::Arc;

use crate::infrastructure::{http::dto::MessageResponse, services::JwtService};

/// Extension key for authenticated user ID
#[derive(Debug)]
pub struct AuthUser {
    pub user_id: uuid::Uuid,
    pub email: String,
}

/// Middleware state containing JWT service
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

/// Middleware that requires authentication
///
/// Extracts JWT from Authorization header, verifies it, and adds user info to request extensions
pub async fn require_auth(
    State(state): State<Arc<AuthMiddlewareState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<MessageResponse>)> {
    // Extract token from Authorization header
    let token = extract_token_from_header(&request)?;

    // Verify token
    let claims = state.jwt_service.verify_token(token).map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new(format!("Invalid token: {e}"))),
        )
    })?;

    // Ensure it's an access token
    if !claims.is_access_token() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new("Invalid token type")),
        ));
    }

    // Add user info to request extensions
    request.extensions_mut().insert(AuthUser {
        user_id: claims.sub,
        email: claims.email,
    });

    Ok(next.run(request).await)
}

/// Middleware that optionally extracts auth info but doesn't fail if missing
pub async fn optional_auth(
    State(state): State<Arc<AuthMiddlewareState>>,
    mut request: Request,
    next: Next,
) -> Response {
    // Try to extract token
    if let Ok(token) = extract_token_from_header(&request) {
        // Try to verify token
        if let Ok(claims) = state.jwt_service.verify_token(token) {
            if claims.is_access_token() {
                // Add user info to request extensions
                request.extensions_mut().insert(AuthUser {
                    user_id: claims.sub,
                    email: claims.email,
                });
            }
        }
    }

    next.run(request).await
}

/// Helper function to extract token from Authorization header
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

    // Check for Bearer token
    if !auth_str.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new(
                "Authorization header must start with 'Bearer '",
            )),
        ));
    }

    let token = &auth_str[7..]; // Skip "Bearer "

    if token.is_empty() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(MessageResponse::new("Token is empty")),
        ));
    }

    Ok(token)
}

/// Axum extractor for authenticated user
///
/// # Example
/// ```
/// use axum::extract::Extension;
///
/// async fn protected_handler(
///     Extension(auth_user): Extension<AuthUser>,
/// ) -> String {
///     format!("Hello, user {}!", auth_user.user_id)
/// }
/// ```
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

// Implement Clone for AuthUser
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
    fn test_extract_bearer_token() {
        // This would require creating a full Request object
        // In practice, you'd use integration tests
    }
}
