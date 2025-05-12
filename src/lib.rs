//! Wood Planks E-commerce Application
//!
//! A web application for browsing and purchasing recycled wood planks
//! with AI assistant for finding the right products for customer needs.

// Re-export modules
pub mod models;
pub mod utils;
pub mod config;
pub mod handlers;
pub mod services;
pub mod ai;

// Re-export types for convenience
pub use models::wood_plank::{
    WoodPlank, NewWoodPlank, WoodType, ProductCategory,
    ProductGrade, FinishType, WoodPlankQuery
};

pub use config::Config;

// Constants
pub const APP_NAME: &str = "Heart Eco Wood Planks";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the application
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // Will be expanded later with more initialization logic
    utils::error::init_error_handling()?;
    Ok(())
}

