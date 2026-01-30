use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "calendar_event_participant")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub calendar_event_id: Uuid,
    pub email: String,
    pub person_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::calendar_event::Entity",
        from = "Column::CalendarEventId",
        to = "super::calendar_event::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    CalendarEvent,
    #[sea_orm(
        belongs_to = "super::person::Entity",
        from = "Column::PersonId",
        to = "super::person::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Person,
}

impl Related<super::calendar_event::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CalendarEvent.def()
    }
}

impl Related<super::person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::CalendarEventParticipant {
        crate::domain::CalendarEventParticipant {
            id: self.id,
            created_at: self.created_at,
            calendar_event_id: self.calendar_event_id,
            email: self.email,
            person_id: self.person_id,
        }
    }
}
