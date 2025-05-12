use std::path::Path;
use log::{info, warn};
use uuid::Uuid;
use sqlx::types::BigDecimal;

use crate::models::wood_plank::{
    NewWoodPlank, WoodType, ProductCategory, ProductGrade, FinishType
};
use crate::utils::error::AppResult;
use crate::utils::file;

/// Represents the current section being parsed in the price list
#[derive(Debug, Clone, PartialEq, Eq)]
enum PriceListSection {
    None,
    HeavyDutyBoxes,
    Pallets,
    AGradeLongTimbers,
    BGradeLongTimbers,
    AGradeShortTimbers,
    BGradeShortTimbers,
    PlanedBothSides,
    PlanedAllRound,
    MachinedTimbers,
    ShortBalticComponents,
    LaminatedBaltic,
    Plywood,
}

/// Parse a price list text file and convert it to WoodPlank objects
pub fn parse_price_list<P>(file_path: P) -> AppResult<Vec<NewWoodPlank>>
where
    P: AsRef<Path>,
{
    let lines = file::read_lines(file_path)?;
    let mut products = Vec::new();
    let mut current_section = PriceListSection::None;
    
    for line in lines {
        let line = line.trim();
        
        // Skip empty lines
        if line.is_empty() {
            continue;
        }
        
        // Check if this line is a section header
        if let Some(section) = identify_section(&line) {
            current_section = section;
            continue;
        }
        
        // Parse product line based on current section
        if current_section != PriceListSection::None {
            if let Some(product) = parse_product_line(line, &current_section) {
                products.push(product);
            }
        }
    }
    
    info!("Parsed {} products from price list", products.len());
    Ok(products)
}

/// Identify which section a line represents
fn identify_section(line: &str) -> Option<PriceListSection> {
    let line_upper = line.to_uppercase();
    
    match line_upper.as_str() {
        "HEAVY DUTY WOODEN BOXES" => Some(PriceListSection::HeavyDutyBoxes),
        "PALLETS" => Some(PriceListSection::Pallets),
        "A GRADE LONG TIMBERS" => Some(PriceListSection::AGradeLongTimbers),
        "B GRADE LONG TIMBERS" => Some(PriceListSection::BGradeLongTimbers),
        "A GRADE SHORT TIMBERS" => Some(PriceListSection::AGradeShortTimbers),
        "B GRADE SHORT TIMBERS" => Some(PriceListSection::BGradeShortTimbers),
        "PLANED BOTH SIDES" => Some(PriceListSection::PlanedBothSides),
        "PLANED ALL ROUND" => Some(PriceListSection::PlanedAllRound),
        "MACHINED TIMBERS" => Some(PriceListSection::MachinedTimbers),
        "SHORT BALTIC COMPONENTS" => Some(PriceListSection::ShortBalticComponents),
        "LAMINATED BALTIC TABLE AND SHELVES TO ORDER" => Some(PriceListSection::LaminatedBaltic),
        "INT/EXTERIOR PLYWOOD" | "INTERIOR PLYWOOD" => Some(PriceListSection::Plywood),
        _ => None,
    }
}

/// Parse a product line into a NewWoodPlank object
fn parse_product_line(line: &str, section: &PriceListSection) -> Option<NewWoodPlank> {
    // Skip lines that don't contain product information
    if !line.contains('X') && !line.contains('x') {
        return None;
    }
    
    // Extract dimensions and price
    let parts: Vec<&str> = line.split_whitespace().collect();
    
    // First try to parse with the standard format:
    // e.g. "23 X 100 X 2500 BALTIC EA R40"
    if parts.len() >= 7 {
        // Attempt to parse dimensions
        if let (Ok(thickness), Ok(width), Ok(length)) = (
            parts[0].parse::<i32>(), 
            parts[2].parse::<i32>(), 
            parts[4].parse::<i32>()
        ) {
            // Find wood type
            let wood_type = identify_wood_type(&parts);
            
            // Find price (usually last element, starting with R)
            let price = extract_price(&parts);
            
            if let Some(price) = price {
                // Build the name from dimensions and type
                let name = format!("{} x {} x {} {}", thickness, width, length, wood_type_str(&wood_type));
                
                // Determine product category and grade based on section
                let (category, grade) = section_to_category_grade(section);
                
                // Determine finish type based on section and parts
                let finish = identify_finish_type(section, &parts);
                
                // Build the NewWoodPlank object
                let product = NewWoodPlank {
                    sku: String::new(), // Will be generated
                    name,
                    category,
                    wood_type,
                    grade,
                    finish,
                    thickness_mm: thickness,
                    width_mm: width,
                    length_mm: length,
                    price,
                    stock_quantity: 10, // Default stock
                    unit_of_measure: "EA".to_string(),
                    description: Some(line.to_string()),
                    image_url: None,
                };
                
                return Some(product);
            }
        }
    }
    
    // If we couldn't parse it with the standard format, log a warning
    warn!("Could not parse product line: {}", line);
    None
}

