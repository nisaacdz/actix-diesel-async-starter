use serde::{Deserialize, Serialize};

const MAX_LIMIT: i64 = 100;

/// Clamp a limit to the valid range (1..=MAX_LIMIT), falling back to the given default.
pub fn clamp_limit(limit: Option<i64>, default: i64) -> i64 {
    limit.unwrap_or(default).clamp(1, MAX_LIMIT)
}

/// Clamp an offset to be non-negative, defaulting to 0.
pub fn clamp_offset(offset: Option<i64>) -> i64 {
    offset.unwrap_or(0).max(0)
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
}

impl<T: Serialize> Paginated<T> {
    pub fn new(items: Vec<T>, total: i64) -> Self {
        Self { items, total }
    }
}
