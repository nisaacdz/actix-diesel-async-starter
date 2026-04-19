use crate::settings::SecuritySettings;
use chrono::{Duration, Utc};
use pasetors::claims::{Claims, ClaimsValidationRules};
use pasetors::keys::SymmetricKey;
use pasetors::token::{Local, UntrustedToken};
use pasetors::version4::V4;
use rand::RngExt;
use serde::{Serialize, de::DeserializeOwned};
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthService {
    key: SymmetricKey<V4>,
    access_token_expiry: Duration,
    refresh_token_expiry: Duration,
}

impl AuthService {
    pub fn new(config: &SecuritySettings) -> Result<Self, String> {
        let key_bytes = if config.key.len() == 64 {
            hex::decode(&config.key).map_err(|e| e.to_string())?
        } else {
            config.key.as_bytes().to_vec()
        };

        if key_bytes.len() != 32 {
            return Err("Key must be exactly 32 bytes long".to_string());
        }

        let key = SymmetricKey::<V4>::from(key_bytes.as_slice())
            .map_err(|e| format!("Failed to create PASETO key: {}", e))?;

        Ok(Self {
            key,
            access_token_expiry: Duration::from_std(config.access_token_expiry)
                .map_err(|e| e.to_string())?,
            refresh_token_expiry: Duration::from_std(config.refresh_token_expiry)
                .map_err(|e| e.to_string())?,
        })
    }

    pub fn generate_secure_otp(&self) -> String {
        let mut rng = rand::rng();
        format!("{:06}", rng.random_range(100000..999999))
    }

    pub fn generate_access_token<U: Serialize>(&self, payload: U) -> Result<String, String> {
        self.generate_token(payload, self.access_token_expiry)
    }

    pub fn generate_refresh_token(&self, id: Uuid) -> Result<String, String> {
        self.generate_token(id, self.refresh_token_expiry)
    }

    fn generate_token<U: Serialize>(&self, payload: U, expiry: Duration) -> Result<String, String> {
        let mut claims = Claims::new().map_err(|e: pasetors::errors::Error| e.to_string())?;
        let payload_json = serde_json::to_value(payload).map_err(|e| e.to_string())?;

        claims
            .add_additional("payload", payload_json)
            .map_err(|e: pasetors::errors::Error| e.to_string())?;

        let expiration = (Utc::now() + expiry).to_rfc3339();
        claims
            .expiration(&expiration)
            .map_err(|e: pasetors::errors::Error| e.to_string())?;

        pasetors::local::encrypt(&self.key, &claims, None, None).map_err(|e| e.to_string())
    }

    pub fn verify_token<U: DeserializeOwned>(&self, token_str: &str) -> Result<U, String> {
        let validation_rules = ClaimsValidationRules::new();
        let untrusted_token =
            UntrustedToken::<Local, V4>::try_from(token_str).map_err(|e| e.to_string())?;

        let trusted_token =
            pasetors::local::decrypt(&self.key, &untrusted_token, &validation_rules, None, None)
                .map_err(|e| e.to_string())?;

        let payload_str = trusted_token.payload();
        let full_payload: serde_json::Value =
            serde_json::from_str(payload_str).map_err(|e: serde_json::Error| e.to_string())?;

        let payload_value = full_payload
            .get("payload")
            .ok_or("No payload claim found")?;

        serde_json::from_value(payload_value.clone()).map_err(|e| e.to_string())
    }
}

pub fn hash_password(password: &str) -> Result<String, String> {
    use argon2::{
        Argon2,
        password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| e.to_string())?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, String> {
    use argon2::{
        Argon2,
        password_hash::{PasswordHash, PasswordVerifier},
    };

    let parsed_hash = PasswordHash::new(password_hash).map_err(|e| e.to_string())?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
