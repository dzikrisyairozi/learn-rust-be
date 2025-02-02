# Handling Data Demo

This project demonstrates JSON handling, data validation, and structured responses in Actix Web.

## Features

- JSON request/response handling with serde
- Data validation using validator
- Structured API responses
- Error handling

## Running the Project

1. Start the server:
```bash
cargo run
```

## Testing the Endpoints

1. Create a Product (with validation):
```bash
# Valid product
curl -X POST http://localhost:8080/api/v1/products \
  -H "Content-Type: application/json" \
  -d '{"name":"Laptop","price":999.99,"category":"Electronics"}'

# Invalid product (will fail validation)
curl -X POST http://localhost:8080/api/v1/products \
  -H "Content-Type: application/json" \
  -d '{"name":"","price":-10,"category":""}'
```

2. Get a Product:
```bash
curl http://localhost:8080/api/v1/products/123
```

## Project Structure

- `models.rs`: Data structures with validation rules
- `handlers.rs`: Request handlers with JSON processing
- `main.rs`: Application setup and routing