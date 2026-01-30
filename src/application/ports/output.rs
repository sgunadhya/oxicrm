use crate::domain::{Company, DomainError, Note, Opportunity, Person, Task, TaskTarget, User, Workspace, WorkspaceMember};
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
    async fn find_all(&self) -> Result<Vec<Opportunity>, DomainError>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Opportunity>, DomainError>;
    async fn create(&self, opportunity: Opportunity) -> Result<Opportunity, DomainError>;
    async fn update(&self, opportunity: Opportunity) -> Result<Opportunity, DomainError>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError>;
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

#[async_trait]
pub trait CompanyRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Company>, DomainError>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Company>, DomainError>;
    async fn create(&self, company: Company) -> Result<Company, DomainError>;
    async fn update(&self, company: Company) -> Result<Company, DomainError>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Task>, DomainError>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Task>, DomainError>;
    async fn create(&self, task: Task) -> Result<Task, DomainError>;
    async fn update(&self, task: Task) -> Result<Task, DomainError>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait NoteRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Note>, DomainError>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<Note>, DomainError>;
    async fn create(&self, note: Note) -> Result<Note, DomainError>;
    async fn update(&self, note: Note) -> Result<Note, DomainError>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait TaskTargetRepository: Send + Sync {
    async fn find_by_task_id(&self, task_id: uuid::Uuid) -> Result<Vec<TaskTarget>, DomainError>;
    async fn create(&self, task_target: TaskTarget) -> Result<TaskTarget, DomainError>;
    async fn delete(&self, id: uuid::Uuid) -> Result<(), DomainError>;
}
