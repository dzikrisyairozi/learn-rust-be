use sqlx::PgPool;
use crate::models::product::{Product, CreateProduct, UpdateProduct};

/// Creates a new product in the database
pub async fn create_product(pool: &PgPool, product: CreateProduct) -> sqlx::Result<Product> {
    sqlx::query_as!(
        Product,
        r#"
        INSERT INTO products (name, description, price, stock)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#,
        product.name,
        product.description,
        product.price,
        product.stock
    )
    .fetch_one(pool)
    .await
}

/// Retrieves a product by its ID
pub async fn get_product(pool: &PgPool, id: i32) -> sqlx::Result<Product> {
    sqlx::query_as!(
        Product,
        "SELECT * FROM products WHERE id = $1",
        id
    )
    .fetch_one(pool)
    .await
}

/// Lists all products with optional pagination
pub async fn list_products(
    pool: &PgPool,
    limit: i64,
    offset: i64
) -> sqlx::Result<Vec<Product>> {
    sqlx::query_as!(
        Product,
        r#"
        SELECT * FROM products
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await
}

/// Updates an existing product
pub async fn update_product(
    pool: &PgPool,
    id: i32,
    product: UpdateProduct,
) -> sqlx::Result<Product> {
    sqlx::query_as!(
        Product,
        r#"
        UPDATE products
        SET 
            name = COALESCE($1, name),
            description = COALESCE($2, description),
            price = COALESCE($3, price),
            stock = COALESCE($4, stock),
            updated_at = NOW()
        WHERE id = $5
        RETURNING *
        "#,
        product.name,
        product.description,
        product.price,
        product.stock,
        id
    )
    .fetch_one(pool)
    .await
}

/// Deletes a product by its ID
pub async fn delete_product(pool: &PgPool, id: i32) -> sqlx::Result<bool> {
    let result = sqlx::query!(
        "DELETE FROM products WHERE id = $1",
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}