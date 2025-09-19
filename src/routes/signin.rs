use poem::web::Data;
use poem_openapi::{Object, payload::Json};

use crate::{domain::error::app_error::AppResult, state::AppState};

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
    let token = state
        .auth_service
        .read()
        .await
        .signin(&payload.email, &payload.password)
        .await?;

    Ok(SigninResponse { token })
}
