# Testing and Observability Demo

This project demonstrates testing and observability features in a Rust Actix Web application.

## Features

- Unit and integration testing
- OpenAPI/Swagger documentation
- Prometheus metrics
- Structured logging
- Request/Response logging

## Setup

1. Install dependencies:
```bash
cargo build
```

2. Run the application:
```bash
RUST_LOG=debug cargo run
```

## Testing

Run the tests:
```bash
cargo test
```

## API Documentation

Visit `http://localhost:8080/swagger-ui/` to view the OpenAPI documentation.

## Metrics

View Prometheus metrics at `http://localhost:8080/metrics`

## Available Endpoints

1. Create User:
```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Dzikri Syairozi","email":"dzikrisyairozi@example.com"}'
```

2. Get User:
```bash
curl http://localhost:8080/api/users/1
```

## Observability Features

1. Logging:
- Request/Response logging
- Structured logging with tracing
- Log levels (debug, info, error)

2. Metrics:
- Request count
- Response times
- Status codes
- Custom metrics

3. Documentation:
- OpenAPI/Swagger UI
- API endpoint documentation
- Request/Response schemas