use crate::helpers::spawn_db_client;
use nurl::nurls::Nurl;

#[tokio::test]
async fn test_nurl_does_not_exist() {
    let db = spawn_db_client().await;
    let nurl = Nurl::new(vec!["http://www.google.nl"]).unwrap();
    let result = db.get_nurl(nurl.id).await;
    assert!(result.is_ok());
    let obj = result.unwrap();
    assert!(obj.is_none());
}

#[tokio::test]
async fn test_exist() {
    let db = spawn_db_client().await;
    let nurl = Nurl::new(vec!["http://www.google.nl"]).unwrap();
    let result = db.save_nurl(&nurl).await;
    assert!(result.is_ok());
    let result = db.get_nurl(nurl.id).await;
    assert!(result.is_ok());
    let obj = result.unwrap();
    assert!(obj.is_some());
}
