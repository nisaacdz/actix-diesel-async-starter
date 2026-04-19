use actix_web::cookie::{Cookie, SameSite};

pub fn create_partitioned_auth_cookie(name: &str, value: String) -> String {
    let cookie = Cookie::build(name, value)
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .path("/")
        .finish();

    // Manually append the Partitioned attribute since it's not supported by cookie v0.16
    format!("{}; Partitioned", cookie)
}

pub fn create_expired_auth_cookie(name: &str) -> String {
    let cookie = Cookie::build(name, "")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::None)
        .path("/")
        .max_age(actix_web::cookie::time::Duration::ZERO)
        .finish();

    format!("{}; Partitioned", cookie)
}
