use crate::application::ports::input::CreateWorkspaceUseCase;
use crate::application::ports::output::WorkspaceRepository;
use crate::domain::entities::{Workspace, WorkspaceMember};
use crate::domain::states::WorkspaceState;
use crate::domain::DomainError;
use async_trait::async_trait;
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateWorkspace {
    workspace_repo: Arc<dyn WorkspaceRepository>,
}

impl CreateWorkspace {
    pub fn new(workspace_repo: Arc<dyn WorkspaceRepository>) -> Self {
        Self { workspace_repo }
    }
}

#[async_trait]
impl CreateWorkspaceUseCase for CreateWorkspace {
    async fn execute(&self, user_id: Uuid, subdomain: String) -> Result<Workspace, DomainError> {
        // 1. Check if subdomain exists
        if let Some(_) = self.workspace_repo.find_by_subdomain(&subdomain).await? {
            return Err(DomainError::Validation(
                "Subdomain already taken".to_string(),
            ));
        }

        // 2. Create Workspace
        let workspace = Workspace {
            id: Uuid::new_v4(),
            subdomain,
            state: WorkspaceState::Active, // Start as active for MVP
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let saved_workspace = self.workspace_repo.create(workspace).await?;

        // 3. Add Creator as Admin
        let member = WorkspaceMember {
            id: Uuid::new_v4(),
            user_id,
            workspace_id: saved_workspace.id,
            role: "Admin".to_string(),
            name: "Admin".to_string(), // Placeholder name until we have user profile
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.workspace_repo.add_member(member).await?;

        Ok(saved_workspace)
    }
}
