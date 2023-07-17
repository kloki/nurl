use super::models::Nurl;
use crate::db::DBClient;
use actix_web::{get, http::header::ContentType, web, HttpResponse};
use askama::Template;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "nurl.html")]
struct NurlTemplate {
    uuid: String,
    urls: Vec<String>,
    views: i32,
}

impl Nurl {
    fn template(&self) -> NurlTemplate {
        NurlTemplate {
            uuid: self.id.to_string(),
            urls: self.urls.iter().map(|s| s.to_string()).collect(),
            views: self.views,
        }
    }
}

#[get("/{uuid}")] // <- define path parameters
pub async fn view_nurl(path: web::Path<String>, db: web::Data<DBClient>) -> HttpResponse {
    let uuid = Uuid::parse_str(&path.into_inner()).unwrap();
    let mut nurl = db.get_nurl(uuid).await.unwrap().unwrap();
    db.add_view(&nurl).await.unwrap();
    nurl.views += 1;
    match nurl.template().render() {
        Ok(s) => HttpResponse::Ok().content_type(ContentType::html()).body(s),
        Err(_) => HttpResponse::Ok()
            .content_type(ContentType::html())
            .body("Oopsie"),
    }
}
