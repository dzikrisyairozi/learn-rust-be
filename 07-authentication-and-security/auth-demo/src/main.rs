mod config;
mod models;
mod handlers;
mod db;
mod middleware;

use actix_web::{web, App, HttpServer, middleware::Logger};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = config::create_pool().await;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let auth_config = config::AuthConfig::new(jwt_secret.as_bytes());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_config.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .service(handlers::auth::register)
                    .service(handlers::auth::login)
                    // Protected routes go here with auth middleware
                    .service(
                        web::scope("/protected")
                            .wrap(middleware::auth::AuthMiddleware::new(auth_config.clone()))
                            // Add protected endpoints here
                    )
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}