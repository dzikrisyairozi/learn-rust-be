# 7. Authentication and Security

In this chapter, we will explore **user authentication and security** in Actix Web using **JWT (JSON Web Tokens)** for stateless authentication. We will also discuss HTTPS/TLS, middleware for authentication, and best practices for handling secrets securely.

## 7.1 Session vs. Token-Based Authentication

### 7.1.1 Session-Based Authentication

- The **server stores user sessions** (e.g., in-memory, Redis, or database).
- A **session ID** is stored in a cookie and sent with each request.
- Requires session management, making it **less scalable** for microservices.

### 7.1.2 Token-Based Authentication (JWT)

- Instead of storing sessions, **JWTs** (JSON Web Tokens) are issued after login.
- The **JWT is stored in the client (browser/local storage) and sent in each request**.
- The server **verifies JWTs without storing session state**, making it **scalable**.

For modern APIs, **JWT-based authentication** is preferred.

## 7.2 Implementing JWT Authentication in Actix Web

### 7.2.1 Adding Dependencies

Add the following to `Cargo.toml`:

```toml
[dependencies]
actix-web = "4"
jsonwebtoken = "9"
serde = { version = "1", features = ["derive"] }
chrono = { version = "0.4", features = ["serde"] }
dotenv = "0.15"
```

### 7.2.2 Generating JWTs

Create a struct for the token claims:

```rust
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, Header, EncodingKey};
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn generate_jwt(user_id: &str, secret: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .expect("Invalid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .expect("JWT encoding failed")
}
```

### 7.2.3 Protecting Routes with JWT Middleware

```rust
use actix_web::{web, HttpRequest, HttpResponse, Responder, Error};
use jsonwebtoken::{decode, DecodingKey, Validation};

fn validate_jwt(token: &str, secret: &str) -> Result<Claims, Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| HttpResponse::Unauthorized().finish())?;

    Ok(token_data.claims)
}

async fn protected_route(req: HttpRequest) -> impl Responder {
    let secret = "your-secret-key";
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..]; // Extract token
                if validate_jwt(token, secret).is_ok() {
                    return HttpResponse::Ok().body("Access granted");
                }
            }
        }
    }
    HttpResponse::Unauthorized().body("Access denied")
}
```

- The `Authorization` header is checked for a **valid JWT**.
- If valid, the user **gains access** to the protected route.

## 7.3 Configuring HTTPS/TLS

Enable **HTTPS** in Actix Web using **TLS certificates**:

### 7.3.1 Generate a Self-Signed Certificate (for local testing)

```bash
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -nodes
```

### 7.3.2 Configure Actix Web to Use TLS

```rust
use actix_web::{HttpServer, App};
use actix_tls::rustls::{
    internal::pemfile::{certs, pkcs8_private_keys},
    NoClientAuth, ServerConfig,
};
use std::fs::File;
use std::io::BufReader;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cert_file = &mut BufReader::new(File::open("cert.pem")?);
    let key_file = &mut BufReader::new(File::open("key.pem")?);

    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();

    let mut config = ServerConfig::new(NoClientAuth::new());
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(|| App::new())
        .bind_rustls("127.0.0.1:8443", config)?
        .run()
        .await
}
```

- **`bind_rustls`** enables HTTPS on **port 8443**.
- Requires **TLS certificates** (`cert.pem`, `key.pem`).

## 7.4 Middleware for Authentication and Authorization

Define a **middleware to check authentication** before accessing routes:

```rust
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, HttpResponse};
use actix_service::{Service, Transform};
use futures::future::{ok, Ready};

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract and validate JWT here
        self.service.call(req)
    }
}
```

- Attach middleware to the Actix app:

```rust
HttpServer::new(|| App::new().wrap(AuthMiddleware))
```

## 7.5 Secure Handling of Secrets and Environment Variables

### 7.5.1 Using `.env` for Secret Storage

Instead of **hardcoding secrets**, store them in `.env`:

```env
JWT_SECRET=supersecretkey
```

Load them in Rust:

```rust
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    println!("Loaded secret: {}", jwt_secret);
}
```

### 7.5.2 Best Practices

✅ **Never hardcode secrets** in source code.  
✅ **Use `.gitignore`** to prevent `.env` from being committed.  
✅ **Use Vaults (AWS Secrets Manager, HashiCorp Vault, etc.)** for production secrets.  
✅ **Rotate Secrets Periodically** to reduce security risks.

## 7.6 Conclusion

In this chapter, we covered:

- **JWT authentication** for stateless, secure APIs.
- **Configuring HTTPS/TLS** for encrypted communication.
- **Middleware for authentication/authorization** in Actix Web.
- **Handling secrets securely** with `.env` and best practices.

In the next chapter, we’ll explore **asynchronous programming and concurrency**, making our API efficient for high traffic.

---

<p align="center">
  <sub>
    © 2025 Learn Rust Backend by <a href="https://github.com/dzikrisyairozi">Dzikri Syairozi</a>. 
    Licensed under the <a href="../LICENSE">MIT License</a>.
  </sub>
</p>
