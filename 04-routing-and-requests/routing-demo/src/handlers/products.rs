use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use crate::models::product::{Product, ProductQuery};

// GET /products - List all products with optional filtering
#[get("/products")]
pub async fn list_products(query: web::Query<ProductQuery>) -> impl Responder {
    // Here you would typically query a database
    // For demo, we'll just return the query parameters
    HttpResponse::Ok().json(query.into_inner())
}

// GET /products/{id} - Get a specific product
#[get("/products/{id}")]
pub async fn get_product(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    let product = Product {
        id: Some(id),
        name: "Demo Product".to_string(),
        price: 29.99,
        category: "Electronics".to_string(),
    };
    HttpResponse::Ok().json(product)
}

// POST /products - Create a new product
#[post("/products")]
pub async fn create_product(product: web::Json<Product>) -> impl Responder {
    HttpResponse::Created().json(product.into_inner())
}

// PUT /products/{id} - Update a product
#[put("/products/{id}")]
pub async fn update_product(
    path: web::Path<u32>,
    product: web::Json<Product>,
) -> impl Responder {
    let id = path.into_inner();
    let mut updated_product = product.into_inner();
    updated_product.id = Some(id);
    HttpResponse::Ok().json(updated_product)
}

// DELETE /products/{id} - Delete a product
#[delete("/products/{id}")]
pub async fn delete_product(path: web::Path<u32>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("Product {} deleted", id))
}