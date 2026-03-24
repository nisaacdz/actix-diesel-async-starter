use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EnvironmentSettings {
    pub name: String,
    pub environment: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecuritySettings {
    #[serde(with = "humantime_serde")]
    pub session_expiry: Duration,
    #[serde(with = "humantime_serde")]
    pub refresh_threshold: Duration,
    #[serde(with = "humantime_serde")]
    pub login_otp_expiry_window: Duration,
    #[serde(with = "humantime_serde")]
    pub signup_otp_expiry_window: Duration,
    pub key: String,
    pub cookie_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebClientsSettings {
    pub website: String,
}

impl WebClientsSettings {
    pub fn allowed_origins(&self) -> Vec<&str> {
        vec![self.website.as_str()]
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub application: EnvironmentSettings,
    pub security: SecuritySettings,
    pub clients: WebClientsSettings,
}
