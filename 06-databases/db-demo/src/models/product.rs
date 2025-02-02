use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;
use validator::Validate;

#[derive(Debug, Serialize)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: Decimal,
    pub stock: i32,
    #[serde(with = "time::serde::timestamp")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::timestamp")]
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProduct {
    #[validate(length(min = 1, max = 100, message = "Product name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(length(max = 500, message = "Description must not exceed 500 characters"))]
    pub description: Option<String>,
    
    pub price: Decimal,
    
    #[validate(range(min = 0, message = "Stock cannot be negative"))]
    pub stock: i32,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProduct {
    #[validate(length(min = 1, max = 100, message = "Product name must be between 1 and 100 characters"))]
    pub name: Option<String>,
    
    #[validate(length(max = 500, message = "Description must not exceed 500 characters"))]
    pub description: Option<String>,
    
    pub price: Option<Decimal>,
    
    #[validate(range(min = 0, message = "Stock cannot be negative"))]
    pub stock: Option<i32>,
}