use super::entities::opportunity::{self, Entity as OpportunityEntity};
use crate::application::ports::output::OpportunityRepository;
use crate::domain::{DomainError, Opportunity, OpportunityStage};
use async_trait::async_trait;
use sea_orm::*;
use uuid::Uuid;

pub struct SeaOrmRepo {
    pub db: DatabaseConnection,
}

#[async_trait]
impl OpportunityRepository for SeaOrmRepo {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Opportunity>, DomainError> {
        let model = OpportunityEntity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?; // Mapping DB error to validation for now

        match model {
            Some(m) => {
                // Map DB model to Domain Entity
                // Note: In real app, handling enum parsing errors properly
                let stage = match m.stage.as_str() {
                    "New" => OpportunityStage::New,
                    "Meeting" => OpportunityStage::Meeting,
                    "Proposal" => OpportunityStage::Proposal,
                    "Customer" => OpportunityStage::Customer,
                    "Lost" => OpportunityStage::Lost,
                    _ => OpportunityStage::New, // Fallback
                };

                Ok(Some(Opportunity {
                    id: m.id,
                    name: m.name,
                    stage,
                    amount_micros: m.amount_micros,
                    created_at: m.created_at,
                    updated_at: m.updated_at,
                    deleted_at: None, // Not implementing soft delete in DB for scaffolding yet
                    position: 0,      // Placeholder
                    close_date: None, // Placeholder
                    point_of_contact_id: None, // Placeholder
                    company_id: None, // Placeholder
                }))
            }
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<Opportunity>, DomainError> {
        let models = OpportunityEntity::find()
            .all(&self.db)
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        let opportunities = models
            .into_iter()
            .map(|m| {
                let stage = match m.stage.as_str() {
                    "New" => OpportunityStage::New,
                    "Meeting" => OpportunityStage::Meeting,
                    "Proposal" => OpportunityStage::Proposal,
                    "Customer" => OpportunityStage::Customer,
                    "Lost" => OpportunityStage::Lost,
                    _ => OpportunityStage::New,
                };
                Opportunity {
                    id: m.id,
                    name: m.name,
                    stage,
                    amount_micros: m.amount_micros,
                    created_at: m.created_at,
                    updated_at: m.updated_at,
                    deleted_at: None,
                    position: 0,
                    close_date: None,
                    point_of_contact_id: None,
                    company_id: None,
                }
            })
            .collect();

        Ok(opportunities)
    }

    async fn save(&self, opportunity: &Opportunity) -> Result<(), DomainError> {
        let stage_str = match opportunity.stage {
            OpportunityStage::New => "New",
            OpportunityStage::Meeting => "Meeting",
            OpportunityStage::Proposal => "Proposal",
            OpportunityStage::Customer => "Customer",
            OpportunityStage::Lost => "Lost",
        };

        // Check if exists to decide insert or update
        // Simplified usage here using ActiveModel
        let active_model = opportunity::ActiveModel {
            id: Set(opportunity.id),
            name: Set(opportunity.name.clone()),
            stage: Set(stage_str.to_string()),
            amount_micros: Set(opportunity.amount_micros),
            created_at: Set(opportunity.created_at),
            updated_at: Set(opportunity.updated_at),
        };

        // For simplicity in scaffolding, using insert w/ on conflict or just simple insert for new
        // Real implementation would check ID existence or use `save` if primary key is set
        // SeaORM `insert` fails if PK exists. `save` checks state.
        // Since we are creating from domain entity which has ID, we treat as "simulated upsert" or check first

        // Doing a simple check first
        let exists = OpportunityEntity::find_by_id(opportunity.id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::Validation(e.to_string()))?;

        if exists.is_some() {
            active_model
                .update(&self.db)
                .await
                .map_err(|e| DomainError::Validation(e.to_string()))?;
        } else {
            active_model
                .insert(&self.db)
                .await
                .map_err(|e| DomainError::Validation(e.to_string()))?;
        }

        Ok(())
    }
}
