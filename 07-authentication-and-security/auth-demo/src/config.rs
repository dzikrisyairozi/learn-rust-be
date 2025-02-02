use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct AuthConfig {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,  // User ID
    pub exp: usize,  // Expiration time
}

impl AuthConfig {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    // Generate JWT token
    pub fn generate_token(&self, user_id: i32) -> jsonwebtoken::errors::Result<String> {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize + 24 * 3600; // 24 hours from now

        let claims = Claims {
            sub: user_id,
            exp,
        };

        jsonwebtoken::encode(&Header::default(), &claims, &self.encoding_key)
    }

    // Validate JWT token
    pub fn validate_token(&self, token: &str) -> jsonwebtoken::errors::Result<Claims> {
        jsonwebtoken::decode(
            token,
            &self.decoding_key,
            &Validation::default(),
        ).map(|data| data.claims)
    }
}

// Database connection pool
pub async fn create_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}