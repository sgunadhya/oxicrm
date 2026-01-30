use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowState {
    Draft,
    Active,
    Deactivated,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpportunityStage {
    Prospecting,
    Qualification,
    Negotiation,
    Won,
    Lost,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserState {
    Unverified,
    Active,
    Suspended,
    Deleted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkspaceState {
    Pending,
    Active,
    Suspended,
}

impl Default for TaskStatus {
    fn default() -> Self {
        Self::Todo
    }
}

impl Default for OpportunityStage {
    fn default() -> Self {
        Self::Prospecting
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowVersionStatus {
    Draft,
    Active,
    Published,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowRunStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStepType {
    Trigger,
    Action,
    Condition,
    Delay,
    CreateRecord,
    SendEmail,
    IfElse,
    Form,
}

impl Default for WorkflowVersionStatus {
    fn default() -> Self {
        Self::Draft
    }
}

impl Default for WorkflowRunStatus {
    fn default() -> Self {
        Self::Running
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectedAccountStatus {
    Connected,
    Failed,
}

impl Default for ConnectedAccountStatus {
    fn default() -> Self {
        Self::Connected
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmailDirection {
    Outbound,
    Inbound,
}

impl Default for EmailDirection {
    fn default() -> Self {
        Self::Outbound
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmailStatus {
    Pending,
    Sent,
    Failed,
    Received,
}

impl Default for EmailStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeadSource {
    WebForm,
    ManualEntry,
    Email,
    Referral,
}

impl Default for LeadSource {
    fn default() -> Self {
        Self::ManualEntry
    }
}

impl LeadSource {
    pub fn to_string(&self) -> &str {
        match self {
            Self::WebForm => "web_form",
            Self::ManualEntry => "manual_entry",
            Self::Email => "email",
            Self::Referral => "referral",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Unqualified,
    Converted,
}

impl Default for LeadStatus {
    fn default() -> Self {
        Self::New
    }
}
