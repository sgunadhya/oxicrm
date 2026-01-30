use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "email")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub direction: String,
    pub status: String,
    pub from_email: String,
    pub to_email: String,
    pub cc_emails: Option<Json>,
    pub bcc_emails: Option<Json>,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub sent_at: Option<DateTimeWithTimeZone>,
    pub failed_at: Option<DateTimeWithTimeZone>,
    pub error_message: Option<String>,
    pub email_template_id: Option<Uuid>,
    pub timeline_activity_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub workflow_id: Option<Uuid>,
    pub workflow_run_id: Option<Uuid>,
    pub metadata: Option<Json>,
    pub workspace_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::Email {
        use crate::domain::states::{EmailDirection, EmailStatus};

        let direction = match self.direction.as_str() {
            "inbound" => EmailDirection::Inbound,
            _ => EmailDirection::Outbound,
        };

        let status = match self.status.as_str() {
            "sent" => EmailStatus::Sent,
            "failed" => EmailStatus::Failed,
            "received" => EmailStatus::Received,
            _ => EmailStatus::Pending,
        };

        // Parse JSON arrays for cc and bcc emails
        let cc_emails = self
            .cc_emails
            .and_then(|json| serde_json::from_value::<Vec<String>>(json.clone()).ok());

        let bcc_emails = self
            .bcc_emails
            .and_then(|json| serde_json::from_value::<Vec<String>>(json.clone()).ok());

        let metadata = self.metadata.map(|json| json.clone());

        crate::domain::Email {
            id: self.id,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
            direction,
            status,
            from_email: self.from_email,
            to_email: self.to_email,
            cc_emails,
            bcc_emails,
            subject: self.subject,
            body_text: self.body_text,
            body_html: self.body_html,
            sent_at: self.sent_at.map(|d| d.into()),
            failed_at: self.failed_at.map(|d| d.into()),
            error_message: self.error_message,
            email_template_id: self.email_template_id,
            timeline_activity_id: self.timeline_activity_id,
            person_id: self.person_id,
            company_id: self.company_id,
            opportunity_id: self.opportunity_id,
            task_id: self.task_id,
            workflow_id: self.workflow_id,
            workflow_run_id: self.workflow_run_id,
            metadata,
            workspace_id: self.workspace_id,
        }
    }
}
