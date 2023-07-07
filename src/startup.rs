use crate::base;
use crate::configuration::Settings;
use crate::db::DBClient;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

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

        let server = run(listener, db_client)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(listener: TcpListener, db_client: DBClient) -> Result<Server, std::io::Error> {
    let db_client = web::Data::new(db_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/", web::get().to(base::hello))
            .route("/health_check", web::get().to(base::health_check))
            .app_data(db_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
