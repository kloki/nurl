use actix_web::body::MessageBody;
use actix_web::middleware::ErrorHandlerResponse;
use actix_web::{dev, http::header::ContentType, HttpResponse, Result};
use lazy_static::lazy_static;

use askama::Template;
#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFound {}

lazy_static! {
    static ref NOT_FOUND: String = NotFound {}.render().unwrap();
}

pub fn not_found<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let new_response = HttpResponse::NotFound()
        .content_type(ContentType::html())
        .body(NOT_FOUND.clone());

    Ok(ErrorHandlerResponse::Response(
        dev::ServiceResponse::new(res.into_parts().0, new_response).map_into_right_body(),
    ))
}

#[derive(Template)]
#[template(path = "bad_request.html")]
struct BadRequest<'a> {
    error: &'a str,
}
pub fn bad_request<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>>
where
    B: MessageBody,
    <B as MessageBody>::Error: std::fmt::Debug,
{
    let (req, _res) = res.into_parts();
    let new_response = HttpResponse::BadRequest()
        .content_type(ContentType::html())
        .body(BadRequest { error: "Oopsie" }.render().unwrap());

    Ok(ErrorHandlerResponse::Response(
        dev::ServiceResponse::new(req, new_response).map_into_right_body(),
    ))
}
