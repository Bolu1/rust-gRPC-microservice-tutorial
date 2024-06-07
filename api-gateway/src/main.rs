use actix_rt::spawn;
use api_gateway::core::{app::init_service_client, config::get_config};
use api_gateway::models::schemas::app::Application;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();
    let service_client = init_service_client(config.server.clone()).await?;
    let application = Application::build(config, service_client).await?;
    let _ = spawn(async {
        let _ = application.run_until_stopped().await;
    })
    .await;

    Ok(())
}
