use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "custom_object_data")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub object_metadata_id: Uuid,
    pub properties: Json,
    pub workspace_id: Uuid,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::object_metadata::Entity",
        from = "Column::ObjectMetadataId",
        to = "super::object_metadata::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    ObjectMetadata,
}

impl Related<super::object_metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ObjectMetadata.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::custom_object_data::CustomObjectData {
        crate::domain::custom_object_data::CustomObjectData {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            object_metadata_id: self.object_metadata_id,
            properties: self.properties,
            workspace_id: self.workspace_id,
        }
    }
}
