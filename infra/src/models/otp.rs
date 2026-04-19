use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::otps)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Otp {
    pub id: Uuid,
    pub phone: String,
    pub code: String,
    pub payload: Option<serde_json::Value>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::otps)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewOtp {
    pub phone: String,
    pub code: String,
    pub payload: Option<serde_json::Value>,
    pub expires_at: DateTime<Utc>,
}
