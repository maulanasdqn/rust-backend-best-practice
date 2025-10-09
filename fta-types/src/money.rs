use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, ToSchema)]
pub struct Money {
    amount: i64,
}

impl Money {
    pub const fn new(amount: i64) -> Self {
        Self { amount }
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn from_decimal(dollars: f64) -> Self {
        Self {
            amount: (dollars * 100.0).round() as i64,
        }
    }

    pub const fn amount(&self) -> i64 {
        self.amount
    }

    #[allow(clippy::cast_precision_loss)]
    pub fn to_decimal(&self) -> f64 {
        self.amount as f64 / 100.0
    }

    #[must_use]
    pub const fn add(&self, other: &Self) -> Self {
        Self {
            amount: self.amount + other.amount,
        }
    }

    #[must_use]
    pub const fn subtract(&self, other: &Self) -> Self {
        Self {
            amount: self.amount - other.amount,
        }
    }

    pub const fn is_positive(&self) -> bool {
        self.amount > 0
    }

    pub const fn is_negative(&self) -> bool {
        self.amount < 0
    }

    pub const fn is_zero(&self) -> bool {
        self.amount == 0
    }
}
