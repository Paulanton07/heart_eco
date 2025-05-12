use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;
use sqlx::types::BigDecimal;

/// Represents wood types available in the inventory
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "wood_type", rename_all = "lowercase")]
pub enum WoodType {
    Baltic,
    Pine,
    Oak,
    Recycled,
    Mixed,
}

/// Represents product categories from the price list
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "product_category", rename_all = "lowercase")]
pub enum ProductCategory {
    HeavyDutyBox,
    Pallet,
    LongTimber,
    ShortTimber,
    PlanedTimber,
    MachinedTimber,
    Component,
    LaminatedTable,
    Plywood,
    Custom,
}

/// Represents quality grades of wood products
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "product_grade", rename_all = "lowercase")]
pub enum ProductGrade {
    AGrade,
    BGrade,
    Standard,
}

/// Represents finish types for wood products
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq)]
#[sqlx(type_name = "finish_type", rename_all = "lowercase")]
pub enum FinishType {
    Rough,
    PlanedBothSides,  // PBS
    PlanedAllRound,   // PAR
    Machined,
    Laminated,
    Raw,
}

/// Errors related to wood plank validation
#[derive(Debug, Error)]
pub enum WoodPlankError {
    #[error("Invalid dimensions: {0}")]
    InvalidDimensions(String),
    
    #[error("Invalid price: {0}")]
    InvalidPrice(String),
    
    #[error("Invalid stock quantity: {0}")]
    InvalidStock(String),
}

