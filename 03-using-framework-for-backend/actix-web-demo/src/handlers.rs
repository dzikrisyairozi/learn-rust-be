use actix_web::{get, post, web, HttpResponse, Responder};
use crate::models::User;

// Basic hello world endpoint
#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix Web!")
}

// Echo endpoint that returns whatever is sent in the path
#[get("/echo/{name}")]
pub async fn echo(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Echo: {}!", name))
}

// JSON response example
#[get("/info")]
pub async fn get_info() -> impl Responder {
    let data = serde_json::json!({
        "app_name": "Actix Web Demo",
        "version": "1.0",
        "author": "Learn Rust Backend"
    });
    
    HttpResponse::Ok().json(data)
}

// Handle POST requests with JSON body
#[post("/users")]
pub async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.0)
}