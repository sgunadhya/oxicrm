use crate::application::ports::output::TaskRepository;
use crate::domain::{DomainError, Task, TaskStatus};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateTaskInput {
    pub title: String,
    pub body: Option<String>,
    pub status: Option<String>,
    pub assignee_id: Option<Uuid>,
    pub due_at: Option<chrono::DateTime<Utc>>,
    pub workspace_id: Uuid,
}

pub struct CreateTask {
    task_repo: Arc<dyn TaskRepository>,
}

impl CreateTask {
    pub fn new(task_repo: Arc<dyn TaskRepository>) -> Self {
        Self { task_repo }
    }

    pub async fn execute(&self, input: CreateTaskInput) -> Result<Task, DomainError> {
        // Parse status from string or default to Todo
        let status = if let Some(status_str) = input.status {
            match status_str.as_str() {
                "TODO" => TaskStatus::Todo,
                "IN_PROGRESS" => TaskStatus::InProgress,
                "DONE" => TaskStatus::Done,
                _ => TaskStatus::Todo,
            }
        } else {
            TaskStatus::Todo
        };

        let task = Task {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            title: input.title,
            body: input.body,
            status,
            position: 0, // Auto-assigned
            assignee_id: input.assignee_id,
            due_at: input.due_at,
            workspace_id: input.workspace_id,
        };

        self.task_repo.create(task).await
    }
}
