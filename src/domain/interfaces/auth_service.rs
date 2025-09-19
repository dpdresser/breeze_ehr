use crate::domain::{
    error::app_error::AppResult,
    types::{email::Email, password::Password},
};

#[async_trait::async_trait]
pub trait AuthService {
    async fn signin(&self, email: &Email, password: &Password) -> AppResult<String>;
    async fn signup(
        &self,
        email: &Email,
        password: &Password,
        redirect_to: Option<&str>,
    ) -> AppResult<()>;
}
