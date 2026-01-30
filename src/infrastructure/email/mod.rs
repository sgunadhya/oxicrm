use crate::application::ports::email::{EmailProvider, SendEmailRequest, SendEmailResponse, TemplateEngine};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// Mock email provider for development and testing
/// Stores sent emails in memory instead of actually sending them
#[derive(Debug, Clone)]
pub struct MockEmailProvider {
    pub sent_emails: Arc<Mutex<Vec<SendEmailRequest>>>,
}

impl MockEmailProvider {
    pub fn new() -> Self {
        Self {
            sent_emails: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn get_sent_emails(&self) -> Vec<SendEmailRequest> {
        self.sent_emails.lock().await.clone()
    }

    pub async fn clear_sent_emails(&self) {
        self.sent_emails.lock().await.clear();
    }
}

impl Default for MockEmailProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmailProvider for MockEmailProvider {
    async fn send_email(&self, request: SendEmailRequest) -> Result<SendEmailResponse, String> {
        self.sent_emails.lock().await.push(request.clone());

        tracing::info!(
            "MockEmailProvider: Sending email to {} - {}",
            request.to,
            request.subject
        );

        Ok(SendEmailResponse {
            message_id: Uuid::new_v4().to_string(),
            status: "sent".to_string(),
            metadata: None,
        })
    }

    async fn verify_configuration(&self) -> Result<bool, String> {
        Ok(true)
    }
}

/// Simple template engine that supports {{variable}} syntax
/// Uses Handlebars-style variable substitution
pub struct SimpleTemplateEngine;

impl SimpleTemplateEngine {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SimpleTemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateEngine for SimpleTemplateEngine {
    fn render(&self, template: &str, variables: &serde_json::Value) -> Result<String, String> {
        let mut result = template.to_string();

        if let Some(obj) = variables.as_object() {
            for (key, value) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                let replacement = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => String::new(),
                    _ => value.to_string(),
                };
                result = result.replace(&placeholder, &replacement);
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_email_provider() {
        let provider = MockEmailProvider::new();

        let request = SendEmailRequest {
            from: "test@example.com".to_string(),
            to: "recipient@example.com".to_string(),
            cc: None,
            bcc: None,
            subject: "Test Email".to_string(),
            body_text: "This is a test".to_string(),
            body_html: None,
            metadata: None,
        };

        let response = provider.send_email(request.clone()).await.unwrap();
        assert_eq!(response.status, "sent");

        let sent_emails = provider.get_sent_emails().await;
        assert_eq!(sent_emails.len(), 1);
        assert_eq!(sent_emails[0].to, "recipient@example.com");
    }

    #[test]
    fn test_simple_template_engine() {
        let engine = SimpleTemplateEngine::new();

        let template = "Hello {{name}}, you have {{count}} messages.";
        let variables = serde_json::json!({
            "name": "John",
            "count": 5
        });

        let result = engine.render(template, &variables).unwrap();
        assert_eq!(result, "Hello John, you have 5 messages.");
    }

    #[test]
    fn test_template_engine_with_missing_variables() {
        let engine = SimpleTemplateEngine::new();

        let template = "Hello {{name}}, welcome to {{platform}}!";
        let variables = serde_json::json!({
            "name": "Jane"
        });

        let result = engine.render(template, &variables).unwrap();
        // Missing variables remain as placeholders
        assert_eq!(result, "Hello Jane, welcome to {{platform}}!");
    }
}
