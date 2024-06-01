use task_service::core::config::get_config;
use task_service::models::schemas::app::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();
    let application = Application::build(config).await?;
    application.run().await?;

    Ok(())
}
