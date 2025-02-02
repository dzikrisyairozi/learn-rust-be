# Database Integration Demo

A comprehensive demonstration of PostgreSQL database integration with Actix Web, featuring complete CRUD operations, data validation, and proper error handling.

## Features

- Complete CRUD operations for products
- PostgreSQL integration using SQLx
- Data validation using validator
- Connection pooling
- Structured error responses
- Request/Response logging
- Pagination support
- Dynamic query building

## Prerequisites

- Rust and Cargo installed
- PostgreSQL (local or Docker)
- Basic understanding of SQL and REST APIs

## Setup

1. Install PostgreSQL:

Using Docker:
```
docker run --name postgres-demo \
  -e POSTGRES_USER=my_user \
  -e POSTGRES_PASSWORD=my_password \
  -e POSTGRES_DB=my_database \
  -p 5432:5432 \
  -d postgres
```

For Ubuntu/Debian:
```
# Install PostgreSQL
sudo apt update
sudo apt install postgresql postgresql-contrib

# Start PostgreSQL service
sudo systemctl start postgresql
sudo systemctl enable postgresql

# Switch to postgres user
sudo -i -u postgres

# Create database and user
psql
CREATE DATABASE my_database;
CREATE USER my_user WITH ENCRYPTED PASSWORD 'my_password';
GRANT ALL PRIVILEGES ON DATABASE my_database TO my_user;
\q

# Optional: Test connection
psql -h localhost -U my_user -d my_database
```

For macOS:
```
# Install using Homebrew
brew install postgresql@14

# Start PostgreSQL service
brew services start postgresql@14

# Create database and user
psql postgres
CREATE DATABASE my_database;
CREATE USER my_user WITH ENCRYPTED PASSWORD 'my_password';
GRANT ALL PRIVILEGES ON DATABASE my_database TO my_user;
\q
```

For Windows:
```
# Download and install PostgreSQL from official website
https://www.postgresql.org/download/windows/

# Open SQL Shell (psql)
psql -U postgres

# Create database and user
CREATE DATABASE my_database;
CREATE USER my_user WITH ENCRYPTED PASSWORD 'my_password';
GRANT ALL PRIVILEGES ON DATABASE my_database TO my_user;
\q
```

Verify Installation:
```
# Test connection with new user
psql -h localhost -U my_user -d my_database

# If it works, you should see:
# psql (14.x)
# Type "help" for help.
# my_database=>
```

2. Create `.env` file:
```
DATABASE_URL=postgres://my_user:my_password@localhost/my_database
RUST_LOG=debug
```

3. Create and run migrations:
```
# Install SQLx CLI
cargo install sqlx-cli

# Create migration
sqlx migrate add create_products_table

# Run migrations
sqlx migrate run
```

4. Start the server:
```
cargo run
```

## API Endpoints

### 1. Create Product
```
curl -X POST http://localhost:8080/api/v1/products \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Gaming Laptop",
    "description": "High-performance gaming laptop",
    "price": 1299.99,
    "stock": 50
  }'
```

### 2. Get Product
```
curl http://localhost:8080/api/v1/products/1
```

### 3. List Products (with pagination)
```
# Default pagination (10 items per page)
curl http://localhost:8080/api/v1/products

# Custom pagination
curl "http://localhost:8080/api/v1/products?limit=20&offset=40"
```

### 4. Update Product
```
curl -X PUT http://localhost:8080/api/v1/products/1 \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Updated Gaming Laptop",
    "price": 1399.99
  }'
```

### 5. Delete Product
```
curl -X DELETE http://localhost:8080/api/v1/products/1
```

## Response Formats

### Success Response
```
{
  "success": true,
  "data": {
    "id": 1,
    "name": "Gaming Laptop",
    "description": "High-performance gaming laptop",
    "price": 1299.99,
    "stock": 50,
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
  },
  "error": null
}
```

### Error Response
```
{
  "success": false,
  "error": "Validation error: price must be greater than 0"
}
```

## Project Structure

- `src/`
  - `main.rs`: Application setup and configuration
  - `config.rs`: Database configuration and connection pool
  - `models/`
    - `product.rs`: Product data structures with validation
  - `handlers/`
    - `products.rs`: Product route handlers
  - `db/`
    - `products.rs`: Database operations for products

## Database Schema

```
CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    price DECIMAL(10,2) NOT NULL,
    stock INT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Trigger for updating updated_at
CREATE TRIGGER update_products_updated_at
    BEFORE UPDATE ON products
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
```

## Error Handling

The application handles various error cases:
- Validation errors (400 Bad Request)
- Not found errors (404 Not Found)
- Database errors (500 Internal Server Error)
- Invalid JSON payload (400 Bad Request)
- Invalid query parameters (400 Bad Request)

## Best Practices Demonstrated

1. **Database**
   - Connection pooling for efficient resource usage
   - Prepared statements to prevent SQL injection
   - Database migrations for schema management
   - Triggers for automatic timestamp updates

2. **API Design**
   - RESTful endpoints
   - Consistent error responses
   - Proper HTTP status codes
   - Request validation
   - Pagination for list endpoints

3. **Code Organization**
   - Modular structure
   - Separation of concerns
   - Error handling middleware
   - Logging middleware