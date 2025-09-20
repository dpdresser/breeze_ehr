use reqwest::{StatusCode, Url};
use secrecy::{ExposeSecret, SecretString};
use serde_json::{Value, json};

use crate::domain::{
    error::app_error::{AppResult, AuthError},
    interfaces::auth_service::AuthService,
    types::{email::Email, password::Password},
};

pub struct SupabaseAuthService {
    pub client: reqwest::Client,
    pub supabase_url: String,
    pub supabase_anon_key: SecretString,
    pub supabase_service_role_key: SecretString,
}

impl SupabaseAuthService {
    pub fn new(
        supabase_url: String,
        supabase_anon_key: SecretString,
        supabase_service_role_key: SecretString,
    ) -> Self {
        let client = reqwest::Client::new();
        Self {
            client,
            supabase_url,
            supabase_anon_key,
            supabase_service_role_key,
        }
    }
}

#[async_trait::async_trait]
impl AuthService for SupabaseAuthService {
    async fn delete_user(&self, user_id: &str) -> AppResult<()> {
        let url = format!("{}/auth/v1/admin/users/{}", self.supabase_url, user_id);

        let resp = self
            .client
            .delete(&url)
            .header("apikey", self.supabase_service_role_key.expose_secret())
            .header(
                "Authorization",
                format!("Bearer {}", self.supabase_service_role_key.expose_secret()),
            )
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| AuthError::DeleteUserError(format!("Failed to send request: {e}")))?;

        let status = resp.status();

