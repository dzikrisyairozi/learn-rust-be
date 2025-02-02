mod handlers;
mod models;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_web_prom::PrometheusMetricsBuilder;
use handlers::users::{create_user, get_user};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::users::create_user,
        handlers::users::get_user
    ),
    components(
        schemas(models::user::User, models::user::CreateUserRequest)
    ),
    tags(
        (name = "users", description = "User management endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Initialize Prometheus metrics
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(prometheus.clone())
            .service(
                web::scope("/api")
                    .service(create_user)
                    .service(get_user)
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}