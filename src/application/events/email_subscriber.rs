use crate::application::ports::messaging::EventBus;
use crate::application::use_cases::send_email::{SendEmail, SendEmailInput};
use std::sync::Arc;

pub struct EmailEventSubscriber {
    event_bus: Arc<dyn EventBus>,
    send_email_use_case: Arc<SendEmail>,
}

impl EmailEventSubscriber {
    pub fn new(event_bus: Arc<dyn EventBus>, send_email_use_case: Arc<SendEmail>) -> Self {
        Self {
            event_bus,
            send_email_use_case,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        // Subscribe to all events using wildcard
        let mut receiver = self.event_bus.subscribe("*").await?;

        let send_email_use_case = self.send_email_use_case.clone();

        // Spawn task to listen for events
        tokio::spawn(async move {
            tracing::info!("EmailEventSubscriber started");

            while let Ok(event) = receiver.recv().await {
                tracing::debug!("EmailEventSubscriber received event: {}", event.topic);

                let result = match event.topic.as_str() {
                    "opportunity.created" => {
                        Self::handle_opportunity_created(&send_email_use_case, &event.payload).await
                    }
                    "task.assigned" => {
                        Self::handle_task_assigned(&send_email_use_case, &event.payload).await
                    }
                    "opportunity.won" => {
                        Self::handle_opportunity_won(&send_email_use_case, &event.payload).await
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

            tracing::warn!("EmailEventSubscriber receiver closed");
        });

        Ok(())
    }

    async fn handle_opportunity_created(
        send_email_use_case: &Arc<SendEmail>,
        payload: &str,
    ) -> Result<(), String> {
        // Parse opportunity data from payload
        let opportunity_data: serde_json::Value = serde_json::from_str(payload)
            .map_err(|e| format!("Failed to parse opportunity data: {}", e))?;

        let opportunity_id = opportunity_data
            .get("id")
            .and_then(|v| v.as_str())
            .and_then(|s| uuid::Uuid::parse_str(s).ok())
            .ok_or_else(|| "Missing or invalid opportunity ID".to_string())?;

        let person_name = opportunity_data
            .get("person_name")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        tracing::info!(
            "Sending notification email for new opportunity: {}",
            opportunity_id
        );

        // Send notification email
        let input = SendEmailInput {
            from_email: "noreply@oxicrm.com".to_string(),
            to_email: "sales@oxicrm.com".to_string(), // TODO: Get from config
            cc_emails: None,
            bcc_emails: None,
            subject: format!("New Opportunity Created: {}", person_name),
            body_text: format!(
                "A new opportunity has been created.\n\nOpportunity ID: {}\nContact: {}\n",
                opportunity_id, person_name
            ),
            body_html: None,
            template_id: None,
            template_variables: None,
            person_id: None,
            company_id: None,
            opportunity_id: Some(opportunity_id),
            task_id: None,
            workflow_id: None,
            workflow_run_id: None,
            workspace_id: uuid::Uuid::default(), // TODO: Resolve workspace for system emails
        };

        send_email_use_case
            .execute(input)
            .await
            .map_err(|e| format!("Failed to send email: {}", e))?;

        Ok(())
    }

    async fn handle_task_assigned(
        send_email_use_case: &Arc<SendEmail>,
        payload: &str,
    ) -> Result<(), String> {
        // Parse task data from payload
        let task_data: serde_json::Value = serde_json::from_str(payload)
            .map_err(|e| format!("Failed to parse task data: {}", e))?;

        let task_id = task_data
            .get("id")
            .and_then(|v| v.as_str())
            .and_then(|s| uuid::Uuid::parse_str(s).ok())
            .ok_or_else(|| "Missing or invalid task ID".to_string())?;

        let assignee_email = task_data
            .get("assignee_email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Missing assignee email".to_string())?;

        let task_title = task_data
            .get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("Untitled Task");

        tracing::info!("Sending task assignment email to: {}", assignee_email);

        // Send assignment notification
        let input = SendEmailInput {
            from_email: "noreply@oxicrm.com".to_string(),
            to_email: assignee_email.to_string(),
            cc_emails: None,
            bcc_emails: None,
            subject: format!("Task Assigned: {}", task_title),
            body_text: format!(
                "You have been assigned a new task.\n\nTask: {}\n\nPlease review and complete it.",
                task_title
            ),
            body_html: None,
            template_id: None,
            template_variables: None,
            person_id: None,
            company_id: None,
            opportunity_id: None,
            task_id: Some(task_id),
            workflow_id: None,
            workflow_run_id: None,
            workspace_id: uuid::Uuid::default(), // TODO: Resolve workspace for system emails
        };

        send_email_use_case
            .execute(input)
            .await
            .map_err(|e| format!("Failed to send email: {}", e))?;

        Ok(())
    }

    async fn handle_opportunity_won(
        send_email_use_case: &Arc<SendEmail>,
        payload: &str,
    ) -> Result<(), String> {
        // Parse opportunity data from payload
        let opportunity_data: serde_json::Value = serde_json::from_str(payload)
            .map_err(|e| format!("Failed to parse opportunity data: {}", e))?;

        let opportunity_id = opportunity_data
            .get("id")
            .and_then(|v| v.as_str())
            .and_then(|s| uuid::Uuid::parse_str(s).ok())
            .ok_or_else(|| "Missing or invalid opportunity ID".to_string())?;

        let person_email = opportunity_data
            .get("person_email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Missing person email".to_string())?;

        let person_name = opportunity_data
            .get("person_name")
            .and_then(|v| v.as_str())
            .unwrap_or("Valued Customer");

        tracing::info!(
            "Sending congratulations email for won opportunity: {}",
            opportunity_id
        );

        // Send congratulations email
        let input = SendEmailInput {
            from_email: "noreply@oxicrm.com".to_string(),
            to_email: person_email.to_string(),
            cc_emails: None,
            bcc_emails: None,
            subject: "Congratulations! 🎉".to_string(),
            body_text: format!(
                "Dear {},\n\nCongratulations on your successful partnership with us!\n\nWe're excited to work together.\n\nBest regards,\nThe Team",
                person_name
            ),
            body_html: None,
            template_id: None,
            template_variables: None,
            person_id: None,
            company_id: None,
            opportunity_id: Some(opportunity_id),
            task_id: None,
            workflow_id: None,
            workflow_run_id: None,
            workspace_id: uuid::Uuid::default(), // TODO: Resolve workspace for system emails
        };

        send_email_use_case
            .execute(input)
            .await
            .map_err(|e| format!("Failed to send email: {}", e))?;

        Ok(())
    }
}
