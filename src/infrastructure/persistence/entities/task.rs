use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "task")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub title: String,
    pub body: Option<String>,
    pub status: String,
    pub position: i32,
    pub assignee_id: Option<Uuid>,
    pub due_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::Task {
        use crate::domain::states::TaskStatus;

        let status = match self.status.as_str() {
            "TODO" => TaskStatus::Todo,
            "IN_PROGRESS" => TaskStatus::InProgress,
            "DONE" => TaskStatus::Done,
            _ => TaskStatus::Todo,
        };

        crate::domain::Task {
            id: self.id,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
            deleted_at: self.deleted_at.map(|d| d.into()),
            title: self.title,
            body: self.body,
            status,
            position: self.position,
            assignee_id: self.assignee_id,
            due_at: self.due_at.map(|d| d.into()),
        }
    }
}
