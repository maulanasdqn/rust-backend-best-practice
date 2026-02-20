use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use abbp_database::{sea_orm, DbPool};
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadinessResponse {
    pub status: String,
    pub database: String,
}

pub async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

pub async fn readiness_check(State(pool): State<Arc<DbPool>>) -> impl IntoResponse {
    match pool
        .execute(Statement::from_string(DbBackend::Postgres, "SELECT 1"))
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(ReadinessResponse {
                status: "ready".to_string(),
                database: "connected".to_string(),
            }),
        ),
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ReadinessResponse {
                    status: "not ready".to_string(),
                    database: "disconnected".to_string(),
                }),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::response::IntoResponse;

    #[tokio::test]
    async fn test_health_check_returns_healthy() {
        let response = health_check().await.into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let health: HealthResponse = serde_json::from_slice(&body).unwrap();

        assert_eq!(health.status, "healthy");
        assert_eq!(health.version, env!("CARGO_PKG_VERSION"));
    }
}
