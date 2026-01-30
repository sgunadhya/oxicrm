use super::entities::Person;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("Validation failed: {0}")]
    Validation(String),
    #[error("Permission denied: {0}")]
    Permission(String),
    #[error("Entity not found")]
    NotFound,
    #[error("Invalid state transition: {0}")]
    InvalidState(String),
}

pub trait HardGuard {
    fn validate(&self) -> Result<(), DomainError>;
}

impl HardGuard for Person {
    fn validate(&self) -> Result<(), DomainError> {
        if self.name.trim().is_empty() {
            return Err(DomainError::Validation("Name cannot be empty".into()));
        }
        for email in &self.emails {
            if !email.contains('@') {
                return Err(DomainError::Validation(format!("Invalid email: {}", email)));
            }
        }
        Ok(())
    }
}
