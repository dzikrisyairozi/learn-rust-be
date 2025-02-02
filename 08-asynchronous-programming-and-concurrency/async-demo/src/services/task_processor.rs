use crate::models::task::{Task, TaskStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

pub struct TaskProcessor {
    tasks: Arc<Mutex<HashMap<Uuid, Task>>>,
    task_sender: mpsc::Sender<Uuid>,
    task_receiver: Mutex<mpsc::Receiver<Uuid>>,
}

impl TaskProcessor {
    pub fn new() -> Self {
        let (task_sender, task_receiver) = mpsc::channel(100);
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            task_sender,
            task_receiver: Mutex::new(task_receiver),
        }
    }

    // Submit a new task for processing
    pub async fn submit_task(&self, task: Task) -> Uuid {
        let task_id = task.id;
        
        // Store task in shared state
        self.tasks.lock().await.insert(task_id, task);
        
        // Send task ID to processing queue
        self.task_sender.send(task_id).await.expect("Failed to send task");
        
        task_id
    }

    // Get task status
    pub async fn get_task(&self, task_id: Uuid) -> Option<Task> {
        self.tasks.lock().await.get(&task_id).cloned()
    }

    // Background task processor
    pub async fn run(&self) {
        let mut receiver = self.task_receiver.lock().await;
        
        while let Some(task_id) = receiver.recv().await {
            // Process task asynchronously
            let tasks = Arc::clone(&self.tasks);
            
            tokio::spawn(async move {
                let mut tasks = tasks.lock().await;
                if let Some(task) = tasks.get_mut(&task_id) {
                    // Update task status to Processing
                    task.status = TaskStatus::Processing;
                    
                    // Simulate async work
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    
                    // Update task status to Completed
                    task.status = TaskStatus::Completed;
                }
            });
        }
    }

    // Batch process multiple tasks
    pub async fn batch_process(&self, tasks: Vec<Task>) -> Vec<Uuid> {
        let mut task_ids = Vec::new();
        
        // Process tasks concurrently using futures
        let futures: Vec<_> = tasks
            .into_iter()
            .map(|task| {
                let task_id = task.id;
                task_ids.push(task_id);
                self.submit_task(task)
            })
            .collect();
        
        futures::future::join_all(futures).await;
        
        task_ids
    }
}