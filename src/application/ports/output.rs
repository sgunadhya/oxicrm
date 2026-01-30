use crate::domain::{Company, DomainError, Opportunity, Person, User};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PersonRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Person>, DomainError>;
    async fn save(&self, person: &Person) -> Result<(), DomainError>;
}

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Company>, DomainError>;
    async fn save(&self, company: &Company) -> Result<(), DomainError>;
}

#[async_trait]
pub trait OpportunityRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Opportunity>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Opportunity>, DomainError>;
    async fn save(&self, opportunity: &Opportunity) -> Result<(), DomainError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn create(&self, user: User) -> Result<User, DomainError>;
}
