use reqwest::redirect::Policy;
use sovaehr::utils::tracing::init_tracing;
use uuid::Uuid;

use crate::helpers::{TestApp, find_link_in_html};

#[tokio::test]
async fn signup_with_invalid_email_returns_400() {
    init_tracing("info");
    let app = TestApp::new().await;

    let email = "invalid-email";
    let password = "Password123!";
    let response = app.post_signup(email, password, None).await;

    assert_eq!(response.status().as_u16(), 400);
    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body");
    assert!(response_body.get("message").is_some());
    assert_eq!(
        response_body.get("message").unwrap().as_str().unwrap(),
        "Invalid email format"
    );
}

#[tokio::test]
async fn signup_with_weak_password_returns_400() {
    init_tracing("info");
    let app = TestApp::new().await;

    let email = "test@example.com";
    let password = "weak";
    let response = app.post_signup(email, password, None).await;

    assert_eq!(response.status().as_u16(), 400);
    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body");
    assert!(response_body.get("message").is_some());
    assert_eq!(
        response_body.get("message").unwrap().as_str().unwrap(),
        "Password does not meet complexity requirements"
    );
}

#[tokio::test]
async fn signup_with_existing_email_returns_409() {
    init_tracing("info");
    let app = TestApp::new().await;

    let email = "owner1@example.com";
    let password = "Password123!";
    let response = app.post_signup(email, password, None).await;

    assert_eq!(response.status().as_u16(), 409);
    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body");
    assert!(response_body.get("message").is_some());
    assert_eq!(
        response_body.get("message").unwrap().as_str().unwrap(),
        "Email already in use"
    );
}

#[tokio::test]
async fn signup_sends_verify_email_and_allows_signin_after_verification_with_200() {
    init_tracing("info");
    let app = TestApp::new().await;

    app.clear_mailpit_messages().await;

    let email = format!("new-user+{}@example.com", Uuid::new_v4());
    let password = "StrongPass123!";

    let signup_response = app.post_signup(&email, password, None).await;
    assert_eq!(signup_response.status().as_u16(), 201);

    let message_response = app
        .poll_mailpit_messages(&email)
        .await
        .unwrap_or_else(|| {
            panic!(
                "No verification email found for {email}. Ensure Supabase email confirmations are enabled."
            )
        });
    let message_body: serde_json::Value = message_response
        .json()
        .await
        .expect("Failed to parse Mailpit message response");

    let extract_link = |field: &str| -> Option<String> {
        let value = message_body.get(field)?;
        if let Some(content) = value.as_str() {
            return find_link_in_html(content);
        }

        value
            .get("Body")
            .and_then(|body| body.as_str())
            .and_then(find_link_in_html)
    };

    let verification_url = extract_link("HTML")
        .or_else(|| extract_link("Text"))
        .expect("No verification link found in email");

    let verification_client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()
        .expect("Failed to build verification client");

    let verify_response = verification_client
        .get(&verification_url)
        .send()
        .await
        .expect("Failed to follow verification link");

    assert!(
        verify_response.status().is_success() || verify_response.status().is_redirection(),
        "Unexpected verification status {}",
        verify_response.status()
    );

    let signin_response = app.post_signin(&email, password).await;
    assert_eq!(signin_response.status().as_u16(), 200);

    let signin_body: serde_json::Value = signin_response
        .json()
        .await
        .expect("Failed to parse signin response body");
    assert!(signin_body.get("token").is_some());
}

#[tokio::test]
async fn signin_with_unverified_email_fails_with_401() {
    init_tracing("info");
    let app = TestApp::new().await;

    app.clear_mailpit_messages().await;

    let email = format!("new-user+{}@example.com", Uuid::new_v4());
    let password = "Password123!";
    let signup_response = app.post_signup(&email, password, None).await;
    assert_eq!(signup_response.status().as_u16(), 201);
    let signin_response = app.post_signin(&email, password).await;
    assert_eq!(signin_response.status().as_u16(), 401);
    let signin_body: serde_json::Value = signin_response
        .json()
        .await
        .expect("Failed to parse signin response body");
    assert!(signin_body.get("message").is_some());
    assert_eq!(
        signin_body.get("message").unwrap().as_str().unwrap(),
        "Email not confirmed"
    );
}
