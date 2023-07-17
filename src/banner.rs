use askama::Template;

use actix_web::{get, http::header::ContentType, web, HttpResponse};

#[derive(Template)]
#[template(path = "banner.html")]
struct Banner<'a> {
    word: &'a str,
}

#[get("/banner/{word}")] // <- define path parameters
pub async fn banner(path: web::Path<String>) -> HttpResponse {
    let banner = Banner {
        word: &path.into_inner(),
    };

    match banner.render() {
        Ok(s) => HttpResponse::Ok().content_type(ContentType::html()).body(s),
        Err(_) => HttpResponse::Ok()
            .content_type(ContentType::html())
            .body("Oopsie"),
    }
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
