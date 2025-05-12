use dotenv::dotenv;
use serde::Deserialize;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
    pub secret_key: String,
    pub api_timeout: u64,
    pub allowed_origins: Vec<String>,
}

impl Config {
    /// Extract database host from the connection string
    pub fn database_host(&self) -> String {
        // Format: postgres://username:password@host:port/database
        if let Some(host_part) = self.database_url.split('@').nth(1) {
            if let Some(host) = host_part.split(':').next() {
                return host.to_string();
            }
        }
        "localhost".to_string()
    }
    
    /// Extract database port from the connection string
    pub fn database_port(&self) -> u16 {
        // Format: postgres://username:password@host:port/database
        if let Some(host_part) = self.database_url.split('@').nth(1) {
            if let Some(port_db) = host_part.split(':').nth(1) {
                if let Some(port) = port_db.split('/').next() {
                    if let Ok(port_num) = port.parse::<u16>() {
                        return port_num;
                    }
                }
            }
        }
        5432 // Default PostgreSQL port
    }
    
    /// Extract database username from the connection string
    pub fn database_username(&self) -> String {
        // Format: postgres://username:password@host:port/database
        if let Some(auth_part) = self.database_url.split("://").nth(1) {
            if let Some(username) = auth_part.split(':').next() {
                return username.to_string();
            }
        }
        "postgres".to_string() // Default username
    }
    
    /// Extract database password from the connection string
    pub fn database_password(&self) -> String {
        // Format: postgres://username:password@host:port/database
        if let Some(auth_part) = self.database_url.split("://").nth(1) {
            if let Some(pass_part) = auth_part.split(':').nth(1) {
                if let Some(password) = pass_part.split('@').next() {
                    return password.to_string();
                }
            }
        }
        "".to_string() // Empty password
    }
    
    /// Extract database name from the connection string
    pub fn database_name(&self) -> String {
        // Format: postgres://username:password@host:port/database
        if let Some(db_part) = self.database_url.split('/').last() {
            return db_part.to_string();
        }
        "postgres".to_string() // Default database name
    }
}

impl Config {
    pub fn init() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set in .env file");
        
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("SERVER_PORT must be a valid port number");
        
        let secret_key = env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set in .env file");
            
        let api_timeout = env::var("API_TIMEOUT")
            .unwrap_or_else(|_| "30".to_string())
            .parse::<u64>()
            .expect("API_TIMEOUT must be a valid number in seconds");
            
        let allowed_origins = env::var("ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:3000,http://localhost:8080".to_string())
            .split(',')
            .map(|s| s.to_string())
            .collect();
        
        Self {
            database_url,
            server_port,
            secret_key,
            api_timeout,
            allowed_origins,
        }
    }
}

