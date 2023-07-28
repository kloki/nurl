use super::models::Nurl;
use crate::db::DBClient;
use actix_web::web;
use actix_web::{http::header::ContentType, HttpResponse};
use askama::Template;
use url::Url;

#[derive(Template)]
#[template(path = "submit.html")]
struct Submit<'a> {
    word: &'a str,
}

pub async fn submit_form() -> HttpResponse {
    let submit = Submit { word: "hello" };
    match submit.render() {
        Ok(s) => HttpResponse::Ok().content_type(ContentType::html()).body(s),
        Err(_) => HttpResponse::Ok()
            .content_type(ContentType::html())
            .body("Oopsie"),
    }
}

#[derive(serde::Deserialize)]
pub struct SubmitForm {
    url_1: Url,
    connection: String,
    url_2: Url,
}

impl SubmitForm {
    fn build(&self) -> Nurl {
        let mut nurl = Nurl::default();
        nurl.urls = vec![
            self.url_1.clone(),
            Url::parse(&format!("http://localhost:8000/banner/{}", self.connection)).unwrap(),
            self.url_2.clone(),
        ];
        nurl
    }
}

pub async fn submit(form: web::Form<SubmitForm>, db: web::Data<DBClient>) -> HttpResponse {
    let nurl = form.0.build();
    db.save_nurl(&nurl).await.unwrap();
    HttpResponse::Created().body(format!(
        "<a href=\"http://localhost:8000/{}\"> Click <\\a>",
        nurl.id.to_string()
    ))
}
