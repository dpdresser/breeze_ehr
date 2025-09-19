use color_eyre::eyre::Result;
use thiserror::Error;
use tracing_error::SpanTrace;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Sign-in failed: {0}")]
    SignInError(String),
    #[error("Sign-up failed: {0}")]
    SignUpError(String),
    #[error("Email already in use")]
    EmailAlreadyInUse,
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    ExpiredToken,
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Password does not meet complexity requirements")]
    WeakPassword,
    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error(transparent)]
    Validation(#[from] ValidationError),
    #[error("Internal server error")]
    Internal {
        #[source]
        source: anyhow::Error,
        span: SpanTrace,
    },
}

impl AppError {
    pub fn internal<E: Into<anyhow::Error>>(e: E) -> Self {
        Self::Internal {
            source: e.into(),
            span: SpanTrace::capture(),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;
