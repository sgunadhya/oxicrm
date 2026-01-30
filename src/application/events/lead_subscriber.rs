use crate::application::ports::messaging::EventBus;
use crate::application::ports::output::TimelineActivityRepository;
use crate::application::use_cases::send_email::{SendEmail, SendEmailInput};
use crate::domain::states::LeadSource;
use crate::domain::{Lead, TimelineActivity};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct LeadEventSubscriber {
    event_bus: Arc<dyn EventBus>,
    send_email_use_case: Arc<SendEmail>,
    timeline_repo: Arc<dyn TimelineActivityRepository>,
}

impl LeadEventSubscriber {
    pub fn new(
        event_bus: Arc<dyn EventBus>,
        send_email_use_case: Arc<SendEmail>,
        timeline_repo: Arc<dyn TimelineActivityRepository>,
    ) -> Self {
        Self {
            event_bus,
            send_email_use_case,
            timeline_repo,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        // Subscribe to lead events
        let mut receiver = self.event_bus.subscribe("lead.*").await?;

        let send_email_use_case = self.send_email_use_case.clone();
        let timeline_repo = self.timeline_repo.clone();

        // Spawn task to listen for events
        tokio::spawn(async move {
            tracing::info!("LeadEventSubscriber started");

            while let Ok(event) = receiver.recv().await {
                tracing::debug!("LeadEventSubscriber received event: {}", event.topic);

                let result = match event.topic.as_str() {
                    "lead.created" => {
                        Self::handle_lead_created(
                            &send_email_use_case,
                            &timeline_repo,
                            &event.payload,
                        )
                        .await
                    }
                    _ => {
                        // Ignore other events
                        Ok(())
                    }
                };

                if let Err(e) = result {
                    tracing::error!("Error handling event {}: {}", event.topic, e);
                }
            }

            tracing::warn!("LeadEventSubscriber receiver closed");
        });

        Ok(())
    }

    async fn handle_lead_created(
        send_email_use_case: &Arc<SendEmail>,
        timeline_repo: &Arc<dyn TimelineActivityRepository>,
        payload: &str,
    ) -> Result<(), String> {
        // Parse lead data from payload
        let lead: Lead =
            serde_json::from_str(payload).map_err(|e| format!("Failed to parse lead: {}", e))?;

        tracing::info!(
            "New lead created: {} {} ({})",
            lead.first_name,
            lead.last_name,
            lead.email
        );

        // Send notification email to sales team
        let source_display = match lead.source {
            LeadSource::WebForm => "Web Form",
            LeadSource::ManualEntry => "Manual Entry",
            LeadSource::Email => "Email",
            LeadSource::Referral => "Referral",
        };

        let input = SendEmailInput {
            from_email: "noreply@oxicrm.com".to_string(),
            to_email: "sales@oxicrm.com".to_string(), // TODO: Get from config
            cc_emails: None,
            bcc_emails: None,
            subject: format!(
                "New Lead: {} {} ({})",
                lead.first_name, lead.last_name, source_display
            ),
            body_text: format!(
                "New lead captured!\n\n\
                Name: {} {}\n\
                Email: {}\n\
                Company: {}\n\
                Phone: {}\n\
                Job Title: {}\n\
                Source: {}\n\
                Score: {}\n\n\
                View in CRM: http://localhost:3001/leads/{}",
                lead.first_name,
                lead.last_name,
                lead.email,
                lead.company_name.as_deref().unwrap_or("N/A"),
                lead.phone.as_deref().unwrap_or("N/A"),
                lead.job_title.as_deref().unwrap_or("N/A"),
                source_display,
                lead.score,
                lead.id
            ),
            body_html: None,
            template_id: None,
            template_variables: None,
            person_id: None,
            company_id: None,
            opportunity_id: None,
            task_id: None,
            workflow_id: None,
            workflow_run_id: None,
            workspace_id: lead.workspace_id,
        };

        send_email_use_case
            .execute(input)
            .await
            .map_err(|e| format!("Failed to send notification email: {}", e))?;

        // Create timeline activity
        let activity = TimelineActivity {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            name: format!("Lead captured via {}", source_display),
            workspace_member_id: lead.assigned_to_id,
            person_id: None,
            company_id: None,
            opportunity_id: None,
            task_id: None,
            note_id: None,
            calendar_event_id: None,
            workflow_id: None,
            workspace_id: lead.workspace_id,
        };

        timeline_repo
            .create(activity)
            .await
            .map_err(|e| format!("Failed to create timeline activity: {}", e))?;

        Ok(())
    }
}
