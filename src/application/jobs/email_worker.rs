use crate::application::ports::email::EmailProvider;
use crate::application::ports::email::SendEmailRequest;
use crate::application::ports::output::EmailRepository;
use crate::application::ports::scheduling::Job;
use crate::domain::states::EmailStatus;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::mpsc;

pub struct EmailJobWorker {
    email_repo: Arc<dyn EmailRepository>,
    email_provider: Arc<dyn EmailProvider>,
    job_receiver: mpsc::Receiver<Job>,
}

impl EmailJobWorker {
    pub fn new(
        email_repo: Arc<dyn EmailRepository>,
        email_provider: Arc<dyn EmailProvider>,
        job_receiver: mpsc::Receiver<Job>,
    ) -> Self {
        Self {
            email_repo,
            email_provider,
            job_receiver,
        }
    }

    pub async fn start(mut self) {
        tracing::info!("EmailJobWorker started");

        while let Some(job) = self.job_receiver.recv().await {
            tracing::debug!("EmailJobWorker processing job: {}", job.name);

            let result = match job.name.as_str() {
                "send_pending_emails" => self.process_pending_emails().await,
                "send_bulk_email" => self.process_bulk_email_job(&job.payload).await,
                _ => {
                    tracing::warn!("Unknown job type: {}", job.name);
                    Ok(())
                }
            };

            if let Err(e) = result {
                tracing::error!("Error processing job {}: {}", job.name, e);
            }
        }

        tracing::warn!("EmailJobWorker receiver closed");
    }

    async fn process_pending_emails(&self) -> Result<(), String> {
        // 1. Get all pending emails
        let pending_emails = self
            .email_repo
            .find_pending()
            .await
            .map_err(|e| format!("Failed to fetch pending emails: {}", e))?;

        if pending_emails.is_empty() {
            tracing::debug!("No pending emails to process");
            return Ok(());
        }

        tracing::info!("Processing {} pending emails", pending_emails.len());

        // 2. Send each via provider and update status
        for email in pending_emails {
            let send_request = SendEmailRequest {
                from: email.from_email.clone(),
                to: email.to_email.clone(),
                cc: email.cc_emails.clone(),
                bcc: email.bcc_emails.clone(),
                subject: email.subject.clone(),
                body_text: email.body_text.clone(),
                body_html: email.body_html.clone(),
                metadata: None,
            };

            let send_result = self.email_provider.send_email(send_request).await;

            // 3. Update status (sent/failed)
            let mut updated_email = email.clone();
            match send_result {
                Ok(response) => {
                    updated_email.status = EmailStatus::Sent;
                    updated_email.sent_at = Some(Utc::now());
                    updated_email.metadata = response.metadata;
                    tracing::info!("Email {} sent successfully", email.id);
                }
                Err(e) => {
                    updated_email.status = EmailStatus::Failed;
                    updated_email.failed_at = Some(Utc::now());
                    updated_email.error_message = Some(e.to_string());
                    tracing::error!("Failed to send email {}: {}", email.id, e);
                }
            }

            self.email_repo
                .update(updated_email)
                .await
                .map_err(|e| format!("Failed to update email status: {}", e))?;
        }

        Ok(())
    }

    async fn process_bulk_email_job(&self, payload: &str) -> Result<(), String> {
        // Parse bulk email job payload
        // Expected format:
        // {
        //   "email_ids": ["uuid1", "uuid2", ...]
        // }

        let job_data: serde_json::Value = serde_json::from_str(payload)
            .map_err(|e| format!("Failed to parse job payload: {}", e))?;

        let email_ids_array = job_data
            .get("email_ids")
            .and_then(|v| v.as_array())
            .ok_or_else(|| "Missing or invalid email_ids in payload".to_string())?;

        tracing::info!("Processing bulk email job with {} emails", email_ids_array.len());

        for email_id_value in email_ids_array {
            let email_id_str = email_id_value
                .as_str()
                .ok_or_else(|| "Invalid email ID format".to_string())?;

            let email_id = uuid::Uuid::parse_str(email_id_str)
                .map_err(|e| format!("Invalid UUID: {}", e))?;

            // Fetch and send each email
            let email = self
                .email_repo
                .find_by_id(email_id)
                .await
                .map_err(|e| format!("Failed to fetch email: {}", e))?
                .ok_or_else(|| format!("Email not found: {}", email_id))?;

            let send_request = SendEmailRequest {
                from: email.from_email.clone(),
                to: email.to_email.clone(),
                cc: email.cc_emails.clone(),
                bcc: email.bcc_emails.clone(),
                subject: email.subject.clone(),
                body_text: email.body_text.clone(),
                body_html: email.body_html.clone(),
                metadata: None,
            };

            let send_result = self.email_provider.send_email(send_request).await;

            // Update status
            let mut updated_email = email;
            match send_result {
                Ok(response) => {
                    updated_email.status = EmailStatus::Sent;
                    updated_email.sent_at = Some(Utc::now());
                    updated_email.metadata = response.metadata;
                    tracing::info!("Bulk email {} sent successfully", email_id);
                }
                Err(e) => {
                    updated_email.status = EmailStatus::Failed;
                    updated_email.failed_at = Some(Utc::now());
                    updated_email.error_message = Some(e.to_string());
                    tracing::error!("Failed to send bulk email {}: {}", email_id, e);
                }
            }

            self.email_repo
                .update(updated_email)
                .await
                .map_err(|e| format!("Failed to update email status: {}", e))?;
        }

        Ok(())
    }
}
