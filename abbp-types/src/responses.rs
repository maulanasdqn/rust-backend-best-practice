#![allow(clippy::option_if_let_else)]

use paginator_utils::PaginatorResponseMeta;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

const API_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

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

/// Response type for endpoints that return only a message without data.
/// Use this instead of SingleResponse<()> for better OpenAPI compatibility.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageOnlyResponse {
    pub message: String,
    pub version: String,
}

impl MessageOnlyResponse {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
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

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn from_paginator_metadata(metadata: &PaginatorResponseMeta) -> Self {
        Self {
            page: metadata.page,
            per_page: metadata.per_page,
            total_pages: metadata.total_pages.unwrap_or(0),
            total_data: u64::from(metadata.total.unwrap_or(0)),
        }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn to_paginator_metadata(&self) -> PaginatorResponseMeta {
        PaginatorResponseMeta {
            page: self.page,
            per_page: self.per_page,
            total: Some(self.total_data as u32),
            total_pages: Some(self.total_pages),
            has_next: self.page < self.total_pages,
            has_prev: self.page > 1,
            next_cursor: None,
            prev_cursor: None,
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
