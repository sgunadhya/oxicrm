use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub email: String,
    pub password_hash: String,
    pub state: String, // Stored as string, mapped to Enum in domain
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(&self) -> crate::domain::entities::User {
        crate::domain::entities::User {
            id: self.id,
            email: self.email.clone(),
            password_hash: self.password_hash.clone(),
            state: serde_json::from_value(serde_json::Value::String(self.state.clone()))
                .unwrap_or(crate::domain::states::UserState::Unverified), // Fallback or handle error better
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
        }
    }
}
