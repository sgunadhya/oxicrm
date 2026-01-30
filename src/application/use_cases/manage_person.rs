use crate::application::ports::output::PersonRepository;
use crate::domain::DomainError;
use std::sync::Arc;
use uuid::Uuid;

pub struct ManagePerson {
    person_repo: Arc<dyn PersonRepository>,
}

impl ManagePerson {
    pub fn new(person_repo: Arc<dyn PersonRepository>) -> Self {
        Self { person_repo }
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.person_repo.delete(id).await
    }
}
