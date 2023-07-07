use crate::helpers::spawn_db_client;

#[tokio::test]
async fn create_nurl() {
    let _db = spawn_db_client().await;
}
