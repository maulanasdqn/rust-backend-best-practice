use chrono::{DateTime, Utc};
use abbp_validation::{prelude::*, ObjectSchema, Validatable};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::domain::User;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateUserRequest {
    pub email: String,
    #[schema(write_only)]
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateUserRequest {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl Validatable for CreateUserRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("email", string().email())
            .field("password", string().min(8).max(128))
            .field("first_name", string().min(1).max(100).optional())
            .field("last_name", string().min(1).max(100).optional())
    }
}

impl Validatable for UpdateUserRequest {
    fn schema() -> ObjectSchema {
        object()
            .field("first_name", string().min(1).max(100).optional())
            .field("last_name", string().min(1).max(100).optional())
    }
}
