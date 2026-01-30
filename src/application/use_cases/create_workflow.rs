use crate::application::ports::output::WorkflowRepository;
use crate::domain::{DomainError, Workflow};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateWorkflowInput {
    pub name: String,
    pub workspace_id: Uuid,
}

pub struct CreateWorkflow {
    workflow_repo: Arc<dyn WorkflowRepository>,
}

impl CreateWorkflow {
    pub fn new(workflow_repo: Arc<dyn WorkflowRepository>) -> Self {
        Self { workflow_repo }
    }

    pub async fn execute(&self, input: CreateWorkflowInput) -> Result<Workflow, DomainError> {
        let workflow = Workflow {
            id: Uuid::new_v4(),
            name: input.name,
            last_published_version_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            workspace_id: input.workspace_id,
        };

        self.workflow_repo.create(workflow).await
    }
}
