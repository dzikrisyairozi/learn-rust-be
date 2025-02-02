use actix_web::{post, get, web, HttpResponse, Responder};
use validator::Validate; // Add this import
use crate::models::{Product, ApiResponse, ErrorResponse};

// Handler for creating a new product with validation
#[post("/products")]
pub async fn create_product(product: web::Json<Product>) -> impl Responder {
    // Extract the product from Json wrapper before validation
    let product = product.into_inner();
    
    // Validate the product data
    if let Err(errors) = product.validate() {
        let error_response = ErrorResponse {
            success: false,
            error: format!("Validation error: {:?}", errors),
        };
        return HttpResponse::BadRequest().json(error_response);
    }

    // Create successful response
    let response = ApiResponse {
        success: true,
        data: Some(product),
        error: None,
    };
    
    HttpResponse::Created().json(response)
}

// Handler for getting a product by ID
#[get("/products/{id}")]
pub async fn get_product(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    
    // Simulate finding a product (in real app, would query database)
    let product = Product {
        id: Some(id),
        name: "Example Product".to_string(),
        price: 29.99,
        category: "Electronics".to_string(),
    };

    let response = ApiResponse {
        success: true,
        data: Some(product),
        error: None,
    };
    
    HttpResponse::Ok().json(response)
}