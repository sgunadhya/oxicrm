use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "opportunity")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub name: String,
    pub stage: String,
    pub amount_micros: Option<i64>,
    pub currency_code: Option<String>,
    pub close_date: Option<Date>,
    pub company_id: Option<Uuid>,
    pub point_of_contact_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> crate::domain::Opportunity {
        use crate::domain::states::OpportunityStage;

        let stage = match self.stage.as_str() {
            "Prospecting" => OpportunityStage::Prospecting,
            "Qualification" => OpportunityStage::Qualification,
            "Negotiation" => OpportunityStage::Negotiation,
            "Won" => OpportunityStage::Won,
            "Lost" => OpportunityStage::Lost,
            _ => OpportunityStage::Prospecting, // Default fallback
        };

        crate::domain::Opportunity {
            id: self.id,
            created_at: self.created_at.into(),
            updated_at: self.updated_at.into(),
            deleted_at: self.deleted_at.map(|d| d.into()),
            name: self.name,
            stage,
            close_date: self.close_date.map(|d| d.into()),
            amount_micros: self.amount_micros,
            currency_code: self.currency_code,
            position: 0, // Default position, will be calculated
            point_of_contact_id: self.point_of_contact_id,
            company_id: self.company_id,
            owner_id: self.owner_id,
        }
    }
}
