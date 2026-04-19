use actix_web::{HttpResponse, ResponseError};
use app::error::AppError;
use serde::Serialize;

use crate::error::ApiError;

pub fn ok<S: Serialize>(body: S) -> HttpResponse {
    HttpResponse::Ok().json(body)
}

pub fn created<S: Serialize>(body: S) -> HttpResponse {
    HttpResponse::Created().json(body)
}

pub fn send_ok_response<S: Serialize>(result: Result<S, AppError>) -> HttpResponse {
    match result {
        Ok(body) => ok(body),
        Err(error) => {
            tracing::error!("Error: {}", error);
            ApiError(error).error_response()
        }
    }
}

pub fn send_created_response<S: Serialize>(result: Result<S, AppError>) -> HttpResponse {
    match result {
        Ok(body) => created(body),
        Err(error) => {
            tracing::error!("Error: {}", error);
            ApiError(error).error_response()
        }
    }
}
