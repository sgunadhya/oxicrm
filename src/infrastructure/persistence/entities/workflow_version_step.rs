use crate::domain::states::WorkflowStepType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workflow_version_step")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub workflow_version_id: Uuid,
    pub step_type: String,
    pub settings: Json,
    pub position: i32,
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
    pub fn to_domain(self) -> crate::domain::WorkflowVersionStep {
        let step_type = match self.step_type.as_str() {
            "trigger" => WorkflowStepType::Trigger,
            "action" => WorkflowStepType::Action,
            "condition" => WorkflowStepType::Condition,
            "delay" => WorkflowStepType::Delay,
            _ => WorkflowStepType::Action,
        };

        crate::domain::WorkflowVersionStep {
            id: self.id,
            created_at: self.created_at,
            workflow_version_id: self.workflow_version_id,
            step_type,
            settings: self.settings,
            position: self.position,
        }
    }
}
