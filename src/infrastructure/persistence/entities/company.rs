use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "company")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub name: String,
    #[sea_orm(unique)]
    pub domain_name: String,
    pub address: Option<String>, // Stored as JSON or Text
    pub employees_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::Company {
        crate::domain::Company {
            id: self.id,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
            deleted_at: self.deleted_at.map(|d| d.into()),
            name: self.name,
            domain_name: self.domain_name,
            address: self.address,
            employees_count: self.employees_count,
            position: 0, // Default position
        }
    }
}
