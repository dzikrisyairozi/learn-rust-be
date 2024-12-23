# Routing and Requests Demo

A comprehensive demonstration of routing and request handling in Actix Web.

## Features

- RESTful endpoints for products
- Query parameter handling
- Path parameter extraction
- JSON request/response handling
- HTTP method handling (GET, POST, PUT, DELETE)
- Proper error responses
- Request validation

## Prerequisites

- Rust and Cargo installed
- curl or Postman for testing

## Running the Project

1. Clone and navigate to the project:
```
git clone <repository-url>
cd routing-demo
```

2. Start the server:
```
cargo run
```

## Testing the Endpoints

1. List Products (with optional filters):
```
# Basic list
curl http://localhost:8080/api/v1/products

# With filters
curl "http://localhost:8080/api/v1/products?category=electronics&min_price=10.00&max_price=50.00"
```

2. Get Single Product:
```
curl http://localhost:8080/api/v1/products/123
```

3. Create Product:
```
curl -X POST http://localhost:8080/api/v1/products \
  -H "Content-Type: application/json" \
  -d '{"name":"New Product","price":29.99,"category":"Electronics"}'
```

4. Update Product:
```
curl -X PUT http://localhost:8080/api/v1/products/123 \
  -H "Content-Type: application/json" \
  -d '{"name":"Updated Product","price":39.99,"category":"Electronics"}'
```

5. Delete Product:
```
curl -X DELETE http://localhost:8080/api/v1/products/123
```

## Project Structure

- `src/models/`: Data structures and validation
- `src/handlers/`: Route handlers and business logic
- `main.rs`: Application setup and configuration

## Error Handling

The application handles various error cases:
- Invalid JSON payloads (400 Bad Request)
- Invalid path parameters (404 Not Found)
- Invalid query parameters (400 Bad Request)

## Notes

This is a demonstration project showing routing patterns in Actix Web. For production use, you would:
- Add database integration
- Implement proper error handling
- Add authentication/authorization
- Add input validation
- Add proper logging