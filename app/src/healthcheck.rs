use serde::Serialize;

use crate::error::AppError;
use infra::db::DbPool;
use infra::services::redis::RedisService;

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct HealthStatus {
    pub database: bool,
    pub redis: bool,
}

pub async fn check_health(pool: &DbPool, redis: &RedisService) -> Result<HealthStatus, AppError> {
    let (db_ok, redis_ok) = futures_util::join!(pool.get(), redis.ping());

    Ok(HealthStatus {
        database: db_ok.is_ok(),
        redis: redis_ok.is_ok(),
    })
}
