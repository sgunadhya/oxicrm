use crate::application::ports::output::UserRepository;
use crate::domain::{entities::User, states::UserState, DomainError};
use chrono::Utc;
use std::sync::Arc;
use uuid::Uuid;

pub struct RegisterUser {
    pub user_repo: Arc<dyn UserRepository>,
}

impl RegisterUser {
    pub async fn execute(&self, email: String, password: String) -> Result<User, DomainError> {
        // Check if user exists
        if let Some(_) = self.user_repo.find_by_email(&email).await? {
            return Err(DomainError::Validation("User already exists".to_string()));
        }

        // Mock hashing for demo
        let password_hash = format!("hashed_{}", password);

        let user = User {
            id: Uuid::new_v4(),
            email,
            password_hash,
            state: UserState::Unverified,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.user_repo.create(user).await
    }
}
