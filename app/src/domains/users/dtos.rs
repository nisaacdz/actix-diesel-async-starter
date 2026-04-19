use crate::utils::serde_utils::deserialize_some;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, utoipa::ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct EditUserProfile {
    #[validate(length(min = 2, message = "Full name must be at least 2 characters"))]
    pub full_name: Option<String>,
    #[validate(email(message = "Invalid email address"))]
    #[serde(deserialize_with = "deserialize_some")]
    pub email: Option<Option<String>>,
}

impl From<EditUserProfile> for infra::models::UpdateUser {
    fn from(value: EditUserProfile) -> Self {
        Self {
            full_name: value.full_name,
            email: value.email,
        }
    }
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileDto {
    pub id: Uuid,
    pub phone: String,
    pub email: Option<String>,
    pub full_name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
