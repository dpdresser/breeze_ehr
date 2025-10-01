use breezeehr::utils::tracing::init_tracing;
use uuid::Uuid;

use crate::helpers::TestApp;

#[tokio::test]
async fn signin_with_valid_credentials_should_return_200_and_token() {
    init_tracing("info");
    let app = TestApp::new().await;

    let email = "owner1@example.com";
    let password = "Password123!";
    let response = app.post_signin(email, password).await;

    assert_eq!(response.status().as_u16(), 200);
    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body");
    assert!(response_body.get("token").is_some());
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

    let verification_url = app.verification_link(&email).await;

    let verify_response = app
        .http_client
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

    let token = signin_body
        .get("token")
        .and_then(|v| v.as_str())
        .expect("token not found in response");

    let response = app.post_retrieve_user_id(token, &email).await;
    assert_eq!(response.status().as_u16(), 200);
    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body");
    let user_id = response_body
        .get("user_id")
        .and_then(|v| v.as_str())
        .expect("user_id not found in response");

    let delete_response = app.delete_user(token, user_id).await;
    assert_eq!(delete_response.status().as_u16(), 200);
}
