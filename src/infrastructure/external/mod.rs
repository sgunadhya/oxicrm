use crate::application::ports::external::WebhookSender;
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug)]
pub struct MockWebhookSender;

#[async_trait]
impl WebhookSender for MockWebhookSender {
    async fn send(&self, url: &str, payload: Value) -> Result<(), String> {
        println!(
            "MockWebhookSender: Sending payload to {} -> {:?}",
            url, payload
        );
        Ok(())
    }
}
