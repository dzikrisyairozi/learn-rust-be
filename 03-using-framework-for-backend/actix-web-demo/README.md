# Actix Web Demo

This project demonstrates basic usage of the Actix Web framework for building a REST API.

## Prerequisites

- Rust and Cargo installed
- Basic understanding of HTTP and REST APIs
- Optional: A tool like `curl` or Postman for testing endpoints

## Setup and Running

1. Clone the repository and navigate to the project:
```
git clone <repository-url>
cd actix-web-demo
```

2. Run the server:
```
cargo run
```
The server will start on http://127.0.0.1:8080

## Available Endpoints

1. Hello World:
```
curl http://127.0.0.1:8080/
```

2. Echo Service:
```
curl http://127.0.0.1:8080/echo/hello
```

3. Get API Info:
```
curl http://127.0.0.1:8080/api/info
```

4. Create User:
```
curl -X POST http://127.0.0.1:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"name":"Dzikri Syairozi","email":"dzikrisyairozi@gmail.com","age":21}'
```

## Project Structure

- `main.rs`: Server setup and configuration
- `handlers.rs`: Route handlers for different endpoints
- `models.rs`: Data structures for the application

## Features Demonstrated

- Basic routing with GET and POST methods
- Path parameters with the echo service
- JSON request/response handling
- Structured project organization
- Error handling with proper HTTP status codes

## Notes

This is a demonstration project showing basic Actix Web concepts. For production use, you'd want to add:
- Error handling middleware
- Database integration
- Authentication
- Logging
- Environment configuration