        if status.is_success() {
            Ok(())
        } else {
            let resp_json: serde_json::Value = resp.json().await.map_err(|e| {
                AuthError::DeleteUserError(format!("Failed to parse response: {e}"))
            })?;

            let message = resp_json
                .get("msg")
                .or_else(|| resp_json.get("message"))
                .or_else(|| resp_json.get("error_description"))
                .or_else(|| resp_json.get("error"))
                .and_then(|v| v.as_str())
                .unwrap_or("Delete user failed");

            Err(AuthError::DeleteUserError(format!(
                "Failed to delete user with status {status}: {message}"
            ))
            .into())
        }
    }

    async fn retrieve_user_id(&self, email: &Email) -> AppResult<String> {
        let mut url =
            Url::parse(&format!("{}/auth/v1/admin/users", self.supabase_url)).map_err(|e| {
                AuthError::RetrieveUserIdError(format!("Failed to build request URL: {e}"))
            })?;
        url.query_pairs_mut()
            .append_pair("email", &format!("eq.{}", email.as_ref().expose_secret()));

        let resp = self
            .client
            .get(url)
            .header("apikey", self.supabase_service_role_key.expose_secret())
            .header(
                "Authorization",
                format!("Bearer {}", self.supabase_service_role_key.expose_secret()),
            )
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| AuthError::RetrieveUserIdError(format!("Failed to send request: {e}")))?;

        let status = resp.status();

        let resp_json: serde_json::Value = resp.json().await.map_err(|e| {
            AuthError::RetrieveUserIdError(format!("Failed to parse response: {e}"))
        })?;

        if !status.is_success() {
            let message = resp_json
                .get("msg")
                .or_else(|| resp_json.get("message"))
                .or_else(|| resp_json.get("error_description"))
                .or_else(|| resp_json.get("error"))
                .and_then(|v| v.as_str())
                .unwrap_or("Retrieve user ID failed");

            return Err(AuthError::RetrieveUserIdError(format!(
                "Failed to retrieve user ID with status {status}: {message}"
            ))
            .into());
        }

        let users = if let Some(arr) = resp_json.as_array() {
            arr
        } else if let Some(arr) = resp_json.get("users").and_then(Value::as_array) {
            arr
        } else {
            return Err(AuthError::RetrieveUserIdError(
                "Expected an array of users in response".to_string(),
            )
            .into());
        };

        if users.is_empty() {
            return Err(AuthError::UserNotFound.into());
        }

        if users.len() > 1 {
            tracing::warn!(
                "Multiple users found with email {}, filtering by exact match",
                email.as_ref().expose_secret()
            );
        }

        let matching_user = users.iter().find(|user| {
            user.get("email")
                .and_then(|v| v.as_str())
                .map(|e| e.eq_ignore_ascii_case(email.as_ref().expose_secret()))
                .unwrap_or(false)
        });

        let matching_user = match matching_user {
            Some(user) => user,
            None => return Err(AuthError::UserNotFound.into()),
        };

        let user_id = matching_user
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                AuthError::RetrieveUserIdError("User ID not found in response".to_string())
            })?;

        Ok(user_id.to_string())
    }

    async fn signin(&self, email: &Email, password: &Password) -> AppResult<String> {
        let url = format!("{}/auth/v1/token?grant_type=password", self.supabase_url);
        let signin_request = json!({
            "email": email.as_ref().expose_secret(),
            "password": password.as_ref().expose_secret()
        });

        let resp = self
            .client
            .post(&url)
            .header("apikey", self.supabase_anon_key.expose_secret())
            .header("Content-Type", "application/json")
            .json(&signin_request)
            .send()
            .await
            .map_err(|e| AuthError::SignInError(format!("Failed to send request: {e}")))?;
        let status = resp.status();

        let resp_json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| AuthError::SignInError(format!("Failed to parse response: {e}")))?;

        if !status.is_success() {
            let message = resp_json
                .get("msg")
                .or_else(|| resp_json.get("message"))
                .or_else(|| resp_json.get("error_description"))
                .or_else(|| resp_json.get("error"))
                .and_then(|v| v.as_str())
                .unwrap_or("Sign-in failed");

            return Err(AuthError::SignInError(message.to_string()).into());
        }

        if let Some(token) = resp_json.get("access_token").and_then(|t| t.as_str()) {
            Ok(token.to_string())
        } else {
            Err(AuthError::SignInError("No access token in response".to_string()).into())
        }
    }

    async fn signup(
        &self,
        email: &Email,
        password: &Password,
        redirect_to: Option<&str>,
    ) -> AppResult<()> {
        let url = format!("{}/auth/v1/signup", self.supabase_url);

        let mut signup_request = json!({
            "email": email.as_ref().expose_secret(),
            "password": password.as_ref().expose_secret(),
            "data": { "origin": "api" },
        });

        if let Some(redirect) = redirect_to {
            signup_request["redirect_to"] = json!(redirect.to_string());
        }

        let resp = self
            .client
            .post(&url)
            .header("apikey", self.supabase_anon_key.expose_secret())
            .header("Content-Type", "application/json")
            .json(&signup_request)
            .send()
            .await
            .map_err(|e| AuthError::SignUpError(format!("Failed to send request: {e}")))?;

        let status = resp.status();

        let body = resp.json::<Value>().await.unwrap_or_else(|e| {
            tracing::warn!(error = %e, "Failed to parse Supabase signup response body as JSON");
            Value::Null
        });

        let message_opt = body
            .get("msg")
            .or_else(|| body.get("message"))
            .or_else(|| body.get("error_description"))
            .or_else(|| body.get("error"))
            .and_then(|v| v.as_str());

        let error_code = body.get("error_code").and_then(|v| v.as_str());

        if status.is_success() {
            return Ok(());
        }

        let existing_user = status == StatusCode::CONFLICT
            || status == StatusCode::UNPROCESSABLE_ENTITY
            || matches!(error_code, Some("user_already_exists"))
            || message_opt
                .map(|m| {
                    m.eq_ignore_ascii_case("User already registered")
                        || m.eq_ignore_ascii_case("Email already in use")
                })
                .unwrap_or(false);

        if existing_user {
            return Err(AuthError::EmailAlreadyInUse.into());
        }

        if status == StatusCode::BAD_REQUEST {
            let message = message_opt.unwrap_or("Bad request");
            return Err(AuthError::SignUpError(message.to_string()).into());
        }

        let message = message_opt.unwrap_or("Supabase signup failed");
        Err(
            AuthError::SignUpError(format!("Sign-up failed with status {status}: {message}"))
                .into(),
        )
    }
}
