use breezeehr::utils::tracing::init_tracing;

use crate::helpers::TestApp;

#[tokio::test]
async fn health_check_should_return_200() {
    init_tracing("info");
    let app = TestApp::new().await;

    let response = app.health_check().await;
    assert_eq!(response.status().as_u16(), 200);
}