/// Map section to product category and grade
fn section_to_category_grade(section: &PriceListSection) -> (ProductCategory, ProductGrade) {
    match section {
        PriceListSection::HeavyDutyBoxes => (ProductCategory::HeavyDutyBox, ProductGrade::Standard),
        PriceListSection::Pallets => (ProductCategory::Pallet, ProductGrade::Standard),
        PriceListSection::AGradeLongTimbers => (ProductCategory::LongTimber, ProductGrade::AGrade),
        PriceListSection::BGradeLongTimbers => (ProductCategory::LongTimber, ProductGrade::BGrade),
        PriceListSection::AGradeShortTimbers => (ProductCategory::ShortTimber, ProductGrade::AGrade),
        PriceListSection::BGradeShortTimbers => (ProductCategory::ShortTimber, ProductGrade::BGrade),
        PriceListSection::PlanedBothSides => (ProductCategory::PlanedTimber, ProductGrade::Standard),
        PriceListSection::PlanedAllRound => (ProductCategory::PlanedTimber, ProductGrade::Standard),
        PriceListSection::MachinedTimbers => (ProductCategory::MachinedTimber, ProductGrade::Standard),
        PriceListSection::ShortBalticComponents => (ProductCategory::Component, ProductGrade::Standard),
        PriceListSection::LaminatedBaltic => (ProductCategory::LaminatedTable, ProductGrade::Standard),
        PriceListSection::Plywood => (ProductCategory::Plywood, ProductGrade::Standard),
        PriceListSection::None => (ProductCategory::Custom, ProductGrade::Standard),
    }
}

/// Identify the finish type based on section and parts
fn identify_finish_type(section: &PriceListSection, parts: &[&str]) -> FinishType {
    match section {
        PriceListSection::PlanedBothSides => FinishType::PlanedBothSides,
        PriceListSection::PlanedAllRound => FinishType::PlanedAllRound,
        PriceListSection::MachinedTimbers => FinishType::Machined,
        PriceListSection::LaminatedBaltic => FinishType::Laminated,
        _ => {
            // Check for PAR in parts
            if parts.contains(&"PAR") {
                FinishType::PlanedAllRound
            } else {
                FinishType::Rough
            }
        }
    }
}

/// Identify wood type from parts
fn identify_wood_type(parts: &[&str]) -> WoodType {
    if parts.contains(&"BALTIC") {
        WoodType::Baltic
    } else if parts.contains(&"PINE") {
        WoodType::Pine
    } else if parts.contains(&"OAK") {
        WoodType::Oak
    } else {
        WoodType::Mixed
    }
}

/// Return a string representation of wood type
fn wood_type_str(wood_type: &WoodType) -> &'static str {
    match wood_type {
        WoodType::Baltic => "Baltic",
        WoodType::Pine => "Pine",
        WoodType::Oak => "Oak",
        WoodType::Recycled => "Recycled",
        WoodType::Mixed => "Mixed",
    }
}

/// Extract price from parts (usually in format "Rxxx")
fn extract_price(parts: &[&str]) -> Option<BigDecimal> {
    // Try to get the last part as price
    if let Some(last_part) = parts.last() {
        if last_part.starts_with('R') {
            // Try to parse as a number from the string after 'R'
            if let Ok(price_str) = last_part[1..].parse::<i32>() {
                return Some(BigDecimal::from(price_str));
            }
        }
    }
    
    // Scan all parts for prices
    for part in parts.iter().rev() {
        if part.starts_with('R') {
            // Try to parse as a number from the string after 'R'
            if let Ok(price_str) = part[1..].parse::<i32>() {
                return Some(BigDecimal::from(price_str));
            }
        }
    }
    
    None
}

/// Execute the seeding process by reading the price list and inserting into the database
pub async fn seed_database<P>(file_path: P, pool: &sqlx::PgPool) -> AppResult<()>
where
    P: AsRef<Path>,
{
    let products = parse_price_list(file_path)?;
    info!("Inserting {} products into database", products.len());
    
    for product in products {
        let sku = product.generate_sku();
        
        // Check if product already exists
        let exists = sqlx::query!("SELECT id FROM wood_planks WHERE sku = $1", sku)
            .fetch_optional(pool)
            .await?
            .is_some();
            
        if exists {
            info!("Product with SKU {} already exists, skipping", sku);
            continue;
        }
            
        let product_with_sku = NewWoodPlank {
            sku,
            ..product
        };
        
        if let Err(e) = product_with_sku.validate() {
            warn!("Invalid product data: {}", e);
            continue;
        }
        
        // Insert the product
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        
        sqlx::query!(
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
        ).execute(pool).await?;
        
        info!("Inserted product: {}", product_with_sku.name);
    }
    
    Ok(())
}
