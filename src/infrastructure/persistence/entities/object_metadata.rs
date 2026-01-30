use crate::domain::metadata::{FieldType, ViewType};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "object_metadata")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    #[sea_orm(unique)]
    pub name_singular: String,
    pub name_plural: String,
    pub description: Option<String>,
    pub workspace_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::field_metadata::Entity")]
    FieldMetadata,
    #[sea_orm(has_many = "super::view::Entity")]
    View,
}

impl Related<super::field_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::FieldMetadata.def()
    }
}

impl Related<super::view::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::View.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::metadata::ObjectMetadata {
        crate::domain::metadata::ObjectMetadata {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            name_singular: self.name_singular,
            name_plural: self.name_plural,
            description: self.description,
            workspace_id: self.workspace_id,
        }
    }
}
