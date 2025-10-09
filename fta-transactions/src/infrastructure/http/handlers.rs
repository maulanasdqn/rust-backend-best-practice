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

use crate::application::{
    CreateTransaction, DeleteTransaction, GetTransaction, ListTransactions, UpdateTransaction,
};

use super::dto::{CreateTransactionRequest, TransactionResponse, UpdateTransactionRequest};

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
  path = "/api/v1/transactions/{id}",
  tag = "Transactions",
  params(
    ("id" = Uuid, Path, description = "Transaction ID")
  ),
  responses(
    (status = 200, description = "Transaction found successfully", body = inline(SingleResponse<TransactionResponse>)),
    (status = 404, description = "Transaction not found", body = ErrorResponse,
     example = json!({"message": "Transaction not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn get_transaction_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<GetTransaction>>,
) -> Result<Json<SingleResponse<TransactionResponse>>, AppError> {
    let transaction = use_case.execute(id).await?;
    let response = SingleResponse::with_message(
        "Transaction retrieved successfully",
        TransactionResponse::from(transaction),
    );
    Ok(Json(response))
}

#[utoipa::path(
  get,
  path = "/api/v1/accounts/{account_id}/transactions",
  tag = "Transactions",
  params(
    ("account_id" = Uuid, Path, description = "Account ID"),
    PaginationQuery
  ),
  responses(
    (status = 200, description = "List of transactions retrieved successfully", body = inline(ListResponse<TransactionResponse>)),
  )
)]
pub async fn list_transactions_handler(
    Path(account_id): Path<Uuid>,
    Query(pagination): Query<PaginationQuery>,
    Extension(use_case): Extension<Arc<ListTransactions>>,
) -> Result<Json<ListResponse<TransactionResponse>>, AppError> {
    let params = PaginationParams::new(pagination.page, pagination.per_page);
    let (transactions, total) = use_case.execute(account_id, &params).await?;
    let responses: Vec<TransactionResponse> = transactions
        .into_iter()
        .map(TransactionResponse::from)
        .collect();

    #[allow(clippy::cast_sign_loss)]
    let meta = PaginationMeta::new(params.page, params.per_page, total.max(0) as u64);
    let response = ListResponse::new(responses, meta);
    Ok(Json(response))
}

#[utoipa::path(
  post,
  path = "/api/v1/accounts/{account_id}/transactions",
  tag = "Transactions",
  params(
    ("account_id" = Uuid, Path, description = "Account ID")
  ),
  request_body = CreateTransactionRequest,
  responses(
    (status = 201, description = "Transaction created successfully", body = inline(SingleResponse<TransactionResponse>)),
    (status = 400, description = "Invalid input", body = ErrorResponse,
     example = json!({"message": "Invalid input", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn create_transaction_handler(
    Path(_account_id): Path<Uuid>,
    Extension(use_case): Extension<Arc<CreateTransaction>>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<Json<SingleResponse<TransactionResponse>>, AppError> {
    let transaction = use_case
        .execute(
            payload.account_id,
            payload.transaction_type,
            payload.amount,
            payload.category,
            payload.description,
            payload.transaction_date,
        )
        .await?;
    let response = SingleResponse::with_message(
        "Transaction created successfully",
        TransactionResponse::from(transaction),
    );
    Ok(Json(response))
}

#[utoipa::path(
  put,
  path = "/api/v1/transactions/{id}",
  tag = "Transactions",
  params(
    ("id" = Uuid, Path, description = "Transaction ID")
  ),
  request_body = UpdateTransactionRequest,
  responses(
    (status = 200, description = "Transaction updated successfully", body = inline(SingleResponse<TransactionResponse>)),
    (status = 404, description = "Transaction not found", body = ErrorResponse,
     example = json!({"message": "Transaction not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn update_transaction_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<UpdateTransaction>>,
    Json(payload): Json<UpdateTransactionRequest>,
) -> Result<Json<SingleResponse<TransactionResponse>>, AppError> {
    let transaction = use_case
        .execute(id, payload.category, payload.description)
        .await?;
    let response = SingleResponse::with_message(
        "Transaction updated successfully",
        TransactionResponse::from(transaction),
    );
    Ok(Json(response))
}

#[utoipa::path(
  delete,
  path = "/api/v1/transactions/{id}",
  tag = "Transactions",
  params(
    ("id" = Uuid, Path, description = "Transaction ID")
  ),
  responses(
    (status = 204, description = "Transaction deleted successfully"),
    (status = 404, description = "Transaction not found", body = ErrorResponse,
     example = json!({"message": "Transaction not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn delete_transaction_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<DeleteTransaction>>,
) -> Result<Json<SingleResponse<()>>, AppError> {
    use_case.execute(id).await?;
    let response = SingleResponse::with_message("Transaction deleted successfully", ());
    Ok(Json(response))
}
