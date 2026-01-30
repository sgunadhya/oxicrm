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
