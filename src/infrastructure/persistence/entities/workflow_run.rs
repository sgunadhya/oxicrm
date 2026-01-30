use crate::domain::states::WorkflowRunStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workflow_run")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub workflow_version_id: Uuid,
    pub status: String,
    pub output: Option<Json>,
    pub error: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::workflow_version::Entity",
        from = "Column::WorkflowVersionId",
        to = "super::workflow_version::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    WorkflowVersion,
}

impl Related<super::workflow_version::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkflowVersion.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::WorkflowRun {
        let status = match self.status.as_str() {
            "pending" => WorkflowRunStatus::Pending,
            "running" => WorkflowRunStatus::Running,
            "completed" => WorkflowRunStatus::Completed,
            "failed" => WorkflowRunStatus::Failed,
            "cancelled" => WorkflowRunStatus::Cancelled,
            _ => WorkflowRunStatus::Pending,
        };

        crate::domain::WorkflowRun {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            workflow_version_id: self.workflow_version_id,
            status,
            output: self.output,
            error: self.error,
        }
    }
}
