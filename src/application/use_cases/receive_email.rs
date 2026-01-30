use crate::application::ports::output::{EmailRepository, TimelineActivityRepository};
use crate::domain::{DomainError, Email, EmailDirection, EmailStatus, HardGuard, TimelineActivity};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct ReceiveEmailInput {
    pub from_email: String,
    pub to_email: String,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub received_at: DateTime<Utc>,
}

pub struct ReceiveEmail {
    email_repo: Arc<dyn EmailRepository>,
    timeline_repo: Arc<dyn TimelineActivityRepository>,
}

impl ReceiveEmail {
    pub fn new(
        email_repo: Arc<dyn EmailRepository>,
        timeline_repo: Arc<dyn TimelineActivityRepository>,
    ) -> Self {
        Self {
            email_repo,
            timeline_repo,
        }
    }

    pub async fn execute(&self, input: ReceiveEmailInput) -> Result<Email, DomainError> {
        // 1. Create inbound email record
        let email = Email {
            id: Uuid::new_v4(),
            created_at: input.received_at,
            updated_at: input.received_at,
            direction: EmailDirection::Inbound,
            status: EmailStatus::Received,
            from_email: input.from_email.clone(),
            to_email: input.to_email.clone(),
            cc_emails: None,
            bcc_emails: None,
            subject: input.subject.clone(),
            body_text: input.body_text.clone(),
            body_html: input.body_html.clone(),
            sent_at: None,
            failed_at: None,
            error_message: None,
            email_template_id: None,
            timeline_activity_id: None,
            person_id: None, // TODO: Match email to person by address
            company_id: None,
            opportunity_id: None,
            task_id: None,
            workflow_id: None,
            workflow_run_id: None,
            metadata: None,
            workspace_id: Uuid::default(), // TODO: Resolve workspace from To address or domain
        };

        // Validate email
        email.validate()?;

        // Create email record
        let email = self.email_repo.create(email).await?;

        // 2. Create timeline activity
        let activity_name = format!("Email received from {}", email.from_email);
        let timeline_activity = TimelineActivity {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            name: activity_name,
            workspace_member_id: None,
            person_id: email.person_id,
            company_id: email.company_id,
            opportunity_id: email.opportunity_id,
            task_id: email.task_id,
            note_id: None,
            calendar_event_id: None,
            workflow_id: email.workflow_id,
            workspace_id: email.workspace_id,
        };

        let timeline_activity = self.timeline_repo.create(timeline_activity).await?;

        // 3. Link timeline activity to email
        let mut final_email = email;
        final_email.timeline_activity_id = Some(timeline_activity.id);
        let final_email = self.email_repo.update(final_email).await?;

        Ok(final_email)
    }
}
