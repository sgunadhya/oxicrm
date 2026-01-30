use crate::domain::Lead;
use crate::domain::states::{LeadSource, LeadStatus};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "lead")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
    pub deleted_at: Option<DateTimeUtc>,
    pub first_name: String,
    pub last_name: String,
    #[sea_orm(unique)]
    pub email: String,
    pub phone: Option<String>,
    pub company_name: Option<String>,
    pub job_title: Option<String>,
    pub source: String,
    pub status: String,
    pub score: i32,
    pub notes: Option<String>,
    pub position: i32,
    pub assigned_to_id: Option<Uuid>,
    pub converted_person_id: Option<Uuid>,
    pub converted_company_id: Option<Uuid>,
    pub converted_opportunity_id: Option<Uuid>,
    pub converted_at: Option<DateTimeUtc>,
    pub last_contacted_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_domain(self) -> Lead {
        let source = match self.source.as_str() {
            "web_form" => LeadSource::WebForm,
            "email" => LeadSource::Email,
            "referral" => LeadSource::Referral,
            _ => LeadSource::ManualEntry,
        };

        let status = match self.status.as_str() {
            "contacted" => LeadStatus::Contacted,
            "qualified" => LeadStatus::Qualified,
            "unqualified" => LeadStatus::Unqualified,
            "converted" => LeadStatus::Converted,
            _ => LeadStatus::New,
        };

        Lead {
            id: self.id,
            created_at: self.created_at,
            updated_at: self.updated_at,
            deleted_at: self.deleted_at,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            phone: self.phone,
            company_name: self.company_name,
            job_title: self.job_title,
            source,
            status,
            score: self.score,
            notes: self.notes,
            position: self.position,
            assigned_to_id: self.assigned_to_id,
            converted_person_id: self.converted_person_id,
            converted_company_id: self.converted_company_id,
            converted_opportunity_id: self.converted_opportunity_id,
            converted_at: self.converted_at,
            last_contacted_at: self.last_contacted_at,
        }
    }
}
