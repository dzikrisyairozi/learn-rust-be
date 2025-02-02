# Asynchronous Programming Demo

A demonstration of asynchronous programming and concurrency patterns in Rust using Actix Web. This project showcases task processing, background jobs, and concurrent operations.

## Features

- Asynchronous task processing
- Background job handling
- Concurrent batch operations
- Real-time task status tracking
- Priority-based task queue
- RESTful API endpoints

## Prerequisites

- Rust and Cargo installed
- Basic understanding of async/await in Rust
- Familiarity with RESTful APIs

## Project Setup

1. Build and run the project:
```bash
cargo build
cargo run
```

The server will start at `http://localhost:8080`

## API Endpoints

### 1. Submit Single Task

Submit a new task for processing:

```bash
curl -X POST http://localhost:8080/api/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Important Task",
    "priority": 1
  }'
```

Response:
```bash
{
  "task_id": "123e4567-e89b-12d3-a456-426614174000"
}
```

### 2. Check Task Status

Get the status of a specific task:

```bash
curl http://localhost:8080/api/tasks/123e4567-e89b-12d3-a456-426614174000
```

Response:
```bash
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "name": "Important Task",
  "priority": 1,
  "status": "Completed"
}
```

### 3. Batch Process Tasks

Submit multiple tasks for concurrent processing:

```bash
curl -X POST http://localhost:8080/api/tasks/batch \
  -H "Content-Type: application/json" \
  -d '[
    {
      "name": "Task 1",
      "priority": 1
    },
    {
      "name": "Task 2",
      "priority": 2
    }
  ]'
```

Response:
```bash
[
  "123e4567-e89b-12d3-a456-426614174000",
  "987fcdeb-51d3-a456-426614174000"
]
```

## Task States

Tasks can be in one of these states:

- `Pending`: Task is queued for processing
- `Processing`: Task is currently being processed
- `Completed`: Task has been successfully completed
- `Failed`: Task processing failed (includes error message)

## Implementation Details

### Task Processing Flow

1. When a task is submitted:
   - A unique UUID is generated
   - Task is stored in shared state
   - Task ID is sent to processing queue

2. Background processor:
   - Continuously monitors queue for new tasks
   - Updates task status as processing occurs
   - Handles task completion or failure

3. Concurrent processing:
   - Multiple tasks can be processed simultaneously
   - Task priority determines processing order
   - Uses Tokio for async execution

## Project Structure

```bash
async-demo/
├── src/
│   ├── main.rs           # Application entry point
│   ├── models/
│   │   └── task.rs       # Task data structures
│   ├── handlers/
│   │   └── tasks.rs      # API endpoint handlers
│   └── services/
│       └── task_processor.rs  # Async task processing
├── Cargo.toml
└── .env
```

## Error Handling

The API returns appropriate HTTP status codes:

- 202 Accepted: Task submitted successfully
- 200 OK: Task status retrieved
- 404 Not Found: Task doesn't exist
- 500 Internal Server Error: Processing error

## Testing the Application

1. Start the server:
```bash
cargo run
```

2. Submit a test task:
```bash
curl -X POST http://localhost:8080/api/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Task",
    "priority": 1
  }'
```

3. Note the returned task ID and check its status:
```bash
curl http://localhost:8080/api/tasks/<returned-task-id>
```

4. Try batch processing:
```bash
curl -X POST http://localhost:8080/api/tasks/batch \
  -H "Content-Type: application/json" \
  -d '[
    {"name": "Batch Task 1", "priority": 1},
    {"name": "Batch Task 2", "priority": 2}
  ]'
```

## Performance Considerations

- The task processor uses a channel size of 100 tasks
- Each task simulates a 2-second processing time
- Batch processing runs tasks concurrently
- Shared state is protected by async Mutex

## Development

To run in development mode with logging:
```bash
RUST_LOG=debug cargo run
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.