use super::models::Nurl;
use crate::db::DBClient;
use crate::startup::ApplicationBaseUrl;
use actix_web::get;
use actix_web::http::StatusCode;
use actix_web::web::{self, Redirect};
use actix_web::{http::header::ContentType, HttpResponse, ResponseError, Result};
use askama::Template;
use url::Url;

#[derive(Template)]
#[template(path = "submit.html")]
struct Submit<'a> {
    word: &'a str,
}

#[derive(Template)]
#[template(path = "submit_complete.html")]
struct SubmitComplete<'a> {
    nurl: &'a str,
}

#[derive(thiserror::Error, Debug)]
pub enum SubmitError {
    #[error("Failed to render template")]
    RenderError,
    #[error("Failed to reach the db")]
    DBError,
}

impl ResponseError for SubmitError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
pub async fn submit_form() -> Result<HttpResponse, SubmitError> {
    let submit = Submit { word: "hello" };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(submit.render().map_err(|_e| SubmitError::RenderError)?))
}

#[derive(serde::Deserialize)]
pub struct SubmitForm {
    url_1: Url,
    connection: String,
    url_2: Url,
}

impl SubmitForm {
    fn build(&self, base_url: &str) -> Nurl {
        let mut nurl = Nurl::default();
        nurl.urls = vec![
            self.url_1.clone(),
            Url::parse(&format!("{}/banner/{}", base_url, self.connection)).unwrap(),
            self.url_2.clone(),
        ];
        nurl
    }
}

pub async fn submit(
    form: web::Form<SubmitForm>,
    db: web::Data<DBClient>,
    base_url: web::Data<ApplicationBaseUrl>,
) -> Result<Redirect, SubmitError> {
    let nurl = form.0.build(&base_url.0);
    db.save_nurl(&nurl)
        .await
        .map_err(|_e| SubmitError::DBError)?;
    Ok(Redirect::new(
        "/submit",
        format!("/submit/complete/{}", nurl.id.to_string()),
    )
    .using_status_code(StatusCode::FOUND))
}

#[get("/submit/complete/{nurl}")] // <- define path parameters
pub async fn submit_complete(
    base_url: web::Data<ApplicationBaseUrl>,
    path: web::Path<String>,
) -> Result<HttpResponse, SubmitError> {
    let submit_complete = SubmitComplete {
        nurl: &format!("{}/{}", base_url.0, path.into_inner()),
    };
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        submit_complete
            .render()
            .map_err(|_e| SubmitError::RenderError)?,
    ))
}
