use super::states::{
    ConnectedAccountStatus, EmailDirection, EmailStatus, LeadSource, LeadStatus, OpportunityStage,
    TaskStatus, UserState, WorkflowRunStatus, WorkflowStepType, WorkflowVersionStatus,
    WorkspaceState,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub subdomain: String,
    pub state: WorkspaceState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceMember {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub workspace_id: Uuid,
    pub role: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub email: String,
    pub password_hash: String,
    pub state: UserState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub email: String,
    pub position: i32,
    pub company_id: Option<Uuid>,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub domain_name: String,
    pub address: Option<String>,
    pub employees_count: i32,
    pub position: i32,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub stage: OpportunityStage,
    pub close_date: Option<chrono::NaiveDate>,
    pub amount_micros: Option<i64>,
    pub currency_code: Option<String>,
    pub position: i32,
    pub point_of_contact_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
    pub workspace_id: Uuid,
}

impl Opportunity {
    pub fn new(name: String, stage: OpportunityStage, amount: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            stage,
            amount_micros: Some(amount),
            currency_code: Some("USD".to_string()),
            close_date: None,
            point_of_contact_id: None,
            company_id: None,
            owner_id: None,
            position: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            workspace_id: Uuid::nil(), // Placeholder, should be passed in
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub title: String,
    pub body: Option<String>,
    pub status: TaskStatus,
    pub position: i32,
    pub assignee_id: Option<Uuid>,
    pub due_at: Option<DateTime<Utc>>,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub title: String,
    pub body_v2: Option<String>,
    pub position: i32,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTarget {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub task_id: Uuid,
    pub person_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub last_published_version_id: Option<Uuid>,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowVersion {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub workflow_id: Uuid,
    pub status: WorkflowVersionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowVersionStep {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub workflow_version_id: Uuid,
    pub step_type: WorkflowStepType,
    pub settings: serde_json::Value,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRun {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub workflow_version_id: Uuid,
    pub status: WorkflowRunStatus,
    pub output: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectedAccount {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub provider: String,
    pub account_owner_id: Uuid,
    pub status: ConnectedAccountStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEvent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub connected_account_id: Uuid,
    pub title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub description: Option<String>,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalendarEventParticipant {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub calendar_event_id: Uuid,
    pub email: String,
    pub person_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineActivity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub workspace_member_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub note_id: Option<Uuid>,
    pub calendar_event_id: Option<Uuid>,
    pub workflow_id: Option<Uuid>,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub direction: EmailDirection,
    pub status: EmailStatus,
    pub from_email: String,
    pub to_email: String,
    pub cc_emails: Option<Vec<String>>,
    pub bcc_emails: Option<Vec<String>>,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub email_template_id: Option<Uuid>,
    pub timeline_activity_id: Option<Uuid>,
    pub person_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
    pub opportunity_id: Option<Uuid>,
    pub task_id: Option<Uuid>,
    pub workflow_id: Option<Uuid>,
    pub workflow_run_id: Option<Uuid>,
    pub metadata: Option<serde_json::Value>,
    pub workspace_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub job_title: Option<String>,
    pub source: LeadSource,
    pub status: LeadStatus,
    pub score: i32,
    pub notes: Option<String>,
    pub position: i32,
    pub assigned_to_id: Option<Uuid>,
    pub converted_person_id: Option<Uuid>,
    pub converted_company_id: Option<Uuid>,
    pub converted_opportunity_id: Option<Uuid>,
    pub converted_at: Option<DateTime<Utc>>,
    pub last_contacted_at: Option<DateTime<Utc>>,
    pub workspace_id: Uuid,
}

impl Lead {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn calculate_score(&self) -> i32 {
        let mut score = 0;
        if !self.email.is_empty() {
            score += 10;
        }
        if self.phone.is_some() {
            score += 30;
        }
        if self.company_name.is_some() {
            score += 20;
        }
        if self.job_title.is_some() {
            score += 15;
        }
        score
    }

    pub fn is_converted(&self) -> bool {
        self.status == LeadStatus::Converted
    }
}
