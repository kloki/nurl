use std::net::TcpListener;

use actix_web::{
    dev::Server,
    http::StatusCode,
    middleware::ErrorHandlers,
    web,
    App,
    HttpServer,
};
use tracing_actix_web::TracingLogger;

use crate::{
    banner,
    base,
    configuration::Settings,
    db::DBClient,
    error_handlers,
    nurls,
};
pub struct Application {
    port: u16,
    server: Server,
}
impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address).expect("Failed to bind to port");
        let port = listener.local_addr().unwrap().port();
        let db_client = DBClient::new(&configuration.database);

        let server = run(listener, db_client, configuration.application.base_url)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

pub fn run(
    listener: TcpListener,
    db_client: DBClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    let db_client = web::Data::new(db_client);
    let base_url = web::Data::new(ApplicationBaseUrl(base_url));
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, error_handlers::not_found))
            .route("/", web::get().to(nurls::submit_form))
            .route("/submit", web::post().to(nurls::submit))
            .route("/submit/complete", web::get().to(nurls::submit_complete))
            .route("/health_check", web::get().to(base::health_check))
            .service(banner::banner)
            .service(nurls::view_nurl)
            .app_data(db_client.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
