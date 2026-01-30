use crate::domain::OpportunityStage;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RecordUseCase: Send + Sync {
    async fn move_board_card(
        &self,
        card_id: Uuid,
        new_stage: OpportunityStage,
    ) -> Result<(), String>;

    async fn list_opportunities(&self) -> Result<Vec<crate::domain::Opportunity>, String>;
}

#[async_trait]
pub trait RegisterUserUseCase: Send + Sync {
    async fn execute(
        &self,
        email: String,
        password: String,
    ) -> Result<crate::domain::entities::User, crate::domain::DomainError>;
}

#[async_trait]
pub trait CreateWorkspaceUseCase: Send + Sync {
    async fn execute(
        &self,
        user_id: Uuid,
        subdomain: String,
    ) -> Result<crate::domain::entities::Workspace, crate::domain::DomainError>;
}
