use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::http::header;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::Error;
use grpc_service::task_service::task_service_client::TaskServiceClient;
use std::net::TcpListener;

use crate::routes::task::task_scope;
use crate::core::config::Settings;
use crate::models::schemas::app::{AppState, Application, ServiceClient};
use tonic::transport::Channel;

use super::config::ServerSettings;

fn event_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let _ = env_logger::try_init();
}

// Make the clients optional
pub async fn init_service_client(service: ServerSettings) -> Result<ServiceClient, Error> {
    let task_service_client: Option<TaskServiceClient<Channel>> =
        match TaskServiceClient::connect(service.task_service_url).await {
            Ok(client) => Some(client),
            Err(_) => None,
        };

    Ok(ServiceClient {
        task_service_client,
    })
}

impl Application {
    pub async fn build(config: Settings, service_client: ServiceClient) -> Result<Self, Error> {
        let address = format!("{}:{}", config.server.host, config.server.port);
        let listener: TcpListener = TcpListener::bind(address)?;
        let port: u16 = listener.local_addr().unwrap().port();
        let app_state: AppState = AppState {
            config: config.clone(),
            service_client: service_client.clone()
        };
        let server = run(listener, app_state).await?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

async fn run(listener: TcpListener, app_state: AppState) -> Result<Server, anyhow::Error> {
    event_logger();
    println!(
        "🛣️🛣️  API gateway is running on http://{}:{} 🛣️🛣️",
        app_state.config.server.host, app_state.config.server.port
    );

    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://0.0.0.0:3000")
            .allowed_origin("http://localhost:5173")
            .allowed_origin("http://0.0.0.0:5173")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PATCH", "PUT"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(task_scope())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
