use dotenv::dotenv;
use std::env;

#[derive(Clone, Debug)]
pub struct ServerSettings {
    pub env: Environment,
    pub host: String,
    pub port: u16,
    pub task_service_url: String,
}

impl ServerSettings {
    pub fn init() -> ServerSettings {
        dotenv().ok();
        let env: Environment = env::var("APP_ENVIRONMENT")
            .unwrap_or_else(|_| "development".into())
            .try_into()
            .expect("Failed to read APP_ENVIRONMENT.");
        let host: String = std::env::var("SERVER_HOST").expect("Failed to read SERVER_HOST.");
        let port: u16 = env::var("SERVER_PORT")
            .expect("Failed to read SERVER_PORT.")
            .parse()
            .unwrap();
        let task_service_url: String = env::var("TASK_SERVICE_URL").expect("Failed to read TASK_SERVICE_URL.");

        ServerSettings {
            env,
            host,
            port,
            task_service_url,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Settings {
    pub server: ServerSettings,
}

impl Settings {
    pub fn init() -> Settings {
        let server = ServerSettings::init();

        Settings { server }
    }
}

pub fn get_config() -> Settings {
    dotenv().ok();
    Settings::init()
}

#[derive(Clone, Debug)]
pub enum Environment {
    Development,
    Production,
    Staging,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
            Environment::Staging => "staging",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            "staging" => Ok(Self::Staging),
            other => Err(format!(
                "{} is not a valid environment. Use either `development` or `production`.",
                other
            )),
        }
    }
}
