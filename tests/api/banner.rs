use crate::helpers::spawn_app;

#[tokio::test]
async fn test_naught_strings() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    for input in vec!["Hello", "--ashle--", "{}{}", "🙂"] {
        let response = client
            .get(&format!("{}/banner/{}", &app.address, input))
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(
            response.status().is_success(),
            "Banner should be able to handle{}",
            input
        );
    }
}
#[tokio::test]
async fn test_naught_strings_that_should_fail() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    for input in vec!["/hellor", "?=whaat"] {
        let response = client
            .get(&format!("{}/banner/{}", &app.address, input))
            .send()
            .await
            .expect("Failed to execute request.");
        assert!(
            response.status().is_client_error(),
            "Banner should no render {}",
            input
        );
    }
}
