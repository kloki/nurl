use askama::Template;

use actix_web::http::StatusCode;
use actix_web::{get, http::header::ContentType, web, HttpResponse, ResponseError, Result};

#[derive(Template)]
#[template(path = "banner.html")]
struct Banner<'a> {
    word: &'a str,
}

#[derive(thiserror::Error, Debug)]
pub enum BannerError {
    #[error("Failed to render template")]
    RenderError,
}

impl ResponseError for BannerError {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
#[get("/banner/{word}")] // <- define path parameters
pub async fn banner(path: web::Path<String>) -> Result<HttpResponse, BannerError> {
    let banner = Banner {
        word: &path.into_inner(),
    };

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(banner.render().map_err(|_e| BannerError::RenderError)?))
}

#[cfg(test)]
mod tests {
    use super::Banner;
    use askama::Template;
    #[tokio::test]
    async fn test_naughty_strings() {
        for input in vec!["Hello", "Hello/", "{}{}", "🙂"] {
            assert!(Banner { word: input }.render().is_ok())
        }
    }
}
