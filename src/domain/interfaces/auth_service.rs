use crate::domain::{
    error::app_error::AppResult,
    types::{email::Email, password::Password},
};

#[async_trait::async_trait]
pub trait AuthService {
    async fn delete_user(&self, user_id: &str) -> AppResult<()>;
    async fn retrieve_user_id(&self, email: &Email) -> AppResult<String>;
    async fn signin(&self, email: &Email, password: &Password) -> AppResult<String>;
    async fn signout(&self, token: &str) -> AppResult<()>;
    async fn signup(
        &self,
        email: &Email,
        password: &Password,
        redirect_to: Option<&str>,
    ) -> AppResult<()>;
}
