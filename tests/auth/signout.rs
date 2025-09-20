use sovaehr::utils::tracing::init_tracing;

use crate::helpers::TestApp;

#[tokio::test]
async fn signout_with_valid_token_should_return_200() {
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

    let token = response_body
        .get("token")
        .and_then(|v| v.as_str())
        .expect("token not found in response");

    let signout_response = app.post_signout(token).await;
    assert_eq!(signout_response.status().as_u16(), 200);
}

#[tokio::test]
async fn signout_with_invalid_token_should_return_401() {
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

    let token = response_body
        .get("token")
        .and_then(|v| v.as_str())
        .expect("token not found in response");

    let response = app.post_signout("invalid_token").await;
    assert_eq!(response.status().as_u16(), 401);

    let signout_response = app.post_signout(token).await;
    assert_eq!(signout_response.status().as_u16(), 200);
}
