use actix_web::{App, HttpServer, web};
mod handlers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Print startup message
    println!("Server starting on http://127.0.0.1:8080");

    // Create and start the HTTP server
    HttpServer::new(|| {
        App::new()
            // Register routes for different endpoints
            .service(handlers::hello)
            .service(handlers::echo)
            .service(
                // Group related endpoints under /api
                web::scope("/api")
                    .service(handlers::get_info)
                    .service(handlers::create_user)
            )
    })
    .bind(("127.0.0.1", 8080))? // Bind to localhost port 8080
    .run()
    .await
}