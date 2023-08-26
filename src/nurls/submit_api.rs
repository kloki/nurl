use actix_web::{
    http::{
        header::ContentType,
        StatusCode,
    },
    web::{
        self,
        Query,
    },
    HttpResponse,
    ResponseError,
    Result,
};
use askama::Template;
use lazy_static::lazy_static;

use super::models::{
    Nurl,
    Nurlet,
};
use crate::{
    db::DBClient,
    startup::ApplicationBaseUrl,
};

#[derive(Template)]
#[template(path = "submit.html")]
struct Submit {}

lazy_static! {
    static ref SUBMIT: String = Submit {}.render().unwrap();
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
pub async fn submit_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(SUBMIT.clone())
}

#[derive(serde::Deserialize)]
pub struct SubmitJson {
    title: String,
    urls: Vec<Nurlet>,
}

impl SubmitJson {
    fn build(self) -> Nurl {
        let mut nurl = Nurl::default();
        nurl.urls = self.urls;
        nurl.title = self.title;
        nurl
    }
}

#[derive(serde::Serialize)]
pub struct SubmitReturn {
    id: String,
}

pub async fn submit(
    json: web::Json<SubmitJson>,
    db: web::Data<DBClient>,
) -> Result<web::Json<SubmitReturn>, SubmitError> {
    let nurl = json.0.build();
    db.save_nurl(&nurl)
        .await
        .map_err(|_e| SubmitError::DBError)?;

    Ok(web::Json(SubmitReturn {
        id: nurl.id.to_string(),
    }))
}

#[derive(serde::Deserialize)]
pub struct NurlQP {
    nurl: String,
}

pub async fn submit_complete(
    base_url: web::Data<ApplicationBaseUrl>,
    qp: Query<NurlQP>,
) -> Result<HttpResponse, SubmitError> {
    let submit_complete = SubmitComplete {
        nurl: &format!("{}/{}", base_url.0, qp.nurl),
    };
    Ok(HttpResponse::Ok().content_type(ContentType::html()).body(
        submit_complete
            .render()
            .map_err(|_e| SubmitError::RenderError)?,
    ))
}

#[cfg(test)]
mod tests {
    use super::SubmitJson;

    #[test]
    fn test_build() {
        let json = SubmitJson {
            title: "title".to_string(),
            urls: vec![
                "hello".to_owned().try_into().unwrap(),
                "https://www.google.nl".to_owned().try_into().unwrap(),
            ],
        };
        json.build();
    }
}
