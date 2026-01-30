use crate::domain::{DomainError, Opportunity, Person, User, Workspace, WorkspaceMember};
use async_trait::async_trait;

#[async_trait]
pub trait PersonRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<Person>, DomainError>;
    async fn create(&self, person: Person) -> Result<Person, DomainError>;
    async fn find_all(&self) -> Result<Vec<Person>, DomainError>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait OpportunityRepository: Send + Sync {
    async fn save(&self, opportunity: &Opportunity) -> Result<(), DomainError>;
    async fn find_all(&self) -> Result<Vec<Opportunity>, DomainError>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Opportunity>, DomainError>;
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn create(&self, user: User) -> Result<User, DomainError>;
}

#[async_trait]
pub trait WorkspaceRepository: Send + Sync {
    async fn create(&self, workspace: Workspace) -> Result<Workspace, DomainError>;
    async fn find_by_subdomain(&self, subdomain: &str) -> Result<Option<Workspace>, DomainError>;
    async fn add_member(&self, member: WorkspaceMember) -> Result<WorkspaceMember, DomainError>;
}
