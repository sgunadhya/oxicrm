use crate::application::ports::output::{LeadRepository, TimelineActivityRepository};
use crate::domain::states::LeadStatus;
use crate::domain::{DomainError, Lead, TimelineActivity};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct ManageLead {
    lead_repo: Arc<dyn LeadRepository>,
    timeline_repo: Arc<dyn TimelineActivityRepository>,
}

impl ManageLead {
    pub fn new(
        lead_repo: Arc<dyn LeadRepository>,
        timeline_repo: Arc<dyn TimelineActivityRepository>,
    ) -> Self {
        Self {
            lead_repo,
            timeline_repo,
        }
    }

    pub async fn list(&self) -> Result<Vec<Lead>, DomainError> {
        self.lead_repo.find_all().await
    }

    pub async fn get(&self, id: Uuid) -> Result<Lead, DomainError> {
        self.lead_repo
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound)
    }

    pub async fn update_status(&self, id: Uuid, status: LeadStatus) -> Result<Lead, DomainError> {
        let mut lead = self.get(id).await?;

        // Don't allow status change if converted
        if lead.is_converted() && status != LeadStatus::Converted {
            return Err(DomainError::InvalidState(
                "Cannot change status of converted lead".into(),
            ));
        }

        lead.status = status;
        lead.updated_at = Utc::now();

        if status == LeadStatus::Contacted {
            lead.last_contacted_at = Some(Utc::now());
        }

        let lead = self.lead_repo.update(lead).await?;

        // Create timeline activity
        let activity = TimelineActivity {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            name: format!("Lead status changed to {:?}", status),
            workspace_member_id: lead.assigned_to_id,
            person_id: lead.converted_person_id,
            company_id: lead.converted_company_id,
            opportunity_id: lead.converted_opportunity_id,
            task_id: None,
            note_id: None,
            calendar_event_id: None,
            workflow_id: None,
            workspace_id: lead.workspace_id,
        };
        self.timeline_repo.create(activity).await?;

        Ok(lead)
    }

    pub async fn assign(&self, id: Uuid, assigned_to_id: Uuid) -> Result<Lead, DomainError> {
        let mut lead = self.get(id).await?;
        lead.assigned_to_id = Some(assigned_to_id);
        lead.updated_at = Utc::now();
        self.lead_repo.update(lead).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.lead_repo.delete(id).await
    }
}
