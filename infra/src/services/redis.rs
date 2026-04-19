use crate::settings::RedisSettings;
use redis::{AsyncCommands, Client, aio::ConnectionManager};
use serde::{Serialize, de::DeserializeOwned};

#[derive(Clone)]
pub struct RedisService {
    connection: ConnectionManager,
}

#[derive(Debug, derive_more::Display)]
pub enum RedisError {
    InvalidUrl(String),
    SerializationFailed(String),
    DeserializationFailed(String),
    RedisError(redis::RedisError),
}

impl std::error::Error for RedisError {}

impl RedisService {
    pub async fn new(settings: &RedisSettings) -> Result<Self, RedisError> {
        let client = Client::open(settings.url.as_str())
            .map_err(|e| RedisError::InvalidUrl(e.to_string()))?;

        let connection = client
            .get_connection_manager()
            .await
            .map_err(RedisError::RedisError)?;

        Ok(Self { connection })
    }

    pub async fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), RedisError> {
        let json_value = serde_json::to_string(value)
            .map_err(|e| RedisError::SerializationFailed(e.to_string()))?;

        let mut con = self.connection.clone();

        con.set(key, json_value)
            .await
            .map_err(RedisError::RedisError)
    }

    pub async fn set_ex<T: Serialize>(
        &self,
        key: &str,
        value: &T,
        ttl_seconds: u64,
    ) -> Result<(), RedisError> {
        let json_value = serde_json::to_string(value)
            .map_err(|e| RedisError::SerializationFailed(e.to_string()))?;

        let mut con = self.connection.clone();

        con.set_ex(key, json_value, ttl_seconds)
            .await
            .map_err(RedisError::RedisError)
    }

    pub async fn set_ex_raw(
        &self,
        key: &str,
        value: &str,
        ttl_seconds: u64,
    ) -> Result<(), RedisError> {
        let mut con = self.connection.clone();

        con.set_ex(key, value, ttl_seconds)
            .await
            .map_err(RedisError::RedisError)
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, RedisError> {
        let mut con = self.connection.clone();

        let json_value: Option<String> = con.get(key).await.map_err(RedisError::RedisError)?;

        match json_value {
            Some(json_str) => {
                let parsed: T = serde_json::from_str(&json_str)
                    .map_err(|e| RedisError::DeserializationFailed(e.to_string()))?;
                Ok(Some(parsed))
            }
            None => Ok(None),
        }
    }

    pub async fn set_nx_ex(
        &self,
        key: &str,
        value: &str,
        ttl_seconds: u64,
    ) -> Result<bool, RedisError> {
        let mut con = self.connection.clone();

        // SET key value NX EX ttl_seconds
        let result: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("NX")
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut con)
            .await
            .map_err(RedisError::RedisError)?;

        // If result is Some("OK"), the lock was acquired
        Ok(result.map(|s| s == "OK").unwrap_or(false))
    }

    pub async fn del(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.connection.clone();
        con.del(key).await.map_err(RedisError::RedisError)
    }

    pub async fn ping(&self) -> Result<(), RedisError> {
        let mut con = self.connection.clone();
        redis::cmd("PING")
            .query_async::<String>(&mut con)
            .await
            .map_err(RedisError::RedisError)?;
        Ok(())
    }

    pub async fn mget(&self, keys: &[String]) -> Result<Vec<Option<String>>, RedisError> {
        if keys.is_empty() {
            return Ok(Vec::new());
        }

        let mut con = self.connection.clone();

        redis::cmd("MGET")
            .arg(keys)
            .query_async(&mut con)
            .await
            .map_err(RedisError::RedisError)
    }
}