/// Full wood plank details as stored in the database
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct WoodPlank {
    pub id: Uuid,
    pub sku: String,                    // Stock Keeping Unit for inventory tracking
    pub name: String,
    pub category: ProductCategory,      // Category from price list
    pub wood_type: WoodType,            // Type of wood
    pub grade: ProductGrade,            // A Grade or B Grade
    pub finish: FinishType,             // Finish type
    pub thickness_mm: i32,              // Thickness in millimeters
    pub width_mm: i32,                  // Width in millimeters
    pub length_mm: i32,                 // Length in millimeters
    pub price: BigDecimal,             // Price in Rands
    pub stock_quantity: i32,
    pub unit_of_measure: String,        // EA (each), etc.
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Used for creating new wood plank entries
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewWoodPlank {
    pub sku: String,
    pub name: String,
    pub category: ProductCategory,
    pub wood_type: WoodType,
    pub grade: ProductGrade,
    pub finish: FinishType,
    pub thickness_mm: i32,
    pub width_mm: i32,
    pub length_mm: i32,
    pub price: BigDecimal,
    pub stock_quantity: i32,
    pub unit_of_measure: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

/// Used for querying wood planks with filters
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WoodPlankQuery {
    pub category: Option<ProductCategory>,
    pub wood_type: Option<WoodType>,
    pub grade: Option<ProductGrade>,
    pub finish: Option<FinishType>,
    pub min_length: Option<i32>,
    pub max_length: Option<i32>,
    pub min_width: Option<i32>,
    pub max_width: Option<i32>,
    pub min_thickness: Option<i32>,
    pub max_thickness: Option<i32>,
    pub min_price: Option<BigDecimal>,
    pub max_price: Option<BigDecimal>,
    pub in_stock: Option<bool>,
    pub search_term: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

impl NewWoodPlank {
    /// Validate the wood plank data
    pub fn validate(&self) -> Result<(), WoodPlankError> {
        // Validate dimensions
        if self.length_mm <= 0 || self.width_mm <= 0 || self.thickness_mm <= 0 {
            return Err(WoodPlankError::InvalidDimensions(
                "All dimensions must be positive".to_string()
            ));
        }
        
        // Validate price
        if self.price <= BigDecimal::from(0) {
            return Err(WoodPlankError::InvalidPrice(
                "Price must be positive".to_string()
            ));
        }
        
        // Validate stock
        if self.stock_quantity < 0 {
            return Err(WoodPlankError::InvalidStock(
                "Stock quantity cannot be negative".to_string()
            ));
        }
        
        Ok(())
    }
    
    /// Create a SKU string if not provided
    pub fn generate_sku(&self) -> String {
        if !self.sku.is_empty() {
            return self.sku.clone();
        }
        
        // Format: CAT-GRADE-DIMxDIMxDIM
        // Example: LT-A-23x100x2500
        let category_code = match self.category {
            ProductCategory::HeavyDutyBox => "HDB",
            ProductCategory::Pallet => "PAL",
            ProductCategory::LongTimber => "LT",
            ProductCategory::ShortTimber => "ST",
            ProductCategory::PlanedTimber => "PT",
            ProductCategory::MachinedTimber => "MT",
            ProductCategory::Component => "COMP",
            ProductCategory::LaminatedTable => "LAM",
            ProductCategory::Plywood => "PLY",
            ProductCategory::Custom => "CUST",
        };
        
        let grade_code = match self.grade {
            ProductGrade::AGrade => "A",
            ProductGrade::BGrade => "B",
            ProductGrade::Standard => "S",
        };
        
        format!(
            "{}-{}-{}x{}x{}", 
            category_code, 
            grade_code,
            self.thickness_mm, 
            self.width_mm, 
            self.length_mm
        )
    }
}

impl WoodPlank {
    /// Check if the wood plank is in stock
    pub fn is_in_stock(&self) -> bool {
        self.stock_quantity > 0
    }
    
    /// Calculate the volume of the wood plank in cubic centimeters
    pub fn volume(&self) -> i32 {
        self.length_mm * self.width_mm * self.thickness_mm
    }
    
    /// Calculate the surface area of the wood plank in square centimeters
    pub fn surface_area(&self) -> i32 {
        2 * (
            self.length_mm * self.width_mm + 
            self.length_mm * self.thickness_mm + 
            self.width_mm * self.thickness_mm
        )
    }
    
    /// Get formatted dimensions as a string (e.g., "23 x 100 x 2500")
    pub fn dimensions_string(&self) -> String {
        format!("{} x {} x {}", self.thickness_mm, self.width_mm, self.length_mm)
    }
    
    /// Get a formatted price string with currency (e.g., "R 350")
    pub fn price_string(&self) -> String {
        format!("R {}", self.price)
    }
    
    /// Parse a standard product description from the price list format
    /// Example: "23 X 100 X 2500 BALTIC EA R40"
    pub fn parse_from_description(desc: &str) -> Option<NewWoodPlank> {
        let parts: Vec<&str> = desc.trim().split_whitespace().collect();
        if parts.len() < 6 {
            return None;
        }
        
        // Parse dimensions
        let thickness = parts[0].parse::<i32>().ok()?;
        let width = parts[2].parse::<i32>().ok()?;
        let length = parts[4].parse::<i32>().ok()?;
        
        // Determine wood type
        let wood_type = if parts.contains(&"BALTIC") {
            WoodType::Baltic
        } else if parts.contains(&"PINE") {
            WoodType::Pine
        } else {
            WoodType::Mixed
        };
        
        // Parse price (assumes format Rxxx)
        let price_str = parts.last()?;
        let price = if price_str.starts_with('R') {
            if let Ok(price_val) = price_str[1..].parse::<i32>() {
                BigDecimal::from(price_val)
            } else {
                return None; // Couldn't parse price
            }
        } else {
            if let Ok(price_val) = price_str.parse::<i32>() {
                BigDecimal::from(price_val)
            } else {
                return None; // Couldn't parse price
            }
        };
        
        // Determine if it's A or B grade
        let grade = if parts.contains(&"B") {
            ProductGrade::BGrade
        } else {
            ProductGrade::AGrade
        };
        
        // Determine finish type
        let finish = if parts.contains(&"PAR") {
            FinishType::PlanedAllRound
        } else if parts.contains(&"PBS") {
            FinishType::PlanedBothSides
        } else if desc.contains("MACHINED") {
            FinishType::Machined
        } else if desc.contains("LAMINATED") {
            FinishType::Laminated
        } else {
            FinishType::Rough
        };
        
        // Create the name
        let name = format!("{} x {} x {} {}", thickness, width, length, 
            match wood_type {
                WoodType::Baltic => "Baltic",
                WoodType::Pine => "Pine",
                WoodType::Oak => "Oak",
                WoodType::Recycled => "Recycled",
                WoodType::Mixed => "Mixed",
            }
        );
        
        // Determine category based on dimensions and description
        let category = if desc.contains("PALLET") {
            ProductCategory::Pallet
        } else if desc.contains("BOX") {
            ProductCategory::HeavyDutyBox
        } else if desc.contains("PLYWOOD") {
            ProductCategory::Plywood
        } else if length > 2000 {
            ProductCategory::LongTimber
        } else {
            ProductCategory::ShortTimber
        };
        
        // Create the new wood plank
        Some(NewWoodPlank {
            sku: String::new(), // Will be generated later
            name,
            category,
            wood_type,
            grade,
            finish,
            thickness_mm: thickness,
            width_mm: width,
            length_mm: length,
            price,
            stock_quantity: 10, // Default value
            unit_of_measure: "EA".to_string(),
            description: Some(desc.to_string()),
            image_url: None,
        })
    }
}
