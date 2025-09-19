use poem::web::Data;
use poem_openapi::{Object, payload::Json};

use crate::{
    domain::{
        error::app_error::AppResult,
        types::{email::Email, password::Password},
    },
    state::AppState,
};

#[derive(Object, Debug)]
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    pub redirect_to: Option<String>,
}

#[derive(Object, Debug)]
pub struct SignupResponse {
    pub message: String,
}

pub async fn signup_handler(
    state: Data<&AppState>,
    payload: Json<SignupRequest>,
) -> AppResult<SignupResponse> {
    let email = Email::new(payload.email.clone())?;
    let password = Password::new(payload.password.clone())?;

    let redirect = payload
        .redirect_to
        .clone()
        .or_else(|| state.email_confirm_redirect.clone());

    state
        .auth_service
        .read()
        .await
        .signup(&email, &password, redirect.as_deref())
        .await?;

    Ok(SignupResponse {
        message: "Signup successful, please check your email to confirm.".to_string(),
    })
}
