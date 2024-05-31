use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::core::config::Settings;

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: Pool<Postgres>,
}

impl Database {
    pub async fn connect(config: &Settings, is_migrate: bool) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&config.db.dns)
            .await
            .expect("Failed to connect to postgres.");

        println!("DB successfully connected ✅");

        if is_migrate {
            Self::migrate(&pool).await;
        }

        Database { pool }
    }

    pub async fn disconnect(&self) {
        self.pool.close().await;
        println!("DB connection disconnected successfully ✅");
    }

    pub async fn migrate(pool: &Pool<Postgres>) {
        match sqlx::migrate!("./migrations").run(pool).await {
            Ok(_) => println!("Migrations executed successfully."),
            Err(e) => eprintln!("Error executing migrations: {}", e),
        };
    }
}
