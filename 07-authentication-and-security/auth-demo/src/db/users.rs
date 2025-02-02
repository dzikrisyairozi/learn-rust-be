use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;
use crate::models::user::{User, RegisterUser, LoginUser};

pub async fn register_user(pool: &PgPool, user: RegisterUser) -> sqlx::Result<User> {
    // Hash password using Argon2
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user.password.as_bytes(), &salt)
        .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
        .to_string();

    sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password_hash, name)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        user.email,
        password_hash,
        user.name
    )
    .fetch_one(pool)
    .await
}

pub async fn login_user(pool: &PgPool, credentials: &LoginUser) -> sqlx::Result<Option<User>> {
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE email = $1",
        credentials.email
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user {
        let parsed_hash = PasswordHash::new(&user.password_hash)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;
        
        if Argon2::default()
            .verify_password(credentials.password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            Ok(Some(user))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}