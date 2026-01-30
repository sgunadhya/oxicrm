use crate::domain::states::WorkflowVersionStatus;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workflow_version")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub workflow_id: Uuid,
    pub status: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::workflow::Entity",
        from = "Column::WorkflowId",
        to = "super::workflow::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Workflow,
    #[sea_orm(has_many = "super::workflow_version_step::Entity")]
    WorkflowVersionStep,
    #[sea_orm(has_many = "super::workflow_run::Entity")]
    WorkflowRun,
}

impl Related<super::workflow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workflow.def()
    }
}

impl Related<super::workflow_version_step::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkflowVersionStep.def()
    }
}

impl Related<super::workflow_run::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkflowRun.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::WorkflowVersion {
        let status = match self.status.as_str() {
            "draft" => WorkflowVersionStatus::Draft,
            "published" => WorkflowVersionStatus::Published,
            "archived" => WorkflowVersionStatus::Archived,
            _ => WorkflowVersionStatus::Draft,
        };

        crate::domain::WorkflowVersion {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            workflow_id: self.workflow_id,
            status,
        }
    }
}
