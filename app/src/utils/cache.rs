use std::future::Future;

use serde::{Serialize, de::DeserializeOwned};
use tracing::warn;

use crate::error::AppError;
use infra::services::redis::RedisService;

/// Read-through Redis cache helper.
///
/// 1. Tries `redis.get(key)` — returns immediately on hit.
/// 2. On miss (or Redis error), executes `query_fn` against the database.
/// 3. Best-effort `redis.set_ex(key, &result, ttl)` — failures are logged but never fail the request.
pub async fn cached_query<T, F>(
    redis: &RedisService,
    key: &str,
    ttl: std::time::Duration,
    query_fn: F,
) -> Result<T, AppError>
where
    T: Serialize + DeserializeOwned,
    F: Future<Output = Result<T, AppError>>,
{
    // 1. Try cache hit
    #[cfg(not(feature = "ci"))]
    match redis.get::<T>(key).await {
        Ok(Some(cached)) => return Ok(cached),
        Ok(None) => {} // cache miss — fall through
        Err(e) => {
            warn!(key = key, error = %e, "Redis cache read failed, falling through to query");
        }
    }

    // 2. Execute the real query
    let result = query_fn.await?;

    let result_json_str = match serde_json::to_string(&result).map_err(AppError::internal) {
        Err(e) => {
            tracing::error!(key = key, error = %e, "Failed to serialize result to JSON");
            return Ok(result);
        }
        Ok(v) => v,
    };

    let redis = redis.clone();
    let key = key.to_string();
    // 3. Best-effort write to cache
    #[cfg(not(feature = "ci"))]
    tokio::task::spawn_local(async move {
        if let Err(e) = redis
            .set_ex_raw(&key, &result_json_str, ttl.as_secs())
            .await
        {
            warn!(key = key, error = %e, "Redis cache write failed");
        }
    });

    Ok(result)
}
