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
