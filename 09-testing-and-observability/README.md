# 9. Testing and Observability

In this chapter, we’ll focus on **testing** Actix Web applications, ensuring they work correctly, and implementing **observability** features such as logging, metrics, and error tracking.

## 9.1 API Testing with Swagger-Like Interface

To document and test your API interactively, you can use **Swagger-like** tools such as **utoipa**.

### 9.1.1 Adding Dependencies

Add `utoipa` and `utoipa-swagger-ui` to `Cargo.toml`:

```toml
[dependencies]
utoipa = { version = "3", features = ["actix_extras"] }
utoipa-swagger-ui = { version = "3", features = ["actix-web"] }
actix-web = "4"
```

### 9.1.2 Generating OpenAPI Docs

```rust
use actix_web::{get, App, HttpServer, Responder};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(paths(index), info(description = "API Documentation"))]
struct ApiDoc;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, API!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- Open `http://127.0.0.1:8080/swagger-ui` to test the API.

## 9.2 Unit and Integration Testing with Actix Web

### 9.2.1 Writing Unit Tests

Rust’s built-in `#[test]` attribute allows writing unit tests.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_math() {
        assert_eq!(2 + 2, 4);
    }
}
```

### 9.2.2 Testing Actix Web Handlers

```rust
#[cfg(test)]
mod tests {
    use actix_web::{test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }
}
```

- `test::init_service()` initializes the Actix Web app.
- `test::TestRequest` simulates HTTP requests.

## 9.3 Using `cargo test` and Actix Testing Crates

Run all tests with:

```bash
cargo test
```

For HTTP testing:

```toml
[dev-dependencies]
actix-http-test = "3"
```

Example:

```rust
use actix_http_test::TestServer;
use actix_web::{web, App, HttpResponse, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, test!")
}

#[actix_web::test]
async fn test_api() {
    let srv = TestServer::start(|| App::new().service(web::resource("/").route(web::get().to(index))));
    let response = srv.get("/").send().await.unwrap();
    assert_eq!(response.status(), 200);
}
```

## 9.4 Adding Logging (`env_logger` or `tracing`)

### 9.4.1 Using `env_logger`

Add:

```toml
[dependencies]
env_logger = "0.10"
log = "0.4"
```

Enable logging:

```rust
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

Run with logging:

```bash
RUST_LOG=info cargo run
```

### 9.4.2 Using `tracing`

For structured logging:

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

Initialize:

```rust
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting application...");
}
```

## 9.5 Monitoring and Metrics (Prometheus)

### 9.5.1 Adding Prometheus Exporter

```toml
[dependencies]
actix-web-prom = "0.6"
```

Expose metrics:

```rust
use actix_web::{web, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let metrics = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(metrics.clone())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

Access Prometheus metrics at `http://127.0.0.1:8080/metrics`.

## 9.6 Crash Reporting and Error Tracking

### 9.6.1 Using Sentry

```toml
[dependencies]
sentry = "0.31"
```

```rust
use sentry::{init, capture_message};

fn main() {
    let _guard = init("your-sentry-dsn-here");

    capture_message("Something went wrong!", sentry::Level::Error);
}
```

## 9.7 Conclusion

In this chapter, we covered:

- **API testing** with Swagger-like tools.
- **Unit and integration testing** for Actix Web.
- **Logging and monitoring** using `env_logger`, `tracing`, and Prometheus.
- **Error tracking** with Sentry.

In the next chapter, we’ll explore **deployment strategies**.

---

<p align="center">
  <sub>
    © 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. 
    Licensed under the <a href="../LICENSE">MIT License</a>.
  </sub>
</p>
