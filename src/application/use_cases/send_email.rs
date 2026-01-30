use crate::application::ports::email::{EmailProvider, SendEmailRequest, TemplateEngine};
use crate::application::ports::output::{
    EmailRepository, EmailTemplateRepository, TimelineActivityRepository,
};
use crate::domain::{DomainError, Email, EmailDirection, EmailStatus, HardGuard, TimelineActivity};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct SendEmailInput {
    pub from_email: String,
    pub to_email: String,
    pub cc_emails: Option<Vec<String>>,
    pub bcc_emails: Option<Vec<String>>,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub template_id: Option<Uuid>,
    pub template_variables: Option<serde_json::Value>,
    pub person_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub workflow_id: Option<Uuid>,
    pub workflow_run_id: Option<Uuid>,
    pub workspace_id: Uuid,
}

pub struct SendEmail {
    email_repo: Arc<dyn EmailRepository>,
    email_template_repo: Arc<dyn EmailTemplateRepository>,
    timeline_repo: Arc<dyn TimelineActivityRepository>,
    email_provider: Arc<dyn EmailProvider>,
    template_engine: Arc<dyn TemplateEngine>,
}

impl SendEmail {
    pub fn new(
        email_repo: Arc<dyn EmailRepository>,
        email_template_repo: Arc<dyn EmailTemplateRepository>,
        timeline_repo: Arc<dyn TimelineActivityRepository>,
        email_provider: Arc<dyn EmailProvider>,
        template_engine: Arc<dyn TemplateEngine>,
    ) -> Self {
        Self {
            email_repo,
            email_template_repo,
            timeline_repo,
            email_provider,
            template_engine,
        }
    }

    pub async fn execute(&self, input: SendEmailInput) -> Result<Email, DomainError> {
        // 1. Resolve template if provided
        let (subject, body_text, body_html, template_id) =
            if let Some(template_id) = input.template_id {
                let template = self
                    .email_template_repo
                    .find_by_id(template_id)
                    .await?
                    .ok_or_else(|| DomainError::NotFound)?;

                let variables = input
                    .template_variables
                    .unwrap_or_else(|| serde_json::json!({}));

                // Render template with variables
                let rendered_subject = self
                    .template_engine
                    .render(&template.subject, &variables)
                    .map_err(|e| {
                    DomainError::InfrastructureError(format!("Template render error: {}", e))
                })?;

                let rendered_body_text = self
                    .template_engine
                    .render(&template.body_text, &variables)
                    .map_err(|e| {
                        DomainError::InfrastructureError(format!("Template render error: {}", e))
                    })?;

                let rendered_body_html = if let Some(html) = &template.body_html {
                    Some(self.template_engine.render(html, &variables).map_err(|e| {
                        DomainError::InfrastructureError(format!("Template render error: {}", e))
                    })?)
                } else {
                    None
                };

                (
                    rendered_subject,
                    rendered_body_text,
                    rendered_body_html,
                    Some(template_id),
                )
            } else {
                (input.subject, input.body_text, input.body_html, None)
            };

        // 2. Create email record with pending status
        let email = Email {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            direction: EmailDirection::Outbound,
            status: EmailStatus::Pending,
            from_email: input.from_email.clone(),
            to_email: input.to_email.clone(),
            cc_emails: input.cc_emails.clone(),
            bcc_emails: input.bcc_emails.clone(),
            subject: subject.clone(),
            body_text: body_text.clone(),
            body_html: body_html.clone(),
            sent_at: None,
            failed_at: None,
            error_message: None,
            email_template_id: template_id,
            timeline_activity_id: None,
            person_id: input.person_id,
            company_id: input.company_id,
            opportunity_id: input.opportunity_id,
            task_id: input.task_id,
            workflow_id: input.workflow_id,
            workflow_run_id: input.workflow_run_id,
            metadata: None,
            workspace_id: input.workspace_id,
        };

        // Validate email
        email.validate()?;

        // Create email record
        let email = self.email_repo.create(email).await?;

        // 3. Send via provider
        let send_request = SendEmailRequest {
            from: input.from_email,
            to: input.to_email,
            cc: input.cc_emails,
            bcc: input.bcc_emails,
            subject,
            body_text,
            body_html,
            metadata: None,
        };

        let send_result = self.email_provider.send_email(send_request).await;

        // 4. Update email status based on result
        let mut updated_email = email.clone();
        match send_result {
            Ok(response) => {
                updated_email.status = EmailStatus::Sent;
                updated_email.sent_at = Some(Utc::now());
                updated_email.metadata = response.metadata;
                tracing::info!("Email sent successfully: {}", response.message_id);
            }
            Err(e) => {
                updated_email.status = EmailStatus::Failed;
                updated_email.failed_at = Some(Utc::now());
                updated_email.error_message = Some(e.to_string());
                tracing::error!("Failed to send email: {}", e);
            }
        }

        let updated_email = self.email_repo.update(updated_email).await?;

        // 5. Create timeline activity
        let activity_name = format!("Email sent to {}", updated_email.to_email);
        let timeline_activity = TimelineActivity {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            name: activity_name,
            workspace_member_id: None,
            person_id: updated_email.person_id,
            company_id: updated_email.company_id,
            opportunity_id: updated_email.opportunity_id,
            task_id: updated_email.task_id,
            note_id: None,
            calendar_event_id: None,
            workflow_id: updated_email.workflow_id,
            workspace_id: updated_email.workspace_id,
        };

        let timeline_activity = self.timeline_repo.create(timeline_activity).await?;

        // 6. Link timeline activity to email
        let mut final_email = updated_email;
        final_email.timeline_activity_id = Some(timeline_activity.id);
        let final_email = self.email_repo.update(final_email).await?;

        Ok(final_email)
    }
}
