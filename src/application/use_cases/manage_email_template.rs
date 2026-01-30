use crate::application::ports::output::EmailTemplateRepository;
use crate::domain::{DomainError, EmailTemplate, HardGuard};
use chrono::Utc;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub struct CreateEmailTemplateInput {
    pub name: String,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub category: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateEmailTemplateInput {
    pub name: Option<String>,
    pub subject: Option<String>,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub category: Option<String>,
}

pub struct ManageEmailTemplate {
    email_template_repo: Arc<dyn EmailTemplateRepository>,
}

impl ManageEmailTemplate {
    pub fn new(email_template_repo: Arc<dyn EmailTemplateRepository>) -> Self {
        Self { email_template_repo }
    }

    pub async fn create(&self, input: CreateEmailTemplateInput) -> Result<EmailTemplate, DomainError> {
        let template = EmailTemplate {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: input.name,
            subject: input.subject,
            body_text: input.body_text,
            body_html: input.body_html,
            category: input.category.unwrap_or_else(|| "manual".to_string()),
        };

        // Validate template
        template.validate()?;

        self.email_template_repo.create(template).await
    }

    pub async fn list(&self) -> Result<Vec<EmailTemplate>, DomainError> {
        self.email_template_repo.find_all().await
    }

    pub async fn get(&self, id: Uuid) -> Result<EmailTemplate, DomainError> {
        self.email_template_repo
            .find_by_id(id)
            .await?
            .ok_or(DomainError::NotFound)
    }

    pub async fn get_by_name(&self, name: &str) -> Result<EmailTemplate, DomainError> {
        self.email_template_repo
            .find_by_name(name)
            .await?
            .ok_or(DomainError::NotFound)
    }

    pub async fn update(&self, id: Uuid, input: UpdateEmailTemplateInput) -> Result<EmailTemplate, DomainError> {
        let mut template = self.get(id).await?;

        if let Some(name) = input.name {
            template.name = name;
        }
        if let Some(subject) = input.subject {
            template.subject = subject;
        }
        if let Some(body_text) = input.body_text {
            template.body_text = body_text;
        }
        if let Some(body_html) = input.body_html {
            template.body_html = Some(body_html);
        }
        if let Some(category) = input.category {
            template.category = category;
        }

        template.updated_at = Utc::now();

        // Validate template
        template.validate()?;

        self.email_template_repo.update(template).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DomainError> {
        self.email_template_repo.delete(id).await
    }
}
