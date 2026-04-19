use chrono::Utc;
use diesel::dsl;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;

use crate::error::AppError;

use infra::db::DbPool;
use infra::models::{NewOtp, NewUser, Otp, User};
use infra::schema::{otps, users};

use infra::services::auth::AuthService;
use infra::services::frogsms::FrogSmsProvider;
use infra::settings::SecuritySettings;

mod dtos;
pub use dtos::*;

pub async fn login_start(
    pool: &DbPool,
    sms: &FrogSmsProvider,
    req: LoginStart,
    auth_service: &AuthService,
    security: &SecuritySettings,
) -> Result<LoginStartSuccess, AppError> {
    let mut conn = pool.get().await.map_err(AppError::internal)?;

    let user_exists = diesel::select(diesel::dsl::exists(
        users::table.filter(users::phone.eq(&req.phone)),
    ))
    .get_result::<bool>(&mut conn)
    .await
    .map_err(AppError::internal)?;

    if !user_exists {
        return Err(AppError::bad_request(anyhow::anyhow!(
            "User does not exist"
        )));
    }

    #[cfg(feature = "ci")]
    let code = "000000".to_string();
    #[cfg(not(feature = "ci"))]
    let code = auth_service.generate_secure_otp();

    let expires_at = Utc::now()
        + chrono::Duration::from_std(security.login_otp_expiry_window)
            .map_err(AppError::internal)?;

    let otp_record = NewOtp {
        phone: req.phone.clone(),
        code: code.clone(),
        payload: None,
        expires_at,
    };

    diesel::insert_into(otps::table)
        .values(&otp_record)
        .on_conflict(otps::phone)
        .do_update()
        .set((
            otps::code.eq(&code),
            otps::payload.eq(None::<serde_json::Value>),
            otps::expires_at.eq(expires_at),
            otps::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)
        .await
        .map_err(AppError::internal)?;

    #[cfg(not(feature = "ci"))]
    {
        let message = format!("Your login code is {}", code);
        sms.send_sms(&req.phone, &message)
            .await
            .map_err(|e| AppError::internal(e).message("Failed to send SMS"))?;
    }

    Ok(LoginStartSuccess {
        message: "Login OTP sent successfully".into(),
    })
}

pub async fn login_complete(
    pool: &DbPool,
    auth_service: &AuthService,
    _security: &SecuritySettings,
    req: LoginComplete,
) -> Result<LoginCompleteSuccess, AppError> {
    let mut conn = pool.get().await.map_err(AppError::internal)?;

    diesel::delete(
        otps::table
            .filter(otps::phone.eq(&req.phone))
            .filter(otps::code.eq(&req.otp))
            .filter(otps::expires_at.gt(Utc::now())),
    )
    .get_result::<Otp>(&mut conn)
    .await
    .optional()
    .map_err(AppError::internal)?
    .ok_or_else(|| AppError::bad_request(anyhow::anyhow!("Invalid or expired OTP")))?;

    let user = users::table
        .filter(users::phone.eq(&req.phone))
        .first::<User>(&mut conn)
        .await
        .optional()
        .map_err(AppError::internal)?
        .ok_or_else(|| AppError::not_found("User not found"))?;

    let auth_user = AuthenticatedUser {
        id: user.id,
        phone: user.phone.clone(),
        full_name: user.full_name.clone(),
    };

    let access_token = auth_service
        .generate_access_token(&auth_user)
        .map_err(|e| AppError::internal(anyhow::anyhow!(e)))?;
    let refresh_token = auth_service
        .generate_refresh_token(user.id)
        .map_err(|e| AppError::internal(anyhow::anyhow!(e)))?;

    Ok(LoginCompleteSuccess {
        access_token,
        refresh_token,
        user: auth_user,
    })
}

pub async fn signup_start(
    pool: &DbPool,
    sms: &FrogSmsProvider,
    req: SignupStart,
    auth_service: &AuthService,
    security: &SecuritySettings,
) -> Result<SignupStartSuccess, AppError> {
    let mut conn = pool.get().await.map_err(AppError::internal)?;

    let user_exists = diesel::select(diesel::dsl::exists(
        users::table.filter(users::phone.eq(&req.phone)),
    ))
    .get_result::<bool>(&mut conn)
    .await
    .map_err(AppError::internal)?;

    if user_exists {
        return Err(AppError::bad_request(anyhow::anyhow!(
            "User already exists"
        )));
    }

    let code = auth_service.generate_secure_otp();
    let expires_at = Utc::now()
        + chrono::Duration::from_std(security.signup_otp_expiry_window)
            .map_err(AppError::internal)?;

    let payload = SignupPayload {
        full_name: req.full_name,
        email: req.email,
    };
    let payload_json = serde_json::to_value(payload).map_err(AppError::internal)?;

    let otp_record = NewOtp {
        phone: req.phone.clone(),
        code: code.clone(),
        payload: Some(payload_json.clone()),
        expires_at,
    };

    diesel::insert_into(otps::table)
        .values(&otp_record)
        .on_conflict(otps::phone)
        .do_update()
        .set((
            otps::code.eq(&code),
            otps::payload.eq(Some(payload_json)),
            otps::expires_at.eq(expires_at),
        ))
        .execute(&mut conn)
        .await
        .map_err(AppError::internal)?;

    let message = format!("Your signup code is {}", code);
    sms.send_sms(&req.phone, &message)
        .await
        .map_err(|e| AppError::internal(anyhow::anyhow!(e)).message("Failed to send SMS"))?;

    Ok(SignupStartSuccess {
        message: "Signup OTP sent successfully".into(),
    })
}

pub async fn signup_complete(
    pool: &DbPool,
    auth_service: &AuthService,
    _security: &SecuritySettings,
    req: SignupComplete,
) -> Result<SignupCompleteSuccess, AppError> {
    let mut conn = pool.get().await.map_err(AppError::internal)?;

    let mut otp_record = diesel::delete(
        otps::table
            .filter(otps::phone.eq(&req.phone))
            .filter(otps::code.eq(&req.otp))
            .filter(otps::expires_at.gt(dsl::now)),
    )
    .get_result::<Otp>(&mut conn)
    .await
    .optional()
    .map_err(AppError::internal)?
    .ok_or_else(|| AppError::bad_request(anyhow::anyhow!("Invalid or expired OTP")))?;

    let payload: SignupPayload =
        serde_json::from_value(otp_record.payload.take().unwrap_or_default())
            .map_err(AppError::internal)?;

    let new_user = NewUser {
        phone: req.phone.clone(),
        email: payload.email,
        full_name: payload.full_name,
    };

    let user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut conn)
        .await
        .map_err(AppError::internal)?;

    let auth_user = AuthenticatedUser {
        id: user.id,
        phone: user.phone.clone(),
        full_name: user.full_name.clone(),
    };

    let access_token = auth_service
        .generate_access_token(&auth_user)
        .map_err(|e| AppError::internal(anyhow::anyhow!(e)))?;
    let refresh_token = auth_service
        .generate_refresh_token(user.id)
        .map_err(|e| AppError::internal(anyhow::anyhow!(e)))?;

    Ok(SignupCompleteSuccess {
        access_token,
        refresh_token,
        user: auth_user,
    })
}

pub async fn refresh_access_token(
    pool: &DbPool,
    auth_service: &AuthService,
    req: RefreshAccessToken,
) -> Result<RefreshSuccess, AppError> {
    let mut conn = pool.get().await.map_err(AppError::internal)?;

    // 1. Verify the refresh token (contains only UUID)
    let user_id = auth_service
        .verify_token::<uuid::Uuid>(&req.refresh_token)
        .map_err(|e| AppError::unauthorized(anyhow::anyhow!(e)))?;

    // 2. Fetch fresh user data from DB
    let user = users::table
        .filter(users::id.eq(user_id))
        .first::<User>(&mut conn)
        .await
        .optional()
        .map_err(AppError::internal)?
        .ok_or_else(|| AppError::unauthorized(anyhow::anyhow!("User no longer exists")))?;

    // 3. Build payload and generate new access token
    let auth_user = AuthenticatedUser {
        id: user.id,
        phone: user.phone.clone(),
        full_name: user.full_name.clone(),
    };

    let access_token = auth_service
        .generate_access_token(&auth_user)
        .map_err(|e| AppError::internal(anyhow::anyhow!(e)))?;

    Ok(RefreshSuccess { access_token })
}
