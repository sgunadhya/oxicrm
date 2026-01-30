use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "timeline_activity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
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

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::TimelineActivity {
        crate::domain::TimelineActivity {
            id: self.id,
            created_at: self.created_at.into(),
            name: self.name,
            workspace_member_id: self.workspace_member_id,
            person_id: self.person_id,
            company_id: self.company_id,
            opportunity_id: self.opportunity_id,
            task_id: self.task_id,
            note_id: self.note_id,
            calendar_event_id: self.calendar_event_id,
            workflow_id: self.workflow_id,
            workspace_id: self.workspace_id,
        }
    }
}
