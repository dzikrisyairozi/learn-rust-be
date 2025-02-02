use crate::models::task::{CreateTaskRequest, Task, TaskStatus};
use crate::services::task_processor::TaskProcessor;
use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Arc;
use uuid::Uuid;

#[post("/tasks")]
pub async fn submit_task(
    processor: web::Data<Arc<TaskProcessor>>,
    request: web::Json<CreateTaskRequest>,
) -> impl Responder {
    // Create new task
    let task = Task {
        id: Uuid::new_v4(),
        name: request.name.clone(),
        priority: request.priority,
        status: TaskStatus::Pending,
    };

    // Submit task for processing
    let task_id = processor.submit_task(task).await;

    HttpResponse::Accepted().json(task_id)
}

#[get("/tasks/{task_id}")]
pub async fn get_task_status(
    processor: web::Data<Arc<TaskProcessor>>,
    task_id: web::Path<Uuid>,
) -> impl Responder {
    match processor.get_task(*task_id).await {
        Some(task) => HttpResponse::Ok().json(task),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/tasks/batch")]
pub async fn batch_process(
    processor: web::Data<Arc<TaskProcessor>>,
    requests: web::Json<Vec<CreateTaskRequest>>,
) -> impl Responder {
    // Create tasks from requests
    let tasks: Vec<Task> = requests
        .into_inner()
        .into_iter()
        .map(|req| Task {
            id: Uuid::new_v4(),
            name: req.name,
            priority: req.priority,
            status: TaskStatus::Pending,
        })
        .collect();

    // Process tasks in batch
    let task_ids = processor.batch_process(tasks).await;

    HttpResponse::Accepted().json(task_ids)
}