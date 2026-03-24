use std::borrow::Cow;

#[derive(Debug)]
pub enum AppErrorInner {
    Internal(anyhow::Error),
    BadRequest(anyhow::Error),
    Unauthorized(anyhow::Error),
    Forbidden(Cow<'static, str>),
    NotFound(Cow<'static, str>),
}

impl AppErrorInner {
    pub fn kind_name(&self) -> &str {
        match self {
            AppErrorInner::Internal(_) => "Internal Error",
            AppErrorInner::BadRequest(_) => "Bad Request",
            AppErrorInner::Unauthorized(_) => "Unauthorized",
            AppErrorInner::Forbidden(_) => "Forbidden",
            AppErrorInner::NotFound(message) => message.as_ref(),
        }
    }

    fn error_message(&self) -> Cow<'static, str> {
        match self {
            AppErrorInner::Internal(e) => e.to_string().into(),
            AppErrorInner::BadRequest(e) => e.to_string().into(),
            AppErrorInner::Unauthorized(e) => e.to_string().into(),
            AppErrorInner::Forbidden(message) => message.clone(),
            AppErrorInner::NotFound(message) => message.clone(),
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    pub error: AppErrorInner,
    pub message: Cow<'static, str>, // we send this to the client, so it should be user-friendly and not contain sensitive info
    pub detail: Option<serde_json::Value>,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = self.error.error_message();
        write!(f, "{message}")
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn new(error: AppErrorInner, message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
            error,
            detail: None,
        }
    }

    pub fn internal(error: impl Into<anyhow::Error>) -> Self {
        Self {
            error: AppErrorInner::Internal(error.into()),
            message: Cow::Borrowed("Something went wrong"),
            detail: None,
        }
    }

    pub fn message(mut self, message: impl Into<Cow<'static, str>>) -> Self {
        self.message = message.into();
        self
    }

    pub fn include_message(mut self) -> Self {
        self.message = self.error.error_message();
        self
    }

    pub fn detail(mut self, data: impl serde::Serialize) -> Self {
        self.detail = serde_json::to_value(data).ok();
        self
    }

    pub fn forbidden(message: impl Into<Cow<'static, str>>) -> Self {
        let message = message.into();
        Self {
            error: AppErrorInner::Forbidden(message.clone()),
            message,
            detail: None,
        }
    }

    pub fn bad_request(error: impl Into<anyhow::Error>) -> Self {
        let error = AppErrorInner::BadRequest(error.into());
        Self {
            message: error.error_message(),
            error,
            detail: None,
        }
    }

    pub fn unauthorized(error: impl Into<anyhow::Error>) -> Self {
        Self {
            error: AppErrorInner::Unauthorized(error.into()),
            message: Cow::Borrowed("Unauthorized"),
            detail: None,
        }
    }

    pub fn not_found(message: impl Into<Cow<'static, str>>) -> Self {
        let message = message.into();
        Self {
            error: AppErrorInner::NotFound(message.clone()),
            message,
            detail: None,
        }
    }
}
