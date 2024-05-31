use crate::models::schemas::app::TaskServiceService;
use anyhow::Error;
use grpc_service::task_service;
use grpc_service::task_service::task_service_server::TaskServiceServer;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic_reflection::server::Builder;

use crate::models::schemas::app::AppState;
use crate::models::schemas::app::Application;
use crate::repository::db::Database;

use crate::core::config::Settings;

fn event_logger() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    let _ = env_logger::try_init();
}

impl Application {
    pub async fn build(config: Settings) -> Result<Self, Error> {
        let database = Database::connect(&config, true).await;
        let address = format!("{}:{}", config.server.host, config.server.port);
        let socket_address: SocketAddr = address.parse().expect("Error creating socket address");
        let app_state: AppState = AppState {
            config: config.clone(),
            db: database,
        };
        Ok(Self {
            socket_address,
            app_state,
        })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        event_logger();
        println!("Server is running on {} 🚀🚀🚀", self.socket_address);
        let task_service = TaskServiceService {
            app_state: self.app_state.clone(),
        };

        let reflection_service = Builder::configure()
            .register_encoded_file_descriptor_set(task_service::FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();

        Server::builder()
            .add_service(TaskServiceServer::new(task_service))
            .add_service(reflection_service)
            .serve(self.socket_address)
            .await?;

        Ok(())
    }
}
