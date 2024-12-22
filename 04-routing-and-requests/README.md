# 04. Routing and Requests

## 4.1 Defining Multiple Routes and Methods

Actix Web provides a flexible routing system that allows you to handle **GET**, **POST**, **PUT**, **DELETE**, and more. Each route is associated with a path and an HTTP method. 

### 4.1.1 Basic GET Route

```rust
use actix_web::{get, App, HttpServer, Responder};

#[get("/hello")]
async fn hello_route() -> impl Responder {
    "Hello from GET route!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- **#[get("/hello")]** ties this function to **GET /hello**.
- Returning a string is enough to respond with that text.

### 4.1.2 Handling Multiple Methods

Actix allows you to define routes for various HTTP methods. Suppose you want a single path `/items` to respond to both **GET** and **POST**:

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn get_items() -> impl Responder {
    HttpResponse::Ok().body("List of items")
}

async fn create_item() -> impl Responder {
    HttpResponse::Created().body("Item created")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/items", web::get().to(get_items))
            .route("/items", web::post().to(create_item))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- **web::get().to(...)**: Defines a GET endpoint for `/items`.
- **web::post().to(...)**: Defines a POST endpoint for `/items`.
- **HttpResponse::Created()** sets the HTTP status code to **201 Created**.

### 4.1.3 Other Methods

You can also define **PUT**, **PATCH**, **DELETE**, and more:

- **web::put().to(put_handler)**
- **web::delete().to(delete_handler)**
- **web::patch().to(patch_handler)**

---

## 4.2 Handling Path Parameters and Query Parameters

### 4.2.1 Path Parameters

Path parameters allow you to capture dynamic segments of the URL (e.g., `/users/123`).

```rust
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/users/{user_id}")]
async fn get_user(path: web::Path<u32>) -> impl Responder {
    format!("User ID: {}", path.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- **`{user_id}`** matches any segment in that place.
- **web::Path<u32>** converts that segment into a `u32`. If parsing fails, Actix will return a 400 error by default.

### 4.2.2 Query Parameters

For queries like `?page=2&limit=10`, define a struct and let Actix parse it:

```rust
use actix_web::{get, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Pagination {
    page: Option<u32>,
    limit: Option<u32>,
}

#[get("/items")]
async fn list_items(query: web::Query<Pagination>) -> impl Responder {
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    format!("Listing items on page {}, limit {}.", page, limit)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(list_items)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- **web::Query<Pagination>** attempts to parse any query params into the `Pagination` struct.
- **`page`** and **`limit`** become `None` if they’re missing, so you can provide defaults.

---

## 4.3 Parsing Request Bodies

### 4.3.1 JSON Requests

Actix integrates **serde** for JSON. By accepting `web::Json<T>`, you automatically get a deserialized struct:

```rust
use actix_web::{post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Person {
    name: String,
    age: u8,
}

#[post("/create_person")]
async fn create_person(person: web::Json<Person>) -> impl Responder {
    format!("Received: {} (age {})", person.name, person.age)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(create_person)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- If JSON parsing fails, Actix automatically returns a **400 Bad Request**.

### 4.3.2 Form Data

For HTML forms (`application/x-www-form-urlencoded`), use `web::Form<T>`:

```rust
use actix_web::{post, web, App, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(form: web::Form<LoginData>) -> impl Responder {
    if form.username == "admin" && form.password == "secret" {
        "Welcome, admin!"
    } else {
        "Invalid credentials!"
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- This is often used for traditional web forms.

### 4.3.3 Raw Body

If you want raw data (e.g., binary or custom formats), you can read the body as **bytes**:

```rust
use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};

#[post("/raw")]
async fn raw_body(body: web::Bytes) -> impl Responder {
    HttpResponse::Ok().body(format!("Received {} bytes", body.len()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(raw_body)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- Perfect for file uploads or custom binary protocols.

---

## 4.4 Custom Error Responses and Error Handling

### 4.4.1 Returning HTTP Status Codes

Use `HttpResponse::<Status>()` to set specific codes:

```rust
use actix_web::{HttpResponse, get, Responder, App, HttpServer};

#[get("/healthy")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Server is healthy!")
}
```

- `HttpResponse::Ok()` -> **200**  
- `HttpResponse::NotFound()` -> **404**  
- `HttpResponse::BadRequest()` -> **400**, etc.

### 4.4.2 Handling Errors with Results

Returning `Result<T, E>` from a handler lets you map Rust errors to HTTP responses:

```rust
use actix_web::{get, App, HttpServer, Responder, Result, HttpResponse, error};

#[get("/might_fail/{value}")]
async fn might_fail(path: actix_web::web::Path<String>) -> Result<String> {
    let val = path.into_inner();
    if val == "error" {
        Err(error::ErrorBadRequest("You triggered an error!"))
    } else {
        Ok(format!("Value is: {}", val))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(might_fail))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
```

- If `Err(```)` is returned, Actix automatically converts it to a corresponding HTTP error response.
- You can define custom error types and use middlewares for more advanced scenarios.

### 4.4.3 Global Error Handlers (Optional)

You can also set up a *global error handler* to capture any unhandled errors, returning a consistent format (like JSON error messages). This is especially useful for APIs.

---

## Conclusion

In this chapter, you learned:

- How to define **multiple endpoints** using Actix Web’s flexible routing system.
- Ways to handle **path parameters** (`/users/{id}`) and **query parameters** (`?page=1&limit=10`).
- Parsing **JSON**, **form data**, or **raw bytes**.
- Managing **errors** and sending **custom HTTP status codes**.

These skills form the backbone of any web application, letting you handle various request types and respond properly. Next, we’ll dive deeper into **handling data**, focusing on structured input/output, data validation, and preparing for database interaction.

<p align="center">
  <sub>
    © 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. 
    Licensed under the <a href="../LICENSE">MIT License</a>.
  </sub>
</p>
