use crate::application::ports::output::OpportunityRepository;
use crate::domain::{DomainError, Opportunity, OpportunityStage};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateOpportunityInput {
    pub id: Uuid,
    pub name: Option<String>,
    pub stage: Option<String>,
    pub amount_micros: Option<i64>,
    pub currency_code: Option<String>,
    pub close_date: Option<chrono::NaiveDate>,
    pub company_id: Option<Uuid>,
    pub point_of_contact_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
}

pub struct ManageOpportunity {
    opportunity_repo: Arc<dyn OpportunityRepository>,
}

impl ManageOpportunity {
    pub fn new(opportunity_repo: Arc<dyn OpportunityRepository>) -> Self {
        Self { opportunity_repo }
    }

    pub async fn get_all(&self) -> Result<Vec<Opportunity>, DomainError> {
        self.opportunity_repo.find_all().await
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Opportunity>, DomainError> {
        self.opportunity_repo.find_by_id(id).await
    }

    pub async fn update(&self, input: UpdateOpportunityInput) -> Result<Opportunity, DomainError> {
        // First fetch the existing opportunity
        let existing = self
            .opportunity_repo
            .find_by_id(input.id)
            .await?
            .ok_or(DomainError::NotFound)?;

        // Parse stage if provided
        let stage = if let Some(stage_str) = input.stage {
            match stage_str.as_str() {
                "Prospecting" => OpportunityStage::Prospecting,
                "Qualification" => OpportunityStage::Qualification,
                "Negotiation" => OpportunityStage::Negotiation,
                "Won" => OpportunityStage::Won,
                "Lost" => OpportunityStage::Lost,
                _ => existing.stage,
            }
        } else {
            existing.stage
        };

        // Build updated opportunity with existing values as defaults
        let updated = Opportunity {
            id: existing.id,
            created_at: existing.created_at,
            updated_at: chrono::Utc::now(),
            deleted_at: existing.deleted_at,
            name: input.name.unwrap_or(existing.name),
            stage,
            amount_micros: input.amount_micros.or(existing.amount_micros),
            currency_code: input.currency_code.or(existing.currency_code),
            close_date: input.close_date.or(existing.close_date),
            company_id: input.company_id.or(existing.company_id),
            point_of_contact_id: input.point_of_contact_id.or(existing.point_of_contact_id),
            owner_id: input.owner_id.or(existing.owner_id),
            position: existing.position,
            workspace_id: existing.workspace_id,
        };

        self.opportunity_repo.update(updated).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.opportunity_repo.delete(id).await
    }
}
