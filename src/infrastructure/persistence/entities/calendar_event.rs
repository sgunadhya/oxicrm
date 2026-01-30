use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "calendar_event")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub connected_account_id: Uuid,
    pub title: String,
    pub start_time: DateTimeWithTimeZone,
    pub end_time: DateTimeWithTimeZone,
    pub description: Option<String>,
    pub workspace_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::CalendarEvent {
        crate::domain::CalendarEvent {
            id: self.id,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
            connected_account_id: self.connected_account_id,
            title: self.title,
            start_time: self.start_time.into(),
            end_time: self.end_time.into(),
            description: self.description,
            workspace_id: self.workspace_id,
        }
    }
}
