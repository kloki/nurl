use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, http::header::ContentType, HttpResponse, Result};

use askama::Template;
#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFound {}
pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_response = HttpResponse::NotFound()
        .content_type(ContentType::html())
        .body(NotFound {}.render().unwrap());

    Ok(ErrorHandlerResponse::Response(
        dev::ServiceResponse::new(res.into_parts().0, new_response).map_into_right_body(),
    ))
}
