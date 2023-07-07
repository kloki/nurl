use actix_web::{HttpRequest, HttpResponse, Responder};


pub async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn hello(_req: HttpRequest) -> impl Responder {
    "🙈 🙉 🙊"
}
