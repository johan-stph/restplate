use crate::server_setup::{close_and_delete_db, spawn_app_local};

mod server_setup;

#[tokio::test]
async fn health_check_working() {
    let mock_app = spawn_app_local().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/health_check", mock_app.address))
        .send()
        .await
        .expect("Failed to execute request");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    close_and_delete_db(mock_app).await;
}
