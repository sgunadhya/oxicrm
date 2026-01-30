use crate::application::ports::output::WorkflowRepository;
use crate::domain::{DomainError, Workflow};
use std::sync::Arc;
use uuid::Uuid;

pub struct ManageWorkflow {
    workflow_repo: Arc<dyn WorkflowRepository>,
}

impl ManageWorkflow {
    pub fn new(workflow_repo: Arc<dyn WorkflowRepository>) -> Self {
        Self { workflow_repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Workflow>, DomainError> {
        self.workflow_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Workflow>, DomainError> {
        self.workflow_repo.find_by_id(id).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.workflow_repo.delete(id).await
    }
}
