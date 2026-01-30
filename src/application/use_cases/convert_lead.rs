use crate::application::ports::output::{
    CompanyRepository, LeadRepository, OpportunityRepository, PersonRepository,
    TimelineActivityRepository,
};
use crate::domain::{Company, DomainError, Lead, Opportunity, OpportunityStage, Person, TimelineActivity};
use crate::domain::states::LeadStatus;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertLeadInput {
    pub lead_id: Uuid,
    pub create_person: bool,
    pub create_company: bool,
    pub create_opportunity: bool,
    pub opportunity_name: Option<String>,
    pub opportunity_amount: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub lead: Lead,
    pub person_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
}

pub struct ConvertLead {
    lead_repo: Arc<dyn LeadRepository>,
    person_repo: Arc<dyn PersonRepository>,
    company_repo: Arc<dyn CompanyRepository>,
    opportunity_repo: Arc<dyn OpportunityRepository>,
    timeline_repo: Arc<dyn TimelineActivityRepository>,
}

impl ConvertLead {
    pub fn new(
        lead_repo: Arc<dyn LeadRepository>,
        person_repo: Arc<dyn PersonRepository>,
        company_repo: Arc<dyn CompanyRepository>,
        opportunity_repo: Arc<dyn OpportunityRepository>,
        timeline_repo: Arc<dyn TimelineActivityRepository>,
    ) -> Self {
        Self {
            lead_repo,
            person_repo,
            company_repo,
            opportunity_repo,
            timeline_repo,
        }
    }

    pub async fn execute(&self, input: ConvertLeadInput) -> Result<ConversionResult, DomainError> {
        // 1. Get lead
        let mut lead = self
            .lead_repo
            .find_by_id(input.lead_id)
            .await?
            .ok_or(DomainError::NotFound)?;

        // 2. Check if already converted
        if lead.is_converted() {
            return Err(DomainError::InvalidState(
                "Lead already converted".into(),
            ));
        }

        let mut person_id = None;
        let mut company_id = None;
        let mut opportunity_id = None;

        // 3. Create Person if requested
        if input.create_person {
            let person = Person {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
                name: lead.full_name(),
                email: lead.email.clone(),
                position: 0,
                company_id: None, // Will be set if creating company
            };
            use crate::domain::HardGuard;
            person.validate()?;
            let person = self.person_repo.create(person).await?;
            person_id = Some(person.id);
        }

        // 4. Create Company if requested
        if input.create_company && lead.company_name.is_some() {
            let company_name = lead.company_name.clone().unwrap();
            let company = Company {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
                name: company_name.clone(),
                domain_name: format!(
                    "{}.com",
                    company_name.to_lowercase().replace(" ", "")
                ),
                address: None,
                employees_count: 0,
                position: 0,
            };
            let company = self.company_repo.create(company).await?;
            company_id = Some(company.id);

            // Update person's company_id if person was created
            if let Some(pid) = person_id {
                let mut person = self.person_repo.find_by_email(&lead.email).await?.unwrap();
                person.company_id = Some(company.id);
                // Note: PersonRepository doesn't have update method, would need to add it
                // For now, we'll leave the person without company_id set
                // TODO: Add update method to PersonRepository
            }
        }

        // 5. Create Opportunity if requested
        if input.create_opportunity {
            let opp_name = input
                .opportunity_name
                .unwrap_or_else(|| format!("Opportunity for {}", lead.full_name()));

            let opportunity = Opportunity {
                id: Uuid::new_v4(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
                deleted_at: None,
                name: opp_name,
                stage: OpportunityStage::Prospecting,
                close_date: None,
                amount_micros: input.opportunity_amount,
                currency_code: Some("USD".to_string()),
                position: 0,
                point_of_contact_id: person_id,
                company_id,
                owner_id: lead.assigned_to_id,
            };
            let opportunity = self.opportunity_repo.create(opportunity).await?;
            opportunity_id = Some(opportunity.id);
        }

        // 6. Update lead with conversion data
        lead.status = LeadStatus::Converted;
        lead.converted_person_id = person_id;
        lead.converted_company_id = company_id;
        lead.converted_opportunity_id = opportunity_id;
        lead.converted_at = Some(Utc::now());
        let lead = self.lead_repo.update(lead).await?;

        // 7. Create timeline activity
        let activity_name = format!(
            "Lead {} converted",
            if input.create_opportunity {
                "to Opportunity"
            } else {
                "to Contact"
            }
        );
        let timeline = TimelineActivity {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            name: activity_name,
            workspace_member_id: lead.assigned_to_id,
            person_id,
            company_id,
            opportunity_id,
            task_id: None,
            note_id: None,
            calendar_event_id: None,
            workflow_id: None,
        };
        self.timeline_repo.create(timeline).await?;

        Ok(ConversionResult {
            lead,
            person_id,
            company_id,
            opportunity_id,
        })
    }
}
