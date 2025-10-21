#![allow(clippy::option_if_let_else)]

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

const API_VERSION: &str = "v0.1.0";

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub message: String,
    pub stack_trace: Vec<String>,
    pub version: String,
}

impl ErrorResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            stack_trace: Vec::new(),
            version: API_VERSION.to_string(),
        }
    }

    pub fn with_stack_trace(message: impl Into<String>, stack_trace: Vec<String>) -> Self {
        Self {
            message: message.into(),
            stack_trace,
            version: API_VERSION.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SingleResponse<T> {
    pub message: String,
    pub data: T,
    pub version: String,
}

impl<T> SingleResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            message: "Success".to_string(),
            data,
            version: API_VERSION.to_string(),
        }
    }

    pub fn with_message(message: impl Into<String>, data: T) -> Self {
        Self {
            message: message.into(),
            data,
            version: API_VERSION.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PaginationMeta {
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
    pub total_data: u64,
}

impl PaginationMeta {
    #[allow(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        clippy::cast_precision_loss
    )]
    pub fn new(page: u32, per_page: u32, total_data: u64) -> Self {
        let total_pages = ((total_data as f64) / f64::from(per_page)).ceil() as u32;
        Self {
            page,
            per_page,
            total_pages,
            total_data,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ListResponse<T> {
    pub message: String,
    pub data: Vec<T>,
    pub meta: PaginationMeta,
    pub version: String,
}

impl<T> ListResponse<T> {
    pub fn new(data: Vec<T>, meta: PaginationMeta) -> Self {
        Self {
            message: "Success Fetching Data".to_string(),
            data,
            meta,
            version: API_VERSION.to_string(),
        }
    }

    pub fn with_message(message: impl Into<String>, data: Vec<T>, meta: PaginationMeta) -> Self {
        Self {
            message: message.into(),
            data,
            meta,
            version: API_VERSION.to_string(),
        }
    }
}
