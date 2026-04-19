use rand::RngExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use url::Url;

use crate::settings::FrogsmsSettings;

#[derive(Debug)]
pub enum SmsError {
    NetworkError(String),
    ApiError { status: u16, message: String },
}

impl std::fmt::Display for SmsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "Network error: {}", e),
            Self::ApiError { status, message } => {
                write!(f, "API error with status {}: {}", status, message)
            }
        }
    }
}

impl std::error::Error for SmsError {}

/// FrogSMS Ghana provider implementation
pub struct FrogSmsProvider {
    api_key: String,
    username: String,
    sender_id: String,
    base_url: Url,
    client: Client,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrogSMSDestinations {
    destination: String,
    #[serde(rename = "msgid")]
    msg_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FrogSmsMessage {
    #[serde(rename = "senderid")]
    sender_id: String,
    destinations: Vec<FrogSMSDestinations>,
    message: String,
    #[serde(rename = "smstype")]
    sms_type: String,
}

impl FrogSmsProvider {
    pub fn new(config: &FrogsmsSettings) -> Result<Self, String> {
        let base_url = Url::parse(&config.base_url).map_err(|e| e.to_string())?;
        Ok(Self {
            api_key: config.api_key.clone(),
            username: config.username.clone(),
            sender_id: config.sender_id.clone(),
            base_url,
            client: Client::new(),
        })
    }

    /// Send SMS via FrogSMS Ghana API
    pub async fn send_sms(&self, phone: &str, message: &str) -> Result<(), SmsError> {
        info!(
            phone = %phone,
            message_length = message.len(),
            "Sending SMS via FrogSMS Ghana"
        );

        let message_id = generate_sms_message_id();

        let destinations = vec![FrogSMSDestinations {
            destination: phone.to_string(),
            msg_id: message_id,
        }];

        let message_data = FrogSmsMessage {
            sender_id: self.sender_id.clone(),
            destinations,
            message: message.to_string(),
            sms_type: "text".to_string(),
        };

        let response = self
            .client
            .post(self.base_url.clone())
            .header("Content-Type", "application/json")
            .header("API-KEY", &self.api_key)
            .header("USERNAME", &self.username)
            .json(&message_data)
            .send()
            .await
            .map_err(|e| {
                error!(error = %e, "Failed to send SMS request");
                SmsError::NetworkError(e.to_string())
            })?;

        let status = response.status();
        if !status.is_success() {
            let error_body = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            warn!(
                status = %status,
                error_body = %error_body,
                "SMS API returned error status"
            );

            return Err(SmsError::ApiError {
                status: status.as_u16(),
                message: error_body,
            });
        }

        info!(phone = %phone, "SMS sent successfully");
        Ok(())
    }
}

pub fn generate_sms_message_id() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const ID_LENGTH: usize = 8;

    let mut rng = rand::rng();
    (0..ID_LENGTH)
        .map(|_| {
            let idx = rng.random_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}
