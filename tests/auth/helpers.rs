use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::redirect::Policy;
use serde_json::{Value, json};
use sovaehr::{App, utils::config::AppConfig};
use std::time::{Duration, Instant};

pub struct TestApp {
    pub address: String,
    pub mailpit_url: String,
    pub http_client: reqwest::Client,
}

impl TestApp {
    pub async fn new() -> Self {
        let settings = AppConfig::for_tests();
        let http_client = reqwest::Client::builder()
            .redirect(Policy::none())
            .build()
            .expect("Failed to build HTTP client");

        let app = App::new(settings.clone());

        // Spawn the server
        tokio::spawn(async move { app.run().await.expect("Failed to start test server") });
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        Self {
            address: settings.app_address,
            mailpit_url: settings.mailpit_url,
            http_client,
        }
    }

    pub async fn health_check(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("http://{}/api/health", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn delete_user(&self, token: &str, user_id: &str) -> reqwest::Response {
        let request_body = json!({ "user_id": user_id });
        self.http_client
            .delete(&format!("http://{}/api/auth/delete_user", &self.address))
            .bearer_auth(token)
            .json(&request_body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_retrieve_user_id(&self, token: &str, email: &str) -> reqwest::Response {
        let request_body = json!({ "email": email });
        self.http_client
            .post(&format!(
                "http://{}/api/auth/retrieve_user_id",
                &self.address
            ))
            .bearer_auth(token)
            .json(&request_body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signin(&self, email: &str, password: &str) -> reqwest::Response {
        let request_body = json!({ "email": email, "password": password });
        self.http_client
            .post(&format!("http://{}/api/auth/signin", &self.address))
            .json(&request_body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn post_signup(
        &self,
        email: &str,
        password: &str,
        redirect_to: Option<&str>,
    ) -> reqwest::Response {
        let mut request_body = json!({ "email": email, "password": password });

        if let Some(redirect) = redirect_to {
            request_body["redirect_to"] = json!(redirect);
        }

        self.http_client
            .post(&format!("http://{}/api/auth/signup", &self.address))
            .json(&request_body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn clear_mailpit_messages(&self) {
        self.http_client
            .delete(format!("{}/api/v1/messages", self.mailpit_url))
            .send()
            .await
            .expect("Failed to execute request.");
    }

    pub async fn wait_for_verification_link(&self, email: &str) -> Option<String> {
        let deadline = Instant::now() + Duration::from_secs(10);
        let poll_interval = Duration::from_millis(250);
        let normalized_email = email.to_ascii_lowercase();

        while Instant::now() < deadline {
            let messages: Value = self
                .http_client
                .get(format!("{}/api/v1/messages", self.mailpit_url))
                .send()
                .await
                .expect("Failed to execute request.")
                .json()
                .await
                .expect("Failed to parse Mailpit response body");

            if let Some(message_id) = find_message_id(&messages, &normalized_email) {
                let message: Value = self
                    .http_client
                    .get(format!(
                        "{}/api/v1/message/{}",
                        self.mailpit_url, message_id
                    ))
                    .send()
                    .await
                    .expect("Failed to execute request.")
                    .json()
                    .await
                    .expect("Failed to parse Mailpit message response");

                if let Some(link) = find_verification_link(&message) {
                    return Some(link);
                }
            }

            tokio::time::sleep(poll_interval).await;
        }

        None
    }

    pub async fn verification_link(&self, email: &str) -> String {
        self.wait_for_verification_link(email).await.unwrap_or_else(|| {
            panic!(
                "No verification email found for {email}. Ensure Supabase email confirmations are enabled."
            )
        })
    }
}

fn find_message_id(messages: &Value, email: &str) -> Option<String> {
    const RECIPIENT_KEYS: [&str; 2] = ["To", "to"];

    if let Some(arr) = messages.as_array() {
        if let Some(id) = find_in_messages(arr, email, &RECIPIENT_KEYS) {
            return Some(id);
        }
    }

    if let Some(arr) = messages.get("messages").and_then(Value::as_array) {
        if let Some(id) = find_in_messages(arr, email, &RECIPIENT_KEYS) {
            return Some(id);
        }
    }

    None
}

fn find_in_messages(messages: &[Value], email: &str, recipient_keys: &[&str]) -> Option<String> {
    messages.iter().find_map(|msg| {
        let matches_recipient = recipient_keys.iter().any(|key| {
            msg.get(key)
                .and_then(Value::as_array)
                .map(|addresses| {
                    addresses.iter().any(|addr| {
                        addr.get("Address")
                            .or_else(|| addr.get("address"))
                            .and_then(Value::as_str)
                            .map(|a| a.eq_ignore_ascii_case(email))
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false)
        });

        if matches_recipient {
            msg.get("ID")
                .or_else(|| msg.get("id"))
                .and_then(Value::as_str)
                .map(|id| id.to_string())
        } else {
            None
        }
    })
}

static LINK_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r#"https?://[^\s"<>]+"#).expect("valid verification link regex"));

pub fn find_link_in_html(html: &str) -> Option<String> {
    LINK_REGEX.find(html).map(|m| {
        let cleaned = m.as_str().trim_end_matches(|c| c == '"' || c == ')');
        cleaned.replace("&amp;", "&")
    })
}

pub fn find_verification_link(message: &Value) -> Option<String> {
    ["HTML", "Text"]
        .iter()
        .find_map(|field| message.get(field).and_then(extract_link_from_value))
}

fn extract_link_from_value(value: &Value) -> Option<String> {
    value.as_str().and_then(find_link_in_html).or_else(|| {
        value
            .get("Body")
            .and_then(Value::as_str)
            .and_then(find_link_in_html)
    })
}
