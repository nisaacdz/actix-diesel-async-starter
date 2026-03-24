use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{otps, users};

#[derive(Queryable, Selectable, Identifiable, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password_hash: Option<String>,
}

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    Debug,
    Clone,
    PartialEq,
    Serialize,
    Deserialize,
)]
#[diesel(belongs_to(User))]
#[diesel(table_name = otps)]
pub struct Otp {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = otps)]
pub struct NewOtp {
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: DateTime<Utc>,
}
