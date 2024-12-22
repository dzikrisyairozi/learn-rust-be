# 3. Using Framework for Backend (Actix Web)

## 3.1 Installing and Setting Up Actix Web

Actix Web is a **fast** and **flexible** framework for building HTTP-based services. It leverages Rust’s concurrency features to handle multiple requests efficiently.

### 3.1.1 Add Actix to Cargo.toml

```toml
[dependencies]
actix-web = "4"
```

- We specify *version 4* (or whichever is the latest stable).
- `cargo build` will download and compile Actix along with any dependencies.

### 3.1.2 Project Structure

If you followed the previous chapters, you can keep using the same project or create a fresh one:

```bash
cargo new actix-web-demo
cd actix-web-demo
```

Inside `Cargo.toml`, include `actix-web` as shown above.

---

## 3.2 Creating Your First Actix Web Application

### 3.2.1 Hello, World! Endpoint

```rust
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, world from Actix!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello) // Register our hello service
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

- We create a function `hello` that returns a simple string.
- The **`#[get("/")]`** attribute defines this function as a *GET* endpoint at the root path (`/`).
- **HttpServer** listens on `127.0.0.1:8080`.
- **App** is where we register our routes and middlewares.

### 3.2.2 Running the Server

```bash
cargo run
```

Open your browser at `http://127.0.0.1:8080/`, and you should see:
```
Hello, world from Actix!
```


- If everything works, you’ve successfully created your first Actix Web backend.

---

## 3.3 Overview of Actix Concepts

Actix is built on a powerful **actor** model and handles concurrency using Rust’s **async** features.

### 3.3.1 Actors

- *Actors* are objects that run in parallel and exchange messages.
- While you can use the lower-level actor system directly, **Actix Web** abstracts most of it, so you typically focus on building routes and handling requests.

### 3.3.2 Concurrency Model

- Actix Web uses **non-blocking async I/O**, letting the framework serve many requests with minimal overhead.
- Each incoming request is handled by an async task under the hood.

### 3.3.3 Why Actix?

- **Performance**: Known to be one of the fastest Rust web frameworks.
- **Ecosystem**: Plenty of plugins and middleware exist for tasks like JWT auth, session handling, etc.
- **Stability**: It has an active community and regular updates, making it solid for production use.

---

**Summary**:
- Actix Web simplifies server creation compared to building one manually.
- You define routes (endpoints) and connect them to async handler functions.
- The internal actor and concurrency model manages requests efficiently.

We’ll expand on this foundation in the next chapters, where we’ll look at more complex routes, request parsing, and how to handle different data in your Actix Web application.

<p align="center">
  <sub>
    © 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. 
    Licensed under the <a href="../LICENSE">MIT License</a>.
  </sub>
</p>
