use actix_web::{HttpResponse, post, web};
use actix_web_validator::Json;

use app::domains::auth::{
    LoginComplete, LoginCompleteSuccess, LoginStart, LoginStartSuccess, RefreshAccessToken,
    RefreshSuccess, SignupComplete, SignupCompleteSuccess, SignupStart, SignupStartSuccess,
    login_complete, login_start, refresh_access_token, signup_complete, signup_start,
};
use infra::db::DbPool;
use infra::services::auth::AuthService;
use infra::services::frogsms::FrogSmsProvider;
use infra::settings::Settings;

use crate::response::send_ok_response;

#[utoipa::path(
    post,
    path = "/auth/login/start",
    request_body = LoginStart,
    responses(
        (status = 200, description = "OTP sent successfully", body = LoginStartSuccess),
        (status = 400, description = "Bad Request"),
    ),
    tag = "Auth"
)]
#[post("/login/start")]
async fn r_login_start(
    pool: web::Data<DbPool>,
    sms: web::Data<FrogSmsProvider>,
    req: Json<LoginStart>,
    auth_service: web::Data<AuthService>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let result = login_start(
        &pool,
        &sms,
        req.into_inner(),
        &auth_service,
        &settings.security,
    )
    .await;
    send_ok_response(result)
}

#[utoipa::path(
    post,
    path = "/auth/login/complete",
    request_body = LoginComplete,
    responses(
        (status = 200, description = "Login successful", body = LoginCompleteSuccess),
        (status = 400, description = "Invalid or expired OTP"),
    ),
    tag = "Auth"
)]
#[post("/login/complete")]
async fn r_login_complete(
    pool: web::Data<DbPool>,
    auth_service: web::Data<AuthService>,
    settings: web::Data<Settings>,
    req: Json<LoginComplete>,
) -> HttpResponse {
    let result = login_complete(&pool, &auth_service, &settings.security, req.into_inner()).await;
    send_ok_response(result)
}

#[utoipa::path(
    post,
    path = "/auth/signup/start",
    request_body = SignupStart,
    responses(
        (status = 200, description = "OTP sent successfully", body = SignupStartSuccess),
        (status = 400, description = "Bad Request"),
    ),
    tag = "Auth"
)]
#[post("/signup/start")]
async fn r_signup_start(
    pool: web::Data<DbPool>,
    sms: web::Data<FrogSmsProvider>,
    req: Json<SignupStart>,
    auth_service: web::Data<AuthService>,
    settings: web::Data<Settings>,
) -> HttpResponse {
    let result = signup_start(
        &pool,
        &sms,
        req.into_inner(),
        &auth_service,
        &settings.security,
    )
    .await;
    send_ok_response(result)
}

#[utoipa::path(
    post,
    path = "/auth/signup/complete",
    request_body = SignupComplete,
    responses(
        (status = 200, description = "Signup successful", body = SignupCompleteSuccess),
        (status = 400, description = "Invalid or expired OTP"),
    ),
    tag = "Auth"
)]
#[post("/signup/complete")]
async fn r_signup_complete(
    pool: web::Data<DbPool>,
    auth_service: web::Data<AuthService>,
    settings: web::Data<Settings>,
    req: Json<SignupComplete>,
) -> HttpResponse {
    let result = signup_complete(&pool, &auth_service, &settings.security, req.into_inner()).await;
    send_ok_response(result)
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    request_body = RefreshAccessToken,
    responses(
        (status = 200, description = "Token refreshed successfully", body = RefreshSuccess),
        (status = 401, description = "Invalid or expired refresh token"),
    ),
    tag = "Auth"
)]
#[post("/refresh")]
async fn r_refresh(
    pool: web::Data<DbPool>,
    auth_service: web::Data<AuthService>,
    req: Json<RefreshAccessToken>,
) -> HttpResponse {
    let result = refresh_access_token(&pool, &auth_service, req.into_inner()).await;
    send_ok_response(result)
}

pub fn configurate_auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(r_login_start)
        .service(r_login_complete)
        .service(r_signup_start)
        .service(r_signup_complete)
        .service(r_refresh);
}
