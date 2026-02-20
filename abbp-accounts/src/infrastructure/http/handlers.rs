use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use abbp_errors::AppError;
use abbp_types::{ErrorResponse, ListResponse, PaginationMeta, SingleResponse};
use std::sync::Arc;
use uuid::Uuid;

use crate::application::{
    CreateAccount, DeactivateAccount, DeleteAccount, GetAccount, ListAccounts, UpdateAccount,
};

use super::dto::{AccountResponse, CreateAccountRequest, UpdateAccountRequest};
use super::query::{ListAccountsQuery, ALLOWED_SORT_FIELDS};

#[utoipa::path(
  get,
  path = "/api/v1/accounts/{id}",
  tag = "Accounts",
  params(("id" = Uuid, Path, description = "Account ID")),
  responses(
    (status = 200, description = "Account found successfully", body = inline(SingleResponse<AccountResponse>)),
    (status = 404, description = "Account not found", body = ErrorResponse,
     example = json!({"message": "Account not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn get_account_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<GetAccount>>,
) -> Result<Json<SingleResponse<AccountResponse>>, AppError> {
    let account = use_case.execute(id).await?;
    let response = SingleResponse::with_message(
        "Account retrieved successfully",
        AccountResponse::from(account),
    );
    Ok(Json(response))
}

#[utoipa::path(
  get,
  path = "/api/v1/accounts",
  tag = "Accounts",
  params(ListAccountsQuery),
  responses(
    (status = 200, description = "List of accounts retrieved successfully", body = inline(ListResponse<AccountResponse>)),
    (status = 400, description = "Invalid query parameters", body = ErrorResponse),
  )
)]
pub async fn list_accounts_handler(
    Query(query): Query<ListAccountsQuery>,
    Extension(use_case): Extension<Arc<ListAccounts>>,
) -> Result<Json<ListResponse<AccountResponse>>, AppError> {
    query.filters.validate().map_err(AppError::BadRequest)?;
    query
        .sort
        .validate(ALLOWED_SORT_FIELDS)
        .map_err(AppError::BadRequest)?;

    let pagination = query.pagination.validate();
    let (accounts, total) = use_case
        .execute(
            &query.filters,
            query.sort.sort_by.as_deref(),
            &query.sort.order.to_string(),
            &pagination,
        )
        .await?;

    let responses: Vec<AccountResponse> = accounts.into_iter().map(AccountResponse::from).collect();
    let total_u64 = u64::try_from(total.max(0)).unwrap_or(0);
    let meta = PaginationMeta::new(pagination.page, pagination.per_page, total_u64);

    Ok(Json(ListResponse::new(responses, meta)))
}

#[utoipa::path(
  post,
  path = "/api/v1/users/{user_id}/accounts",
  tag = "Accounts",
  params(("user_id" = Uuid, Path, description = "User ID")),
  request_body = CreateAccountRequest,
  responses(
    (status = 201, description = "Account created successfully", body = inline(SingleResponse<AccountResponse>)),
    (status = 400, description = "Invalid input", body = ErrorResponse,
     example = json!({"message": "Invalid input", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn create_account_handler(
    Path(user_id): Path<Uuid>,
    Extension(use_case): Extension<Arc<CreateAccount>>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<Json<SingleResponse<AccountResponse>>, AppError> {
    let account = use_case
        .execute(
            user_id,
            payload.name,
            payload.account_type,
            payload.initial_balance,
            payload.currency,
        )
        .await?;
    let response = SingleResponse::with_message(
        "Account created successfully",
        AccountResponse::from(account),
    );
    Ok(Json(response))
}

#[utoipa::path(
  put,
  path = "/api/v1/accounts/{id}",
  tag = "Accounts",
  params(("id" = Uuid, Path, description = "Account ID")),
  request_body = UpdateAccountRequest,
  responses(
    (status = 200, description = "Account updated successfully", body = inline(SingleResponse<AccountResponse>)),
    (status = 404, description = "Account not found", body = ErrorResponse,
     example = json!({"message": "Account not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn update_account_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<UpdateAccount>>,
    Json(payload): Json<UpdateAccountRequest>,
) -> Result<Json<SingleResponse<AccountResponse>>, AppError> {
    let account = use_case.execute(id, payload.name).await?;
    let response = SingleResponse::with_message(
        "Account updated successfully",
        AccountResponse::from(account),
    );
    Ok(Json(response))
}

#[utoipa::path(
  patch,
  path = "/api/v1/accounts/{id}/deactivate",
  tag = "Accounts",
  params(("id" = Uuid, Path, description = "Account ID")),
  responses(
    (status = 200, description = "Account deactivated successfully", body = inline(SingleResponse<AccountResponse>)),
    (status = 404, description = "Account not found", body = ErrorResponse,
     example = json!({"message": "Account not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn deactivate_account_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<DeactivateAccount>>,
) -> Result<Json<SingleResponse<AccountResponse>>, AppError> {
    let account = use_case.execute(id).await?;
    let response = SingleResponse::with_message(
        "Account deactivated successfully",
        AccountResponse::from(account),
    );
    Ok(Json(response))
}

#[utoipa::path(
  delete,
  path = "/api/v1/accounts/{id}",
  tag = "Accounts",
  params(("id" = Uuid, Path, description = "Account ID")),
  responses(
    (status = 204, description = "Account deleted successfully"),
    (status = 404, description = "Account not found", body = ErrorResponse,
     example = json!({"message": "Account not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn delete_account_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<DeleteAccount>>,
) -> Result<Json<SingleResponse<()>>, AppError> {
    use_case.execute(id).await?;
    Ok(Json(SingleResponse::with_message(
        "Account deleted successfully",
        (),
    )))
}
