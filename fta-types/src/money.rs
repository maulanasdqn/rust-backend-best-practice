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

    pub fn from_decimal(dollars: f64) -> Self {
        const MIN_CENTS: f64 = -9.223_372_036_854_775e18;
        const MAX_CENTS: f64 = 9.223_372_036_854_775e18;

        let cents = (dollars * 100.0).round().clamp(MIN_CENTS, MAX_CENTS);

        let amount = if cents.is_finite() {
            cents.trunc() as i64
        } else {
            0
        };

        Self { amount }
    }

    pub const fn amount(&self) -> i64 {
        self.amount
    }

    pub fn to_decimal(&self) -> f64 {
        let whole_dollars = self.amount / 100;
        let remaining_cents = i8::try_from(self.amount % 100).unwrap_or(0);

        let dollars_float =
            i32::try_from(whole_dollars).map_or_else(|_| whole_dollars as f64, f64::from);
        let cents_float = f64::from(remaining_cents);

        dollars_float + (cents_float / 100.0)
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
