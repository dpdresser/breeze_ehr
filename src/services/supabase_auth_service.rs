use crate::domain::{
    error::app_error::{AppResult, AuthError},
    interfaces::auth_service::AuthService,
};

pub struct SupabaseAuthService {
    pub client: reqwest::Client,
    pub supabase_url: String,
    pub supabase_key: String,
}

impl SupabaseAuthService {
    pub fn new(supabase_url: &str, supabase_key: &str) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            supabase_url: supabase_url.to_string(),
            supabase_key: supabase_key.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl AuthService for SupabaseAuthService {
    async fn signin(&self, email: &str, password: &str) -> AppResult<String> {
        let url = format!("{}/auth/v1/token?grant_type=password", self.supabase_url);
        let params = serde_json::json!({
            "email": email,
            "password": password
        });

        let resp = self
            .client
            .post(&url)
            .header("apikey", &self.supabase_key)
            .header("Content-Type", "application/json")
            .json(&params)
            .send()
            .await
            .map_err(|e| AuthError::SignInError(format!("Failed to send request: {e}")))?;

        if !resp.status().is_success() {
            return Err(AuthError::SignInError(format!(
                "Sign-in failed with status: {}",
                resp.status()
            ))
            .into());
        }

        let resp_json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AuthError::SignInError(format!("Failed to parse response: {e}")))?;

        if let Some(token) = resp_json.get("access_token").and_then(|t| t.as_str()) {
            Ok(token.to_string())
        } else {
            Err(AuthError::SignInError("No access token in response".to_string()).into())
        }
    }
}
