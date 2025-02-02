use actix_web::{web, HttpResponse, post};
use serde_json::json;
use validator::Validate;

use crate::{
    config::AuthConfig,
    db,
    models::user::{LoginUser, RegisterUser},
};

#[post("/register")]
pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    user: web::Json<RegisterUser>,
) -> HttpResponse {
    if let Err(errors) = user.validate() {
        return HttpResponse::BadRequest().json(json!({
            "error": format!("Validation error: {:?}", errors)
        }));
    }

    match db::users::register_user(&pool, user.into_inner()).await {
        Ok(user) => HttpResponse::Created().json(user),
        Err(e) => {
            tracing::error!("Failed to register user: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Failed to register user"
            }))
        }
    }
}

#[post("/login")]
pub async fn login(
    pool: web::Data<sqlx::PgPool>,
    auth_config: web::Data<AuthConfig>,
    credentials: web::Json<LoginUser>,
) -> HttpResponse {
    match db::users::login_user(&pool, &credentials).await {
        Ok(Some(user)) => {
            match auth_config.generate_token(user.id) {
                Ok(token) => HttpResponse::Ok().json(json!({
                    "token": token,
                    "user": user
                })),
                Err(e) => {
                    tracing::error!("Failed to generate token: {:?}", e);
                    HttpResponse::InternalServerError().json(json!({
                        "error": "Authentication failed"
                    }))
                }
            }
        }
        Ok(None) => HttpResponse::Unauthorized().json(json!({
            "error": "Invalid credentials"
        })),
        Err(e) => {
            tracing::error!("Login error: {:?}", e);
            HttpResponse::InternalServerError().json(json!({
                "error": "Authentication failed"
            }))
        }
    }
}