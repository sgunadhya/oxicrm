use crate::application::ports::output::TaskRepository;
use crate::domain::{DomainError, Task, TaskStatus};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateTaskInput {
    pub id: Uuid,
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub assignee_id: Option<Uuid>,
    pub due_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct ManageTask {
    task_repo: Arc<dyn TaskRepository>,
}

impl ManageTask {
    pub fn new(task_repo: Arc<dyn TaskRepository>) -> Self {
        Self { task_repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Task>, DomainError> {
        self.task_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Task>, DomainError> {
        self.task_repo.find_by_id(id).await
    }

    pub async fn update(&self, input: UpdateTaskInput) -> Result<Task, DomainError> {
        // First fetch the existing task
        let existing = self
            .task_repo
            .find_by_id(input.id)
            .await?
            .ok_or(DomainError::NotFound)?;

        // Parse status if provided
        let status = if let Some(status_str) = input.status {
            match status_str.as_str() {
                "TODO" => TaskStatus::Todo,
                "IN_PROGRESS" => TaskStatus::InProgress,
                "DONE" => TaskStatus::Done,
                _ => existing.status,
            }
        } else {
            existing.status
        };

        // Build updated task
        let updated = Task {
            id: existing.id,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now(),
            deleted_at: existing.deleted_at,
            title: input.title.unwrap_or(existing.title),
            body: input.body.or(existing.body),
            status,
            position: existing.position,
            assignee_id: input.assignee_id.or(existing.assignee_id),
            due_at: input.due_at.or(existing.due_at),
        };

        self.task_repo.update(updated).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.task_repo.delete(id).await
    }
}
