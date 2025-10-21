use super::dto::{BudgetResponse, CreateBudgetRequest, UpdateBudgetRequest};
use crate::application::{
    CreateBudget, DeactivateBudget, DeleteBudget, GetBudget, ListBudgets, UpdateBudget,
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use chrono::Utc;
use fta_errors::AppError;
use fta_types::{ErrorResponse, ListResponse, PaginationMeta, SingleResponse};
use paginator_rs::PaginationParams;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use utoipa::IntoParams;
use uuid::Uuid;

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
    20
}

#[utoipa::path(
  get,
  path = "/api/v1/budgets/{id}",
  tag = "Budgets",
  params(
    ("id" = Uuid, Path, description = "Budget ID")
  ),
  responses(
    (status = 200, description = "Budget found successfully", body = inline(SingleResponse<BudgetResponse>)),
    (status = 404, description = "Budget not found", body = ErrorResponse,
     example = json!({"message": "Budget not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn get_budget_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<GetBudget>>,
) -> Result<Json<SingleResponse<BudgetResponse>>, AppError> {
    let budget = use_case.execute(id).await?;
    let response = SingleResponse::with_message(
        "Budget retrieved successfully",
        BudgetResponse::from(budget),
    );
    Ok(Json(response))
}

#[utoipa::path(
  get,
  path = "/api/v1/users/{user_id}/budgets",
  tag = "Budgets",
  params(
    ("user_id" = Uuid, Path, description = "User ID"),
    PaginationQuery
  ),
  responses(
    (status = 200, description = "List of budgets retrieved successfully", body = inline(ListResponse<BudgetResponse>)),
  )
)]
pub async fn list_budgets_handler(
    Path(user_id): Path<Uuid>,
    Query(pagination): Query<PaginationQuery>,
    Extension(use_case): Extension<Arc<ListBudgets>>,
) -> Result<Json<ListResponse<BudgetResponse>>, AppError> {
    let params = PaginationParams::new(pagination.page, pagination.per_page);
    let (budgets, total) = use_case.execute(user_id, &params).await?;
    let responses: Vec<BudgetResponse> = budgets.into_iter().map(BudgetResponse::from).collect();

    let total_u64 = u64::try_from(total.max(0)).unwrap_or(0);
    let meta = PaginationMeta::new(params.page, params.per_page, total_u64);
    let response = ListResponse::new(responses, meta);

    Ok(Json(response))
}

#[utoipa::path(
  post,
  path = "/api/v1/users/{user_id}/budgets",
  tag = "Budgets",
  params(
    ("user_id" = Uuid, Path, description = "User ID")
  ),
  request_body = CreateBudgetRequest,
  responses(
    (status = 201, description = "Budget created successfully", body = inline(SingleResponse<BudgetResponse>)),
    (status = 400, description = "Invalid input", body = ErrorResponse,
     example = json!({"message": "Invalid input", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn create_budget_handler(
    Path(user_id): Path<Uuid>,
    Extension(use_case): Extension<Arc<CreateBudget>>,
    Json(payload): Json<CreateBudgetRequest>,
) -> Result<Json<SingleResponse<BudgetResponse>>, AppError> {
    let budget = use_case
        .execute(
            user_id,
            payload.category,
            payload.amount,
            payload.period,
            payload.start_date,
        )
        .await?;
    let response =
        SingleResponse::with_message("Budget created successfully", BudgetResponse::from(budget));
    Ok(Json(response))
}

#[utoipa::path(
  put,
  path = "/api/v1/budgets/{id}",
  tag = "Budgets",
  params(
    ("id" = Uuid, Path, description = "Budget ID")
  ),
  request_body = UpdateBudgetRequest,
  responses(
    (status = 200, description = "Budget updated successfully", body = inline(SingleResponse<BudgetResponse>)),
    (status = 404, description = "Budget not found", body = ErrorResponse,
     example = json!({"message": "Budget not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn update_budget_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<UpdateBudget>>,
    Json(payload): Json<UpdateBudgetRequest>,
) -> Result<Json<SingleResponse<BudgetResponse>>, AppError> {
    let budget = use_case
        .execute(id, payload.category, payload.amount)
        .await?;
    let response =
        SingleResponse::with_message("Budget updated successfully", BudgetResponse::from(budget));
    Ok(Json(response))
}

#[utoipa::path(
  patch,
  path = "/api/v1/budgets/{id}/deactivate",
  tag = "Budgets",
  params(
    ("id" = Uuid, Path, description = "Budget ID")
  ),
  responses(
    (status = 200, description = "Budget deactivated successfully", body = inline(SingleResponse<BudgetResponse>)),
    (status = 404, description = "Budget not found", body = ErrorResponse,
     example = json!({"message": "Budget not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn deactivate_budget_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<DeactivateBudget>>,
) -> Result<Json<SingleResponse<BudgetResponse>>, AppError> {
    let budget = use_case.execute(id, Utc::now()).await?;
    let response = SingleResponse::with_message(
        "Budget deactivated successfully",
        BudgetResponse::from(budget),
    );
    Ok(Json(response))
}

#[utoipa::path(
  delete,
  path = "/api/v1/budgets/{id}",
  tag = "Budgets",
  params(
    ("id" = Uuid, Path, description = "Budget ID")
  ),
  responses(
    (status = 204, description = "Budget deleted successfully"),
    (status = 404, description = "Budget not found", body = ErrorResponse,
     example = json!({"message": "Budget not found", "stack_trace": [], "version": "v0.1.0"}))
  )
)]
pub async fn delete_budget_handler(
    Path(id): Path<Uuid>,
    Extension(use_case): Extension<Arc<DeleteBudget>>,
) -> Result<Json<SingleResponse<()>>, AppError> {
    use_case.execute(id).await?;
    let response = SingleResponse::with_message("Budget deleted successfully", ());
    Ok(Json(response))
}
