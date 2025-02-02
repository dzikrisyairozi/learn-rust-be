use actix_web::{web, HttpResponse, post, get, put, delete};
use serde_json::json;
use validator::Validate;
use crate::models::product::{CreateProduct, UpdateProduct};
use crate::db;

/// Handler for creating a new product
/// POST /api/v1/products
#[post("/products")]
pub async fn create_product(
    pool: web::Data<sqlx::PgPool>,
    product: web::Json<CreateProduct>,
) -> HttpResponse {
    // Validate the product data
    if let Err(errors) = product.0.validate() {  // Changed from product.validate()
        return HttpResponse::BadRequest().json(json!({
            "error": format!("Validation error: {:?}", errors)
        }));
    }

    match db::products::create_product(&pool, product.into_inner()).await {
        Ok(product) => HttpResponse::Created().json(product),
        Err(e) => {
            tracing::error!("Failed to create product: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to create product"
            }))
        }
    }
}

/// Handler for retrieving a product by ID
/// GET /api/v1/products/{id}
#[get("/products/{id}")]
pub async fn get_product(
    pool: web::Data<sqlx::PgPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    match db::products::get_product(&pool, id.into_inner()).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({
                "error": "Product not found"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to get product: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to get product"
            }))
        }
    }
}

/// Handler for listing products with pagination
/// GET /api/v1/products?limit=10&offset=0
#[get("/products")]
pub async fn list_products(
    pool: web::Data<sqlx::PgPool>,
    query: web::Query<ListProductsQuery>,
) -> HttpResponse {
    let limit = query.limit.unwrap_or(10).min(100) as i64;
    let offset = query.offset.unwrap_or(0) as i64;

    match db::products::list_products(&pool, limit, offset).await {
        Ok(products) => HttpResponse::Ok().json(products),
        Err(e) => {
            tracing::error!("Failed to list products: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to list products"
            }))
        }
    }
}

/// Handler for updating a product
/// PUT /api/v1/products/{id}
#[put("/products/{id}")]
pub async fn update_product(
    pool: web::Data<sqlx::PgPool>,
    id: web::Path<i32>,
    product: web::Json<UpdateProduct>,
) -> HttpResponse {
    // Validate the update data
    if let Err(errors) = product.0.validate() {  
        return HttpResponse::BadRequest().json(json!({
            "error": format!("Validation error: {:?}", errors)
        }));
    }

    match db::products::update_product(&pool, id.into_inner(), product.into_inner()).await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(sqlx::Error::RowNotFound) => {
            HttpResponse::NotFound().json(json!({
                "error": "Product not found"
            }))
        }
        Err(e) => {
            tracing::error!("Failed to update product: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to update product"
            }))
        }
    }
}

/// Handler for deleting a product
/// DELETE /api/v1/products/{id}
#[delete("/products/{id}")]
pub async fn delete_product(
    pool: web::Data<sqlx::PgPool>,
    id: web::Path<i32>,
) -> HttpResponse {
    match db::products::delete_product(&pool, id.into_inner()).await {
        Ok(true) => HttpResponse::NoContent().finish(),
        Ok(false) => HttpResponse::NotFound().json(json!({
            "error": "Product not found"
        })),
        Err(e) => {
            tracing::error!("Failed to delete product: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to delete product"
            }))
        }
    }
}

#[derive(serde::Deserialize)]
pub struct ListProductsQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}