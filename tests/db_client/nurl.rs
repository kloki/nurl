use crate::helpers::spawn_db_client;
use nurl::nurls::Nurl;
use uuid::Uuid;

fn assert_exist<T, U: std::fmt::Debug>(obj: Result<Option<T>, U>) -> T {
    assert!(obj.is_ok(), "db call caused an error");
    let obj = obj.unwrap();
    assert!(obj.is_some(), "obj does not exist in db");
    obj.unwrap()
}

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
    let obj = assert_exist(result);
    assert_eq!(obj.urls.len(), 1);
}
#[tokio::test]
async fn test_get_url_set() {
    let db = spawn_db_client().await;
    let nurl = Nurl::new(vec!["http://www.google.nl", "http://www.google.de"]).unwrap();
    let result = db.save_nurl(&nurl).await;
    assert!(result.is_ok());
    let result = db.get_url_set(nurl.id).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 2);
}
#[tokio::test]
async fn test_get_url_set_does_not_exist() {
    let db = spawn_db_client().await;
    let result = db.get_url_set(Uuid::new_v4()).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[tokio::test]
async fn test_count_views() {
    let mut db = spawn_db_client().await;
    let nurl = Nurl::new(vec!["http://www.google.nl"]).unwrap();
    let result = db.save_nurl(&nurl).await;
    assert!(result.is_ok());
    let result = db.get_nurl(nurl.id).await;
    let nurl = assert_exist(result);
    assert_eq!(nurl.views, 0);
    let result = db.add_view(&nurl).await;
    assert!(result.is_ok());
    let result = db.get_nurl(nurl.id).await;
    let nurl = assert_exist(result);
    assert_eq!(nurl.views, 1);
}
