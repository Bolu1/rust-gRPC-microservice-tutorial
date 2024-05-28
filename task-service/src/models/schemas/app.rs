use crate::core::config::Settings;
use crate::repository::db::Database;
use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct AppState {
    pub config: Settings,
    pub db: Database,
}

pub struct Application {
    pub socket_address: SocketAddr,
    pub app_state: AppState,
}

#[derive(Debug, Clone)]
pub struct ProjectServiceService {
    pub app_state: AppState,
}
