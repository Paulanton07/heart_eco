use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),
    
    #[error("Parse error: {0}")]
    ParseError(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type AppResult<T> = Result<T, AppError>;

/// Initialize error handling for the application
pub fn init_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    // Set up error handling - use a safer method than set_var
    std::env::var("RUST_BACKTRACE").unwrap_or_else(|_| {
        std::env::var_os("RUST_BACKTRACE").map(|_| "1".to_string()).unwrap_or_else(|| {
            if cfg!(debug_assertions) {
                "1".to_string()
            } else {
                "0".to_string()
            }
        })
    });
    
    // Initialize logger if not already done
    if std::env::var("RUST_LOG").is_err() {
        // Instead of setting directly, we'll just use the default if not set
        let _ = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    }
    
    // This will be a no-op if the logger is already initialized
    match env_logger::try_init() {
        Ok(_) => log::info!("Logger initialized"),
        Err(_) => {}, // Logger likely already initialized
    }
    
    Ok(())
}

/// Convert any error to an AppError
pub fn map_error<E: std::error::Error + Send + Sync + 'static>(err: E) -> AppError {
    log::error!("Error occurred: {}", err);
    AppError::Unknown(err.to_string())
}

/// Implementation for converting Box<dyn Error> to AppError
impl From<Box<dyn std::error::Error>> for AppError {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        AppError::Unknown(error.to_string())
    }
}

/// Implementation for converting strings to AppError
impl From<String> for AppError {
    fn from(error: String) -> Self {
        AppError::Unknown(error)
    }
}

/// Implementation for converting &str to AppError
impl From<&str> for AppError {
    fn from(error: &str) -> Self {
        AppError::Unknown(error.to_string())
    }
}
