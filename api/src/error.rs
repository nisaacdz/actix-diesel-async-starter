use std::borrow::Cow;

use actix_web::{HttpRequest, HttpResponse, ResponseError, error, http::StatusCode};
use app::error::{AppError, AppErrorInner};

#[derive(Debug, derive_more::Display, derive_more::Error)]
pub struct ApiError(pub AppError);

impl AsRef<AppError> for ApiError {
    fn as_ref(&self) -> &AppError {
        &self.0
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self.0.error {
            AppErrorInner::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorInner::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppErrorInner::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppErrorInner::Forbidden(_) => StatusCode::FORBIDDEN,
            AppErrorInner::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct ResponseBody<'a> {
            pub message: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            pub detail: &'a Option<serde_json::Value>,
        }

        let body = ResponseBody {
            message: &self.0.message,
            detail: &self.0.detail,
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}

impl ApiError {
    pub fn internal(error: impl Into<anyhow::Error>) -> Self {
        Self(AppError::internal(error))
    }
    pub fn bad_request(error: impl Into<anyhow::Error>) -> Self {
        Self(AppError::bad_request(error))
    }
    pub fn unauthorized(error: impl Into<anyhow::Error>) -> Self {
        Self(AppError::unauthorized(error))
    }
    pub fn forbidden(message: impl Into<Cow<'static, str>>) -> Self {
        Self(AppError::forbidden(message))
    }
    pub fn not_found(message: impl Into<Cow<'static, str>>) -> Self {
        Self(AppError::not_found(message))
    }
}

// pub fn validate_json_error_handler(
//     err: actix_web_validator::Error,
//     _req: &HttpRequest,
// ) -> error::Error {
//     let response = ApiError::bad_request(err);
//     response.into()
// }

pub fn validate_json_error_handler(
    err: actix_web_validator::Error,
    _req: &HttpRequest,
) -> error::Error {
    let response = match &err {
        actix_web_validator::Error::Validate(v_errors) => {
            AppError::bad_request(anyhow::anyhow!(v_errors.to_string())).detail(v_errors)
        }
        _ => AppError::bad_request(err),
    };
    ApiError(response).into()
}

pub fn validate_query_error_handler(
    err: actix_web_validator::Error,
    _req: &HttpRequest,
) -> error::Error {
    let response = match &err {
        actix_web_validator::Error::Validate(v_errors) => {
            AppError::bad_request(anyhow::anyhow!(v_errors.to_string())).detail(v_errors)
        }
        _ => AppError::bad_request(err),
    };
    ApiError(response).into()
}

pub fn validate_path_error_handler(
    err: actix_web_validator::Error,
    _req: &HttpRequest,
) -> error::Error {
    let response = match &err {
        actix_web_validator::Error::Validate(v_errors) => {
            AppError::bad_request(anyhow::anyhow!(v_errors.to_string())).detail(v_errors)
        }
        _ => AppError::bad_request(err),
    };
    ApiError(response).into()
}

pub fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let response = ApiError::bad_request(err);
    response.into()
}

pub fn query_error_handler(err: error::QueryPayloadError, _req: &HttpRequest) -> error::Error {
    let response = ApiError::bad_request(err);
    response.into()
}

pub fn path_error_handler(err: error::PathError, _req: &HttpRequest) -> error::Error {
    let response = ApiError::bad_request(err);
    response.into()
}

pub async fn not_found_handler() -> HttpResponse {
    ApiError::not_found("Resource not found").error_response()
}
