use super::states::{OpportunityStage, TaskStatus};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub emails: Vec<String>,
    pub position: i32,
    pub company_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub domain_name: String,
    pub address: Option<String>,
    pub employees: Option<i32>,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub name: String,
    pub stage: OpportunityStage,
    pub close_date: Option<chrono::NaiveDate>,
    pub amount_micros: Option<i64>,
    pub position: i32,
    pub point_of_contact_id: Option<Uuid>,
    pub company_id: Option<Uuid>,
}

impl Opportunity {
    pub fn new(name: String, stage: OpportunityStage, amount: i64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            stage,
            amount_micros: Some(amount),
            close_date: None,
            point_of_contact_id: None,
            company_id: None,
            position: 0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub title: String,
    pub body: Option<String>,
    pub status: TaskStatus,
    pub position: i32,
    pub assignee_id: Option<Uuid>,
}
