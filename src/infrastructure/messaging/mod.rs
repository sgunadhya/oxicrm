use crate::application::ports::messaging::{DomainEvent, EventBus};
use async_trait::async_trait;
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct InMemoryEventBus {
    sender: broadcast::Sender<DomainEvent>,
}

impl InMemoryEventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }
}

#[async_trait]
impl EventBus for InMemoryEventBus {
    async fn publish(&self, event: &DomainEvent) -> Result<(), String> {
        self.sender.send(event.clone()).map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, _topic: &str) -> Result<broadcast::Receiver<DomainEvent>, String> {
        // Simple implementation subscribes to everything for now
        Ok(self.sender.subscribe())
    }
}
