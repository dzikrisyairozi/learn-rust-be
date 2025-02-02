mod models;
mod handlers;
mod services;

use actix_web::{web, App, HttpServer, middleware::Logger};
use services::task_processor::TaskProcessor;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create shared task processor
    let task_processor = Arc::new(TaskProcessor::new());
    let task_processor_clone = task_processor.clone();

    // Start background processing
    tokio::spawn(async move {
        task_processor_clone.run().await;
    });

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(task_processor.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(handlers::tasks::submit_task)
                    .service(handlers::tasks::get_task_status)
                    .service(handlers::tasks::batch_process)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}