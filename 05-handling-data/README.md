# 5. Handling Data

In this chapter, we will focus on working with data in Rust backend development using Actix Web. This includes handling JSON requests and responses, validating input data, and structuring API responses correctly.

## 5.1 Working with JSON in Actix (Using `serde`)

JSON is the most common data format used for web APIs. Rust uses the `serde` crate for serializing (converting Rust structs to JSON) and deserializing (parsing JSON into Rust structs).

### 5.1.1 Adding Dependencies

Add `serde` and `serde_json` to `Cargo.toml`:

```toml
[dependencies]
actix-web = "4"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### 5.1.2 Receiving JSON Requests

To parse JSON requests, we use `web::Json<T>`, where `T` is a struct with `serde` attributes.

```rust
use actix_web::{post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}

#[post("/create_user")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    format!("User {} is {} years old", user.name, user.age)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- `serde::Deserialize` allows `User` to be automatically filled from a JSON request.
- If the request body is invalid JSON, Actix returns a **400 Bad Request**.

### 5.1.3 Sending JSON Responses

Use `serde::Serialize` to return structured JSON responses.

```rust
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ResponseMessage {
    message: String,
}

#[get("/status")]
async fn status() -> impl Responder {
    let response = ResponseMessage {
        message: "Service is running".to_string(),
    };

    HttpResponse::Ok().json(response) // Converts struct to JSON
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(status)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## 5.2 Validating Request Data

Sometimes we need to enforce rules like "name must not be empty" or "age must be between 18-99". The `validator` crate helps with this.

### 5.2.1 Adding `validator` to `Cargo.toml`

```toml
[dependencies]
validator = { version = "0.16", features = ["derive"] }
```

### 5.2.2 Validating Struct Fields

Use `#[validate]` attributes to define validation rules.

```rust
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
struct RegisterUser {
    #[validate(length(min = 3))]
    name: String,

    #[validate(range(min = 18, max = 99))]
    age: u8,
}

#[post("/register")]
async fn register(user: web::Json<RegisterUser>) -> impl Responder {
    if let Err(errors) = user.validate() {
        return HttpResponse::BadRequest().json(errors);
    }
    
    HttpResponse::Ok().body("User registered successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(register)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- If validation fails, we return a **400 Bad Request** with detailed error messages.

## 5.3 Returning Structured Responses with Status Codes and Headers

### 5.3.1 Custom Success Responses

Returning JSON responses with HTTP status codes:

```rust
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: T,
}

#[get("/info")]
async fn info() -> impl Responder {
    let response = ApiResponse {
        success: true,
        data: "This is an example response",
    };

    HttpResponse::Ok().json(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(info)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### 5.3.2 Handling Errors Gracefully

Instead of just returning raw strings, we can use a consistent JSON error response format.

```rust
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

#[get("/fail")]
async fn fail() -> impl Responder {
    let error_response = ErrorResponse {
        success: false,
        error: "Something went wrong".to_string(),
    };

    HttpResponse::InternalServerError().json(error_response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(fail)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### 5.3.3 Setting Custom Headers

Sometimes, APIs need to return custom headers.

```rust
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/custom_headers")]
async fn custom_headers() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("X-Custom-Header", "RustActix"))
        .body("Check headers!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(custom_headers)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## 5.4 Conclusion

In this chapter, we covered:
- **Working with JSON** in Actix Web using `serde`.
- **Validating input data** with the `validator` crate.
- **Returning structured responses** with proper status codes and headers.

This provides a strong foundation for handling structured data in Rust backends. In the next chapter, we’ll connect our backend to a **database (PostgreSQL)** and perform CRUD operations.

---

<p align="center">
  <sub>
    © 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. 
    Licensed under the <a href="../LICENSE">MIT License</a>.
  </sub>
</p>
