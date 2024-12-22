use serde::{Deserialize, Serialize};

// User model for JSON serialization/deserialization
#[derive(Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub email: String,
    pub age: Option<u32>,
}