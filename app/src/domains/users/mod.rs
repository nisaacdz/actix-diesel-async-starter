use crate::domains::auth::AuthenticatedUser;
use crate::error::AppError;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use infra::db::DbPool;
use infra::models::{UpdateUser, User};
use infra::schema::users;

mod dtos;
pub use dtos::*;

/// Fetch the full profile of the authenticated user.
pub async fn get_profile(
    pool: &DbPool,
    user: &AuthenticatedUser,
) -> Result<UserProfileDto, AppError> {
    let mut conn = pool.get().await.map_err(AppError::internal)?;

    let user_row: User = users::table
        .filter(users::id.eq(user.id))
        .select(User::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(AppError::internal)?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    Ok(UserProfileDto {
        id: user_row.id,
        phone: user_row.phone,
        email: user_row.email,
        full_name: user_row.full_name,
        created_at: user_row.created_at,
    })
}

/// Update user profile metadata (name, email).
pub async fn update_profile(
    pool: &DbPool,
    user: &AuthenticatedUser,
    edit_profile: EditUserProfile,
) -> Result<UserProfileDto, AppError> {
    let mut conn = pool.get().await.map_err(AppError::internal)?;

    let change_set: UpdateUser = edit_profile.into();

    let user_row: User = diesel::update(users::table.filter(users::id.eq(user.id)))
        .set(&change_set)
        .get_result(&mut conn)
        .await
        .map_err(AppError::internal)?;

    Ok(UserProfileDto {
        id: user_row.id,
        phone: user_row.phone,
        email: user_row.email,
        full_name: user_row.full_name,
        created_at: user_row.created_at,
    })
}
