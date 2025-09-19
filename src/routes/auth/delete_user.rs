use poem::web::Data;
use poem_openapi::{Object, payload::Json};

use crate::{domain::error::app_error::AppResult, state::AppState};

#[derive(Object, Debug)]
pub struct DeleteUserRequest {
    pub user_id: String,
}

pub async fn delete_user_handler(
    state: Data<&AppState>,
    payload: Json<DeleteUserRequest>,
) -> AppResult<()> {
    state
        .auth_service
        .read()
        .await
        .delete_user(&payload.user_id)
        .await?;

    Ok(())
}
