-- Migration script for Wood Planks E-commerce Application
-- Creates required ENUM types and tables for the wood recycling company

-- Create custom ENUM types to match our Rust models
CREATE TYPE wood_type AS ENUM (
    'baltic',
    'pine',
    'oak',
    'recycled',
    'mixed'
);

CREATE TYPE product_category AS ENUM (
    'heavydutybox',
    'pallet',
    'longtimber',
    'shorttimber',
    'planedtimber',
    'machinedtimber',
    'component',
    'laminatedtable',
    'plywood',
    'custom'
);

CREATE TYPE product_grade AS ENUM (
    'agrade',
    'bgrade',
    'standard'
);

CREATE TYPE finish_type AS ENUM (
    'rough',
    'planedbothsides',
    'planedallround',
    'machined',
    'laminated',
    'raw'
);

-- Create wood_planks table
CREATE TABLE IF NOT EXISTS wood_planks (
    -- Primary key and identification
    id UUID PRIMARY KEY,
    sku VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    
    -- Classification fields
    category product_category NOT NULL,
    wood_type wood_type NOT NULL,
    grade product_grade NOT NULL,
    finish finish_type NOT NULL,
    
    -- Dimensions
    thickness_mm INTEGER NOT NULL CHECK (thickness_mm > 0),
    width_mm INTEGER NOT NULL CHECK (width_mm > 0),
    length_mm INTEGER NOT NULL CHECK (length_mm > 0),
    
    -- Pricing and inventory
    price DECIMAL(10, 2) NOT NULL CHECK (price >= 0),
    stock_quantity INTEGER NOT NULL DEFAULT 0 CHECK (stock_quantity >= 0),
    unit_of_measure VARCHAR(10) NOT NULL DEFAULT 'EA',
    
    -- Additional information
    description TEXT,
    image_url VARCHAR(512),
    
    -- Timestamps
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create a function to automatically update the updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create a trigger to automatically update updated_at on each update
CREATE TRIGGER update_wood_planks_updated_at
BEFORE UPDATE ON wood_planks
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Create indexes for common query patterns
CREATE INDEX idx_wood_planks_category ON wood_planks(category);
CREATE INDEX idx_wood_planks_wood_type ON wood_planks(wood_type);
CREATE INDEX idx_wood_planks_grade ON wood_planks(grade);
CREATE INDEX idx_wood_planks_finish ON wood_planks(finish);
CREATE INDEX idx_wood_planks_dimensions ON wood_planks(thickness_mm, width_mm, length_mm);
CREATE INDEX idx_wood_planks_price ON wood_planks(price);
CREATE INDEX idx_wood_planks_stock ON wood_planks(stock_quantity) WHERE stock_quantity > 0;

-- Add comments for documentation
COMMENT ON TABLE wood_planks IS 'Inventory of wood planks available for sale';
COMMENT ON COLUMN wood_planks.id IS 'Unique identifier for the wood plank';
COMMENT ON COLUMN wood_planks.sku IS 'Stock Keeping Unit for inventory tracking';
COMMENT ON COLUMN wood_planks.category IS 'Product category from price list (e.g., long timber, pallet)';
COMMENT ON COLUMN wood_planks.wood_type IS 'Type of wood (e.g., Baltic, Pine)';
COMMENT ON COLUMN wood_planks.grade IS 'Quality grade (A Grade, B Grade)';
COMMENT ON COLUMN wood_planks.finish IS 'Finish type (e.g., rough, planed)';
COMMENT ON COLUMN wood_planks.thickness_mm IS 'Thickness in millimeters';
COMMENT ON COLUMN wood_planks.width_mm IS 'Width in millimeters';
COMMENT ON COLUMN wood_planks.length_mm IS 'Length in millimeters';
COMMENT ON COLUMN wood_planks.price IS 'Price in Rands (ZAR)';
COMMENT ON COLUMN wood_planks.stock_quantity IS 'Current stock quantity available';
COMMENT ON COLUMN wood_planks.unit_of_measure IS 'Unit of measurement (e.g., EA for each)';
COMMENT ON COLUMN wood_planks.created_at IS 'Timestamp when record was created';
COMMENT ON COLUMN wood_planks.updated_at IS 'Timestamp when record was last updated';
