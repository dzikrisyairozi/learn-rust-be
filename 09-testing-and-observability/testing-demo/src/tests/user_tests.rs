use actix_web::{test, web, App};
use crate::handlers::users::{create_user, get_user};
use crate::models::user::{User, CreateUserRequest};

#[actix_rt::test]
async fn test_create_user() {
    // Arrange
    let app = test::init_service(
        App::new().service(create_user)
    ).await;

    let user_data = CreateUserRequest {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };

    // Act
    let req = test::TestRequest::post()
        .uri("/users")
        .set_json(&user_data)
        .to_request();
    
    let resp = test::call_service(&app, req).await;

    // Assert
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_user() {
    // Arrange
    let app = test::init_service(
        App::new().service(web::scope("/api").service(get_user))
    ).await;

    // Act
    let req = test::TestRequest::get()
        .uri("/api/users/1")
        .to_request();
    
    let resp: User = test::call_and_read_body_json(&app, req).await;

    // Assert
    assert_eq!(resp.id, 1);
    assert_eq!(resp.name, "Test User");
}