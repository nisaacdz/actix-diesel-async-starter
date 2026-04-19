use actix_web::{HttpResponse, get, patch, web};
use app::domains::auth::AuthenticatedUser;
use app::domains::users::{EditUserProfile, get_profile, update_profile};
use infra::db::DbPool;
use std::rc::Rc;

use crate::response::{ok, send_ok_response};

#[utoipa::path(
    get,
    path = "/users/me",
    responses(
        (status = 200, description = "Returns the current authenticated user", body = AuthenticatedUser),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Users",
    security(
        ("cookieAuth" = [])
    )
)]
#[get("/me")]
async fn r_users_me(user: web::ReqData<Rc<AuthenticatedUser>>) -> HttpResponse {
    ok(user.as_ref().clone())
}

#[utoipa::path(
    get,
    path = "/users/profile",
    responses(
        (status = 200, description = "Authenticated user's profile metadata"),
        (status = 401, description = "Unauthorized"),
    ),
    tag = "Users",
    security(("cookieAuth" = []))
)]
#[get("/profile")]
async fn r_get_profile(
    pool: web::Data<DbPool>,
    user: web::ReqData<Rc<AuthenticatedUser>>,
) -> HttpResponse {
    let result = get_profile(&pool, &user).await;
    send_ok_response(result)
}

#[utoipa::path(
    patch,
    path = "/users/profile",
    request_body = EditUserProfile,
    responses(
        (status = 200, description = "Profile updated successfully"),
        (status = 400, description = "Invalid input"),
    ),
    tag = "Users",
    security(("cookieAuth" = []))
)]
#[patch("/profile")]
async fn r_update_profile(
    pool: web::Data<DbPool>,
    user: web::ReqData<Rc<AuthenticatedUser>>,
    req: actix_web_validator::Json<EditUserProfile>,
) -> HttpResponse {
    let result = update_profile(&pool, &user, req.into_inner()).await;
    send_ok_response(result)
}

pub fn configure_users_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(r_users_me)
        .service(r_get_profile)
        .service(r_update_profile);
}
