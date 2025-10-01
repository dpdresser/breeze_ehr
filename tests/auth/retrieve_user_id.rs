use breeze_ehr::utils::tracing::init_tracing;

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
    let token = response_body
        .get("token")
        .and_then(|v| v.as_str())
        .expect("token not found in response");

    let response = app.post_retrieve_user_id(token, &email).await;
    assert_eq!(response.status().as_u16(), 200);
    let response_body: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse response body");
    assert!(response_body.get("user_id").is_some());
}
