# 6. Databases (PostgreSQL)

In this chapter, we will integrate a **PostgreSQL** database into our Rust backend using **SQLx**. This includes setting up the database, connecting from Actix Web, handling migrations, and performing CRUD operations.

## 6.1 Setting up PostgreSQL Locally or via Docker

### 6.1.1 Installing PostgreSQL Locally

To install PostgreSQL on your system:

- **Linux (Debian/Ubuntu):**  
  ```bash
  sudo apt update && sudo apt install postgresql postgresql-contrib
  ```

- **MacOS (Homebrew):**  
  ```bash
  brew install postgresql
  brew services start postgresql
  ```

- **Windows:**  
  Download and install from [official PostgreSQL site](https://www.postgresql.org/download/).

After installation, create a database:
```bash
sudo -u postgres psql
CREATE DATABASE my_database;
CREATE USER my_user WITH PASSWORD 'my_password';
GRANT ALL PRIVILEGES ON DATABASE my_database TO my_user;
```

### 6.1.2 Running PostgreSQL via Docker

If you prefer **Docker**, run:
```bash
docker run --name my_postgres -e POSTGRES_USER=my_user -e POSTGRES_PASSWORD=my_password -e POSTGRES_DB=my_database -p 5432:5432 -d postgres
```
This sets up PostgreSQL in a container, accessible on `localhost:5432`.

## 6.2 Connecting to PostgreSQL with SQLx

### 6.2.1 Adding Dependencies

Update your `Cargo.toml`:

```toml
[dependencies]
actix-web = "4"
sqlx = { version = "0.6", features = ["postgres", "runtime-tokio-native-tls"] }
tokio = { version = "1", features = ["full"] }
dotenv = "0.15"
```

### 6.2.2 Setting Up Database Pool

Create a `.env` file:

```env
DATABASE_URL=postgres://my_user:my_password@localhost/my_database
```

Then, initialize the database connection:

```rust
use sqlx::postgres::PgPoolOptions;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

## 6.3 Database Seeding

Seeding means inserting initial data. Create a `seeds.sql` file:

```sql
INSERT INTO users (name, email) VALUES ('Dzikri', 'dzikri@example.com'), ('Syairozi', 'syairozi@example.com');
```

Run it:
```bash
psql -U my_user -d my_database -f seeds.sql
```

## 6.4 Performing CRUD Operations

### 6.4.1 Creating a User

```rust
use actix_web::{post, web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::Deserialize;

#[derive(Deserialize)]
struct NewUser {
    name: String,
    email: String,
}

#[post("/users")]
async fn create_user(pool: web::Data<PgPool>, user: web::Json<NewUser>) -> impl Responder {
    let result = sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
        .bind(&user.name)
        .bind(&user.email)
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Created().body("User created"),
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}
```

### 6.4.2 Reading Users

```rust
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::{PgPool, Row};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

#[get("/users")]
async fn get_users(pool: web::Data<PgPool>) -> impl Responder {
    let rows = sqlx::query("SELECT id, name, email FROM users")
        .fetch_all(pool.get_ref())
        .await;

    match rows {
        Ok(rows) => {
            let users: Vec<User> = rows.iter().map(|row| User {
                id: row.get(0),
                name: row.get(1),
                email: row.get(2),
            }).collect();

            HttpResponse::Ok().json(users)
        },
        Err(_) => HttpResponse::InternalServerError().body("Error fetching users"),
    }
}
```

### 6.4.3 Updating a User

```rust
use actix_web::{put, web, HttpResponse, Responder};
use sqlx::PgPool;

#[put("/users/{id}")]
async fn update_user(
    pool: web::Data<PgPool>,
    user_id: web::Path<i32>,
    user_data: web::Json<NewUser>,
) -> impl Responder {
    let result = sqlx::query("UPDATE users SET name = $1, email = $2 WHERE id = $3")
        .bind(&user_data.name)
        .bind(&user_data.email)
        .bind(user_id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User updated"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating user"),
    }
}
```

### 6.4.4 Deleting a User

```rust
use actix_web::{delete, web, HttpResponse, Responder};
use sqlx::PgPool;

#[delete("/users/{id}")]
async fn delete_user(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(user_id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting user"),
    }
}
```

## 6.5 Database Schema Migrations and Best Practices

### 6.5.1 Using SQLx Migrations

SQLx provides built-in migration support. Install the CLI:

```bash
cargo install sqlx-cli
```

Create a migration:

```bash
sqlx migrate add create_users_table
```

This generates a file in `migrations/`. Edit it:

```sql
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL
);
```

Run migrations:

```bash
sqlx migrate run
```

### 6.5.2 Best Practices

- **Use Connection Pools** (`PgPool`) to avoid excessive connections.
- **Prepare Queries** to optimize performance.
- **Use Migrations** instead of manual schema changes.
- **Validate Input** to prevent SQL injection.

## 6.6 Conclusion

In this chapter, we covered:

- **Setting up PostgreSQL** locally and using Docker.
- **Connecting Actix Web to PostgreSQL** using SQLx.
- **Seeding the database** with initial data.
- **Performing CRUD operations** using SQLx queries.
- **Managing schema migrations** with `sqlx migrate`.

In the next chapter, we’ll **implement authentication and security** to protect our API.

---

<p align="center">
  <sub>
    © 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. 
    Licensed under the <a href="../LICENSE">MIT License</a>.
  </sub>
</p>
