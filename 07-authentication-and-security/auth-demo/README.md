# Authentication Demo

A comprehensive demonstration of user authentication in Rust using Actix Web, featuring JWT-based authentication, password hashing with Argon2, and PostgreSQL for user storage.

## Features

- User registration with email validation
- Secure password hashing using Argon2
- JWT-based authentication
- Protected routes using middleware
- PostgreSQL integration with SQLx
- Input validation
- Structured error responses
- Request/Response logging

## Prerequisites

- Rust and Cargo installed
- PostgreSQL (local or Docker)
- Basic understanding of REST APIs and JWT

## Setup

### 1. Database Setup

Using Docker:
```bash
docker run --name postgres-auth \
  -e POSTGRES_USER=my_user \
  -e POSTGRES_PASSWORD=my_password \
  -e POSTGRES_DB=auth_db \
  -p 5432:5432 \
  -d postgres
```

For Ubuntu/Debian:
```bash
# Install PostgreSQL
sudo apt update
sudo apt install postgresql postgresql-contrib

# Start PostgreSQL service
sudo systemctl start postgresql
sudo systemctl enable postgresql

# Create database and user
sudo -u postgres psql
CREATE DATABASE auth_db;
CREATE USER my_user WITH PASSWORD 'my_password';
GRANT ALL PRIVILEGES ON DATABASE auth_db TO my_user;
```

### 2. Environment Configuration

Create `.env` file:
```bash
DATABASE_URL=postgres://my_user:my_password@localhost/auth_db
JWT_SECRET=your_very_long_and_secure_secret_key_here
RUST_LOG=debug
```

### 3. Database Migration

```bash
# Install SQLx CLI
cargo install sqlx-cli

# Run migrations
sqlx migrate run
```

### 4. Start the Server

```bash
cargo run
```

The server will start on http://127.0.0.1:8080

## API Endpoints

### 1. Register User
```bash
curl -X POST http://localhost:8080/api/v1/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "dzikri@syairozi.com",
    "password": "securepassword123",
    "name": "Dzikri Syairozi"
  }'
```

### 2. Login
```bash
curl -X POST http://localhost:8080/api/v1/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "dzikri@syairozi.com",
    "password": "securepassword123"
  }'
```

### 3. Access Protected Route
```bash
curl http://localhost:8080/api/v1/protected/resource \
  -H "Authorization: Bearer your_jwt_token_here"
```

## Response Formats

### Success Response
```bash
{
  "success": true,
  "data": {
    "id": 1,
    "email": "dzikri@syairozi.com",
    "name": "Dzikri Syairozi"
  }
}
```

### Error Response
```bash
{
  "success": false,
  "error": "Invalid credentials"
}
```

## Project Structure

- `src/`
  - `main.rs`: Application setup and configuration
  - `config.rs`: JWT and database configuration
  - `models/`
    - `user.rs`: User data structures and validation
  - `handlers/`
    - `auth.rs`: Authentication route handlers
  - `db/`
    - `users.rs`: Database operations for users
  - `middleware/`
    - `auth.rs`: JWT validation middleware

## Security Features

1. **Password Security**
   - Argon2 password hashing
   - Salt generation per password
   - Configurable hash parameters

2. **JWT Implementation**
   - Token expiration
   - Secure secret key handling
   - Authorization header validation

3. **Input Validation**
   - Email format validation
   - Password length requirements
   - Required field checking

## Development Notes

- JWT tokens expire after 24 hours
- Passwords must be at least 8 characters
- Email addresses must be unique in the system
- All endpoints return JSON responses
- Protected routes require valid JWT in Authorization header

## Error Handling

The application handles various error cases:
- Invalid credentials
- Duplicate email registration
- Invalid JWT tokens
- Database connection issues
- Input validation failures

## Testing

Run the tests:
```bash
cargo test
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.