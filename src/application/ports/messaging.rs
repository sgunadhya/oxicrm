use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DomainEvent {
    pub topic: String,
    pub payload: String,
}

#[async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: &DomainEvent) -> Result<(), String>;
    async fn subscribe(
        &self,
        topic: &str,
    ) -> Result<tokio::sync::broadcast::Receiver<DomainEvent>, String>;
}
