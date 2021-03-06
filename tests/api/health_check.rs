use crate::app::TestApp;

#[actix_rt::test]
async fn test_health_check() {
    let app = TestApp::new().await;

    let response = reqwest::Client::new()
        .get(format!("{}/health-check", app.address))
        .send()
        .await
        .expect("failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}
