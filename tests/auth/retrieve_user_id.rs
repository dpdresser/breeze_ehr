use sovaehr::utils::tracing::init_tracing;

use crate::helpers::TestApp;

#[tokio::test]
async fn retrieve_user_id_returns_200() {
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

    let response = app.post_retrieve_user_id(&email).await;
    assert_eq!(response.status().as_u16(), 200);
    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body");
    assert!(response_body.get("user_id").is_some());
}
