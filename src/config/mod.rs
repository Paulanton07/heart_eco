mod config;

pub use config::Config;

use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;

/// Database configuration helper functions
pub mod db {
    use super::*;
    use crate::utils::error::AppResult;
    use log::info;
    use std::time::Duration;

    /// Create and return a database connection pool
    pub async fn create_pool(config: &Config) -> AppResult<sqlx::PgPool> {
        info!("Creating database connection pool...");
        
        let options = PgConnectOptions::new()
            .host(&config.database_host())
            .port(config.database_port())
            .username(&config.database_username())
            .password(&config.database_password())
            .database(&config.database_name())
            .log_statements(log::LevelFilter::Debug)
            .log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(1));
        
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
            
        info!("Database connection pool created successfully");
        Ok(pool)
    }
}
