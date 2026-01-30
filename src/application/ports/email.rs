use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailRequest {
    pub from: String,
    pub to: String,
    pub cc: Option<Vec<String>>,
    pub bcc: Option<Vec<String>>,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailResponse {
    pub message_id: String,
    pub status: String,
    pub metadata: Option<serde_json::Value>,
}

#[async_trait]
pub trait EmailProvider: Send + Sync {
    async fn send_email(&self, request: SendEmailRequest) -> Result<SendEmailResponse, String>;
    async fn verify_configuration(&self) -> Result<bool, String>;
}

pub trait TemplateEngine: Send + Sync {
    fn render(&self, template: &str, variables: &serde_json::Value) -> Result<String, String>;
}
