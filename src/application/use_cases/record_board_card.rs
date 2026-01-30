use crate::application::ports::input::RecordUseCase;
use crate::application::ports::output::OpportunityRepository;
use crate::domain::OpportunityStage;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

pub struct RecordBoardCard {
    pub opportunity_repo: Arc<dyn OpportunityRepository>,
}

#[async_trait]
impl RecordUseCase for RecordBoardCard {
    async fn move_board_card(
        &self,
        card_id: Uuid,
        new_stage: OpportunityStage,
    ) -> Result<(), String> {
        let mut opportunity = self
            .opportunity_repo
            .find_by_id(card_id)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("Opportunity not found")?;

        opportunity.stage = new_stage;
        opportunity.updated_at = chrono::Utc::now();

        // In a real app, we'd check invariants here

        self.opportunity_repo
            .update(opportunity)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn list_opportunities(&self) -> Result<Vec<crate::domain::Opportunity>, String> {
        self.opportunity_repo
            .find_all()
            .await
            .map_err(|e| e.to_string())
    }
}
