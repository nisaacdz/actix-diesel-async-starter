use crate::settings::SecuritySettings;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use bitcode;
use chacha20poly1305::{
    XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit},
};
use chrono::{Duration, Utc};
use rand::RngCore;

#[derive(Debug, bitcode::Encode, bitcode::Decode)]
pub struct AuthSession<U> {
    pub iat: i64,
    pub exp: i64,
    pub payload: U,
}

pub enum TokenStatus {
    /// Token is valid, no refresh needed
    Valid,
    /// Token is valid but should be refreshed (iat is stale)
    NeedsRefresh,
    /// Token has expired
    Expired,
}

#[derive(Clone)]
pub struct AuthService {
    cipher: XChaCha20Poly1305,
    session_expiry: Duration,
    refresh_threshold: Duration,
}

impl AuthService {
    pub fn new(config: &SecuritySettings) -> Result<Self, String> {
        if config.key.len() != 32 {
            return Err("Key must be exactly 32 bytes long".to_string());
        }
        let key_bytes = config.key.as_bytes();
        let key = chacha20poly1305::Key::from_slice(key_bytes);

        Ok(Self {
            cipher: XChaCha20Poly1305::new(key),
            session_expiry: Duration::from_std(config.session_expiry).map_err(|e| e.to_string())?,
            refresh_threshold: Duration::from_std(config.refresh_threshold)
                .map_err(|e| e.to_string())?,
        })
    }

    pub fn generate_secure_otp(&self) -> String {
        use rand::Rng;
        let mut rng = rand::rng();
        format!("{:06}", rng.random_range(100000..999999))
    }

    pub fn create_token<U: bitcode::Encode>(&self, payload: U) -> Result<String, String> {
        let now = Utc::now();
        let session = AuthSession {
            iat: now.timestamp(),
            exp: (now + self.session_expiry).timestamp(),
            payload,
        };

        let payload_bytes = bitcode::encode(&session);

        let mut nonce = XNonce::default();
        rand::rng().fill_bytes(&mut nonce);

        let ciphertext = self
            .cipher
            .encrypt(&nonce, payload_bytes.as_ref())
            .map_err(|_| "Encryption failed".to_string())?;

        let mut final_buffer = Vec::with_capacity(nonce.len() + ciphertext.len());
        final_buffer.extend_from_slice(&nonce);
        final_buffer.extend_from_slice(&ciphertext);
        Ok(BASE64.encode(final_buffer))
    }

    pub fn verify_token<U: bitcode::DecodeOwned>(
        &self,
        token_str: &str,
    ) -> Result<AuthSession<U>, String> {
        let encrypted_data = BASE64
            .decode(token_str)
            .map_err(|_| "Invalid token encoding".to_string())?;

        if encrypted_data.len() < 24 {
            return Err("Token too short".to_string());
        }
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(24);
        let nonce = XNonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| "Invalid token signature or data".to_string())?;

        let session: AuthSession<U> =
            bitcode::decode(&plaintext).map_err(|_| "Invalid session data".to_string())?;

        if session.exp < Utc::now().timestamp() {
            return Err("Token expired".to_string());
        }

        Ok(session)
    }

    pub fn check_token_status(&self, iat: i64) -> TokenStatus {
        let now = Utc::now().timestamp();
        let age_seconds = now - iat;
        let threshold_seconds = self.refresh_threshold.num_seconds();

        if age_seconds > threshold_seconds {
            TokenStatus::NeedsRefresh
        } else {
            TokenStatus::Valid
        }
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
