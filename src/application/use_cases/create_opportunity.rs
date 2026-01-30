use crate::application::ports::output::OpportunityRepository;
use crate::domain::{DomainError, Opportunity, OpportunityStage};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateOpportunityInput {
    pub name: String,
    pub stage: Option<String>,
    pub amount_micros: Option<i64>,
    pub currency_code: Option<String>,
    pub close_date: Option<chrono::NaiveDate>,
    pub company_id: Option<Uuid>,
    pub point_of_contact_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
    pub workspace_id: Uuid,
}

pub struct CreateOpportunity {
    opportunity_repo: Arc<dyn OpportunityRepository>,
}

impl CreateOpportunity {
    pub fn new(opportunity_repo: Arc<dyn OpportunityRepository>) -> Self {
        Self { opportunity_repo }
    }

    pub async fn execute(&self, input: CreateOpportunityInput) -> Result<Opportunity, DomainError> {
        // Parse stage from string or default to Prospecting
        let stage = if let Some(stage_str) = input.stage {
            match stage_str.as_str() {
                "Prospecting" => OpportunityStage::Prospecting,
                "Qualification" => OpportunityStage::Qualification,
                "Negotiation" => OpportunityStage::Negotiation,
                "Won" => OpportunityStage::Won,
                "Lost" => OpportunityStage::Lost,
                _ => OpportunityStage::Prospecting,
            }
        } else {
            OpportunityStage::Prospecting
        };

        let opportunity = Opportunity {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            name: input.name,
            stage,
            amount_micros: input.amount_micros,
            currency_code: input.currency_code.or(Some("USD".to_string())),
            close_date: input.close_date,
            company_id: input.company_id,
            point_of_contact_id: input.point_of_contact_id,
            owner_id: input.owner_id,
            position: 0,
            workspace_id: input.workspace_id,
        };

        self.opportunity_repo.create(opportunity).await
    }
}
