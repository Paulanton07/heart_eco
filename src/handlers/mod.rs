// Placeholder for handler modules
// Will be implemented in future phases

// Re-export handler modules
// pub mod user;
// pub mod product;
// pub mod cart;
// pub mod checkout;

/// Health check handler
pub async fn health_check() -> &'static str {
    "OK"
}

