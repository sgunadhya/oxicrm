use crate::application::ports::output::CompanyRepository;
use crate::domain::{Company, DomainError};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct UpdateCompanyInput {
    pub name: String,
    pub domain_name: String,
    pub address: Option<String>,
    pub employees_count: Option<i32>,
}

pub struct ManageCompany {
    company_repo: Arc<dyn CompanyRepository>,
}

impl ManageCompany {
    pub fn new(company_repo: Arc<dyn CompanyRepository>) -> Self {
        Self { company_repo }
    }

    pub async fn update(
        &self,
        id: Uuid,
        input: UpdateCompanyInput,
    ) -> Result<Company, DomainError> {
        let current = self.company_repo.find_by_id(id).await?;
        if let Some(mut company) = current {
            company.name = input.name;
            company.domain_name = input.domain_name;
            company.address = input.address;
            if let Some(count) = input.employees_count {
                company.employees_count = count;
            }
            self.company_repo.update(company).await
        } else {
            Err(DomainError::Validation("Company not found".to_string()))
        }
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.company_repo.delete(id).await
    }
}

pub struct UpdateCompany {
    company_repo: Arc<dyn CompanyRepository>,
}

impl UpdateCompany {
    pub fn new(company_repo: Arc<dyn CompanyRepository>) -> Self {
        Self { company_repo }
    }

    pub async fn execute(
        &self,
        id: Uuid,
        input: UpdateCompanyInput,
    ) -> Result<Company, DomainError> {
        let current = self.company_repo.find_by_id(id).await?;
        if let Some(mut company) = current {
            company.name = input.name;
            company.domain_name = input.domain_name;
            company.address = input.address;
            if let Some(count) = input.employees_count {
                company.employees_count = count;
            }
            // UpdatedAt is handled by Repo usually or Domain logic. Repo has logic to set updated_at.

            self.company_repo.update(company).await
        } else {
            Err(DomainError::Validation("Company not found".to_string()))
        }
    }
}

pub struct DeleteCompany {
    company_repo: Arc<dyn CompanyRepository>,
}

impl DeleteCompany {
    pub fn new(company_repo: Arc<dyn CompanyRepository>) -> Self {
        Self { company_repo }
    }

    pub async fn execute(&self, id: Uuid) -> Result<(), DomainError> {
        self.company_repo.delete(id).await
    }
}
