use crate::domain::metadata::FieldType;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "field_metadata")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub object_metadata_id: Uuid,
    pub name: String,
    pub r#type: String, // "type" is a reserved keyword
    pub is_custom: bool,
    pub settings: Option<Json>,
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
    pub fn to_domain(self) -> crate::domain::metadata::FieldMetadata {
        let field_type = match self.r#type.as_str() {
            "Number" => FieldType::Number,
            "Date" => FieldType::Date,
            "Boolean" => FieldType::Boolean,
            "Select" => FieldType::Select,
            "Relation" => FieldType::Relation,
            "Json" => FieldType::Json,
            _ => FieldType::Text,
        };

        crate::domain::metadata::FieldMetadata {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            object_metadata_id: self.object_metadata_id,
            name: self.name,
            field_type,
            is_custom: self.is_custom,
            settings: self.settings,
        }
    }
}
