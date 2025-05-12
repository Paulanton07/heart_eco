use std::path::Path;
use std::time::Instant;
use std::io::Write;
use uuid::Uuid;
use log::{info, warn, error};

use woodplanks_ecommerce::{
    utils::seeder,
    utils::error::{AppResult, AppError},
    config::{Config, self},
    models::wood_plank::{
        NewWoodPlank, ProductCategory, WoodType, ProductGrade, FinishType
    }
};

#[tokio::main]
async fn main() -> AppResult<()> {
    // Initialize error handling and logging
    if let Err(e) = woodplanks_ecommerce::utils::error::init_error_handling() {
        println!("Warning: Error initializing error handling: {}", e);
    }
    
    // Initialize application
    println!("=== Heart Eco Wood Planks Inventory Seeder ===");
    println!("This tool will import the price list into the database.");
    
    // Load configuration
    let config = Config::init();
    
    // Verify configuration
    println!("Configuration loaded:");
    println!("  Database: {}", config.database_url);
    println!("  Server port: {}", config.server_port);
    
    // Set up database connection
    println!("Connecting to database at {}...", config.database_host());
    let pool = config::db::create_pool(&config).await?;
    
    println!("Database connection established.");
    
    // Path to price list file
    let price_list_path = "price_list.txt";
    if !Path::new(price_list_path).exists() {
        return Err(format!("Price list file not found: {}", price_list_path).into());
    }
    
    // Parse price list into products
    println!("Reading price list from {}", price_list_path);
    let products = seeder::parse_price_list(price_list_path)?;
    println!("Found {} products in price list", products.len());
    
    // Insert products into database
    let start_time = Instant::now();
    let mut success_count = 0;
    let mut error_count = 0;
    
    println!("Beginning database import...");
    println!("Total products to import: {}", products.len());
    
    // Create a progress tracking indicator
    let total = products.len();
    for (index, product) in products.into_iter().enumerate() {
        // Show progress
        print!("\rProcessing product {}/{} ({:.1}%)... ", 
            index + 1, 
            total, 
            (index as f32 + 1.0) / (total as f32) * 100.0
        );
        std::io::stdout().flush().ok();
        
        // Generate SKU if not provided
        let sku = if product.sku.is_empty() {
            product.generate_sku()
        } else {
            product.sku.clone()
        };
        
        // Validate product data
        if let Err(e) = product.validate() {
            warn!("Skipping invalid product '{}': {}", product.name, e);
            error_count += 1;
            continue;
        }
        
        // Use a transaction for better error handling
        let result = sqlx::query!("SELECT id FROM wood_planks WHERE sku = $1", sku)
            .fetch_optional(&pool)
            .await;
        
        // Proper error handling for database operations
        let exists = match result {
            Ok(optional_row) => optional_row.is_some(),
            Err(e) => {
                error!("Database error checking for existing product: {}", e);
                error_count += 1;
                continue;
            }
        };
            
        if exists {
            info!("Product with SKU {} already exists, skipping", sku);
            success_count += 1;
            continue;
        }
        
        // Insert the product
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        
        let product_with_sku = NewWoodPlank {
            sku,
            ..product
        };
        
        // Use a try block for better error handling
        let result = sqlx::query!(
            r#"INSERT INTO wood_planks (
                id, sku, name, category, wood_type, grade, finish, 
                thickness_mm, width_mm, length_mm, price, stock_quantity,
                unit_of_measure, description, image_url, created_at, updated_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            )"#,
            id,
            product_with_sku.sku,
            product_with_sku.name,
            product_with_sku.category as ProductCategory,
            product_with_sku.wood_type as WoodType,
            product_with_sku.grade as ProductGrade,
            product_with_sku.finish as FinishType,
            product_with_sku.thickness_mm,
            product_with_sku.width_mm,
            product_with_sku.length_mm,
            product_with_sku.price,
            product_with_sku.stock_quantity,
            product_with_sku.unit_of_measure,
            product_with_sku.description,
            product_with_sku.image_url,
            now,
            now
        ).execute(&pool).await;
            
        match result {
            Ok(_) => {
                success_count += 1;
            }
            Err(e) => {
                error!("Failed to insert product '{}': {}", product_with_sku.name, e);
                error_count += 1;
            }
        }
    }
    println!();
    
    let elapsed = start_time.elapsed();
    println!("\nImport completed in {:.2?}", elapsed);
    println!("Successfully imported: {} products", success_count);
    println!("Failed to import: {} products", error_count);
    
    // Summary statistics
    if success_count > 0 {
        println!("\nImport successful!");
        
        // Count products by category using a raw query to handle enum types
        let categories = sqlx::query_as::<_, (String, i64)>(
            r#"SELECT category::text, COUNT(*) as count 
               FROM wood_planks 
               GROUP BY category 
               ORDER BY count DESC"#
        )
        .fetch_all(&pool)
        .await;
        
        if let Ok(categories) = categories {
            println!("\nProduct categories in database:");
            for (category, count) in categories {
                println!("  {}: {}", category, count);
            }
        }
    } else {
        println!("\nNo products were imported. Please check for errors.");
    }
    
    Ok(())
}

