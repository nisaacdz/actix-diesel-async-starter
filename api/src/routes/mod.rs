pub mod auth;
pub mod users;

use actix_web::{web, web::ServiceConfig};

use crate::middlewares::auth::auth_middleware;
use crate::routes::auth::configurate_auth_routes;
use crate::routes::users::configure_users_routes;

pub fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/auth").configure(configurate_auth_routes));
    cfg.service(
        web::scope("/users")
            .configure(configure_users_routes)
            .wrap(actix_web::middleware::from_fn(auth_middleware)),
    );
}
