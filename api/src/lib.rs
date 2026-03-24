use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer, web};
use infra::services::auth::AuthService;
use infra::settings::Settings;

use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;

pub mod error;
pub mod middlewares;
pub mod openapi;
pub mod response;
pub mod routes;
pub mod utils;

pub async fn run(settings: Settings) -> std::io::Result<()> {
    let pool_data = web::Data::new(
        infra::db::init_pool(&settings.database).expect("Failed to initialize pool"),
    );
    let auth_service = web::Data::new(
        AuthService::new(&settings.security).expect("Failed to initialize auth service"),
    );
    let app_settings = web::Data::new(settings.clone());

    let governor_conf = GovernorConfigBuilder::default()
        .period(std::time::Duration::from_millis(250))
        .burst_size(100)
        .finish()
        .expect("Failed to configure rate limiting");

    HttpServer::new(move || {
        let pool_data = pool_data.clone();
        let auth_service = auth_service.clone();
        let app_settings = app_settings.clone();

        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        let openapi = openapi::ApiDoc::openapi();

        App::new()
            .service(
                utoipa_swagger_ui::SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", openapi.clone()),
            )
            .wrap(cors)
            .wrap(Governor::new(&governor_conf))
            .wrap(TracingLogger::default())
            .app_data(pool_data)
            .app_data(app_settings)
            .app_data(auth_service)
            .app_data(
                actix_web_validator::JsonConfig::default()
                    .error_handler(error::validate_json_error_handler),
            )
            .app_data(
                actix_web_validator::QueryConfig::default()
                    .error_handler(error::validate_query_error_handler),
            )
            .app_data(
                actix_web_validator::PathConfig::default()
                    .error_handler(error::validate_path_error_handler),
            )
            .app_data(web::JsonConfig::default().error_handler(error::json_error_handler))
            .app_data(web::QueryConfig::default().error_handler(error::query_error_handler))
            .app_data(web::PathConfig::default().error_handler(error::path_error_handler))
            .service(web::scope("/api/v1").configure(routes::configure_routes))
            .service(web::scope("/crons").configure(routes::configure_cron_routes))
            .service(web::resource("/api/healthcheck").route(web::get().to(healthcheck)))
            .default_service(web::route().to(error::not_found_handler))
    })
    .bind((settings.server.host.as_str(), settings.server.port))?
    .run()
    .await
}

#[utoipa::path(
    get,
    path = "/api/healthcheck",
    responses(
        (status = 200, description = "Service health status", body = app::healthcheck::HealthStatus),
    ),
    tag = "Health"
)]
async fn healthcheck(pool: web::Data<infra::db::DbPool>) -> actix_web::HttpResponse {
    let result = app::healthcheck::check_health(&pool).await;
    response::send_ok_response(result)
}
