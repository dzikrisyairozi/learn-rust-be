use actix_web::{middleware, App, HttpServer, web};
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // Add middleware for logging
            .wrap(middleware::Logger::default())
            
            // Product routes
            .service(
                web::scope("/api/v1")
                    .service(handlers::products::list_products)
                    .service(handlers::products::get_product)
                    .service(handlers::products::create_product)
                    .service(handlers::products::update_product)
                    .service(handlers::products::delete_product)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}