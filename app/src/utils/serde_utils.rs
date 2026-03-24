use serde::{Deserialize, Deserializer};

/// Deserializer helper for `Option<Option<T>>` fields.
///
/// Use with `#[serde(default, deserialize_with = "deserialize_some")]` to
/// distinguish between:
/// - Field omitted from JSON -> `None` (don't update)
/// - Field explicitly set to `null` -> `Some(None)` (set DB column to NULL)
/// - Field set to a value -> `Some(Some(value))` (update DB column)
pub fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Deserialize::deserialize(deserializer).map(Some)
}
