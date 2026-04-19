use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    web::Data,
};
use app::domains::auth::AuthenticatedUser;
use infra::services::auth::AuthService;
use std::rc::Rc;

use crate::error::ApiError;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let auth_service = req
        .app_data::<Data<AuthService>>()
        .ok_or_else(|| ApiError::internal(anyhow::anyhow!("AuthService not in app_data")))?
        .get_ref();

    let auth_header = req
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)
        .ok_or_else(|| ApiError::unauthorized(anyhow::anyhow!("Missing Authorization header")))?
        .to_str()
        .map_err(|_| {
            ApiError::unauthorized(anyhow::anyhow!("Invalid Authorization header format"))
        })?;

    let token_str = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        ApiError::unauthorized(anyhow::anyhow!(
            "Authorization header must start with Bearer "
        ))
    })?;

    let user = auth_service
        .verify_token::<AuthenticatedUser>(token_str)
        .map_err(|e| ApiError::unauthorized(anyhow::anyhow!(e)))?;

    req.extensions_mut().insert(Rc::new(user));

    next.call(req).await
}
