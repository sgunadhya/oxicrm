use crate::application::ports::output::PersonRepository;
use crate::domain::{DomainError, Person};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct CreatePerson {
    person_repo: Arc<dyn PersonRepository>,
}

impl CreatePerson {
    pub fn new(person_repo: Arc<dyn PersonRepository>) -> Self {
        Self { person_repo }
    }

    pub async fn execute(
        &self,
        name: String,
        email: String,
        position: i32,
    ) -> Result<Person, DomainError> {
        // Enforce invariants first (uniqueness check via find_by_email)
        if (self.person_repo.find_by_email(&email).await?).is_some() {
            return Err(DomainError::Validation("Email already exists".to_string()));
        }

        let new_person = Person {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            name,
            email,
            position,
            company_id: None,
        };

        // Validate domain invariants
        use crate::domain::HardGuard;
        new_person.validate()?;

        self.person_repo.create(new_person).await
    }
}
