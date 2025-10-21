use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use fta_errors::AppError;
use fta_types::{
    ErrorResponse, ListResponse, PaginationMeta, PaginationQuery, SingleResponse, SortQuery,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::IntoParams;
use uuid::Uuid;

use crate::application::{
    CreateTransaction, DeleteTransaction, GetTransaction, ListTransactions, UpdateTransaction,
};

use super::dto::{CreateTransactionRequest, TransactionResponse, UpdateTransactionRequest};
use super::filters::TransactionFilters;

#[derive(Debug, Deserialize, Serialize, IntoParams)]
pub struct ListTransactionsQuery {
    #[serde(flatten)]
    pub pagination: PaginationQuery,

    #[serde(flatten)]
    pub sort: SortQuery,

    #[serde(flatten)]
    pub filters: TransactionFilters,
}

const ALLOWED_SORT_FIELDS: &[&str] = &[
    "amount",
    "transaction_date",
    "category",
    "created_at",
    "updated_at",
];

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
  path = "/api/v1/transactions",
  tag = "Transactions",
  params(
    ListTransactionsQuery
  ),
  responses(
    (status = 200, description = "List of transactions retrieved successfully", body = inline(ListResponse<TransactionResponse>)),
    (status = 400, description = "Invalid query parameters", body = ErrorResponse),
  )
)]
pub async fn list_transactions_handler(
    Query(query): Query<ListTransactionsQuery>,
    Extension(use_case): Extension<Arc<ListTransactions>>,
) -> Result<Json<ListResponse<TransactionResponse>>, AppError> {
    query.filters.validate().map_err(AppError::BadRequest)?;

    query
        .sort
        .validate(ALLOWED_SORT_FIELDS)
        .map_err(AppError::BadRequest)?;

    let pagination = query.pagination.validate();

    let (transactions, total) = use_case
        .execute(
            &query.filters,
            query.sort.sort_by.as_deref(),
            &query.sort.order.to_string(),
            &pagination,
        )
        .await?;

    let responses: Vec<TransactionResponse> = transactions
        .into_iter()
        .map(TransactionResponse::from)
        .collect();

    let total_u64 = u64::try_from(total.max(0)).unwrap_or(0);
    let meta = PaginationMeta::new(pagination.page, pagination.per_page, total_u64);
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
