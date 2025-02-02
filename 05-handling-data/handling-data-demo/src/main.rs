use actix_web::{App, HttpServer, web, middleware};
mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // Add logging middleware
            .wrap(middleware::Logger::default())
            // Register our handlers
            .service(
                web::scope("/api/v1")
                    .service(handlers::create_product)
                    .service(handlers::get_product)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}