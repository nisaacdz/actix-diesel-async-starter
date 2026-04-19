use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

pub fn serialize_duration<S>(duration: &chrono::Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    #[derive(Serialize)]
    struct DurationMinutesVal {
        minutes: i64,
    }
    let val = DurationMinutesVal {
        minutes: duration.num_minutes(),
    };
    val.serialize(serializer)
}

pub fn serialize_optional_duration<S>(
    duration: &Option<chrono::Duration>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match duration {
        Some(d) => serialize_duration(d, serializer),
        None => serializer.serialize_none(),
    }
}
