use poem_openapi::{ApiResponse, Object, payload::Json};
use serde::Serialize;
use serde_json::Value;

use crate::domain::error::app_error::{AppError, AuthError, ValidationError};

#[derive(Object, Serialize, Debug)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub request_id: String,
}

#[derive(ApiResponse, Debug)]
pub enum AppHttpResponse {
    #[oai(status = 200)]
    Ok(Json<Value>),
    #[oai(status = 201)]
    Created(Json<Value>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorBody>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorBody>),
    #[oai(status = 403)]
    Forbidden(Json<ErrorBody>),
    #[oai(status = 404)]
    NotFound(Json<ErrorBody>),
    #[oai(status = 409)]
    Conflict(Json<ErrorBody>),
    #[oai(status = 500)]
    InternalServerError(Json<ErrorBody>),
}

impl AppHttpResponse {
    fn body(code: &str, message: &str, request_id: &str) -> Json<ErrorBody> {
        Json(ErrorBody {
            code: code.to_string(),
            message: message.to_string(),
            request_id: request_id.to_string(),
        })
    }

    pub fn from_app_error(error: AppError, request_id: &str) -> Self {
        match error {
            AppError::Auth(ae) => match ae {
                AuthError::SignInError(msg) => {
                    AppHttpResponse::Unauthorized(Self::body("sign_in_error", &msg, request_id))
                }
                AuthError::SignOutError(msg) => {
                    AppHttpResponse::Unauthorized(Self::body("sign_out_error", &msg, request_id))
                }
                AuthError::SignUpError(msg) => {
                    AppHttpResponse::BadRequest(Self::body("sign_up_error", &msg, request_id))
                }
                AuthError::EmailAlreadyInUse => AppHttpResponse::Conflict(Self::body(
                    "email_already_in_use",
                    &ae.to_string(),
                    request_id,
                )),
                AuthError::DeleteUserError(msg) => AppHttpResponse::InternalServerError(
                    Self::body("delete_user_error", &msg, request_id),
                ),
                AuthError::RetrieveUserIdError(msg) => AppHttpResponse::InternalServerError(
                    Self::body("retrieve_user_id_error", &msg, request_id),
                ),
                AuthError::UserNotFound => AppHttpResponse::NotFound(Self::body(
                    "user_not_found",
                    &ae.to_string(),
                    request_id,
                )),
                AuthError::MissingToken => AppHttpResponse::Unauthorized(Self::body(
                    "missing_token",
                    &ae.to_string(),
                    request_id,
                )),
                AuthError::InvalidToken => AppHttpResponse::Unauthorized(Self::body(
                    "invalid_token",
                    &ae.to_string(),
                    request_id,
                )),
                AuthError::ExpiredToken => AppHttpResponse::Unauthorized(Self::body(
                    "expired_token",
                    &ae.to_string(),
                    request_id,
                )),
            },
            AppError::Validation(ve) => match ve {
                ValidationError::InvalidEmail => AppHttpResponse::BadRequest(Self::body(
                    "invalid_email",
                    &ve.to_string(),
                    request_id,
                )),
                ValidationError::WeakPassword => AppHttpResponse::BadRequest(Self::body(
                    "weak_password",
                    &ve.to_string(),
                    request_id,
                )),
                ValidationError::InvalidInput(_) => AppHttpResponse::BadRequest(Self::body(
                    "invalid_input",
                    &ve.to_string(),
                    request_id,
                )),
            },
            AppError::Internal { .. } => AppHttpResponse::InternalServerError(Self::body(
                "internal_server_error",
                "An internal server error occurred",
                request_id,
            )),
        }
    }
}
