use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl DateRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, String> {
        if end < start {
            return Err("End date must be after start date".to_string());
        }
        Ok(Self { start, end })
    }

    pub fn contains(&self, date: &DateTime<Utc>) -> bool {
        date >= &self.start && date <= &self.end
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn duration_days(&self) -> i64 {
        (self.end - self.start).num_days()
    }
}
