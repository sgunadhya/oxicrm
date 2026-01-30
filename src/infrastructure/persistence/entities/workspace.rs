use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workspaces")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub subdomain: String,
    pub state: String,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::workspace_member::Entity")]
    WorkspaceMembers,
}

impl Related<super::workspace_member::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkspaceMembers.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::Workspace {
        use crate::domain::states::WorkspaceState;
        let state = match self.state.as_str() {
            "Pending" => WorkspaceState::Pending,
            "Active" => WorkspaceState::Active,
            "Suspended" => WorkspaceState::Suspended,
            _ => WorkspaceState::Pending,
        };
        crate::domain::Workspace {
            id: self.id,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
            subdomain: self.subdomain,
            state,
        }
    }
}
