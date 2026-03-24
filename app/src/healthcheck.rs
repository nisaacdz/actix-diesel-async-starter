use infra::db::DbPool;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct HealthStatus {
    pub status: String,
    pub database: String,
}

pub async fn check_health(pool: &DbPool) -> Result<HealthStatus, crate::error::AppError> {
    let database_status = match pool.get().await {
        Ok(_) => "ok".to_string(),
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            "error".to_string()
        }
    };

    let overall_status = if database_status == "ok" {
        "ok".to_string()
    } else {
        "error".to_string()
    };

    Ok(HealthStatus {
        status: overall_status,
        database: database_status,
    })
}
