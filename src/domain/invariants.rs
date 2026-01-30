use super::entities::{Email, EmailTemplate, Person};
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
    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}

pub trait HardGuard {
    fn validate(&self) -> Result<(), DomainError>;
}

impl HardGuard for Person {
    fn validate(&self) -> Result<(), DomainError> {
        if self.name.trim().is_empty() {
            return Err(DomainError::Validation("Name cannot be empty".into()));
        }
        if !self.email.contains('@') {
            return Err(DomainError::Validation(format!(
                "Invalid email: {}",
                self.email
            )));
        }
        Ok(())
    }
}

impl HardGuard for EmailTemplate {
    fn validate(&self) -> Result<(), DomainError> {
        if self.name.trim().is_empty() {
            return Err(DomainError::Validation("Template name cannot be empty".into()));
        }
        if self.subject.trim().is_empty() {
            return Err(DomainError::Validation("Template subject cannot be empty".into()));
        }
        if self.body_text.trim().is_empty() {
            return Err(DomainError::Validation("Template body cannot be empty".into()));
        }
        Ok(())
    }
}

impl HardGuard for Email {
    fn validate(&self) -> Result<(), DomainError> {
        if !self.from_email.contains('@') {
            return Err(DomainError::Validation(format!(
                "Invalid from_email: {}",
                self.from_email
            )));
        }
        if !self.to_email.contains('@') {
            return Err(DomainError::Validation(format!(
                "Invalid to_email: {}",
                self.to_email
            )));
        }
        if self.subject.trim().is_empty() {
            return Err(DomainError::Validation("Email subject cannot be empty".into()));
        }
        if self.body_text.trim().is_empty() {
            return Err(DomainError::Validation("Email body cannot be empty".into()));
        }

        // Validate cc_emails if present
        if let Some(cc_emails) = &self.cc_emails {
            for email in cc_emails {
                if !email.contains('@') {
                    return Err(DomainError::Validation(format!(
                        "Invalid cc_email: {}",
                        email
                    )));
                }
            }
        }

        // Validate bcc_emails if present
        if let Some(bcc_emails) = &self.bcc_emails {
            for email in bcc_emails {
                if !email.contains('@') {
                    return Err(DomainError::Validation(format!(
                        "Invalid bcc_email: {}",
                        email
                    )));
                }
            }
        }

        Ok(())
    }
}
