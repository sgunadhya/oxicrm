use crate::application::ports::output::CompanyRepository;
use crate::domain::{Company, DomainError};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateCompanyInput {
    pub name: String,
    pub domain_name: String,
    pub address: Option<String>,
    pub employees_count: Option<i32>,
}

pub struct CreateCompany {
    company_repo: Arc<dyn CompanyRepository>,
}

impl CreateCompany {
    pub fn new(company_repo: Arc<dyn CompanyRepository>) -> Self {
        Self { company_repo }
    }

    pub async fn execute(&self, input: CreateCompanyInput) -> Result<Company, DomainError> {
        // Here we could add validation (e.g. check domain uniqueness explicitly if not handled by DB constraint,
        // but DB constraint is usually enough for a start, or check it here for better error msg).
        // For MVP, we'll let DB constraint handle uniqueness error translation or do a check.

        // We could implement find_by_domain in repo if we strictly want logic here.

        let company = Company {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            name: input.name,
            domain_name: input.domain_name,
            address: input.address,
            employees_count: input.employees_count.unwrap_or(0),
            // Deprecated field in struct, but needs to be filled if generic. Wait, I renamed it in struct?
            // Ah, I renamed `employees` to `employees_count` in struct in step 1345.
            // But let's verify if `employees` field still exists or if I replaced it.
            // I replaced `employees: Option<i32>` with `employees_count: i32`.
            // So `employees` field should NOT be here.
            position: 0,
        };

        self.company_repo.create(company).await
    }
}
