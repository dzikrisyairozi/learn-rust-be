use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::config::AuthConfig;

pub struct AuthMiddleware {
    auth_config: AuthConfig,
}

impl AuthMiddleware {
    pub fn new(auth_config: AuthConfig) -> Self {
        Self { auth_config }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            auth_config: self.auth_config.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    auth_config: AuthConfig,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract and validate JWT here
        if let Some(auth_header) = req.headers().get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    if let Ok(claims) = self.auth_config.validate_token(token) {
                        req.extensions_mut().insert(claims);
                        return Box::pin(self.service.call(req));
                    }
                }
            }
        }
        // No valid token found
        Box::pin(ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))))
    }
}