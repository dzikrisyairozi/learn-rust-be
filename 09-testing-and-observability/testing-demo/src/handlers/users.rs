use actix_web::{get, post, web, HttpResponse, Responder};
use tracing::info;
use crate::models::user::{User, CreateUserRequest};
use utoipa::OpenApi;

/// Create a new user
#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Bad request")
    )
)]
#[post("/users")]
pub async fn create_user(user: web::Json<CreateUserRequest>) -> impl Responder {
    info!("Creating new user: {}", user.name);
    
    // Simulate user creation
    let new_user = User {
        id: 1,
        name: user.name.clone(),
        email: user.email.clone(),
    };

    HttpResponse::Created().json(new_user)
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = User),
        (status = 404, description = "User not found")
    )
)]
#[get("/users/{id}")]
pub async fn get_user(id: web::Path<i32>) -> impl Responder {
    info!("Fetching user with id: {}", id);
    
    // Simulate fetching user
    let user = User {
        id: *id,
        name: "Dzikri Syairozi".to_string(),
        email: "dzikrisyairozi@example.com".to_string(),
    };

    HttpResponse::Ok().json(user)
}