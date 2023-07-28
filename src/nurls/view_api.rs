use super::models::Nurl;
use crate::db::DBClient;

use actix_web::http::StatusCode;
use actix_web::{get, http::header::ContentType, web, HttpResponse, ResponseError, Result};
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
#[derive(thiserror::Error, Debug)]
pub enum NurlViewError {
    #[error("Failed to render template")]
    RenderError,
    #[error("Failed to reach the db")]
    DBError,
}

impl ResponseError for NurlViewError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

#[get("/{uuid}")] // <- define path parameters
pub async fn view_nurl(
    path: web::Path<String>,
    db: web::Data<DBClient>,
) -> Result<Option<HttpResponse>, NurlViewError> {
    let uuid = match Uuid::parse_str(&path.into_inner()) {
        Err(_) => return Ok(None),
        Ok(s) => s,
    };

    let mut nurl = match db
        .get_nurl(uuid)
        .await
        .map_err(|_e| NurlViewError::DBError)?
    {
        None => return Ok(None),
        Some(s) => s,
    };
    db.add_view(&nurl)
        .await
        .map_err(|_e| NurlViewError::DBError)?;
    nurl.views += 1;
    Ok(Some(
        HttpResponse::Ok().content_type(ContentType::html()).body(
            nurl.template()
                .render()
                .map_err(|_e| NurlViewError::RenderError)?,
        ),
    ))
}
