use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use abbp_errors::AppError;
use abbp_types::{
    ErrorResponse, ListResponse, PaginationMeta, PaginationQuery, SingleResponse, SortQuery,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::application::{CreateUser, DeleteUser, GetUser, ListUsers, UpdateUser};

use super::dto::{CreateUserRequest, UpdateUserRequest, UserResponse};
use super::filters::UserFilters;

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct ListUsersQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    #[serde(flatten)]
    pub sort: SortQuery,

    #[serde(flatten)]
    pub filters: UserFilters,
}

const ALLOWED_SORT_FIELDS: &[&str] = &[
    "email",
    "first_name",
    "last_name",
    "created_at",
    "updated_at",
];

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
    ListUsersQuery
  ),
  responses(
    (status = 200, description = "List of users retrieved successfully", body = inline(ListResponse<UserResponse>)),
    (status = 400, description = "Invalid query parameters", body = ErrorResponse),
  )
)]
pub async fn list_users_handler(
    Query(query): Query<ListUsersQuery>,
    Extension(use_case): Extension<Arc<ListUsers>>,
) -> Result<Json<ListResponse<UserResponse>>, AppError> {
    query.filters.validate().map_err(AppError::BadRequest)?;

    query
        .sort
        .validate(ALLOWED_SORT_FIELDS)
        .map_err(AppError::BadRequest)?;

    let pagination = query.pagination.validate();

    let (users, total) = use_case
        .execute(
            &query.filters,
            query.sort.sort_by.as_deref(),
            &query.sort.order.to_string(),
            &pagination,
        )
        .await?;

    let responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();

    let total_u64 = u64::try_from(total.max(0)).unwrap_or(0);
    let meta = PaginationMeta::new(pagination.page, pagination.per_page, total_u64);
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
