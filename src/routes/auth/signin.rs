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
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

#[derive(Object, Debug)]
pub struct SigninResponse {
    pub token: String,
}

pub async fn signin_handler(
    state: Data<&AppState>,
    payload: Json<SigninRequest>,
) -> AppResult<SigninResponse> {
    let email = Email::new(payload.email.clone())?;
    let password = Password::new(payload.password.clone())?;

    let token = state
        .auth_service
        .read()
        .await
        .signin(&email, &password)
        .await?;

    Ok(SigninResponse { token })
}
