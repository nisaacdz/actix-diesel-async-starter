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
pub struct RedisSettings {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecuritySettings {
    #[serde(with = "humantime_serde")]
    pub access_token_expiry: Duration,
    #[serde(with = "humantime_serde")]
    pub refresh_token_expiry: Duration,
    #[serde(with = "humantime_serde")]
    pub login_otp_expiry_window: Duration,
    #[serde(with = "humantime_serde")]
    pub signup_otp_expiry_window: Duration,
    pub key: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WebClientsSettings {
    pub website: String,
    pub admin_panel: String,
}

impl WebClientsSettings {
    pub fn allowed_origins(&self) -> Vec<&str> {
        vec![self.website.as_str(), self.admin_panel.as_str()]
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub application: EnvironmentSettings,
    pub security: SecuritySettings,
    pub clients: WebClientsSettings,
    pub frogsms: FrogsmsSettings,
    pub redis: RedisSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FrogsmsSettings {
    pub api_key: String,
    pub username: String,
    pub sender_id: String,
    pub base_url: String,
}
