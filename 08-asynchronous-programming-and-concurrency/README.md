# 8. Asynchronous Programming and Concurrency

In this chapter, we will explore **asynchronous programming and concurrency** in Rust, focusing on the **Tokio runtime** and how it integrates with Actix Web. We will cover async/await, spawning tasks, concurrency patterns, and scaling concurrent connections.

## 8.1 Quick Refresher on Rust Async/Await

Rust's async model allows efficient multitasking without blocking threads. Instead of traditional threads, Rust uses **async functions** and **await** to suspend execution while waiting for a task to complete.

### 8.1.1 Async Basics

```rust
use tokio::time::{sleep, Duration};

async fn say_hello() {
    println!("Hello");
    sleep(Duration::from_secs(1)).await;
    println!("World!");
}

#[tokio::main]
async fn main() {
    say_hello().await;
}
```

- `async fn` marks a function as asynchronous.
- `.await` pauses execution until the async operation completes.
- `tokio::main` is used to run an async function as the program entry point.

## 8.2 Using Tokio Runtime in Actix Web

Actix Web is built on **Tokio**, an async runtime that manages tasks efficiently. While Actix Web already provides an async runtime, you may need **explicit Tokio features** for database queries, background tasks, or concurrency.

### 8.2.1 Adding Dependencies

Ensure your `Cargo.toml` includes:

```toml
[dependencies]
actix-web = "4"
tokio = { version = "1", features = ["full"] }
```

### 8.2.2 Running an Async Actix Server

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, async world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
```

- `#[actix_web::main]` initializes the Actix runtime.
- Requests are handled asynchronously without blocking threads.

## 8.3 Spawning Tasks and Concurrency Patterns

### 8.3.1 Spawning Async Tasks

Use `tokio::spawn` to run background tasks:

```rust
use tokio::task;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let handle = task::spawn(async {
        sleep(Duration::from_secs(2)).await;
        println!("Task completed!");
    });

    println!("Waiting for task...");
    handle.await.unwrap(); // Waits for the async task to complete
}
```

### 8.3.2 Using Channels for Message Passing

Rust’s `tokio::sync::mpsc` allows safe message passing:

```rust
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    tokio::spawn(async move {
        tx.send("Hello from another task!").await.unwrap();
    });

    if let Some(msg) = rx.recv().await {
        println!("{}", msg);
    }
}
```

- **mpsc::channel(32)** creates a message channel.
- **tx.send(...)** sends a message from another async task.
- **rx.recv().await** receives messages in the main async task.

### 8.3.3 Shared State with `tokio::sync::Mutex`

Rust’s ownership model prevents mutable state across tasks. Use `tokio::sync::Mutex`:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let counter_clone = counter.clone();
    tokio::spawn(async move {
        let mut num = counter_clone.lock().await;
        *num += 1;
        println!("Incremented: {}", *num);
    }).await.unwrap();
}
```

- `Arc<Mutex<T>>` allows **shared mutable state** across async tasks.
- `.lock().await` acquires the lock, ensuring safe concurrent access.

## 8.4 Scaling Concurrent Connections

### 8.4.1 Handling Multiple Requests Efficiently

Actix Web automatically handles multiple connections using **worker threads** and **async tasks**. You can adjust concurrency settings:

```rust
HttpServer::new(|| App::new())
    .workers(4) // Set number of worker threads
    .bind("127.0.0.1:8080")?
    .run()
    .await
```

### 8.4.2 Connection Pooling for Databases

If using **SQLx**, create a connection pool:

```rust
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/mydb")
        .await?;

    Ok(())
}
```

- **max_connections(5)** limits open database connections, preventing overload.

### 8.4.3 Using Batching for Performance

Batch processing reduces request overhead:

```rust
use futures::future::join_all;

async fn process_batch() {
    let tasks: Vec<_> = (0..5).map(|i| tokio::spawn(async move {
        println!("Processing task {}", i);
    })).collect();

    join_all(tasks).await;
}

#[tokio::main]
async fn main() {
    process_batch().await;
}
```

- `join_all(tasks).await` waits for multiple tasks to finish concurrently.

## 8.5 Conclusion

In this chapter, we covered:

- **Async/Await fundamentals** in Rust.
- **Using Tokio with Actix Web** for concurrent request handling.
- **Concurrency patterns** like **task spawning, channels, and shared state**.
- **Optimizing scalability** with worker threads and database connection pooling.

In the next chapter, we’ll focus on **Testing and Observability**, ensuring that our backend is reliable and maintainable.

---

<p align="center">
  <sub>
    © 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. 
    Licensed under the <a href="../LICENSE">MIT License</a>.
  </sub>
</p>
