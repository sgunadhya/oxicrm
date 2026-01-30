use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait WebhookSender: Send + Sync {
    async fn send(&self, url: &str, payload: Value) -> Result<(), String>;
}
