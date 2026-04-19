use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(
    Serialize, Deserialize, Debug, Clone, bitcode::Encode, bitcode::Decode, utoipa::ToSchema,
)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub phone: String,
    pub full_name: String,
}

// ── Login ───────────────────────────────────────────────────────────────
#[derive(Deserialize, utoipa::ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginStart {
    pub phone: String,
}

#[derive(Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginStartSuccess {
    pub message: String,
}

#[derive(Deserialize, utoipa::ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginComplete {
    pub phone: String,
    #[validate(length(equal = 6, message = "OTP must be exactly 6 characters"))]
    pub otp: String,
}

#[derive(Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginCompleteSuccess {
    pub access_token: String,
    pub refresh_token: String,
    pub user: AuthenticatedUser,
}

// ── Signup ───────────────────────────────────────────────────────────────

#[derive(Deserialize, utoipa::ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignupStart {
    pub phone: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: Option<String>,
    #[validate(length(min = 1, max = 100, message = "Full name must be 1-100 characters"))]
    pub full_name: String,
}

#[derive(Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SignupStartSuccess {
    pub message: String,
}

#[derive(Deserialize, utoipa::ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SignupComplete {
    pub phone: String,
    #[validate(length(equal = 6, message = "OTP must be exactly 6 characters"))]
    pub otp: String,
}

#[derive(Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SignupCompleteSuccess {
    pub access_token: String,
    pub refresh_token: String,
    pub user: AuthenticatedUser,
}

#[derive(Serialize, Deserialize, Debug, Clone, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SignupPayload {
    pub full_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Deserialize, utoipa::ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RefreshAccessToken {
    pub refresh_token: String,
}

#[derive(Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshSuccess {
    pub access_token: String,
}
