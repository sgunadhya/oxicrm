use crate::application::ports::output::{
    WorkflowRunRepository, WorkflowVersionRepository, WorkflowVersionStepRepository,
};
use crate::application::use_cases::send_email::{SendEmail, SendEmailInput};
use crate::domain::states::{WorkflowRunStatus, WorkflowStepType};
use crate::domain::{DomainError, WorkflowRun, WorkflowVersionStep};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct WorkflowExecutor {
    workflow_run_repo: Arc<dyn WorkflowRunRepository>,
    workflow_version_repo: Arc<dyn WorkflowVersionRepository>,
    workflow_step_repo: Arc<dyn WorkflowVersionStepRepository>,
    send_email_use_case: Arc<SendEmail>,
}

impl WorkflowExecutor {
    pub fn new(
        workflow_run_repo: Arc<dyn WorkflowRunRepository>,
        workflow_version_repo: Arc<dyn WorkflowVersionRepository>,
        workflow_step_repo: Arc<dyn WorkflowVersionStepRepository>,
        send_email_use_case: Arc<SendEmail>,
    ) -> Self {
        Self {
            workflow_run_repo,
            workflow_version_repo,
            workflow_step_repo,
            send_email_use_case,
        }
    }

    pub async fn execute_workflow(
        &self,
        workflow_version_id: Uuid,
    ) -> Result<WorkflowRun, DomainError> {
        // 1. Create workflow run
        let mut workflow_run = WorkflowRun {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            workflow_version_id,
            status: WorkflowRunStatus::Running,
            output: None,
            error: None,
        };

        let mut workflow_run = self.workflow_run_repo.create(workflow_run.clone()).await?;

        // 2. Get workflow steps (ordered by position)
        let mut steps = self
            .workflow_step_repo
            .find_by_version_id(workflow_version_id)
            .await?;

        steps.sort_by_key(|step| step.position);

        // 3. Execute each step based on step_type
        for step in steps {
            let execute_result = self.execute_step(&step, &workflow_run).await;

            if let Err(e) = execute_result {
                // Mark workflow as failed
                workflow_run.status = WorkflowRunStatus::Failed;
                workflow_run.error = Some(e.to_string());
                workflow_run.updated_at = Utc::now();
                return self.workflow_run_repo.update(workflow_run).await;
            }
        }

        // 4. Mark workflow as completed
        workflow_run.status = WorkflowRunStatus::Completed;
        workflow_run.updated_at = Utc::now();
        self.workflow_run_repo.update(workflow_run).await
    }

    async fn execute_step(
        &self,
        step: &WorkflowVersionStep,
        workflow_run: &WorkflowRun,
    ) -> Result<(), DomainError> {
        match &step.step_type {
            WorkflowStepType::SendEmail => {
                self.execute_send_email_step(&step.settings, workflow_run)
                    .await
            }
            WorkflowStepType::CreateRecord => {
                // TODO: Implement create record step
                tracing::warn!("CreateRecord step not implemented yet");
                Ok(())
            }
            WorkflowStepType::IfElse => {
                // TODO: Implement if-else conditional step
                tracing::warn!("IfElse step not implemented yet");
                Ok(())
            }
            WorkflowStepType::Form => {
                // TODO: Implement form step
                tracing::warn!("Form step not implemented yet");
                Ok(())
            }
            _ => {
                tracing::warn!("Step type not implemented yet");
                Ok(())
            }
        }
    }

    async fn execute_send_email_step(
        &self,
        settings: &serde_json::Value,
        workflow_run: &WorkflowRun,
    ) -> Result<(), DomainError> {
        // Parse settings JSON
        // Expected format:
        // {
        //   "from_email": "sender@example.com",
        //   "to_email": "recipient@example.com",
        //   "subject": "Email subject",
        //   "body_text": "Email body",
        //   "template_id": "uuid",
        //   "template_variables": { "key": "value" }
        // }

        let from_email = settings
            .get("from_email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DomainError::Validation("Missing from_email in settings".into()))?
            .to_string();

        let to_email = settings
            .get("to_email")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DomainError::Validation("Missing to_email in settings".into()))?
            .to_string();

        let subject = settings
            .get("subject")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let body_text = settings
            .get("body_text")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let template_id = settings
            .get("template_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok());

        let template_variables = settings.get("template_variables").cloned();

        let cc_emails = settings
            .get("cc_emails")
            .and_then(|v| serde_json::from_value::<Vec<String>>(v.clone()).ok());

        let bcc_emails = settings
            .get("bcc_emails")
            .and_then(|v| serde_json::from_value::<Vec<String>>(v.clone()).ok());

        let body_html = settings
            .get("body_html")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let input = SendEmailInput {
            from_email,
            to_email,
            cc_emails,
            bcc_emails,
            subject,
            body_text,
            body_html,
            template_id,
            template_variables,
            person_id: None,
            company_id: None,
            opportunity_id: None,
            task_id: None,
            workflow_id: None,
            workflow_run_id: Some(workflow_run.id),
            workspace_id: Uuid::default(), // TODO: Propagate workspace_id from WorkflowRun
        };

        self.send_email_use_case.execute(input).await?;

        Ok(())
    }
}
