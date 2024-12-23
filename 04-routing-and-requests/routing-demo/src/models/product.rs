use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: Option<u32>,
    pub name: String,
    pub price: f64,
    pub category: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductQuery {
    pub category: Option<String>,
    pub min_price: Option<f64>,
    pub max_price: Option<f64>,
}