use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "note")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub title: String,
    pub body_v2: Option<String>,
    pub position: i32,
    pub workspace_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::Note {
        crate::domain::Note {
            id: self.id,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
            deleted_at: self.deleted_at.map(|d| d.into()),
            title: self.title,
            body_v2: self.body_v2,
            position: self.position,
            workspace_id: self.workspace_id,
        }
    }
}
