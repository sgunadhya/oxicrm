use crate::application::ports::messaging::{DomainEvent, EventBus};
use crate::application::ports::output::LeadRepository;
use crate::domain::states::{LeadSource, LeadStatus};
use crate::domain::{DomainError, Lead};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLeadInput {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub job_title: Option<String>,
    pub source: LeadSource,
    pub notes: Option<String>,
    pub workspace_id: Uuid,
}

pub struct CreateLead {
    lead_repo: Arc<dyn LeadRepository>,
    event_bus: Arc<dyn EventBus>,
}

impl CreateLead {
    pub fn new(lead_repo: Arc<dyn LeadRepository>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            lead_repo,
            event_bus,
        }
    }

    pub async fn execute(&self, input: CreateLeadInput) -> Result<Lead, DomainError> {
        // 1. Check email uniqueness
        if (self.lead_repo.find_by_email(&input.email).await?).is_some() {
            return Err(DomainError::Validation("Email already exists".into()));
        }

        // 2. Create lead entity
        let mut lead = Lead {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            first_name: input.first_name,
            last_name: input.last_name,
            email: input.email,
            phone: input.phone,
            company_name: input.company_name,
            job_title: input.job_title,
            source: input.source,
            status: LeadStatus::New,
            score: 0, // Will be calculated next
            notes: input.notes,
            position: 0,
            assigned_to_id: None, // Will be assigned next
            converted_person_id: None,
            converted_company_id: None,
            converted_opportunity_id: None,
            converted_at: None,
            last_contacted_at: None,
            workspace_id: input.workspace_id,
        };

        // 3. Calculate score
        let score = lead.calculate_score();
        lead.score = score;

        // 4. Auto-assign using round-robin (placeholder for now)
        let assigned_to_id = self.find_next_assignee().await?;
        lead.assigned_to_id = assigned_to_id;

        // 5. Validate
        use crate::domain::HardGuard;
        lead.validate()?;

        // 6. Save
        let lead = self.lead_repo.create(lead).await?;

        // 7. Publish event
        self.event_bus
            .publish(&DomainEvent {
                topic: "lead.created".to_string(),
                payload: serde_json::to_string(&lead).unwrap_or_default(),
            })
            .await
            .ok(); // Don't fail the use case if event publishing fails

        Ok(lead)
    }

    async fn find_next_assignee(&self) -> Result<Option<Uuid>, DomainError> {
        // Round-robin: Find workspace member with fewest assigned leads
        // For now, return None (manual assignment)
        // TODO: Implement round-robin logic
        Ok(None)
    }
}
