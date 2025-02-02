# 10. Deployment

In this chapter, we will explore **how to deploy a Rust backend built with Actix Web**. This includes building for production, containerizing the app using Docker, configuring environments, deploying to a cloud platform, and handling database migrations.

## 10.1 Building for Proaduction

### 10.1.1 Compiling with `--release`

For optimal performance, always compile Rust applications with the `--release` flag:

```bash
cargo build --release
```

The binary will be available at:

```bash
target/release/my_app
```

- This optimizes the binary for speed and size.
- The **debug symbols** are removed, reducing binary size.

### 10.1.2 Cross-Compilation

If you need to compile for a different platform (e.g., building on Linux for Windows):

```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

## 10.2 Containerizing Actix Web with Docker

Docker allows you to package and run the application in a **consistent environment**, making it easy to deploy.

### 10.2.1 Writing a Dockerfile

Create a `Dockerfile`:

```dockerfile
# Use Rust official image for building
FROM rust:1.74 AS builder

WORKDIR /usr/src/app

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source code
COPY . .
RUN cargo build --release

# Final lightweight image
FROM debian:bullseye-slim

WORKDIR /app

# Copy only the built binary
COPY --from=builder /usr/src/app/target/release/my_app /app/

EXPOSE 8080

CMD ["/app/my_app"]
```

### 10.2.2 Building and Running the Docker Container

```bash
docker build -t my_rust_app .
docker run -p 8080:8080 my_rust_app
```

- The containerized app is now accessible at **`http://localhost:8080`**.

## 10.3 Environment Configuration

In production, we manage configurations using **environment variables**.

### 10.3.1 Using `.env` Files

Use the `dotenv` crate to load environment variables:

```toml
[dependencies]
dotenv = "0.15"
```

Example `.env` file:

```env
DATABASE_URL=postgres://user:password@db_host/my_database
APP_ENV=production
```

Load environment variables:

```rust
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database: {}", db_url);
}
```

## 10.4 Deploying to DigitalOcean

### 10.4.1 Setting Up a Droplet

1. Create a **DigitalOcean Droplet** with Ubuntu 22.04.
2. SSH into the server:

   ```bash
   ssh root@your-server-ip
   ```

### 10.4.2 Installing Rust on the Server

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 10.4.3 Deploying the App

1. Upload the built binary to the server:

   ```bash
   scp target/release/my_app root@your-server-ip:/usr/local/bin/my_app
   ```

2. Run the app:

   ```bash
   ./usr/local/bin/my_app
   ```

3. Set up **systemd** for automatic restarts:

   ```bash
   nano /etc/systemd/system/my_rust_app.service
   ```

   ```ini
   [Unit]
   Description=Rust Actix Web App
   After=network.target

   [Service]
   ExecStart=/usr/local/bin/my_app
   Restart=always
   User=root

   [Install]
   WantedBy=multi-user.target
   ```

   Reload systemd and enable the service:

   ```bash
   systemctl daemon-reload
   systemctl enable my_rust_app
   systemctl start my_rust_app
   ```

## 10.5 Handling Migrations and Zero-Downtime Restarts

### 10.5.1 Running SQLx Migrations on Deployment

If your app uses **SQLx**, run migrations on deployment:

```bash
sqlx migrate run
```

For Docker:

```bash
docker exec -it my_rust_app sqlx migrate run
```

### 10.5.2 Zero-Downtime Restarts

Use **systemd** with:

```bash
systemctl restart my_rust_app
```

Or, use **rolling updates** in Docker:

```bash
docker run --rm -d -p 8080:8080 my_rust_app
```

## 10.6 Conclusion

In this chapter, we covered:

- **Building for production** using `--release` and cross-compilation.
- **Containerizing with Docker** for portability.
- **Configuring environments** using `.env` and environment variables.
- **Deploying to DigitalOcean** and setting up **systemd** for automatic restarts.
- **Handling database migrations** and ensuring zero-downtime restarts.

This completes the Rust backend series! 🎉

---

<p align="center">
  <sub>
    © 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. 
    Licensed under the <a href="../LICENSE">MIT License</a>.
  </sub>
</p>
