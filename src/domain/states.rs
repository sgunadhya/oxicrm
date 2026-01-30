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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowRunStatus {
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStepType {
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
