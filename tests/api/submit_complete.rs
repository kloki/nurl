use crate::helpers::spawn_app;

#[tokio::test]
async fn test_include_qp() {
    //qp is not validated
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!(
            "{}/submit/complete?nurl={}",
            &app.address, "wrong"
        ))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status().as_u16(), 200);
}
#[tokio::test]
async fn test_missing_qp() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/submit/complete", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status().as_u16(), 400);
}
