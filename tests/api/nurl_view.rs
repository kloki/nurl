use nurl::nurls::Nurl;
use uuid::Uuid;

use crate::helpers::spawn_app;

#[tokio::test]
async fn test_invalid_uuid() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/{}", &app.address, "wrong"))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status().as_u16(), 404);
}

#[tokio::test]
async fn test_nurl_doesnt_exist() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/{}", &app.address, Uuid::new_v4()))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status().as_u16(), 404);
}
#[tokio::test]
async fn test_nurl_success() {
    let app = spawn_app().await;
    let nurl = Nurl::new("test", vec!["http://www.google.nl", "http://www.google.de"]).unwrap();
    app.db_client.save_nurl(&nurl).await.unwrap();
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/{}", &app.address, nurl.id))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status().as_u16(), 200);
}
