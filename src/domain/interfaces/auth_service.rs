use crate::domain::error::app_error::AppResult;

#[async_trait::async_trait]
pub trait AuthService {
    async fn signin(&self, email: &str, password: &str) -> AppResult<String>;
}
