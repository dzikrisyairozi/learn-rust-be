use serde::{Deserialize, Serialize};
use validator::Validate;

// Product model with validation rules
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Product {
    pub id: Option<u32>,
    
    #[validate(length(min = 1, max = 100, message = "Product name must be between 1 and 100 characters"))]
    pub name: String,
    
    #[validate(range(min = 0.01, message = "Price must be greater than 0"))]
    pub price: f64,
    
    #[validate(length(min = 1, message = "Category is required"))]
    pub category: String,
}

// Generic API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}