use crate::core::config::Settings;
use actix_web::dev::Server;
use grpc_service::task_service::task_service_client::TaskServiceClient;
use tonic::transport::Channel;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Settings,
    pub service_client: ServiceClient,
}

pub struct Application {
    pub port: u16,
    pub server: Server,
}

#[derive(Debug, Clone)]
pub struct ServiceClient {
    pub task_service_client: Option<TaskServiceClient<Channel>>,
}
