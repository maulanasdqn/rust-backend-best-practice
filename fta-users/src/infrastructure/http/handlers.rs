use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use fta_errors::AppError;
use fta_types::{ErrorResponse, ListResponse, PaginationMeta, SingleResponse};
use paginator_rs::PaginationParams;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::application::{CreateUser, DeleteUser, GetUser, ListUsers, UpdateUser};

use super::dto::{CreateUserRequest, UpdateUserRequest, UserResponse};

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct PaginationQuery {
    #[serde(default = "default_page")]
    pub page: u32,
    #[serde(default = "default_per_page")]
    pub per_page: u32,
}

const fn default_page() -> u32 {
    1
}

const fn default_per_page() -> u32 {
    10
}

#[utoipa::path(
  get,
  path = "/api/v1/users/{id}",
  tag = "Users",
  params(
    ("id" = Uuid, Path, description = "User ID")
  ),
  responses(
    (status = 200, description = "User found successfully", body = inline(SingleResponse<UserResponse>)),
    (status = 404, description = "User not found", body = ErrorResponse,
     example = json!({"message": "User not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn get_user_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<GetUser>>,
) -> Result<Json<SingleResponse<UserResponse>>, AppError> {
    let user = use_case.execute(id).await?;
    let response =
        SingleResponse::with_message("User retrieved successfully", UserResponse::from(user));
    Ok(Json(response))
}

#[utoipa::path(
  get,
  path = "/api/v1/users",
  tag = "Users",
  params(
    PaginationQuery
  ),
  responses(
    (status = 200, description = "List of users retrieved successfully", body = inline(ListResponse<UserResponse>)),
  )
)]
pub async fn list_users_handler(
    Query(pagination): Query<PaginationQuery>,
    Extension(use_case): Extension<Arc<ListUsers>>,
) -> Result<Json<ListResponse<UserResponse>>, AppError> {
    let params = PaginationParams::new(pagination.page, pagination.per_page);
    let (users, total) = use_case.execute(&params).await?;
    let responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    let total_u64 = u64::try_from(total.max(0)).unwrap_or(0);
    let meta = PaginationMeta::new(params.page, params.per_page, total_u64);
    let response = ListResponse::new(responses, meta);

    Ok(Json(response))
}

#[utoipa::path(
  post,
  path = "/api/v1/users",
  tag = "Users",
  request_body = CreateUserRequest,
  responses(
    (status = 201, description = "User created successfully", body = inline(SingleResponse<UserResponse>)),
    (status = 400, description = "Invalid input", body = ErrorResponse,
     example = json!({"message": "Invalid input", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn create_user_handler(
    Extension(use_case): Extension<Arc<CreateUser>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<SingleResponse<UserResponse>>, AppError> {
    let user = use_case
        .execute(
            payload.email,
            payload.password,
            payload.first_name,
            payload.last_name,
        )
        .await?;
    let response =
        SingleResponse::with_message("User created successfully", UserResponse::from(user));
    Ok(Json(response))
}

#[utoipa::path(
  put,
  path = "/api/v1/users/{id}",
  tag = "Users",
  params(
    ("id" = Uuid, Path, description = "User ID")
  ),
  request_body = UpdateUserRequest,
  responses(
    (status = 200, description = "User updated successfully", body = inline(SingleResponse<UserResponse>)),
    (status = 404, description = "User not found", body = ErrorResponse,
     example = json!({"message": "User not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn update_user_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<UpdateUser>>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<SingleResponse<UserResponse>>, AppError> {
    let user = use_case
        .execute(id, payload.first_name, payload.last_name)
        .await?;
    let response =
        SingleResponse::with_message("User updated successfully", UserResponse::from(user));
    Ok(Json(response))
}

#[utoipa::path(
  delete,
  path = "/api/v1/users/{id}",
  tag = "Users",
  params(
    ("id" = Uuid, Path, description = "User ID")
  ),
  responses(
    (status = 204, description = "User deleted successfully"),
    (status = 404, description = "User not found", body = ErrorResponse,
     example = json!({"message": "User not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn delete_user_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<DeleteUser>>,
) -> Result<Json<SingleResponse<()>>, AppError> {
    use_case.execute(id).await?;
    let response = SingleResponse::with_message("User deleted successfully", ());
    Ok(Json(response))
}
