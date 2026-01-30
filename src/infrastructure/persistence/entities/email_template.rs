use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "email_template")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(unique)]
    pub name: String,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub category: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::EmailTemplate {
        crate::domain::EmailTemplate {
            id: self.id,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
            name: self.name,
            subject: self.subject,
            body_text: self.body_text,
            body_html: self.body_html,
            category: self.category,
        }
    }
}
