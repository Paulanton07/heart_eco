use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

/// Errors related to shopping cart operations
#[derive(Debug, Error)]
pub enum CartError {
    #[error("Item not in stock: {0}")]
    OutOfStock(Uuid),
    
    #[error("Invalid quantity: {0}")]
    InvalidQuantity(String),
    
    #[error("Item not found in cart: {0}")]
    ItemNotFound(Uuid),
}

/// Shopping cart with items
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Cart {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Individual item in a shopping cart
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CartItem {
    pub id: Uuid,
    pub cart_id: Uuid,
    pub wood_plank_id: Uuid,
    pub quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    
    // Denormalized fields for performance
    #[serde(skip_deserializing)]
    pub wood_plank_name: Option<String>,
    #[serde(skip_deserializing)]
    pub wood_plank_price: Option<f64>,
    #[serde(skip_deserializing)]
    pub wood_plank_image_url: Option<String>,
}

/// DTO for adding items to cart
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddToCartRequest {
    pub wood_plank_id: Uuid,
    pub quantity: i32,
}

/// DTO for cart summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartSummary {
    pub cart_id: Uuid,
    pub items: Vec<CartItemSummary>,
    pub total_items: i32,
    pub subtotal: f64,
}

/// DTO for cart item details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartItemSummary {
    pub id: Uuid,
    pub wood_plank_id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
    pub image_url: Option<String>,
    pub item_subtotal: f64,
}

impl Cart {
    /// Create a new cart
    pub fn new(user_id: Option<Uuid>, session_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            session_id,
            created_at: now,
            updated_at: now,
        }
    }
}

impl CartItem {
    /// Create a new cart item
    pub fn new(cart_id: Uuid, wood_plank_id: Uuid, quantity: i32) -> Result<Self, CartError> {
        if quantity <= 0 {
            return Err(CartError::InvalidQuantity("Quantity must be positive".to_string()));
        }
        
        let now = Utc::now();
        Ok(Self {
            id: Uuid::new_v4(),
            cart_id,
            wood_plank_id,
            quantity,
            created_at: now,
            updated_at: now,
            wood_plank_name: None,
            wood_plank_price: None,
            wood_plank_image_url: None,
        })
    }
    
    /// Calculate the subtotal for this item
    pub fn subtotal(&self) -> Option<f64> {
        self.wood_plank_price.map(|price| price * self.quantity as f64)
    }
}

