use utoipa::Modify;
use utoipa::OpenApi;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

use app::domains::auth::*;
use app::domains::users::{EditUserProfile, UserProfileDto};
use app::healthcheck::HealthStatus;

#[derive(OpenApi)]
#[openapi(
    servers(
        (url = "/api/v1")
    ),
    paths(
        // Auth
        crate::routes::auth::r_login_start,
        crate::routes::auth::r_login_complete,
        crate::routes::auth::r_signup_start,
        crate::routes::auth::r_signup_complete,
        crate::routes::auth::r_refresh,
        // Users
        crate::routes::users::r_users_me,
        crate::routes::users::r_get_profile,
        crate::routes::users::r_update_profile,
        // Health
        crate::healthcheck,
    ),
    components(
        schemas(
            AuthenticatedUser,
            LoginStart,
            LoginStartSuccess,
            LoginComplete,
            LoginCompleteSuccess,
            SignupStart,
            SignupStartSuccess,
            SignupComplete,
            SignupCompleteSuccess,
            RefreshAccessToken,
            RefreshSuccess,
            // User schemas
            EditUserProfile,
            UserProfileDto,
            // Healthcheck
            HealthStatus,
        )
    ),
    tags(
        (name = "Auth", description = "Authentication Endpoints"),
        (name = "Users", description = "User Endpoints"),
        (name = "Health", description = "Service Health Endpoints")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            )
        }
    }
}
