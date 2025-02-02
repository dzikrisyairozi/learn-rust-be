mod config;
mod models;
mod handlers;
mod db;

use actix_web::{web, App, HttpServer, middleware};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables and initialize logging
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    // Create database connection pool
    let pool = config::create_pool().await;

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Add database pool to app state
            .app_data(web::Data::new(pool.clone()))
            // Add logging middleware
            .wrap(middleware::Logger::default())
            // Configure routes under /api/v1
            .service(
                web::scope("/api/v1")
                    .service(handlers::products::create_product)
                    .service(handlers::products::get_product)
                    .service(handlers::products::list_products)
                    .service(handlers::products::update_product)
                    .service(handlers::products::delete_product)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}